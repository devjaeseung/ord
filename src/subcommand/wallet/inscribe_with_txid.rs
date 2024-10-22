use bincode::deserialize;
use bitcoin::address::NetworkChecked;
use bitcoin::key::constants::SCHNORR_SIGNATURE_SIZE;
use bitcoin::key::{KeyPair, Secp256k1, TapTweak, TweakedKeyPair, TweakedPublicKey, UntweakedKeyPair, UntweakedPublicKey, XOnlyPublicKey};
use bitcoin::policy::MAX_STANDARD_TX_WEIGHT;
use bitcoin::{PrivateKey, secp256k1};
use bitcoin::sighash::{Prevouts, SighashCache, TapSighashType};
use bitcoin::taproot::{ControlBlock, LeafVersion, Signature, TapLeafHash};
use bitcoincore_rpc::bitcoincore_rpc_json::{GetRawTransactionResult,SignRawTransactionInput};
use bitcoincore_rpc::{json};
use redb::{Database, TableDefinition};
use std::collections::BTreeMap;
use bitcoin::{OutPoint, TxOut};
use bitcoin::blockdata::script::Script;
use crate::wallet::batch::{Mode, ParentInfo, RevealTransaction};
use super::*;

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

#[derive(Debug, Parser)]
#[clap(group(
    ArgGroup::new("input")
    .required(true)
    .multiple(true)
    .args(&["signed_commit_txid"])
))]
pub(crate) struct InscribeWithTxid {
    #[command(flatten)]
    shared: SharedArgs, // 여러 명령어에서 공유하는 인수들을 포함하는 구조체

    #[arg(
        long,
        help = "Include <AMOUNT> postage with inscription. [default: 10000sat]"
    )]
    pub(crate) postage: Option<Amount>, // 인스크립션에 포함할 우편 요금
    #[arg(long, help = "Send inscription to <DESTINATION>.")]
    pub(crate) destination: Option<Address<NetworkUnchecked>>,
    #[arg(long, help = "The txid of the commit transaction.")]
    pub(crate) taproot_address: String,
    #[arg(long, help = "The signed_commit_txid of the commit transaction.")]
    pub(crate) signed_commit_txid: String,
}

