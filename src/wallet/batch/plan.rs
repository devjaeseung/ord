use super::*;

pub struct Plan {
  pub(crate) commit_fee_rate: FeeRate, // 커밋 트랜잭션에 대한 수수료 비율입니다. 이 값은 트랜잭션의 빠른 처리를 보장하기 위해 네트워크에 지불할 수수료를 결정합니다.
  pub(crate) destinations: Vec<Address>, //  트랜잭션의 목적지 주소 목록입니다. 이 주소들은 트랜잭션의 출력으로, 각 주소는 비트코인을 수신하게 됩니다.
  pub(crate) dry_run: bool, // 드라이 런 모드 플래그입니다. true로 설정되면 실제로 트랜잭션을 전송하지 않고 시뮬레이션만 수행합니다.
  pub(crate) etching: Option<Etching>, // 인스크립션에 대한 추가 정보를 포함하는 선택적 필드입니다. 이 값이 설정되면, 인스크립션과 관련된 추가 데이터가 포함됩니다.
  pub(crate) inscriptions: Vec<Inscription>, // 트랜잭션에 포함될 인스크립션 목록입니다. 인스크립션은 비트코인 트랜잭션에 추가되는 사용자 정의 데이터입니다.
  pub(crate) mode: Mode, // 트랜잭션의 모드를 지정합니다. 모드는 트랜잭션의 처리 방법에 영향을 미칠 수 있는 다양한 설정을 포함합니다.
  pub(crate) no_backup: bool, // 백업 방지 플래그입니다. true로 설정되면 트랜잭션 데이터의 백업이 생성되지 않습니다.
  pub(crate) no_limit: bool, // 제한 없음 플래그입니다. true로 설정되면 트랜잭션에 적용되는 표준 제한을 무시하고 처리합니다.
  pub(crate) parent_info: Option<ParentInfo>, // 부모 트랜잭션에 대한 정보를 포함하는 선택적 필드입니다. 이 값이 설정되면, 부모 트랜잭션과 관련된 추가 데이터가 포함됩니다.
  pub(crate) postages: Vec<Amount>, // 트랜잭션에 대한 우편 요금 목록입니다. 각 요금은 트랜잭션의 출력으로, 비트코인 금액을 나타냅니다.
  pub(crate) reinscribe: bool, // 다시 인스크립션할지 여부를 나타내는 플래그입니다. true로 설정되면 기존 인스크립션을 다시 작성합니다.
  pub(crate) reveal_fee_rate: FeeRate, // 리빌 트랜잭션에 대한 수수료 비율입니다. 이 값은 리빌 트랜잭션의 빠른 처리를 보장하기 위해 네트워크에 지불할 수수료를 결정합니다.
  pub(crate) reveal_satpoints: Vec<(SatPoint, TxOut)>, // 리빌할 좌표와 해당 출력의 목록입니다. 각 좌표는 비트코인 블록체인의 특정 위치를 나타냅니다.
  pub(crate) satpoint: Option<SatPoint>, // 사토시 좌표를 나타내는 선택적 필드입니다. 이 값이 설정되면, 트랜잭션의 특정 위치를 가리킵니다.
}

impl Default for Plan {
  fn default() -> Self {
    Self {
      commit_fee_rate: 1.0.try_into().unwrap(), // 커밋 트랜잭션에 대한 수수료 비율을 나타냅니다. 기본 값은 1.0입니다. try_into 메서드를 사용하여 타입 변환을 시도하고, 변환이 실패하면 unwrap을 통해 패닉이 발생합니다.
      destinations: Vec::new(), // 트랜잭션의 목적지 주소를 저장하는 벡터입니다. 기본 값은 빈 벡터입니다.
      dry_run: false, // 드라이 런 모드 플래그입니다. 기본 값은 false로 설정됩니다.
      etching: None, // 런(Etching)에 대한 정보입니다. 기본 값은 None입니다.
      inscriptions: Vec::new(), // 트랜잭션에 포함될 인스크립션(문구) 목록입니다. 기본 값은 빈 벡터입니다.
      mode: Mode::SharedOutput, // 트랜잭션 모드 설정입니다. 기본 값은 Mode::SharedOutput입니다.
      no_backup: false, // 백업 방지 플래그입니다. 기본 값은 false로 설정됩니다.
      no_limit: false, // 제한 없음 플래그입니다. 기본 값은 false로 설정됩니다.
      parent_info: None, // 부모 트랜잭션 정보입니다. 기본 값은 None입니다.
      postages: vec![Amount::from_sat(10_000)], // 트랜잭션에 대한 우편 요금입니다. 기본 값은 10,000 사토시로 설정된 우편 요금을 포함하는 벡터입니다.
      reinscribe: false, // 다시 인스크립션할지 여부를 나타내는 플래그입니다. 기본 값은 false로 설정됩니다.
      reveal_fee_rate: 1.0.try_into().unwrap(), // 리빌 트랜잭션에 대한 수수료 비율을 나타냅니다. 기본 값은 1.0입니다.
      reveal_satpoints: Vec::new(), // 리빌할 좌표 목록입니다. 기본 값은 빈 벡터입니다.
      satpoint: None, // 사토시 좌표를 나타냅니다. 기본 값은 None입니다.
    }
  }
}

impl Plan {
  
