use bitcoin::key::{TapTweak, TweakedKeyPair, UntweakedKeyPair};
use bitcoin::secp256k1::{Secp256k1, XOnlyPublicKey};
use bitcoin::secp256k1::rand::{self};
use bitcoin::taproot::{ControlBlock, LeafVersion, TaprootBuilder};
use super::*;
use bitcoin::ScriptBuf;
use bitcoin::*;
use bitcoin::Address;
use anyhow::Result;
use bitcoin::address::NetworkUnchecked;
use serde::{Serialize, Deserialize}; // Import the necessary traits
use bincode::serialize;
use redb::{Database, TableDefinition};
use std::path::PathBuf;
// Import bincode for serialization

#[derive(Serialize, Deserialize)]
pub struct TaprootData {
    inscriptions: Vec<u8>,
    reveal_script: ScriptBuf,
    control_block: ControlBlock,
    taproot_address: String,
    key_pair: UntweakedKeyPair,
    recovery_keypair: TweakedKeyPair,
}
// TAPROOT_DATA_TABLE을 정의합니다.
const TAPROOT_DATA_TABLE: TableDefinition<&[u8], &[u8]> = TableDefinition::new("TAPROOT_DATA_TABLE");

#[derive(Serialize, Deserialize)] // Derive the Serialize and Deserialize traits
pub struct Output {
    pub addresses: Vec<Address<NetworkUnchecked>>,
}
#[derive(Debug, Parser)]
#[clap(group(
    ArgGroup::new("input")
    .required(true)
    .args(&["file"])
))]
pub struct CreateTR {
    #[command(flatten)]
    shared: SharedArgs, // 여러 명령어에서 공유하는 인수들을 포함하는 구조체
    #[arg(
        long,
        help = "Inscribe sat with contents of <FILE>. May be omitted if `--delegate` is supplied."
    )]
    pub(crate) file: Option<PathBuf>, // 인스크립션에 사용할 파일 경로
    #[arg(short, long, help = "Generate TapRoot <NUMBER> addresses.")]
    number: Option<u64>,
    #[arg(
        long,
        help = "Include JSON in file at <METADATA> converted to CBOR as inscription metadata",
        conflicts_with = "cbor_metadata"
    )]
    // conflicts_with = "cbor_metadata" // "cbor_metadata"와 충돌하므로 동시에 사용할 수 없음
    pub(crate) json_metadata: Option<PathBuf>, // JSON 형식의 메타데이터 파일 경로
    #[clap(long, help = "Set inscription metaprotocol to <METAPROTOCOL>.")]
    pub(crate) metaprotocol: Option<String>, // 인스크립션 메타프로토콜 설정
    #[clap(long, help = "Make inscription a child of <PARENT>.")]
    pub(crate) parent: Option<InscriptionId>, // 인스크립션의 부모 ID
    #[arg(
        long,
        help = "Include CBOR in file at <METADATA> as inscription metadata",
        conflicts_with = "json_metadata"
    )]
    // conflicts_with = "json_metadata" // "json_metadata"와 충돌하므로 동시에 사용할 수 없음
    pub(crate) cbor_metadata: Option<PathBuf>, // CBOR 형식의 메타데이터 파일 경로
    #[arg(long, help = "Delegate inscription content to <DELEGATE>.")]
    pub(crate) delegate: Option<InscriptionId>, // 인스크립션 내용을 대리할 대리자 ID
}

impl CreateTR {
    pub(crate) fn run(self, wallet: Wallet) -> SubcommandResult {
        let mut addresses: Vec<Address<NetworkUnchecked>> = Vec::new();
        println!("[/subcommand/wallet/create_tr.rs] Create TapRoot Address");
        println!("[/subcommand/wallet/create_tr.rs] file : {:?}",self.file);
        println!("[/subcommand/wallet/create_tr.rs] feeRate : {:?}",self.shared.fee_rate);
        
        let number_of_addresses = self.number.unwrap_or(1);
        println!("[create_tr] Number of addresses to generate: {}", number_of_addresses);

        for _ in 0..number_of_addresses {
            // Generate Taproot address
            match self.generate_taproot_address(&wallet) {
                Ok(address) => addresses.push(address),
                Err(e) => return Err(anyhow!("Failed to generate Taproot address: {}", e)),
            }
        }
        
        println!("[create_tr] All generated addresses: {:?}", addresses);

        Ok(Some(Box::new(Output { addresses })))
        //Ok(addresses)
    }
    // 메타데이터를 파싱하는 함수
    fn parse_metadata(cbor: Option<PathBuf>, json: Option<PathBuf>) -> crate::Result<Option<Vec<u8>>> {
        if let Some(path) = cbor {
            // CBOR 메타데이터 처리
            let cbor = fs::read(path)?;
            let _value: Value = ciborium::from_reader(Cursor::new(cbor.clone()))
                .context("failed to parse CBOR metadata")?;

            Ok(Some(cbor))
        } else if let Some(path) = json {
            // JSON 메타데이터 처리 및 CBOR 변환
            let value: serde_json::Value =
                serde_json::from_reader(fs::File::open(path)?).context("failed to parse JSON metadata")?;
            let mut cbor = Vec::new();
            ciborium::into_writer(&value, &mut cbor)?;

            Ok(Some(cbor))
        } else {
            // 메타데이터가 없을 경우
            Ok(None)
        }
    }