impl InscribeWithTxid {
    pub(crate) fn run(self, wallet: Wallet) -> SubcommandResult {
        println!("[/subcommand/wallet/inscribe_with_txid.rs] Running inscribe_with_txid command");
        println!("[/subcommand/wallet/inscribe_with_txid.rs] self.shared.fee_rate : {:?}",self.shared.fee_rate);

        //let utxos: &BTreeMap<OutPoint, TxOut>;
        let signed_commit_txid:Txid = self.signed_commit_txid.parse()?;

        let signed_commit_tx_result = wallet.bitcoin_client().get_raw_transaction(&signed_commit_txid, None);
        println!("[/subcommand/wallet/inscribe_with_txid.rs] [commitTx 조회] signed_commit_tx_result: {:?}", signed_commit_tx_result);
        let signed_commit_tx = signed_commit_tx_result.unwrap();
        println!("[/subcommand/wallet/inscribe_with_txid.rs] [commitTx 조회] signed_commit_tx: {:?}", signed_commit_tx);
        let commit_vout = signed_commit_tx.output.iter();
        println!("[/subcommand/wallet/inscribe_with_txid.rs] [commitTx 조회] commit_vout: {:?}", commit_vout);
        
        // Parse the String into an Address<NetworkChecked>
        let address_unchecked: Address<NetworkUnchecked> = self.taproot_address.parse()?;

        // Convert the Address<NetworkUnchecked> to Address<NetworkChecked>
        let address_checked: Address<NetworkChecked> = address_unchecked.assume_checked();

        // Create a Vec containing the checked address
        let address_vec: Vec<&Address<NetworkChecked>> = vec![&address_checked];

        // Convert Vec to a slice
        let address_slice: &[&Address<NetworkChecked>] = &address_vec[..];

        // Wrap the slice in an Option
        let address_option = Some(address_slice);

        println!("[/batch/plan.rs] fn inscribe / address_option : {:?}", address_option);

        // let utxos_json = wallet.bitcoin_client().list_unspent(None, None,address_option, None, None);
        // 
        // let utxos = Self::convert_to_btreemap(utxos_json);

        // let utxos_json = wallet.bitcoin_client().get_tx_out(&signed_commit_txid,0,None);

        // Fetch the transaction details by txid
        let transaction = wallet.bitcoin_client().get_raw_transaction_info(&signed_commit_txid.clone(), None)?;

        println!("[/batch/plan.rs] fn inscribe / transaction : {:?}", transaction);
        
        // Initialize the BTreeMap to store the UTXOs
        // let mut utxos: BTreeMap<OutPoint, TxOut> = BTreeMap::new();
        // 
        // // Iterate over the outputs of the transaction
        // for (vout, output) in transaction.vout.iter().enumerate() {
        //     // Create an OutPoint for each output
        //     let outpoint = OutPoint {
        //         txid: signed_commit_txid.clone(),
        //         vout: vout as u32,
        //     };
        // 
        //     // Insert the OutPoint and the corresponding TxOut into the BTreeMap
        //     utxos.insert(outpoint, output.clone());
        // }

        let utxos = Self::convert_to_utxo_map(transaction);
        
        println!("[/batch/plan.rs] fn inscribe / utxos : {:?}", utxos);

        println!("[/subcommand/wallet/inscribe_with_txid.rs] DB 불러오기 시작");
        // Database 열기
        let db = Database::open("inscription.redb").unwrap();

        println!("[/subcommand/wallet/inscribe_with_txid.rs] DB 불러옴");
        // 읽기 트랜잭션 시작
        let read_txn = db.begin_read().unwrap();

        // 테이블 열기
        let table = read_txn.open_table(TAPROOT_DATA_TABLE).unwrap();


        // 특정 키로 값 읽기 (예: taproot 주소로 데이터 검색)
        let key = self.taproot_address.clone(); // 이 값을 저장할 때 사용한 주소로 바꿔야 합니다.
        
        println!("[/subcommand/wallet/inscribe_with_txid.rs] DB 데이터 파싱 시작");
        if let Some(serialized_data_guard) = table.get(key.as_bytes()).unwrap() {
            println!("[/subcommand/wallet/inscribe_with_txid.rs] [DB조회 성공] taproot_address: {:?}", serialized_data_guard.value());
            // AccessGuard에서 &str 참조 추출
            let serialized_data: &[u8] = serialized_data_guard.value();

            // 직렬화된 데이터를 TaprootData 구조체로 복원
            let mut taproot_data: TaprootData = deserialize(serialized_data)?;

            // TaprootData 구조체 출력 (혹은 다른 작업)
            println!("[/subcommand/wallet/inscribe_with_txid.rs] [DB조회 성공] taproot_address: {:?}", taproot_data.taproot_address);
            println!("[/subcommand/wallet/inscribe_with_txid.rs] [DB조회 성공] inscriptions: {:?}", taproot_data.inscriptions);
            println!("[/subcommand/wallet/inscribe_with_txid.rs] [DB조회 성공] control_block: {:?}", taproot_data.control_block);
            println!("[/subcommand/wallet/inscribe_with_txid.rs] [DB조회 성공] reveal_script: {:?}", taproot_data.reveal_script);
            println!("[/subcommand/wallet/inscribe_with_txid.rs] [DB조회 성공] key_pair: {:?}", taproot_data.key_pair);
            println!("[/subcommand/wallet/inscribe_with_txid.rs] [DB조회 성공] recovery_keypair: {:?}", taproot_data.recovery_keypair);

            let result = self.create_batch_transactions(
                &wallet,self.taproot_address.clone(),
                utxos.clone(),
                &signed_commit_tx,
                signed_commit_txid,
                address_checked,
                taproot_data.inscriptions,
                taproot_data.control_block,
                taproot_data.reveal_script,
                taproot_data.key_pair,
                taproot_data.recovery_keypair,
            );
    
            println!("[/batch/plan.rs] fn inscribe / result123 : {:?}", result);

            if let Ok(reveal_transaction) = result {

                let result_keypair = taproot_data.recovery_keypair;
                println!("[/batch/plan.rs] fn inscribe / result_keypair : {:?}", result_keypair);
                let result_secret_key = result_keypair.to_inner().secret_key();
                println!("[/batch/plan.rs] fn inscribe / result_secret_key : {:?}", result_secret_key);
                let result_revealtx = reveal_transaction.reveal_tx;
                println!("[/batch/plan.rs] fn inscribe / result_revealtx : {:?}", result_revealtx);

                // recovery_key_pair로부터 프라이빗 키를 생성하고 이를 출력합니다.
                let result_private_key = PrivateKey::new(
                    result_secret_key,
                    wallet.chain().network(),
                );
                println!("[/batch/plan.rs] fn inscribe / result_private_key : {:?}", result_private_key);
                
                let result = wallet.bitcoin_client().sign_raw_transaction_with_wallet(
                    &result_revealtx,
                    Some(
                        &signed_commit_tx
                            .output
                            .iter()
                            .enumerate()
                            .map(|(vout, output)| SignRawTransactionInput {
                                txid: signed_commit_txid,
                                vout: vout.try_into().unwrap(),
                                script_pub_key: output.script_pubkey.clone(),
                                redeem_script: None,
                                amount: Some(Amount::from_sat(output.value)),
                            })
                            .collect::<Vec<SignRawTransactionInput>>(),
                    ),
                    None
                )?;

                println!("[/batch/plan.rs] fn inscribe / [서명완료] Signed reveal_tx result : {:?}", result);


                ensure!(
                  result.complete,
                  format!("Failed to sign reveal transaction: {:?}", result.errors)
                );

                let signed_reveal_tx = result.hex;
                println!("[/batch/plan.rs] fn inscribe / [서명완료] Signed reveal_tx.hex : {:?}", signed_reveal_tx);

                // 백업 및 트랜잭션 전송
                // no_backup 플래그가 설정되어 있지 않으면 복구 키를 백업합니다.
                // if !self.no_backup {
                //     Self::backup_recovery_key(wallet, recovery_key_pair)?;
                // }

                // reveal_tx를 전송하고, 성공 여부를 확인한 후 결과를 반환합니다.
                let reveal = match wallet
                    .bitcoin_client()
                    .send_raw_transaction(&signed_reveal_tx)
                {
                    Ok(txid) => txid,
                    Err(err) => {
                        return Err(anyhow!(
                "Failed to send reveal transaction: {err}\nCommit tx will be recovered once mined"
              ))
                    }
                };

                println!("[/batch/plan.rs] fn inscribe / Sent reveal_tx, txid : {:?}", reveal);

                let output = "inscription 완료";

                Ok(Some(Box::new(output)))

            } else {
                // Handle the error here

                let output = "inscription 실패";

                Ok(Some(Box::new(output)))
            }
        } else {

            let output = "데이터 불러오기 실패";

            Ok(Some(Box::new(output)))
        }
        
        // reveal_tx에 서명하고, 성공 여부를 확인한 후 결과를 signed_reveal_tx로 저장합니다.
        // let result = wallet.bitcoin_client().sign_raw_transaction_with_wallet(
        //     &reveal_result.unwrap().reveal_tx,
        //     Some(
        //         &signed_commit_tx
        //             .output
        //             .iter()
        //             .enumerate()
        //             .map(|(vout, output)| SignRawTransactionInput {
        //                 txid: signed_commit_txid,
        //                 vout: vout.try_into().unwrap(),
        //                 script_pub_key: output.script_pubkey.clone(),
        //                 redeem_script: None,
        //                 amount: Some(Amount::from_sat(output.value)),
        //             })
        //             .collect::<Vec<SignRawTransactionInput>>(),
        //     ),
        //     None,
        // )?;

    }