  // input을 사용하여 Bitcoin 트랜잭션을 생성하고 서명하며, 
  // 필요할 경우 해당 트랜잭션을 네트워크에 전송하여 실행하는 역할을 합니다. 
  // 전체적으로 트랜잭션을 생성하고, 서명하고, 전송하는 과정을 포함하고 있습니다. 
  pub(crate) fn inscribe(
    &self,
    locked_utxos: &BTreeSet<OutPoint>, // 잠긴 상태의 UTXO(OutPoint)의 집합입니다. 잠긴 UTXO는 다른 트랜잭션에서 사용할 수 없습니다.
    runic_utxos: BTreeSet<OutPoint>, // 특정 런(Rune)과 관련된 UTXO의 집합입니다.
    utxos: &BTreeMap<OutPoint, TxOut>, // 트랜잭션 출력의 매핑으로, 각 OutPoint와 관련된 TxOut(트랜잭션 출력)을 나타냅니다.
    wallet: &Wallet, // 사용자의 지갑 정보를 포함하는 구조체입니다. 지갑은 트랜잭션을 생성하고 서명하는 데 필요한 정보를 포함합니다.
  ) -> SubcommandResult {
    println!("[/batch/plan.rs] fn inscribe : inscribe ");
    println!("[/batch/plan.rs] fn inscribe : inscribe / locked_utxos : {:?}",locked_utxos);
    println!("[/batch/plan.rs] fn inscribe : inscribe / runic_utxos : {:?}",runic_utxos);
    println!("[/batch/plan.rs] fn inscribe : inscribe / utxos : {:?}",utxos);

    
    // create_batch_transactions 함수를 호출하여 커밋 트랜잭션과 리빌 트랜잭션을 생성합니다.
    // 이 함수는 생성된 트랜잭션 및 관련 정보를 포함하는 Transactions 구조체를 반환합니다.
    let Transactions {
      commit_tx, // 커밋 트랜잭션 객체. 인스크립션(Inscription)을 포함한 첫 번째 트랜잭션입니다.
      commit_vout, // 커밋 트랜잭션의 출력 인덱스. 커밋 트랜잭션 내에서 특정 출력 위치를 가리킵니다.
      reveal_tx, // 리빌 트랜잭션 객체. 인스크립션을 공개하는 트랜잭션입니다.
      recovery_key_pair, // 트랜잭션 복구를 위한 키 쌍. 리빌 트랜잭션을 서명하는 데 사용됩니다.
      total_fees, // 트랜잭션 수수료의 총합.
      rune, // 생성된 룬(Rune) 정보.
    } = self.create_batch_transactions(
      wallet.inscriptions().clone(), // 지갑에 저장된 인스크립션 목록.
      wallet.chain(), // 지갑의 체인 정보. 
      locked_utxos.clone(), // 잠긴 UTXO(Unspent Transaction Outputs) 목록.
      runic_utxos, // 룬과 관련된 UTXO 목록.
      utxos.clone(), // 사용 가능한 UTXO 목록.
      [wallet.get_change_address()?, wallet.get_change_address()?], // 잔돈 주소 배열.
      wallet.get_change_address()?, // 잔돈 주소.
    )?;

    println!("[/batch/plan.rs] fn inscribe / Created batch transactions Transactions : ");
    println!("[/batch/plan.rs] fn inscribe / commit_tx : {:?}", commit_tx);
    println!("[/batch/plan.rs] fn inscribe / commit_vout : {:?}", commit_vout);
    println!("[/batch/plan.rs] fn inscribe / reveal_tx : {:?}", reveal_tx);
    println!("[/batch/plan.rs] fn inscribe / total_fees : {:?}", total_fees);
    println!("[/batch/plan.rs] fn inscribe / rune : {:?}", rune);

    // 만약 dry_run 플래그가 설정되어 있으면, 
    // 실제로 트랜잭션을 전송하지 않고 PSBT(Partially Signed Bitcoin Transaction)를 생성하여 반환
    if self.dry_run {
      println!("[/batch/plan.rs] fn inscribe Dry run mode enabled");
      let commit_psbt = wallet
        .bitcoin_client()
        .wallet_process_psbt(
          &base64::engine::general_purpose::STANDARD
            .encode(Psbt::from_unsigned_tx(Self::remove_witnesses(commit_tx.clone()))?.serialize()),
          Some(false),
          None,
          None,
        )?
        .psbt;

      println!("[/batch/plan.rs] fn inscribe / commit_psbt : {:?}", commit_psbt);

      let reveal_psbt = Psbt::from_unsigned_tx(Self::remove_witnesses(reveal_tx.clone()))?;

      println!("[/batch/plan.rs] fn inscribe / reveal_psbt : {:?}", reveal_psbt);
      
      return Ok(Some(Box::new(self.output(
        commit_tx.txid(),
        Some(commit_psbt),
        reveal_tx.txid(),
        false,
        Some(base64::engine::general_purpose::STANDARD.encode(reveal_psbt.serialize())),
        total_fees,
        self.inscriptions.clone(),
        rune,
      ))));
    }

    println!("[/batch/plan.rs] fn inscribe / Signing commit_tx");
    
    // commit_tx에 서명하고 결과를 signed_commit_tx로 저장합니다.
    let signed_commit_tx = wallet
      .bitcoin_client()
      .sign_raw_transaction_with_wallet(&commit_tx, None, None)?
      .hex;

    println!("[/batch/plan.rs] fn inscribe / Signed commit_tx : {:?}", signed_commit_tx);
    
    // reveal_tx에 서명하고, 성공 여부를 확인한 후 결과를 signed_reveal_tx로 저장합니다.
    let result = wallet.bitcoin_client().sign_raw_transaction_with_wallet(
      &reveal_tx,
      Some(
        &commit_tx
          .output
          .iter()
          .enumerate()
          .map(|(vout, output)| SignRawTransactionInput {
            txid: commit_tx.txid(),
            vout: vout.try_into().unwrap(),
            script_pub_key: output.script_pubkey.clone(),
            redeem_script: None,
            amount: Some(Amount::from_sat(output.value)),
          })
          .collect::<Vec<SignRawTransactionInput>>(),
      ),
      None,
    )?;

    println!("[/batch/plan.rs] fn inscribe / Signed reveal_tx result : {:?}", result);


    ensure!(
      result.complete,
      format!("Failed to sign reveal transaction: {:?}", result.errors)
    );

    let signed_reveal_tx = result.hex;
    println!("[/batch/plan.rs] fn inscribe / Signed reveal_tx.hex : {:?}", signed_reveal_tx);

    // 백업 및 트랜잭션 전송
    // no_backup 플래그가 설정되어 있지 않으면 복구 키를 백업합니다.
    if !self.no_backup {
      Self::backup_recovery_key(wallet, recovery_key_pair)?;
    }

    println!("[/batch/plan.rs] fn inscribe / Sending commit_tx");
    // commit_tx를 전송하고 commit_txid를 저장합니다.
    let commit_txid = wallet
      .bitcoin_client()
      .send_raw_transaction(&signed_commit_tx)?;

    println!("[/batch/plan.rs] fn inscribe / Sent commit_tx, txid : {:?}", commit_txid);

    
    if let Some(ref rune_info) = rune {
      // 만약 rune이 존재하면 commit 트랜잭션의 출력 중 하나를 잠금 설정하고, commit 및 reveal 트랜잭션을 저장합니다.
      println!("[/batch/plan.rs] fn inscribe / Locking unspent outputs for rune");
      wallet.bitcoin_client().lock_unspent(&[OutPoint {
        txid: commit_txid,
        vout: commit_vout.try_into().unwrap(),
      }])?;

      let commit = consensus::encode::deserialize::<Transaction>(&signed_commit_tx)?;
      let reveal = consensus::encode::deserialize::<Transaction>(&signed_reveal_tx)?;

      println!("[/batch/plan.rs] fn inscribe / Saving etching");
      wallet.save_etching(
        &rune_info.rune.rune,
        &commit,
        &reveal,
        self.output(
          commit.txid(),
          None,
          reveal.txid(),
          false,
          None,
          total_fees,
          self.inscriptions.clone(),
          rune.clone(),
        ),
      )?;

      println!("[/batch/plan.rs] fn inscribe / Waiting for maturation");
      Ok(Some(Box::new(
        wallet.wait_for_maturation(rune_info.rune.rune)?,
      )))
    } else {
      
      // reveal_tx를 전송하고, 성공 여부를 확인한 후 결과를 반환합니다.
      let reveal = match wallet
        .bitcoin_client()
        .send_raw_transaction(&signed_reveal_tx)
      {
        Ok(txid) => txid,
        Err(err) => {
          return Err(anyhow!(
        "Failed to send reveal transaction: {err}\nCommit tx {commit_txid} will be recovered once mined"
      ))
        }
      };

      println!("[/batch/plan.rs] fn inscribe / Sent reveal_tx, txid : {:?}", reveal);
      
      Ok(Some(Box::new(self.output(
        commit_txid,
        None,
        reveal,
        true,
        None,
        total_fees,
        self.inscriptions.clone(),
        rune,
      ))))
    }
  }