    fn generate_taproot_address(&self, wallet: &Wallet) -> Result<Address<NetworkUnchecked>> {
        // Taproot key pair generation
        let secp256k1 = Secp256k1::new();
        let key_pair = UntweakedKeyPair::new(&secp256k1, &mut rand::thread_rng());
        println!("[/subcommand/wallet/create_tr.rs] Generated new key pair with key_pair: {:?}", key_pair);
        let (public_key, _parity) = XOnlyPublicKey::from_keypair(&key_pair);
        println!("[/subcommand/wallet/create_tr.rs] Generated new key pair with public key: {:?}", public_key);
        
        
        let inscriptions = vec![Inscription::new(
            wallet.chain(), // 체인 설정
                self.shared.compress, // 압축 설정
            self.delegate, // 대리자 설정
            CreateTR::parse_metadata(self.cbor_metadata.clone(), self.json_metadata.clone())?, // 메타데이터 파싱
            self.metaprotocol.clone(), // 메타프로토콜 설정
            self.parent.into_iter().collect(), // 부모 설정
            self.file.clone(), // 파일 설정
            None,
            None,
        )?];

        println!("[/subcommand/wallet/create_tr.rs] inscriptions 데이터 : {:?}", inscriptions);
        
        // Create reveal script (without Inscription data) -> inscription data 받는 것으로 수정하기
        println!("[/batch/plan.rs] fn : create_batch_transactions / Creating reveal script for inscriptions");
        let reveal_script = Inscription::append_batch_reveal_script(
            &inscriptions,
            ScriptBuf::builder()
                .push_slice(public_key.serialize())
                .push_opcode(opcodes::all::OP_CHECKSIG),
        );
        
        //let reveal_script = ScriptBuf::builder().push_slice(public_key.serialize()).push_opcode(bitcoin::opcodes::all::OP_CHECKSIG).into_script();
        println!("[/subcommand/wallet/create_tr.rs] Reveal script created successfully");
        println!("[/subcommand/wallet/create_tr.rs] Reveal script : {:?}",reveal_script);

        // Setup Taproot spending information
        let taproot_spend_info = TaprootBuilder::new()
            .add_leaf(0, reveal_script.clone())
            .expect("adding leaf should work")
            .finalize(&secp256k1, public_key)
            .expect("finalizing taproot builder should work");
        println!("[/subcommand/wallet/create_tr.rs] Taproot spending information set up successfully");
        println!("[/subcommand/wallet/create_tr.rs] taproot_spend_info : {:?}",taproot_spend_info);

        // Generate control block
        let _control_block = taproot_spend_info
            .control_block(&(reveal_script.clone(), LeafVersion::TapScript))
            .expect("should compute control block");
        println!("[/subcommand/wallet/create_tr.rs] Control block generated successfully");
        println!("[/subcommand/wallet/create_tr.rs] _control_block : {:?}",_control_block);

        // Create Taproot address
        let taproot_address = Address::p2tr_tweaked(taproot_spend_info.output_key(), wallet.chain().network());
        println!("[/subcommand/wallet/create_tr.rs] Taproot address: {:?}", taproot_address);
        let taproot_address_str = taproot_address.to_string();
        let unchecked_taproot_address = Address::<NetworkUnchecked>::from_str(&taproot_address_str)?;
        println!("[/subcommand/wallet/create_tr.rs] unchecked_taproot_address: {:?}", unchecked_taproot_address);

        let recovery_key_pair = key_pair.tap_tweak(&secp256k1, taproot_spend_info.merkle_root());

        // TaprootData 구조체에 데이터 저장
        let taproot_data = TaprootData {
            inscriptions: serialize(&inscriptions).unwrap(),
            reveal_script: reveal_script,
            control_block: _control_block,
            taproot_address: taproot_address_str,
            key_pair: key_pair,
            recovery_keypair: recovery_key_pair,
        };

        // TaprootData를 Vec<u8>로 직렬화
        let serialized_data: Vec<u8> = serialize(&taproot_data)?;
        let serialized_data_slice: &[u8] = serialized_data.as_slice(); // Vec<u8>를 &[u8]로 변환

        // Database에 데이터 저장
        let db = Database::create("inscription.redb").unwrap();
        let mut write_txn = db.begin_write().unwrap();
        {
            // table을 열고 데이터를 삽입하는 동안에는 write_txn의 불변 참조를 사용합니다.
            let mut table = write_txn.open_table(TAPROOT_DATA_TABLE).unwrap();
            table.insert(taproot_data.taproot_address.as_bytes(), serialized_data_slice).unwrap();
        }

        write_txn.commit().unwrap();


        Ok(unchecked_taproot_address)
    }
    
    
}