    pub(crate) fn create_batch_transactions(
        &self,
        wallet: &Wallet,
        taproot_address : String,
        mut utxos: BTreeMap<OutPoint, TxOut>,
        signed_commit_tx : &Transaction,
        signed_commit_txid : Txid,
        taproot_address_checked: Address<NetworkChecked>,
        inscriptions: Vec<u8>,
        control_block: ControlBlock,
        reveal_script: ScriptBuf,
        untweaked_key_pair: UntweakedKeyPair,
        recovery_key_pair: TweakedKeyPair
        
    ) -> Result<RevealTransaction> {

        println!("[/batch/plan.rs] fn : create_batch_transactions Start! ");
        println!("[/batch/plan.rs] fn : create_batch_transactions / wallet_inscriptions : {:?} ",inscriptions);
        println!("[/batch/plan.rs] fn : create_batch_transactions / chain : {:?} ",&wallet.chain());
        println!("[/batch/plan.rs] fn : create_batch_transactions / utxos : {:?} ",utxos);

        
        let postages = vec![self.postage.unwrap_or(crate::TARGET_POSTAGE)];

        // 수수료를 sat로 환산합니다.
        println!("[/subcommand/wallet/inscribe_with_txid.rs] fn : Calculating total postage");
        let total_postage = postages.iter().map(|amount| amount.to_sat()).sum();
        println!("[/subcommand/wallet/inscribe_with_txid.rs] fn : Total postage: {}", total_postage);

        let mut reveal_inputs = Vec::new();
        let mut reveal_outputs = Vec::new();
        let mode = batch::Mode::SeparateOutputs; // 모드 설정

        let reveal_fee_rate: FeeRate = 1.0.try_into().unwrap();

        let no_limit = false;

        // 목적지 주소를 설정
        println!("[/subcommand/wallet/inscribe_with_txid.rs] self.destination.clone(): {:?}", self.destination.clone());
        let destination = match self.destination.clone() {
            Some(destination) => destination.require_network(Network::Regtest)?, // 네트워크 요구사항 확인
            None => wallet.get_change_address()?, // 주소가 없는 경우 변경 주소를 가져옴
        };
        println!("[/subcommand/wallet/inscribe_with_txid.rs] Destination: {:?}", destination);

        let destinations = vec![destination];
        
        // inscription mode를 어떤 것인지 설정
        // match mode {
        //     Mode::SameSat => { // 여러 sat에 inscription할때 모두 동일한 sat를 공유할 수 있도록 합니다.
        //         println!("[/batch/plan.rs] fn : create_batch_transactions / Current Mode: {:?} ", mode);
        //         println!("[/batch/plan.rs] fn : create_batch_transactions / Postages Length: {}", postages.len());
        //         println!("[/batch/plan.rs] fn : create_batch_transactions / Destinations Length: {}", destinations.len());
        //         assert_eq!(
        //             postages.len(),
        //             1,
        //             "invariant: same-sat has only one postage"
        //         );
        //         assert_eq!(
        //             destinations.len(),
        //             1,
        //             "invariant: same-sat has only one destination"
        //         );
        //     }
        //     Mode::SeparateOutputs | Mode::SatPoints => {
        //         // SatPoints : inscription의 위치를 정밀하게 정할때 / 
        //         // SeparateOutpus : inscription을 별도의 각각 다른 output에 할당할때 사용
        //         println!("[/batch/plan.rs] fn : create_batch_transactions / Current Mode: {:?}", mode);
        //         println!("[/batch/plan.rs] fn : create_batch_transactions / Destinations Length: {}", destinations.len());
        //         println!("[/batch/plan.rs] fn : create_batch_transactions / Inscriptions Length: {}", inscriptions.len());
        //         println!("[/batch/plan.rs] fn : create_batch_transactions / Postages Length: {}", postages.len());
        //         assert_eq!(
        //             destinations.len(),
        //             inscriptions.len(),
        //             "invariant: destination addresses and number of inscriptions doesn't match"
        //         );
        //         assert_eq!(
        //             destinations.len(),
        //             postages.len(),
        //             "invariant: destination addresses and number of postages doesn't match"
        //         );
        //     }
        //     Mode::SharedOutput => { // 여러개의 inscription을 하나의 output에 공유할때 그룹화함
        //         println!("[/batch/plan.rs] fn : create_batch_transactions / Current Mode: {:?}", mode);
        //         println!("[/batch/plan.rs] fn : create_batch_transactions / Destinations Length: {}", destinations.len());
        //         println!("[/batch/plan.rs] fn : create_batch_transactions / Postages Length: {}", postages.len());
        //         println!("[/batch/plan.rs] fn : create_batch_transactions / Inscriptions Length: {}", inscriptions.len());
        //         assert_eq!(
        //             destinations.len(),
        //             1,
        //             "invariant: shared-output has only one destination"
        //         );
        //         assert_eq!(
        //             postages.len(),
        //             inscriptions.len(),
        //             "invariant: postages and number of inscriptions doesn't match"
        //         );
        //     }
        // }

        

        // reveal input에 null outpoint를 추가합니다.
        reveal_inputs.push(OutPoint::null());
        println!("[/subcommand/wallet/inscribe_with_txid.rs] fn : Added null outpoint to reveal inputs");

        // destination를 반복하여 각각에 대한 output을 생성합니다.
        // 각 output의 값은 현재 모드에 따라 결정됩니다.
        // SeparateOutputs 및 SatPoints 모드의 경우 해당 우편 요금 금액을 사용하고,
        // SharedOutput 및 SameSat 모드의 경우 총 우편 요금 금액을 사용합니다.
        println!("[/subcommand/wallet/inscribe_with_txid.rs] fn : Creating outputs for destinations");
        for (i, destination) in destinations.iter().enumerate() {
            reveal_outputs.push(TxOut {
                script_pubkey: destination.script_pubkey(),
                value: match mode {
                    Mode::SeparateOutputs | Mode::SatPoints => postages[i].to_sat(),
                    Mode::SharedOutput | Mode::SameSat => total_postage,
                },
            });
            println!("[/subcommand/wallet/inscribe_with_txid.rs] fn : Added output for destination {}: {:?}", i, destination);
        }

        let rune:Option<(Option<Address>,SpacedRune,Option<u32>)>;
        let premine= 0;
        let runestone:Option<Runestone>;
        let mode = batch::Mode::SeparateOutputs;
        let parent_info: Option<ParentInfo> = None;
        let reveal_satpoints: Vec<(SatPoint, TxOut)> = Vec::new();



        //premine = 0;
        rune = None;
        runestone = None;

        let secp256k1 = Secp256k1::new();
        // let key_pair = UntweakedKeyPair::new(&secp256k1, &mut rand::thread_rng());
        
        // Bitcoin Taproot 기능을 활용하여 트랜잭션을 생성하고 서명하는 과정을 다룹니다.
        // Taproot 키와 스크립트를 사용하여 트랜잭션을 생성하고, 해당 트랜잭션에 서명한 후 UTXO를 업데이트하고,
        // 최종적으로 트랜잭션 세부 정보를 반환합니다.

        //Calculate the commit input count
        //self.parent_info가 있는지와 self.reveal_satpoints의 길이를 합산하여 계산됩니다. 이는 commit 트랜잭션 입력의 수를 나타냅니다.
        //let commit_input = usize::from(self.parent_info.is_some()) + self.reveal_satpoints.len();
        //println!("[/subcommand/wallet/inscribe_with_txid.rs] fn : create_batch_transactions / Calculated commit input count: {}", commit_input);

        
        println!("[/batch/plan.rs] fn :  / build_reveal_transaction param &control_block : {:?}", &control_block.serialize());
        println!("[/batch/plan.rs] fn :  / build_reveal_transaction param self.reveal_fee_rate : {:?}", &reveal_fee_rate.clone());
        println!("[/batch/plan.rs] fn :  / build_reveal_transaction param reveal_outputs.clone() : {:?}", reveal_outputs.clone());
        println!("[/batch/plan.rs] fn :  / build_reveal_transaction param 4 reveal_inputs.clone() : {:?}", reveal_inputs.clone());
        println!("[/batch/plan.rs] fn :  / build_reveal_transaction param &reveal_script : {:?}", &reveal_script);
        println!("[/batch/plan.rs] fn :  / build_reveal_transaction param rune.is_some() : {:?}", rune.is_some());

        // Build the reveal transaction to estimate the fee
        // reveal 트랜잭션을 생성하고 수수료를 계산합니다.
        let (_reveal_tx, reveal_fee) = Self::build_reveal_transaction(
            0,
            &control_block.serialize(),
            reveal_fee_rate,
            reveal_outputs.clone(),
            reveal_inputs.clone(),
            &reveal_script,
            rune.is_some(),
        );

        println!("[/subcommand/wallet/inscribe_with_txid.rs] fn : create_batch_transactions / Built reveal transaction , _reveal_tx: {:?}", _reveal_tx);
        println!("[/subcommand/wallet/inscribe_with_txid.rs] fn : create_batch_transactions / Built reveal transaction to estimate fee, reveal_fee: {}", reveal_fee);
        // target_value는 reveal 수수료, 전체 postage 및 premine 값에 따라 결정됩니다.
        let mut target_value = reveal_fee;

        // Add total postage if mode is not SatPoints
        if mode != Mode::SatPoints {
            target_value += Amount::from_sat(total_postage);
        }

        // Add premine postage if premine is greater than 0
        if premine > 0 {
            target_value += TARGET_POSTAGE;
        }
        println!("[/subcommand/wallet/inscribe_with_txid.rs] fn : create_batch_transactions / Calculated target value: {}", target_value);


        let tap_root_addess_parse: Address = Address::from_str(&taproot_address.clone()).unwrap().require_network(Network::Regtest).unwrap();

        // Find the output for the commit transaction
        // commit 트랜잭션의 output을 찾습니다.
        let (vout, _commit_output) = signed_commit_tx
            .output
            .iter()
            .enumerate()
            .find(|(_vout, output)| output.script_pubkey == tap_root_addess_parse.script_pubkey())
            .expect("should find sat commit/inscription output");
        println!("[/batch/plan.rs] fn : create_batch_transactions / Found commit output: vout = {}", vout);


        println!("[/batch/plan.rs] fn : create_batch_transactions / Using provided signed commit transaction ID and vout");
        reveal_inputs[0] = OutPoint {
            txid: signed_commit_txid,  // Use the provided signed commit transaction ID
            vout: vout.try_into().unwrap(),         // Use the provided vout of the commit output
        };
        // reveal_inputs[0] = OutPoint {
        //     txid: signed_commit_txid,  // Use the provided signed commit transaction ID
        //     vout: commit_vout,         // Use the provided vout of the commit output
        // };
        println!("[/subcommand/wallet/inscribe_with_txid.rs] fn : create_batch_transactions / Updated reveal inputs with commit transaction output");

        // Build the final reveal transaction
        // 최종 reveal 트랜잭션을 빌드합니다.
        let (mut reveal_tx, _fee) = Self::build_reveal_transaction(
            0,
            &control_block.serialize(),
            reveal_fee_rate,
            reveal_outputs.clone(),
            reveal_inputs,
            &reveal_script,
            rune.is_some(),
        );
        println!("[/subcommand/wallet/inscribe_with_txid.rs] fn : create_batch_transactions / Built final reveal transaction");

        // Ensure outputs are not dust
        // 모든 output이 dust가 아닌지 확인합니다.
        for output in reveal_tx.output.iter() {
            ensure!(
                    output.value >= output.script_pubkey.dust_value().to_sat(),
                    "commit transaction output would be dust"
                  );
        }
        println!("[/subcommand/wallet/inscribe_with_txid.rs] fn : create_batch_transactions / Checked reveal transaction outputs for dust");

        // Prepare prevouts for sighash calculation
        // sighash 계산을 위해 prevouts를 준비합니다.
        let mut prevouts = Vec::new();

        if let Some(parent_info) = parent_info.clone() {
            prevouts.push(parent_info.tx_out);
        }

        if mode == Mode::SatPoints {
            for (_satpoint, txout) in reveal_satpoints.iter() {
                prevouts.push(txout.clone());
            }
        }

        // prevouts.push(unsigned_commit_tx.output[vout].clone());
        // prevouts.push(reveal_tx.output[commit_vout as usize].clone());
        prevouts.push(signed_commit_tx.output[vout].clone());
        println!("[/subcommand/wallet/inscribe_with_txid.rs] fn : create_batch_transactions / Prepared prevouts for sighash calculation");

        // Create sighash for the reveal transaction
        // reveal 트랜잭션에 대한 sighash를 계산합니다.
        let mut sighash_cache = SighashCache::new(&mut reveal_tx);
        println!("[/subcommand/wallet/inscribe_with_txid.rs] fn : create_batch_transactions / sighash_cache {:?}",sighash_cache);

        println!("[/subcommand/wallet/inscribe_with_txid.rs] fn : create_batch_transactions / sighash_hash 만들기전 Parm 리스트 값들");
        println!("[/subcommand/wallet/inscribe_with_txid.rs] fn : create_batch_transactions / &prevouts {:?}",&prevouts);
        println!("[/subcommand/wallet/inscribe_with_txid.rs] fn : create_batch_transactions / &reveal_script {:?}",&reveal_script);
        
        let sighash = sighash_cache
            .taproot_script_spend_signature_hash(
                0,
                &Prevouts::All(&prevouts),
                TapLeafHash::from_script(&reveal_script, LeafVersion::TapScript),
                TapSighashType::Default,
            )
            .expect("signature hash should compute");
        println!("[/subcommand/wallet/inscribe_with_txid.rs] fn : create_batch_transactions / Calculated sighash for reveal transaction");
        println!("[/subcommand/wallet/inscribe_with_txid.rs] fn : create_batch_transactions / sighash {:?}",&sighash);

        // Sign the reveal transaction
        // reveal 트랜잭션에 서명합니다.
        let sig = secp256k1.sign_schnorr(
            &secp256k1::Message::from_slice(sighash.as_ref())
                .expect("should be cryptographically secure hash"),
            &untweaked_key_pair,
        );
        println!("[/subcommand/wallet/inscribe_with_txid.rs] fn : create_batch_transactions / Signed reveal transaction sig : {:?}",sig);

        // Update witness for the reveal transaction
        // reveal 트랜잭션의 witness를 업데이트합니다.
        let witness = sighash_cache
            .witness_mut(0)
            .expect("getting mutable witness reference should work");
        println!("[/subcommand/wallet/inscribe_with_txid.rs] fn : create_batch_transactions / reveal 트랜잭션의 witness를 업데이트1 : {:?}",witness);
        
        witness.push(
            Signature {
                sig,
                hash_ty: TapSighashType::Default,
            }
                .to_vec(),
        );

        println!("[/subcommand/wallet/inscribe_with_txid.rs] fn : create_batch_transactions / reveal 트랜잭션의 witness를 업데이트2 : {:?}",witness);
        witness.push(reveal_script);
        println!("[/subcommand/wallet/inscribe_with_txid.rs] fn : create_batch_transactions / reveal 트랜잭션의 witness를 업데이트3 : {:?}",witness);
        witness.push(&control_block.serialize());
        println!("[/subcommand/wallet/inscribe_with_txid.rs] fn : create_batch_transactions / reveal 트랜잭션의 witness를 업데이트4 : {:?}",witness);

        // Validate the recovery key pair
        // recovery 키 쌍을 검증합니다.
        // let recovery_key_pair = recovery_key_pair.tap_tweak(&secp256k1, taproot_spend_info.merkle_root());
       
        let (x_only_pub_key, _parity) = recovery_key_pair.to_inner().x_only_public_key();
        assert_eq!(
            Address::p2tr_tweaked(
                TweakedPublicKey::dangerous_assume_tweaked(x_only_pub_key),
                wallet.chain().network(),
            ),
            taproot_address_checked
        );
        println!("[/batch/plan.rs] fn : create_batch_transactions / Validated recovery key pair");


        // Check the reveal transaction weight
        // reveal 트랜잭션의 무게를 확인합니다.
        let reveal_weight = reveal_tx.weight();

        if !no_limit && reveal_weight > bitcoin::Weight::from_wu(MAX_STANDARD_TX_WEIGHT.into()) {
            bail!(
                    "reveal transaction weight greater than {MAX_STANDARD_TX_WEIGHT} (MAX_STANDARD_TX_WEIGHT): {reveal_weight}"
                  );
        }

        println!("[/subcommand/wallet/inscribe_with_txid.rs] fn : create_batch_transactions / Checked reveal transaction weight: {}", reveal_weight);

        // Update UTXOs with the reveal transaction
        // reveal 트랜잭션으로 UTXO를 업데이트합니다.
        utxos.insert(
            reveal_tx.input[0].previous_output,
            signed_commit_tx.output[reveal_tx.input[0].previous_output.vout as usize]
                .clone(),
        );
        println!("[/subcommand/wallet/inscribe_with_txid.rs] fn : create_batch_transactions / Updated UTXOs with reveal transaction");

        // Calculate total fees
        // commit 및 reveal 트랜잭션의 수수료를 계산합니다.
        //let total_fees = Self::calculate_fee(&signed_commit_tx, &utxos) + Self::calculate_fee(&reveal_tx, &utxos);
        let total_fees = Self::calculate_fee(&reveal_tx, &utxos);
        println!("[/subcommand/wallet/inscribe_with_txid.rs] fn : create_batch_transactions / Calculated total fees: {}", total_fees);

        println!("[/subcommand/wallet/inscribe_with_txid.rs] fn : create_batch_transactions / 조립 완료 !!!");
        Ok(
            RevealTransaction{
                reveal_tx: reveal_tx,
                key_pair:untweaked_key_pair,
            }
        )
        
    }