  fn remove_witnesses(mut transaction: Transaction) -> Transaction {
    for txin in transaction.input.iter_mut() {
      txin.witness = Witness::new();
    }

    transaction
  }

  fn output(
    &self, // 함수가 구조체의 메서드로 호출됨을 나타냅니다.
    commit: Txid, // 커밋 트랜잭션 ID (Txid 타입).
    commit_psbt: Option<String>, // 커밋의 PSBT(Partially Signed Bitcoin Transaction) 문자열 옵션.
    reveal: Txid, // 공개 트랜잭션 ID (Txid 타입).
    reveal_broadcast: bool, // 공개 트랜잭션이 브로드캐스트 되었는지 여부 (bool 타입).
    reveal_psbt: Option<String>, // 공개의 PSBT 문자열 옵션.
    total_fees: u64, // 총 수수료 (u64 타입).
    inscriptions: Vec<Inscription>, // Inscription 객체의 벡터.
    rune: Option<RuneInfo>, // RuneInfo 객체의 옵션.
  ) -> Output {

    println!("[/batch/plan.rs] fn output / called with:");
    println!("[/batch/plan.rs] fn output  commit: {:?}", commit);
    println!("[/batch/plan.rs] fn output  commit_psbt: {:?}", commit_psbt);
    println!("[/batch/plan.rs] fn output  reveal: {:?}", reveal);
    println!("[/batch/plan.rs] fn output  reveal_broadcast: {:?}", reveal_broadcast);
    println!("[/batch/plan.rs] fn output  reveal_psbt: {:?}", reveal_psbt);
    println!("[/batch/plan.rs] fn output  total_fees: {}", total_fees);
    println!("[/batch/plan.rs] fn output  inscriptions: {:?}", inscriptions);
    println!("[/batch/plan.rs] fn output  rune: {:?}", rune);
    println!("[/batch/plan.rs] fn output  self.mode: {:?}", self.mode);
    println!("[/batch/plan.rs] fn output  self.parent_info: {:?}", self.parent_info);

    
    
    let mut inscriptions_output = Vec::new();
    for i in 0..inscriptions.len() {
      let index = u32::try_from(i).unwrap();
      println!("[/batch/plan.rs] fn output / Processing inscription {}: {:?}", i, inscriptions[i]);
      println!("[/batch/plan.rs] fn output  index: {}", index);

      let vout = match self.mode {
        Mode::SharedOutput | Mode::SameSat => {
          if self.parent_info.is_some() {
            1
          } else {
            0
          }
        }
        Mode::SeparateOutputs | Mode::SatPoints => {
          if self.parent_info.is_some() {
            index + 1
          } else {
            index
          }
        }
      };
      println!("[/batch/plan.rs] fn output  vout: {}", vout);

      let offset = match self.mode {
        Mode::SharedOutput => self.postages[0..i]
          .iter()
          .map(|amount| amount.to_sat())
          .sum(),
        Mode::SeparateOutputs | Mode::SameSat | Mode::SatPoints => 0,
      };
      println!("[/batch/plan.rs] fn output  offset: {}", offset);

      let destination = match self.mode {
        Mode::SameSat | Mode::SharedOutput => &self.destinations[0],
        Mode::SatPoints | Mode::SeparateOutputs => &self.destinations[i],
      };
      println!("[/batch/plan.rs] fn output  destination: {:?}", destination);

      inscriptions_output.push(InscriptionInfo {
        id: InscriptionId {
          txid: reveal,
          index,
        },
        destination: uncheck(destination),
        location: SatPoint {
          outpoint: OutPoint { txid: reveal, vout },
          offset,
        },
      });
      println!("[/batch/plan.rs] fn output inscriptions_output: {:?}", inscriptions_output.last().unwrap());
    }

    let output = Output {
      commit,
      commit_psbt,
      inscriptions: inscriptions_output,
      parent: self.parent_info.clone().map(|info| info.id),
      reveal,
      reveal_broadcast,
      reveal_psbt,
      rune,
      total_fees,
    };
    println!("[/batch/plan.rs] fn output / Output: {:?}", output);

    output
    
  }

