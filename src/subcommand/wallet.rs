use {
  super::*,
  crate::wallet::{batch, wallet_constructor::WalletConstructor, Wallet},
  bitcoincore_rpc::bitcoincore_rpc_json::ListDescriptorsResult,
  shared_args::SharedArgs,
};

pub mod balance;
mod batch_command;
pub mod cardinals;
pub mod create;
pub mod dump;
pub mod inscribe;
pub mod inscriptions;
mod label;
pub mod mint;
pub mod outputs;
pub mod pending;
pub mod receive;
pub mod restore;
pub mod resume;
pub mod runics;
pub mod sats;
pub mod send;
mod shared_args;
pub mod transactions;
mod create_tr;
mod reveal_inscription;
mod inscribe_with_txid;

#[derive(Debug, Parser)]
pub(crate) struct WalletCommand {
  #[arg(long, default_value = "ord", help = "Use wallet named <WALLET>.")]
  pub(crate) name: String,
  #[arg(long, alias = "nosync", help = "Do not update index.")]
  pub(crate) no_sync: bool,
  #[arg(
    long,
    help = "Use ord running at <SERVER_URL>. [default: http://localhost:80]"
  )]
  pub(crate) server_url: Option<Url>,
  #[command(subcommand)]
  pub(crate) subcommand: Subcommand,
}

#[derive(Debug, Parser)]
#[allow(clippy::large_enum_variant)]
pub(crate) enum Subcommand {
  #[command(about = "Get wallet balance")]
  Balance,
  #[command(about = "Create inscriptions and runes")]
  Batch(batch_command::Batch),
  #[command(about = "List unspent cardinal outputs in wallet")]
  Cardinals,
  #[command(about = "Create new wallet")]
  Create(create::Create),
  #[command(about = "Dump wallet descriptors")]
  Dump,
  #[command(about = "Create inscription")]
  Inscribe(inscribe::Inscribe),
  #[command(about = "List wallet inscriptions")]
  Inscriptions,
  #[command(about = "Export output labels")]
  Label,
  #[command(about = "Mint a rune")]
  Mint(mint::Mint),
  #[command(about = "List all unspent outputs in wallet")]
  Outputs(outputs::Outputs),
  #[command(about = "List pending etchings")]
  Pending(pending::Pending),
  #[command(about = "Generate receive address")]
  Receive(receive::Receive),
  #[command(about = "Restore wallet")]
  Restore(restore::Restore),
  #[command(about = "Resume pending etchings")]
  Resume(resume::Resume),
  #[command(about = "List unspent runic outputs in wallet")]
  Runics,
  #[command(about = "List wallet satoshis")]
  Sats(sats::Sats),
  #[command(about = "Send sat or inscription")]
  Send(send::Send),
  #[command(about = "See wallet transactions")]
  Transactions(transactions::Transactions),
  #[command(about = "Create TapRoot Address for commit Tx")]
  CreateTR(create_tr::CreateTR),
  #[command(about = "Create TapRoot Address for commit Tx")]
  InscribeWithTxid(inscribe_with_txid::InscribeWithTxid),
}

impl WalletCommand {
  pub(crate) fn run(self, settings: Settings) -> SubcommandResult {

    println!("[wallet.rs] Running WalletCommand with subcommand: {:?}", self.subcommand);

    match self.subcommand {
      Subcommand::Create(create) => {
        println!("[wallet.rs] Creating wallet");
        return create.run(self.name, &settings)
      },
      Subcommand::Restore(restore) => {
        println!("[wallet.rs] Restoring wallet");
        return restore.run(self.name, &settings)
      },
      _ => {}
    };

    let wallet = WalletConstructor::construct(
      self.name.clone(),
      self.no_sync,
      settings.clone(),
      self
        .server_url
        .as_ref()
        .map(Url::as_str)
        .or(settings.server_url())
        .unwrap_or("http://127.0.0.1:80")
        .parse::<Url>()
        .context("invalid server URL")?,
    )?;
    println!("[wallet.rs] Wallet constructed");


    match self.subcommand {
      Subcommand::Balance => {
        println!("[wallet.rs] Running Balance subcommand");
        balance::run(wallet)
      },
      Subcommand::Batch(batch) => {
        println!("[wallet.rs] Running Batch subcommand");
        batch.run(wallet)
      },
      Subcommand::Cardinals => {
        println!("[wallet.rs] Running Cardinals subcommand");
        cardinals::run(wallet)
      },
      Subcommand::Create(_) | Subcommand::Restore(_) => unreachable!(),
      Subcommand::Dump => {
        println!("[wallet.rs] Running Dump subcommand");
        dump::run(wallet)
      },
      Subcommand::Inscribe(inscribe) => {
        println!("[wallet.rs] Running Inscribe subcommand");
        inscribe.run(wallet)
      },
      Subcommand::Inscriptions => {
        println!("[wallet.rs] Running Inscriptions subcommand");
        inscriptions::run(wallet)
      },
      Subcommand::Label => {
        println!("[wallet.rs] Running Label subcommand");
        label::run(wallet)
      },
      Subcommand::Mint(mint) => {
        println!("[wallet.rs] Running Mint subcommand");
        mint.run(wallet)
      },
      Subcommand::Outputs(outputs) => {
        println!("[wallet.rs] Running Outputs subcommand");
        outputs.run(wallet)
      },
      Subcommand::Pending(pending) => {
        println!("[wallet.rs] Running Pending subcommand");
        pending.run(wallet)
      },
      Subcommand::Receive(receive) => {
        println!("[wallet.rs] Running Receive subcommand");
        receive.run(wallet)
      },
      Subcommand::Resume(resume) => {
        println!("[wallet.rs] Running Resume subcommand");
        resume.run(wallet)
      },
      Subcommand::Runics => {
        println!("[wallet.rs] Running Runics subcommand");
        runics::run(wallet)
      },
      Subcommand::Sats(sats) => {
        println!("[wallet.rs] Running Sats subcommand");
        sats.run(wallet)
      },
      Subcommand::Send(send) => {
        println!("[wallet.rs] Running Send subcommand");
        send.run(wallet)
      },
      Subcommand::Transactions(transactions) => {
        println!("[wallet.rs] Running Transactions subcommand");
        transactions.run(wallet)
      },
      Subcommand::CreateTR(create_tr) => {
        println!("[wallet.rs] Running create_tr subcommand");
        create_tr.run(wallet)
      },
      Subcommand::InscribeWithTxid(inscribe_with_txid) => {
        println!("[wallet.rs] Running inscribe_with_txid subcommand");
        inscribe_with_txid.run(wallet)
      },
    }
  }
}