    // 트랜잭션을 생성하고 수수료를 계산하는 함수
    fn build_reveal_transaction(
        commit_input_index: usize, // commit_input_index: 커밋 입력 인덱스 (usize 타입).
        control_block: &Vec<u8>, // control_block: ControlBlock에 대한 참조.
        fee_rate: FeeRate, // fee_rate: 수수료율 (FeeRate 타입).
        output: Vec<TxOut>, // output: 출력 벡터 (Vec<TxOut> 타입).
        input: Vec<OutPoint>, // input: 입력 벡터 (Vec<OutPoint> 타입).
        script: &Script, // script: 스크립트에 대한 참조 (&Script 타입)
        etching: bool, // etching: 부울 값 (bool 타입), 트랜잭션의 특성을 결정.
    ) -> (Transaction, Amount) {

        println!("[/subcommand/wallet/inscribe_with_txid.rs] fn build_reveal_transaction / 트랜잭션 생성");

        let reveal_tx = Transaction {
            input: input
                .into_iter()
                .map(|previous_output| TxIn {
                    previous_output,
                    script_sig: script::Builder::new().into_script(),
                    witness: Witness::new(),
                    sequence: if etching {
                        Sequence::from_height(Runestone::COMMIT_CONFIRMATIONS - 1)
                    } else {
                        Sequence::ENABLE_RBF_NO_LOCKTIME
                    },
                })
                .collect(),
            output,
            lock_time: LockTime::ZERO,
            version: 2,
        };

        println!("[/subcommand/wallet/inscribe_with_txid.rs] fn build_reveal_transaction / Initial reveal_tx: {:?}", reveal_tx);

        // 수수료 계산을 위한 트랜잭션 복사 및 수정
        let fee = {
            let mut reveal_tx = reveal_tx.clone();
            println!("[/subcommand/wallet/inscribe_with_txid.rs] fn build_reveal_transaction / Cloned reveal_tx for fee calculation: {:?}", reveal_tx);

            for (current_index, txin) in reveal_tx.input.iter_mut().enumerate() {
                // add dummy inscription witness for reveal input/commit output
                // 각 입력의 witness 필드를 업데이트합니다.
                // commit_input_index와 일치하는 입력에 대해서는 witness에 서명, 스크립트, 컨트롤 블록을 추가합니다.
                // 다른 입력에 대해서는 더미 서명을 추가합니다.
                if current_index == commit_input_index {
                    txin.witness.push(
                        Signature::from_slice(&[0; SCHNORR_SIGNATURE_SIZE])
                            .unwrap()
                            .to_vec(),
                    );
                    txin.witness.push(script);
                    txin.witness.push(&control_block);
                } else {
                    txin.witness = Witness::from_slice(&[&[0; SCHNORR_SIGNATURE_SIZE]]);
                }
                println!("[/subcommand/wallet/inscribe_with_txid.rs] fn build_reveal_transaction / Updated txin at index {}: {:?}", current_index, txin);
            }

            fee_rate.fee(reveal_tx.vsize())
        };

        println!("[/subcommand/wallet/inscribe_with_txid.rs] fn build_reveal_transaction / Calculated fee: {:?}", fee);

        (reveal_tx, fee)
    }