  pub(crate) fn create_batch_transactions(
    &self,
    wallet_inscriptions: BTreeMap<SatPoint, Vec<InscriptionId>>, // 각 SatPoint와 관련된 여러 InscriptionId를 매핑한 자료 구조입니다. SatPoint는 특정 트랜잭션의 위치를 나타내며, InscriptionId는 이와 관련된 특정 각인(데이터) ID를 나타냅니다.
    chain: Chain, // 현재 사용 중인 블록체인의 정보를 나타냅니다. 이는 메인넷, 테스트넷 또는 레그테스트와 같은 네트워크를 포함할 수 있습니다.
    locked_utxos: BTreeSet<OutPoint>, // 현재 잠긴 상태의 UTXO(사용되지 않은 트랜잭션 출력)들을 나타냅니다. 잠긴 UTXO는 다른 트랜잭션에서 사용할 수 없습니다.
    runic_utxos: BTreeSet<OutPoint>, // 특정 런(Rune)과 관련된 UTXO들을 나타냅니다.
    mut utxos: BTreeMap<OutPoint, TxOut>, // 트랜잭션 출력의 매핑으로, 각 OutPoint와 관련된 TxOut(트랜잭션 출력)을 나타냅니다.
    commit_change: [Address; 2], // 커밋 트랜잭션에서 발생한 잔액을 받기 위한 두 개의 주소 배열입니다.
    reveal_change: Address, // 리빌 트랜잭션에서 발생한 잔액을 받기 위한 주소입니다.
  ) -> Result<Transactions> {

    println!("[/batch/plan.rs] fn : create_batch_transactions ");
    println!("[/batch/plan.rs] fn : create_batch_transactions / wallet_inscriptions : {:?} ",wallet_inscriptions);
    println!("[/batch/plan.rs] fn : create_batch_transactions / chain : {:?} ",chain);
    println!("[/batch/plan.rs] fn : create_batch_transactions / locked_utxos : {:?} ",locked_utxos);
    println!("[/batch/plan.rs] fn : create_batch_transactions / runic_utxos : {:?} ",runic_utxos);
    println!("[/batch/plan.rs] fn : create_batch_transactions / utxos : {:?} ",utxos);
    println!("[/batch/plan.rs] fn : create_batch_transactions / commit_change : {:?} ",commit_change);
    println!("[/batch/plan.rs] fn : create_batch_transactions / reveal_change : {:?} ",reveal_change);
    
    
    // parent_info에 값이 존재한다면..
    if let Some(parent_info) = &self.parent_info {
      println!("[/batch/plan.rs] fn : create_batch_transactions / Parent Info ID: {:?}", parent_info.id);
      for inscription in &self.inscriptions {
        println!("[/batch/plan.rs] fn : create_batch_transactions / Inscription: {:?}", inscription);
        println!("[/batch/plan.rs] fn : create_batch_transactions / Inscription Parents: {:?}", inscription.parents());
        assert_eq!(inscription.parents(), vec![parent_info.id]);
      }
    }

    // inscription mode를 어떤 것인지 설정
    match self.mode {
      Mode::SameSat => { // 여러 sat에 inscription할때 모두 동일한 sat를 공유할 수 있도록 합니다.
        println!("[/batch/plan.rs] fn : create_batch_transactions / Current Mode: {:?} ", self.mode);
        println!("[/batch/plan.rs] fn : create_batch_transactions / Postages Length: {}", self.postages.len());
        println!("[/batch/plan.rs] fn : create_batch_transactions / Destinations Length: {}", self.destinations.len());
        assert_eq!(
          self.postages.len(),
          1,
          "invariant: same-sat has only one postage"
        );
        assert_eq!(
          self.destinations.len(),
          1,
          "invariant: same-sat has only one destination"
        );
      }
      Mode::SeparateOutputs | Mode::SatPoints => { 
        // SatPoints : inscription의 위치를 정밀하게 정할때 / 
        // SeparateOutpus : inscription을 별도의 각각 다른 output에 할당할때 사용
        println!("[/batch/plan.rs] fn : create_batch_transactions / Current Mode: {:?}", self.mode);
        println!("[/batch/plan.rs] fn : create_batch_transactions / Destinations Length: {}", self.destinations.len());
        println!("[/batch/plan.rs] fn : create_batch_transactions / Inscriptions Length: {}", self.inscriptions.len());
        println!("[/batch/plan.rs] fn : create_batch_transactions / Postages Length: {}", self.postages.len());
        assert_eq!(
          self.destinations.len(),
          self.inscriptions.len(),
          "invariant: destination addresses and number of inscriptions doesn't match"
        );
        assert_eq!(
          self.destinations.len(),
          self.postages.len(),
          "invariant: destination addresses and number of postages doesn't match"
        );
      }
      Mode::SharedOutput => { // 여러개의 inscription을 하나의 output에 공유할때 그룹화함
        println!("[/batch/plan.rs] fn : create_batch_transactions / Current Mode: {:?}", self.mode);
        println!("[/batch/plan.rs] fn : create_batch_transactions / Destinations Length: {}", self.destinations.len());
        println!("[/batch/plan.rs] fn : create_batch_transactions / Postages Length: {}", self.postages.len());
        println!("[/batch/plan.rs] fn : create_batch_transactions / Inscriptions Length: {}", self.inscriptions.len());
        assert_eq!(
          self.destinations.len(),
          1,
          "invariant: shared-output has only one destination"
        );
        assert_eq!(
          self.postages.len(),
          self.inscriptions.len(),
          "invariant: postages and number of inscriptions doesn't match"
        );
      }
    }
    
    // inscription할 satpoint를 찾고, 없다면 지갑에서 사용 가능한 UTXO를 찾습니다.
    let satpoint = if let Some(satpoint) = self.satpoint {
      println!("[/batch/plan.rs] fn : create_batch_transactions / Using provided satpoint: {:?}", satpoint);
      satpoint
    } else {
      println!("[/batch/plan.rs] fn : create_batch_transactions / No satpoint provided, searching for suitable UTXO...");
      
      let inscribed_utxos = wallet_inscriptions
        .keys()
        .map(|satpoint| satpoint.outpoint)
        .collect::<BTreeSet<OutPoint>>();

      println!("[/batch/plan.rs] fn : create_batch_transactions / Inscribed UTXOs collected: {:?}", inscribed_utxos);

      // utxos
      //   .iter()
      //   .find(|(outpoint, txout)| {
      //     txout.value > 0
      //       && !inscribed_utxos.contains(outpoint)
      //       && !locked_utxos.contains(outpoint)
      //       && !runic_utxos.contains(outpoint)
      //   })
      //   .map(|(outpoint, _amount)| SatPoint {
      //     outpoint: *outpoint,
      //     offset: 0,
      //   })
      //   .ok_or_else(|| anyhow!("wallet contains no cardinal utxos"))?
      utxos
          .iter()
          .find(|(outpoint, txout)| {
            let is_valid = txout.value > 0
                && !inscribed_utxos.contains(outpoint)
                && !locked_utxos.contains(outpoint)
                && !runic_utxos.contains(outpoint);

            println!("[/batch/plan.rs] fn : create_batch_transactions / Checking UTXO: outpoint={:?}, value={}, is_valid={}",
                outpoint,
                txout.value,
                is_valid
            );

            is_valid
          })
          .map(|(outpoint, _amount)| {
            println!("[/batch/plan.rs] fn : create_batch_transactions / Found suitable UTXO: {:?}", outpoint);
            SatPoint {
              outpoint: *outpoint,
              offset: 0,
            }
          })
          .ok_or_else(|| {
            println!("[/batch/plan.rs] fn : create_batch_transactions / No suitable UTXO found. Wallet contains no cardinal UTXOs.");
            anyhow!("wallet contains no cardinal utxos")
          })?
    };
  
    
    // reinscription을 진행할지 판단하는 코드
    // satpoint에 이미 데이터가 새겨져 있는지 확인하는 코드
    let mut reinscription = false;
    println!("[/batch/plan.rs] fn : create_batch_transactions / Starting to check wallet inscriptions against the given satpoint: {:?}", satpoint);
    
    for (inscribed_satpoint, inscription_ids) in &wallet_inscriptions {
      println!("[/batch/plan.rs] fn : create_batch_transactions / Checking inscribed_satpoint: {:?} with inscription_ids: {:?}", inscribed_satpoint, inscription_ids);
      if *inscribed_satpoint == satpoint {
        reinscription = true;
        println!("[/batch/plan.rs] fn : create_batch_transactions / Found matching satpoint {:?} in wallet inscriptions. reinscription set to true.", satpoint);

        if self.reinscribe {
          println!("[/batch/plan.rs] fn : create_batch_transactions / Reinscribe flag is set. Continuing to the next inscription.");
          continue;
        }

        println!("[/batch/plan.rs] fn : create_batch_transactions / Sat at {:?} is already inscribed. Aborting.", satpoint);
        bail!("sat at {} already inscribed", satpoint);
      }

      if inscribed_satpoint.outpoint == satpoint.outpoint {
        bail!(
          "utxo {} with sat {inscribed_satpoint} already inscribed with the following inscriptions:\n{}",
          satpoint.outpoint,
          inscription_ids
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join("\n"),
        );
      }
    }
    println!("[/batch/plan.rs] fn : create_batch_transactions / Completed checking wallet inscriptions. reinscription: {}", reinscription);

    // reinsciption 플래그가 설정되어 있지만, 현재 작업이 실제로 reinscription이 아닌지 확인, 이 조건이 참이면 오류 메시지와 함께 작업 중단.
    if self.reinscribe && !reinscription {
      bail!("reinscribe flag set but this would not be a reinscription");
    }
    println!("[/batch/plan.rs] fn : create_batch_transactions / reinscribe flag checked successfully");
    
    // tapRoot 주소에서 사용될 키쌍을 생성합니다.
    println!("[/batch/plan.rs] fn : create_batch_transactions / Setting up Secp256k1 context and generating key pair");
    let secp256k1 = Secp256k1::new();
    let key_pair = UntweakedKeyPair::new(&secp256k1, &mut rand::thread_rng());
    println!("[/batch/plan.rs] fn : create_batch_transactions / Generated new key pair : {:?}", key_pair);
    let (public_key, _parity) = XOnlyPublicKey::from_keypair(&key_pair);
    println!("[/batch/plan.rs] fn : create_batch_transactions / Generated new key pair with public key: {:?}", public_key);

    // inscription에 대한 reveal script를 추가하여 구성합니다. 이 스크립트에는 공개키와 OP_CHECKSIG 연산이 포함됩니다.
    println!("[/batch/plan.rs] fn : create_batch_transactions / Creating reveal script for inscriptions");
    let reveal_script = Inscription::append_batch_reveal_script(
      &self.inscriptions,
      ScriptBuf::builder()
        .push_slice(public_key.serialize())
        .push_opcode(opcodes::all::OP_CHECKSIG),
    );
    println!("[/batch/plan.rs] fn : create_batch_transactions / Reveal script created successfully");
    
    // 새 tapRoot 빌더를 생성하고 , reveal script를 사용하여 leaf 노드를 추가합니다.
    // tapRoot 빌더를 마무리하여 tapRoot Spending 정보를 생성합니다.
    println!("[/batch/plan.rs] fn : create_batch_transactions / Setting up Taproot spending information");
    let taproot_spend_info = TaprootBuilder::new()
      .add_leaf(0, reveal_script.clone())
      .expect("adding leaf should work")
      .finalize(&secp256k1, public_key)
      .expect("finalizing taproot builder should work");
    println!("[/batch/plan.rs] fn : create_batch_transactions / Taproot spending information set up successfully");
    
    
    // reveal script 및 tapScript leaf 버전을 사용하여 tapRoot spending 정보에 대한 제어 블록을 계산합니다.
    println!("[/batch/plan.rs] fn : create_batch_transactions / Generating control block");
    let control_block = taproot_spend_info
      .control_block(&(reveal_script.clone(), LeafVersion::TapScript))
      .expect("should compute control block");
    println!("[/batch/plan.rs] fn : create_batch_transactions / Control block generated successfully");

    // 탭루트 output key와 체인에서 네트워크 유형을 사용하여 P2TR(Pay to TapRoot) tweaked address를 생성합니다.
    println!("[/batch/plan.rs] fn : create_batch_transactions / Creating Taproot address");
    let commit_tx_address = Address::p2tr_tweaked(taproot_spend_info.output_key(), chain.network());
    println!("[/batch/plan.rs] fn : create_batch_transactions / Taproot address: {:?}", commit_tx_address);
    
    // 수수료를 sat로 환산합니다.
    println!("[/batch/plan.rs] fn : create_batch_transactions / Calculating total postage");
    let total_postage = self.postages.iter().map(|amount| amount.to_sat()).sum();
    println!("[/batch/plan.rs] fn : create_batch_transactions / Total postage: {}", total_postage);

    let mut reveal_inputs = Vec::new();
    let mut reveal_outputs = Vec::new();

    // parentInfo가 있는 경우, parent의 outpoint를 reveal input에 추가하고 parent의 destination과 트랜잭션 output 값으로 출력을 생성합니다.
    if let Some(ParentInfo {
      location,
      id: _,
      destination,
      tx_out,
    }) = self.parent_info.clone()
    {
      println!("[/batch/plan.rs] fn : create_batch_transactions / Adding parent info to reveal inputs and outputs");
      reveal_inputs.push(location.outpoint);
      reveal_outputs.push(TxOut {
        script_pubkey: destination.script_pubkey(),
        value: tx_out.value,
      });
    }

    println!("[/batch/plan.rs] fn :  / 1 reveal_inputs.clone() : {:?}", reveal_inputs.clone());

    // 모드가 SatPoints인 경우, reveal input에 reveal 좌표의 outpoint를 더합니다.
    if self.mode == Mode::SatPoints {
      println!("[/batch/plan.rs] fn : create_batch_transactions / Handling SatPoints mode");
      for (satpoint, _txout) in self.reveal_satpoints.iter() {
        reveal_inputs.push(satpoint.outpoint);
      }
    }

    println!("[/batch/plan.rs] fn :  / 2 reveal_inputs.clone() : {:?}", reveal_inputs.clone());

    // reveal input에 null outpoint를 추가합니다.
    reveal_inputs.push(OutPoint::null());

    println!("[/batch/plan.rs] fn :  / 3 reveal_inputs.clone() : {:?}", reveal_inputs.clone());
    println!("[/batch/plan.rs] fn : create_batch_transactions / Added null outpoint to reveal inputs");

    // destination를 반복하여 각각에 대한 output을 생성합니다. 
    // 각 output의 값은 현재 모드에 따라 결정됩니다. 
    // SeparateOutputs 및 SatPoints 모드의 경우 해당 우편 요금 금액을 사용하고, 
    // SharedOutput 및 SameSat 모드의 경우 총 우편 요금 금액을 사용합니다.
    println!("[/batch/plan.rs] fn : create_batch_transactions / Creating outputs for destinations");
    for (i, destination) in self.destinations.iter().enumerate() {
      reveal_outputs.push(TxOut {
        script_pubkey: destination.script_pubkey(),
        value: match self.mode {
          Mode::SeparateOutputs | Mode::SatPoints => self.postages[i].to_sat(),
          Mode::SharedOutput | Mode::SameSat => total_postage,
        },
      });
      println!("[/batch/plan.rs] fn : create_batch_transactions / Added output for destination {}: {:?}", i, destination);
    }

    let rune;
    let premine;
    let runestone;

    if let Some(etching) = self.etching {
      let vout;
      let destination;
      premine = etching.premine.to_integer(etching.divisibility)?;

      if premine > 0 {
        let output = u32::try_from(reveal_outputs.len()).unwrap();
        destination = Some(reveal_change.clone());

        reveal_outputs.push(TxOut {
          script_pubkey: reveal_change.into(),
          value: TARGET_POSTAGE.to_sat(),
        });

        vout = Some(output);
      } else {
        vout = None;
        destination = None;
      }

      let inner = Runestone {
        edicts: Vec::new(),
        etching: Some(ordinals::Etching {
          divisibility: (etching.divisibility > 0).then_some(etching.divisibility),
          premine: (premine > 0).then_some(premine),
          rune: Some(etching.rune.rune),
          spacers: (etching.rune.spacers > 0).then_some(etching.rune.spacers),
          symbol: Some(etching.symbol),
          terms: etching
            .terms
            .map(|terms| -> Result<ordinals::Terms> {
              Ok(ordinals::Terms {
                cap: (terms.cap > 0).then_some(terms.cap),
                height: (
                  terms.height.and_then(|range| (range.start)),
                  terms.height.and_then(|range| (range.end)),
                ),
                amount: Some(terms.amount.to_integer(etching.divisibility)?),
                offset: (
                  terms.offset.and_then(|range| (range.start)),
                  terms.offset.and_then(|range| (range.end)),
                ),
              })
            })
            .transpose()?,
          turbo: etching.turbo,
        }),
        mint: None,
        pointer: (premine > 0).then_some((reveal_outputs.len() - 1).try_into().unwrap()),
      };

      let script_pubkey = inner.encipher();

      runestone = Some(inner);

      ensure!(
        self.no_limit || script_pubkey.len() <= 82,
        "runestone greater than maximum OP_RETURN size: {} > 82",
        script_pubkey.len()
      );

      reveal_outputs.push(TxOut {
        script_pubkey,
        value: 0,
      });

      rune = Some((destination, etching.rune, vout));
    } else {
      premine = 0;
      rune = None;
      runestone = None;
    }
    
    // Bitcoin Taproot 기능을 활용하여 트랜잭션을 생성하고 서명하는 과정을 다룹니다. 
    // Taproot 키와 스크립트를 사용하여 트랜잭션을 생성하고, 해당 트랜잭션에 서명한 후 UTXO를 업데이트하고, 
    // 최종적으로 트랜잭션 세부 정보를 반환합니다.

    // Calculate the commit input count
    // self.parent_info가 있는지와 self.reveal_satpoints의 길이를 합산하여 계산됩니다. 이는 commit 트랜잭션 입력의 수를 나타냅니다.
    let commit_input = usize::from(self.parent_info.is_some()) + self.reveal_satpoints.len();
    println!("[/batch/plan.rs] fn : create_batch_transactions / Calculated commit self.parent_info.is_some(): {}", self.parent_info.is_some());
    println!("[/batch/plan.rs] fn : create_batch_transactions / Calculated commit self.reveal_satpoints.len(): {}", self.reveal_satpoints.len());
    println!("[/batch/plan.rs] fn : create_batch_transactions / Calculated commit input count: {}", commit_input);

    // Build the reveal transaction to estimate the fee
    // reveal 트랜잭션을 생성하고 수수료를 계산합니다.
    // commit_input 실제는 0 
    println!("[/batch/plan.rs] fn :  / build_reveal_transaction param commit_input : {:?}", commit_input);
    println!("[/batch/plan.rs] fn :  / build_reveal_transaction param &control_block : {:?}", &control_block);
    println!("[/batch/plan.rs] fn :  / build_reveal_transaction param self.reveal_fee_rate : {:?}", self.reveal_fee_rate);
    println!("[/batch/plan.rs] fn :  / build_reveal_transaction param reveal_outputs.clone() : {:?}", reveal_outputs.clone());
    println!("[/batch/plan.rs] fn :  / build_reveal_transaction param 4 reveal_inputs.clone() : {:?}", reveal_inputs.clone());
    println!("[/batch/plan.rs] fn :  / build_reveal_transaction param &reveal_script : {:?}", &reveal_script);
    println!("[/batch/plan.rs] fn :  / build_reveal_transaction param rune.is_some() : {:?}", rune.is_some());
    
    let (_reveal_tx, reveal_fee) = Self::build_reveal_transaction(
      commit_input,
      &control_block,
      self.reveal_fee_rate,
      reveal_outputs.clone(),
      reveal_inputs.clone(),
      &reveal_script,
      rune.is_some(),
    );

    println!("[/batch/plan.rs] fn : create_batch_transactions / Built reveal transaction to estimate fee, reveal_fee: {}", reveal_fee);
    // target_value는 reveal 수수료, 전체 postage 및 premine 값에 따라 결정됩니다.
    let mut target_value = reveal_fee;

    // Add total postage if mode is not SatPoints
    if self.mode != Mode::SatPoints {
      target_value += Amount::from_sat(total_postage);
    }

    // Add premine postage if premine is greater than 0
    if premine > 0 {
      target_value += TARGET_POSTAGE;
    }
    println!("[/batch/plan.rs] fn : create_batch_transactions / Calculated target value: {}", target_value);

    // Build the unsigned commit transaction
    // Unsigned commit 트랜잭션을 생성합니다.
    let unsigned_commit_tx = TransactionBuilder::new(
      satpoint,
      wallet_inscriptions,
      utxos.clone(),
      locked_utxos.clone(),
      runic_utxos,
      commit_tx_address.clone(),
      commit_change,
      self.commit_fee_rate,
      Target::Value(target_value),
    )
    .build_transaction()?;
    println!("[/batch/plan.rs] fn : create_batch_transactions / Built unsigned commit transaction");

    // Find the output for the commit transaction
    // commit 트랜잭션의 output을 찾습니다.
    let (vout, _commit_output) = unsigned_commit_tx
      .output
      .iter()
      .enumerate()
      .find(|(_vout, output)| output.script_pubkey == commit_tx_address.script_pubkey())
      .expect("should find sat commit/inscription output");
    println!("[/batch/plan.rs] fn : create_batch_transactions / Found commit output: vout = {}", vout);

    // Update reveal inputs with the commit transaction output
    // reveal_inputs를 commit 트랜잭션 output으로 업데이트합니다.
    reveal_inputs[commit_input] = OutPoint {
      txid: unsigned_commit_tx.txid(),
      vout: vout.try_into().unwrap(),
    };
    println!("[/batch/plan.rs] fn : create_batch_transactions / Updated reveal inputs with commit transaction output");

    // Build the final reveal transaction
    // 최종 reveal 트랜잭션을 빌드합니다.
    let (mut reveal_tx, _fee) = Self::build_reveal_transaction(
      commit_input,
      &control_block,
      self.reveal_fee_rate,
      reveal_outputs.clone(),
      reveal_inputs,
      &reveal_script,
      rune.is_some(),
    );
    println!("[/batch/plan.rs] fn : create_batch_transactions / Built final reveal transaction");

    // Ensure outputs are not dust
    // 모든 output이 dust가 아닌지 확인합니다.
    for output in reveal_tx.output.iter() {
      ensure!(
        output.value >= output.script_pubkey.dust_value().to_sat(),
        "commit transaction output would be dust"
      );
    }
    println!("[/batch/plan.rs] fn : create_batch_transactions / Checked reveal transaction outputs for dust");

    // Prepare prevouts for sighash calculation
    // sighash 계산을 위해 prevouts를 준비합니다.
    let mut prevouts = Vec::new();

    if let Some(parent_info) = self.parent_info.clone() {
      prevouts.push(parent_info.tx_out);
    }

    if self.mode == Mode::SatPoints {
      for (_satpoint, txout) in self.reveal_satpoints.iter() {
        prevouts.push(txout.clone());
      }
    }

    prevouts.push(unsigned_commit_tx.output[vout].clone());
    println!("[/batch/plan.rs] fn : create_batch_transactions / Prepared prevouts for sighash calculation");

    // Create sighash for the reveal transaction
    // reveal 트랜잭션에 대한 sighash를 계산합니다.
    let mut sighash_cache = SighashCache::new(&mut reveal_tx);
    println!("[/subcommand/wallet/inscribe_with_txid.rs] fn : create_batch_transactions / sighash_cache {:?}",sighash_cache);

    println!("[/subcommand/wallet/inscribe_with_txid.rs] fn : create_batch_transactions / sighash_hash 만들기전 Parm 리스트 값들");
    println!("[/subcommand/wallet/inscribe_with_txid.rs] fn : create_batch_transactions / &prevouts {:?}",&prevouts);
    println!("[/subcommand/wallet/inscribe_with_txid.rs] fn : create_batch_transactions / &reveal_script {:?}",&reveal_script);
    
    let sighash = sighash_cache
      .taproot_script_spend_signature_hash(
        commit_input,
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
      &key_pair,
    );
    println!("[/subcommand/wallet/inscribe_with_txid.rs] fn : create_batch_transactions / Signed reveal transaction sig : {:?}",sig);

    // Update witness for the reveal transaction
    // reveal 트랜잭션의 witness를 업데이트합니다.
    let witness = sighash_cache
      .witness_mut(commit_input)
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
    let recovery_key_pair = key_pair.tap_tweak(&secp256k1, taproot_spend_info.merkle_root());

    let (x_only_pub_key, _parity) = recovery_key_pair.to_inner().x_only_public_key();
    assert_eq!(
      Address::p2tr_tweaked(
        TweakedPublicKey::dangerous_assume_tweaked(x_only_pub_key),
        chain.network(),
      ),
      commit_tx_address
    );
    println!("[/batch/plan.rs] fn : create_batch_transactions / Validated recovery key pair");

    // Check the reveal transaction weight
    // reveal 트랜잭션의 무게를 확인합니다.
    let reveal_weight = reveal_tx.weight();

    if !self.no_limit && reveal_weight > bitcoin::Weight::from_wu(MAX_STANDARD_TX_WEIGHT.into()) {
      bail!(
        "reveal transaction weight greater than {MAX_STANDARD_TX_WEIGHT} (MAX_STANDARD_TX_WEIGHT): {reveal_weight}"
      );
    }

    println!("[/batch/plan.rs] fn : create_batch_transactions / Checked reveal transaction weight: {}", reveal_weight);

    // Update UTXOs with the reveal transaction
    // reveal 트랜잭션으로 UTXO를 업데이트합니다.
    utxos.insert(
      reveal_tx.input[commit_input].previous_output,
      unsigned_commit_tx.output[reveal_tx.input[commit_input].previous_output.vout as usize]
        .clone(),
    );
    println!("[/batch/plan.rs] fn : create_batch_transactions / Updated UTXOs with reveal transaction");
  
    // Calculate total fees
    // commit 및 reveal 트랜잭션의 수수료를 계산합니다.
    let total_fees =
      Self::calculate_fee(&unsigned_commit_tx, &utxos) + Self::calculate_fee(&reveal_tx, &utxos);
    println!("[/batch/plan.rs] fn : create_batch_transactions / Calculated total fees: {}", total_fees);

    match (Runestone::decipher(&reveal_tx), runestone) {
      (Some(actual), Some(expected)) => assert_eq!(
        actual,
        Artifact::Runestone(expected),
        "commit transaction runestone did not match expected runestone"
      ),
      (Some(_), None) => panic!("commit transaction contained runestone, but none was expected"),
      (None, Some(_)) => {
        panic!("commit transaction did not contain runestone, but one was expected")
      }
      (None, None) => {}
    }

    let rune = rune.map(|(destination, rune, vout)| RuneInfo {
      destination: destination.map(|destination| uncheck(&destination)),
      location: vout.map(|vout| OutPoint {
        txid: reveal_tx.txid(),
        vout,
      }),
      rune,
    });

    Ok(Transactions {
      commit_tx: unsigned_commit_tx,
      commit_vout: vout,
      recovery_key_pair,
      reveal_tx,
      rune,
      total_fees,
    })
  }
 
