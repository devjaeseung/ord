use super::*;

#[derive(Debug, Parser)]
#[clap(group(
  ArgGroup::new("input")
  .required(true)
  .multiple(true)
  .args(&["delegate", "file"])
))]
// .required(true) >> "input" 그룹의 인수들이 필수임을 지정
// .multiple(true) >> 여러 인수를 받을 수 있도록 지정
// .args(&["delegate", "file"]) >> "delegate"와 "file" 인수를 "input" 그룹에 추가
pub(crate) struct Inscribe {
  #[command(flatten)]
  shared: SharedArgs, // 여러 명령어에서 공유하는 인수들을 포함하는 구조체
  #[arg(
    long,
    help = "Include CBOR in file at <METADATA> as inscription metadata",
    conflicts_with = "json_metadata" 
  )]
    // conflicts_with = "json_metadata" // "json_metadata"와 충돌하므로 동시에 사용할 수 없음
  pub(crate) cbor_metadata: Option<PathBuf>, // CBOR 형식의 메타데이터 파일 경로
  #[arg(long, help = "Delegate inscription content to <DELEGATE>.")]
  pub(crate) delegate: Option<InscriptionId>, // 인스크립션 내용을 대리할 대리자 ID
  #[arg(long, help = "Send inscription to <DESTINATION>.")]
  pub(crate) destination: Option<Address<NetworkUnchecked>>, // 인스크립션을 보낼 목적지 주소
  #[arg(
    long,
    help = "Inscribe sat with contents of <FILE>. May be omitted if `--delegate` is supplied."
  )]
  pub(crate) file: Option<PathBuf>, // 인스크립션에 사용할 파일 경로
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
    help = "Include <AMOUNT> postage with inscription. [default: 10000sat]"
  )]
  pub(crate) postage: Option<Amount>, // 인스크립션에 포함할 우편 요금
  #[clap(long, help = "Allow reinscription.")]
  pub(crate) reinscribe: bool, // 다시 인스크립션할지 여부를 나타내는 플래그
  #[arg(long, help = "Inscribe <SAT>.", conflicts_with = "satpoint")]
  pub(crate) sat: Option<Sat>, // 인스크립션할 사토시
  #[arg(long, help = "Inscribe <SATPOINT>.", conflicts_with = "sat")]
  pub(crate) satpoint: Option<SatPoint>, // 인스크립션할 사토시 포인트
}