    fn calculate_fee(tx: &Transaction, utxos: &BTreeMap<OutPoint, TxOut>) -> u64 {
        // tx: 수수료를 계산할 트랜잭션에 대한 참조 (&Transaction 타입).
        // utxos: 사용 가능한 UTXO(Unspent Transaction Outputs)의 맵 (&BTreeMap<OutPoint, TxOut> 타입).
        println!("[/subcommand/wallet/inscribe_with_txid.rs] fn calculate_fee / called with:");
        println!("[/subcommand/wallet/inscribe_with_txid.rs] fn calculate_fee /   tx: {:?}", tx);
        println!("[/subcommand/wallet/inscribe_with_txid.rs] fn calculate_fee /   utxos: {:?}", utxos);
        // tx.input
        //   .iter()
        //   .map(|txin| utxos.get(&txin.previous_output).unwrap().value)
        //   .sum::<u64>()
        //   .checked_sub(tx.output.iter().map(|txout| txout.value).sum::<u64>())
        //   .unwrap()
        
        // let input_sum: u64 = tx.input
        //     .iter()
        //     .map(|txin| {
        //         let value = utxos.get(&txin.previous_output).unwrap().value;
        //         println!("[/subcommand/wallet/inscribe_with_txid.rs] fn calculate_fee / Input value for {:?}: {}", txin.previous_output, value);
        //         value
        //     })
        //     .sum();

        let input_sum: u64 = tx.input
            .iter()
            .map(|txin| {
                utxos.get(&txin.previous_output)
                    .map_or_else(
                        || {
                            println!("[/subcommand/wallet/inscribe_with_txid.rs] fn calculate_fee / Missing UTXO for {:?}", txin.previous_output);
                            0 // or handle the error appropriately
                        },
                        |txout| {
                            let value = txout.value;
                            println!("[/subcommand/wallet/inscribe_with_txid.rs] fn calculate_fee / Input value for {:?}: {}", txin.previous_output, value);
                            value
                        }
                    )
            })
            .sum();

        println!("[/subcommand/wallet/inscribe_with_txid.rs] fn calculate_fee / Total input value: {}", input_sum);

        let output_sum: u64 = tx.output.iter().map(|txout| {
            let value = txout.value;
            println!("[/subcommand/wallet/inscribe_with_txid.rs] fn calculate_fee / Output value: {}", value);
            value
        }).sum();

        println!("[/subcommand/wallet/inscribe_with_txid.rs] fn calculate_fee / Total output value: {}", output_sum);

        //let fee = input_sum.checked_sub(output_sum).unwrap();
        let fee = input_sum.checked_sub(output_sum).unwrap_or_else(|| {
            println!("Error: input_sum is less than output_sum, resulting in underflow.");
            0 // or handle the error as needed
        });
        println!("[/subcommand/wallet/inscribe_with_txid.rs] fn calculate_fee / Calculated fee: {}", fee);

        fee
    }