  // 지갑과 연관된 복구 키를 백업하는 기능을 하는 함수입니다. 여기서 Wallet과 TweakedKeyPair는 각각 지갑과 키 쌍을 나타냅니다. 
  // 코드는 복구 프라이빗 키를 생성하고, 이를 비트코인 클라이언트를 통해 지갑에 임포트하는 작업을 수행합니다.
  fn backup_recovery_key(wallet: &Wallet, recovery_key_pair: TweakedKeyPair) -> Result {
    // wallet: 지갑 객체에 대한 참조.
    // recovery_key_pair: 복구 키 쌍 객체.
    
    
    // recovery_key_pair로부터 프라이빗 키를 생성하고 이를 출력합니다.
    let recovery_private_key = PrivateKey::new(
      recovery_key_pair.to_inner().secret_key(),
      wallet.chain().network(),
    );
    println!("[/batch/plan.rs] fn : backup_recovery_key / Recovery private key: {:?}", recovery_private_key);
    
    // 프라이빗 키를 사용하여 디스크립터 정보를 가져오고 이를 출력합니다.
    let info = wallet
      .bitcoin_client()
      .get_descriptor_info(&format!("rawtr({})", recovery_private_key.to_wif()))?;
    println!("[/batch/plan.rs] fn : backup_recovery_key / Descriptor info: {:?}", info);

    // 디스크립터를 비트코인 클라이언트에 임포트하고 응답을 출력합니다.
    let response = wallet
      .bitcoin_client()
      .import_descriptors(vec![ImportDescriptors {
        descriptor: format!("rawtr({})#{}", recovery_private_key.to_wif(), info.checksum),
        timestamp: Timestamp::Now,
        active: Some(false),
        range: None,
        next_index: None,
        internal: Some(false),
        label: Some("commit tx recovery key".to_string()),
      }])?;

    println!("[/batch/plan.rs] fn : backup_recovery_key / Import descriptors response: {:?}", response);

    for result in response {
      if !result.success {
        println!("[/batch/plan.rs] fn : backup_recovery_key / Descriptor import failed for: {:?}", result);
        return Err(anyhow!("commit tx recovery key import failed"));
      }
    }

    println!("[/batch/plan.rs] fn : backup_recovery_key / Recovery key backup successful");
    Ok(())
  }