impl Inscribe {
  pub(crate) fn run(self, wallet: Wallet) -> SubcommandResult {
    println!("[inscribe.rs] Running inscribe command");
    println!("[inscribe.rs] shared {:?}",self.shared);
    println!("[inscribe.rs] cbor_metadata {:?}",self.cbor_metadata);
    println!("[inscribe.rs] delegate {:?}",self.delegate);
    println!("[inscribe.rs] destination {:?}",self.destination);
    println!("[inscribe.rs] file {:?}",self.file);
    println!("[inscribe.rs] json_metadata {:?}",self.json_metadata);
    println!("[inscribe.rs] metaprotocol {:?}",self.metaprotocol);
    println!("[inscribe.rs] parent {:?}",self.parent);
    println!("[inscribe.rs] postage {:?}",self.postage);
    println!("[inscribe.rs] reinscribe {:?}",self.reinscribe);
    println!("[inscribe.rs] sat {:?}",self.sat);
    println!("[inscribe.rs] satpoint {:?}",self.satpoint);
    

    // 체인 정보를 가져옴
    let chain = wallet.chain();
    println!("[inscribe.rs] Chain: {:?}", chain);

    // 대리자가 존재하는 경우 확인
    if let Some(delegate) = self.delegate {
      println!("[inscribe.rs] Delegate: {:?}", delegate);
      ensure! {
        wallet.inscription_exists(delegate)?, // 대리자가 존재하는지 확인
        "delegate {delegate} does not exist"
      }
    }

    // 목적지 주소를 설정
    println!("[inscribe.rs] self.destination.clone(): {:?}", self.destination.clone());
    let destination = match self.destination.clone() {
      Some(destination) => destination.require_network(chain.network())?, // 네트워크 요구사항 확인
      None => wallet.get_change_address()?, // 주소가 없는 경우 변경 주소를 가져옴
    };
    println!("[inscribe.rs] Destination: {:?}", destination);

    // 인스크립션 작업을 위한 계획 설정
    batch::Plan {
      commit_fee_rate: self.shared.commit_fee_rate.unwrap_or(self.shared.fee_rate), // 커밋 수수료율 설정
      destinations: vec![destination], // 목적지 주소 설정
      dry_run: self.shared.dry_run, // 드라이런 설정
      etching: None, // etching 설정
      inscriptions: vec![Inscription::new(
        chain, // 체인 설정
        self.shared.compress, // 압축 설정
        self.delegate, // 대리자 설정
        Inscribe::parse_metadata(self.cbor_metadata, self.json_metadata)?, // 메타데이터 파싱
        self.metaprotocol, // 메타프로토콜 설정
        self.parent.into_iter().collect(), // 부모 설정
        self.file, // 파일 설정
        None,
        None,
      )?],
      mode: batch::Mode::SeparateOutputs, // 모드 설정
      no_backup: self.shared.no_backup, // 백업 금지 설정
      no_limit: self.shared.no_limit, // 제한 없음 설정
      parent_info: wallet.get_parent_info(self.parent)?, // 부모 정보 설정
      postages: vec![self.postage.unwrap_or(TARGET_POSTAGE)], // 우편 요금 설정
      reinscribe: self.reinscribe, // 재인스크립션 설정
      reveal_fee_rate: self.shared.fee_rate, // 리빌 수수료율 설정
      reveal_satpoints: Vec::new(), // 리빌 좌표 설정
      satpoint: if let Some(sat) = self.sat {
        println!("[inscribe.rs] Sat: {:?}", sat);
        Some(wallet.find_sat_in_outputs(sat)?) // 사토시 설정
      } else {
        self.satpoint // 사토시 포인트 설정
      },
    }
        .inscribe(
          &wallet.locked_utxos().clone().into_keys().collect(), // 잠긴 UTXO 설정
          wallet.get_runic_outputs()?, // Runic UTXO 설정
          wallet.utxos(), // UTXO 설정
          &wallet, // 지갑 설정
        )
  }

  // 메타데이터를 파싱하는 함수
  fn parse_metadata(cbor: Option<PathBuf>, json: Option<PathBuf>) -> Result<Option<Vec<u8>>> {
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
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn cbor_and_json_metadata_flags_conflict() {
    assert_regex_match!(
      Arguments::try_parse_from([
        "ord",
        "wallet",
        "inscribe",
        "--cbor-metadata",
        "foo",
        "--json-metadata",
        "bar",
        "--file",
        "baz",
      ])
      .unwrap_err()
      .to_string(),
      ".*--cbor-metadata.*cannot be used with.*--json-metadata.*"
    );
  }

  #[test]
  fn satpoint_and_sat_flags_conflict() {
    assert_regex_match!(
      Arguments::try_parse_from([
        "ord",
        "--index-sats",
        "wallet",
        "inscribe",
        "--sat",
        "50000000000",
        "--satpoint",
        "038112028c55f3f77cc0b8b413df51f70675f66be443212da0642b7636f68a00:1:0",
        "--file",
        "baz",
      ])
      .unwrap_err()
      .to_string(),
      ".*--sat.*cannot be used with.*--satpoint.*"
    );
  }

  #[test]
  fn delegate_or_file_must_be_set() {
    assert_regex_match!(
      Arguments::try_parse_from(["ord", "wallet", "inscribe", "--fee-rate", "1"])
        .unwrap_err()
        .to_string(),
      r".*required arguments.*--delegate <DELEGATE>\|--file <FILE>.*"
    );

    assert!(Arguments::try_parse_from([
      "ord",
      "wallet",
      "inscribe",
      "--file",
      "hello.txt",
      "--fee-rate",
      "1"
    ])
    .is_ok());

    assert!(Arguments::try_parse_from([
      "ord",
      "wallet",
      "inscribe",
      "--delegate",
      "038112028c55f3f77cc0b8b413df51f70675f66be443212da0642b7636f68a00i0",
      "--fee-rate",
      "1"
    ])
    .is_ok());

    assert!(Arguments::try_parse_from([
      "ord",
      "wallet",
      "inscribe",
      "--file",
      "hello.txt",
      "--delegate",
      "038112028c55f3f77cc0b8b413df51f70675f66be443212da0642b7636f68a00i0",
      "--fee-rate",
      "1"
    ])
    .is_ok());
  }
}