    // Result<Option<GetTxResult>> 타입을 BTreeSet<OutPoint>로 변환하는 함수
    // fn result_to_btree_set(result: Result<Option<GetTxOutResult>>) -> BTreeSet<OutPoint> {
    //     let mut outpoints = BTreeSet::new(); // 빈 BTreeSet을 생성합니다.
    // 
    //     if let Ok(Some(tx_result)) = result { // Result와 Option을 풀어서 값이 있는지 확인합니다.
    //         // 트랜잭션의 OutPoint를 생성하고 BTreeSet에 삽입합니다.
    //         let outpoint = OutPoint::new(tx_result.txid, tx_result.vout);
    //         outpoints.insert(outpoint); // BTreeSet에 OutPoint를 추가합니다.
    //     }
    // 
    //     outpoints // BTreeSet을 반환합니다.
    // }


    // fn result_to_btree_set(result: Result<Option<GetTxOutResult>>, txid: Txid) -> BTreeSet<OutPoint> {
    //     let mut outpoints = BTreeSet::new(); // 빈 BTreeSet을 생성합니다.
    // 
    //     if let Ok(Some(tx_out_result)) = result { // Result와 Option을 풀어서 값이 있는지 확인합니다.
    //         // 트랜잭션의 OutPoint를 생성하고 BTreeSet에 삽입합니다.
    //         let outpoint = OutPoint::new(txid, tx_out_result.n); // txid와 vout 인덱스(n)을 사용하여 OutPoint 생성
    //         outpoints.insert(outpoint); // BTreeSet에 OutPoint를 추가합니다.
    //     }
    // 
    //     outpoints // BTreeSet을 반환합니다.
    // }
    