  // 트랜잭션을 생성하고 수수료를 계산하는 함수
  fn build_reveal_transaction(
    commit_input_index: usize, // commit_input_index: 커밋 입력 인덱스 (usize 타입).
    control_block: &ControlBlock, // control_block: ControlBlock에 대한 참조.
    fee_rate: FeeRate, // fee_rate: 수수료율 (FeeRate 타입).
    output: Vec<TxOut>, // output: 출력 벡터 (Vec<TxOut> 타입).
    input: Vec<OutPoint>, // input: 입력 벡터 (Vec<OutPoint> 타입).
    script: &Script, // script: 스크립트에 대한 참조 (&Script 타입)
    etching: bool, // etching: 부울 값 (bool 타입), 트랜잭션의 특성을 결정.
  ) -> (Transaction, Amount) {

    println!("[/batch/plan.rs] fn build_reveal_transaction / 트랜잭션 생성");
    
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

    println!("[/batch/plan.rs] fn build_reveal_transaction / Initial reveal_tx: {:?}", reveal_tx);

    // 수수료 계산을 위한 트랜잭션 복사 및 수정
    let fee = {
      let mut reveal_tx = reveal_tx.clone();
      println!("[/batch/plan.rs] fn build_reveal_transaction / Cloned reveal_tx for fee calculation: {:?}", reveal_tx);

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
          txin.witness.push(&control_block.serialize());
        } else {
          txin.witness = Witness::from_slice(&[&[0; SCHNORR_SIGNATURE_SIZE]]);
        }
        println!("[/batch/plan.rs] fn build_reveal_transaction / Updated txin at index {}: {:?}", current_index, txin);
      }