    // Assuming you have a function or a method to get the `Result<Vec<json::ListUnspentResultEntry>>`
    fn convert_to_btreemap(result: bitcoincore_rpc::Result<Vec<json::ListUnspentResultEntry>>) -> BTreeMap<OutPoint, TxOut> {
    
        // Initialize an empty BTreeMap
        let mut utxos: BTreeMap<OutPoint, TxOut> = BTreeMap::new();
        
        // Check if the result is Ok or Err
        let entries = match result {
            Ok(entries) => entries,
            Err(e) => return utxos, // Return the error if result is Err
        };
        
    
        // Iterate over the entries
        for entry in entries {
            // Convert the ListUnspentResultEntry to OutPoint
            let outpoint = OutPoint {
                txid: entry.txid, // Use the txid from the entry
                vout: entry.vout, // Use the vout from the entry
            };
    
            // Convert the ListUnspentResultEntry to TxOut
            let txout = TxOut {
                value: entry.amount.to_sat(), // Convert the Amount to satoshis
                script_pubkey: entry.script_pub_key.clone(), // Clone the script_pub_key from the entry
            };
    
            // Insert the OutPoint and TxOut into the BTreeMap
            utxos.insert(outpoint, txout);
        }
    
        // Return the populated BTreeMap
        utxos
    }

    // fn get_utxos_by_txid(
    //     txid: Txid,
    // ) -> Result<BTreeMap<OutPoint, TxOut>, Box<dyn std::error::Error>> {
    //     // Fetch the transaction details by txid
    //     let transaction = rpc_client.get_raw_transaction_info(&txid, None)?;
    // 
    //     // Initialize the BTreeMap to store the UTXOs
    //     let mut utxos: BTreeMap<OutPoint, TxOut> = BTreeMap::new();
    // 
    //     // Iterate over the outputs of the transaction
    //     for (vout, output) in transaction.vout.iter().enumerate() {
    //         // Create an OutPoint for each output
    //         let outpoint = OutPoint {
    //             txid,
    //             vout: vout as u32,
    //         };
    // 
    //         // Insert the OutPoint and the corresponding TxOut into the BTreeMap
    //         utxos.insert(outpoint, output.clone());
    //     }
    // 
    //     Ok(utxos)
    // }

    fn convert_to_utxo_map(
        tx_result: GetRawTransactionResult,
    ) -> BTreeMap<OutPoint, TxOut> {
        // Create a BTreeMap to hold the UTXOs
        let mut utxo_map: BTreeMap<OutPoint, TxOut> = BTreeMap::new();

        // Ensure the Result is successful and destructure to obtain the GetRawTransactionResult
        //let tx_result = tx_result?;

        // Iterate over the vout array in the transaction result
        for (vout_index, vout) in tx_result.vout.iter().enumerate() {
            // Create an OutPoint from the txid and the index of the vout
            let outpoint = OutPoint {
                txid: tx_result.txid, // Txid from the transaction
                vout: 0, // The index in the vout array
            };

            // Create a TxOut from the vout entry
            let txout = TxOut {
                value: vout.value.to_sat(), // Value in satoshis
                script_pubkey: vout.script_pub_key.clone().script().unwrap(), // The scriptPubKey
            };

            // Insert the OutPoint and TxOut into the BTreeMap
            utxo_map.insert(outpoint, txout);
        }

        // Return the populated BTreeMap
        utxo_map
    }

}

// if let Some(serialized_data_guard) = table.get(key.as_bytes()).unwrap() {
// Ok(Transactions {
//     commit_tx: unsigned_commit_tx,
//     commit_vout: vout,
//     recovery_key_pair,
//     reveal_tx,
//     rune,
//     total_fees,
// })

// println!("데이터를 찾을 수 없습니다.");

// let output = "inscription 성공";

// Ok(Some(Box::new(output)))
// } else {
// println!("데이터를 찾을 수 없습니다.");

// let output = "inscription 실패";
// 
// Ok(Some(Box::new(output)))

// let trasnaction_void: Transaction = Transaction {
// version: 0,
// lock_time: LockTime::ZERO,
// input: vec![],
// output: vec![],
// };
// 
// let secp256k1 = Secp256k1::new();
// 
// let empty_keypair : UntweakedKeyPair = UntweakedKeyPair::new(&secp256k1,&mut rand::thread_rng());
// let empty_revealscript : ScriptBuf = ScriptBuf::new();
// 
// let (public_key, _parity) = XOnlyPublicKey::from_keypair(&empty_keypair);
// 
// let taproot_spend_info = TaprootBuilder::new().add_leaf(0, empty_revealscript.clone())
// .expect("adding leaf should work")
// .finalize(&secp256k1, public_key)
// .expect("finalizing taproot builder should work");
// 
// let empty_recovery_keypair : TweakedKeyPair = empty_keypair.tap_tweak(&secp256k1, taproot_spend_info.merkle_root());
// 
// let empty_reveal_script : ScriptBuf = ScriptBuf::new();
// 
// Ok(
// RevealTransaction{
// reveal_tx: trasnaction_void,
// key_pair: empty_keypair,
// recovert_key_pair:empty_recovery_keypair,
// }
// )
// }