      fee_rate.fee(reveal_tx.vsize())
    };

    println!("[/batch/plan.rs] fn build_reveal_transaction / Calculated fee: {:?}", fee);

    (reveal_tx, fee)
  }

  fn calculate_fee(tx: &Transaction, utxos: &BTreeMap<OutPoint, TxOut>) -> u64 {
    // tx: 수수료를 계산할 트랜잭션에 대한 참조 (&Transaction 타입).
    // utxos: 사용 가능한 UTXO(Unspent Transaction Outputs)의 맵 (&BTreeMap<OutPoint, TxOut> 타입).
    println!("[/batch/plan.rs] fn calculate_fee / called with:");
    println!("[/batch/plan.rs] fn calculate_fee /   tx: {:?}", tx);
    println!("[/batch/plan.rs] fn calculate_fee /   utxos: {:?}", utxos);
    // tx.input
    //   .iter()
    //   .map(|txin| utxos.get(&txin.previous_output).unwrap().value)
    //   .sum::<u64>()
    //   .checked_sub(tx.output.iter().map(|txout| txout.value).sum::<u64>())
    //   .unwrap()
    let input_sum: u64 = tx.input
        .iter()
        .map(|txin| {
          let value = utxos.get(&txin.previous_output).unwrap().value;
          println!("[/batch/plan.rs] fn calculate_fee / Input value for {:?}: {}", txin.previous_output, value);
          value
        })
        .sum();

    println!("[/batch/plan.rs] fn calculate_fee / Total input value: {}", input_sum);

    let output_sum: u64 = tx.output.iter().map(|txout| {
      let value = txout.value;
      println!("[/batch/plan.rs] fn calculate_fee / Output value: {}", value);
      value
    }).sum();

    println!("[/batch/plan.rs] fn calculate_fee / Total output value: {}", output_sum);

    let fee = input_sum.checked_sub(output_sum).unwrap();
    println!("[/batch/plan.rs] fn calculate_fee / Calculated fee: {}", fee);

    fee
  }
}
