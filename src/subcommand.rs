use super::*;

pub mod balances;
pub mod decode;
pub mod env;
pub mod epochs;
pub mod find;
pub mod index;
pub mod list;
pub mod parse;
pub mod runes;
pub(crate) mod server;
mod settings;
pub mod subsidy;
pub mod supply;
pub mod teleburn;
pub mod traits;
pub mod wallet;

#[derive(Debug, Parser)]
pub(crate) enum Subcommand {
  #[command(about = "List all rune balances")]
  Balances,
  #[command(about = "Decode a transaction")]
  Decode(decode::Decode),
  #[command(about = "Start a regtest ord and bitcoind instance")]
  Env(env::Env),
  #[command(about = "List the first satoshis of each reward epoch")]
  Epochs,
  #[command(about = "Find a satoshi's current location")]
  Find(find::Find),
  #[command(subcommand, about = "Index commands")]
  Index(index::IndexSubcommand),
  #[command(about = "List the satoshis in an output")]
  List(list::List),
  #[command(about = "Parse a satoshi from ordinal notation")]
  Parse(parse::Parse),
  #[command(about = "List all runes")]
  Runes,
  #[command(about = "Run the explorer server")]
  Server(server::Server),
  #[command(about = "Display settings")]
  Settings,
  #[command(about = "Display information about a block's subsidy")]
  Subsidy(subsidy::Subsidy),
  #[command(about = "Display Bitcoin supply information")]
  Supply,
  #[command(about = "Generate teleburn addresses")]
  Teleburn(teleburn::Teleburn),
  #[command(about = "Display satoshi traits")]
  Traits(traits::Traits),
  #[command(about = "Wallet commands")]
  Wallet(wallet::WalletCommand),
}

impl Subcommand {
  pub(crate) fn run(self, settings: Settings) -> SubcommandResult {
    println!("[subcommand.rs] Running subcommand: {:?}", self);

    match self {
      Self::Balances => {
        println!("[subcommand.rs] Balances subcommand");
        balances::run(settings)
      }
      Self::Decode(decode) => {
        println!("[subcommand.rs] Decode subcommand");
        decode.run(settings)
      }
      Self::Env(env) => {
        println!("[subcommand.rs] Env subcommand");
        env.run()
      }
      Self::Epochs => {
        println!("[subcommand.rs] Epochs subcommand");
        epochs::run()
      }
      Self::Find(find) => {
        println!("[subcommand.rs] Find subcommand");
        find.run(settings)
      }
      Self::Index(index) => {
        println!("[subcommand.rs] Index subcommand");
        index.run(settings)
      }
      Self::List(list) => {
        println!("[subcommand.rs] List subcommand");
        list.run(settings)
      }
      Self::Parse(parse) => {
        println!("[subcommand.rs] Parse subcommand");
        parse.run()
      }
      Self::Runes => {
        println!("[subcommand.rs] Runes subcommand");
        runes::run(settings)
      }
      Self::Server(server) => {
        println!("[subcommand.rs] Server subcommand");
        let index = Arc::new(Index::open(&settings)?);
        let handle = axum_server::Handle::new();
        LISTENERS.lock().unwrap().push(handle.clone());
        server.run(settings, index, handle)
      }
      Self::Settings => {
        println!("[subcommand.rs] Settings subcommand");
        settings::run(settings)
      }
      Self::Subsidy(subsidy) => {
        println!("[subcommand.rs] Subsidy subcommand");
        subsidy.run()
      }
      Self::Supply => {
        println!("[subcommand.rs] Supply subcommand");
        supply::run()
      }
      Self::Teleburn(teleburn) => {
        println!("[subcommand.rs] Teleburn subcommand");
        teleburn.run()
      }
      Self::Traits(traits) => {
        println!("[subcommand.rs] Traits subcommand");
        traits.run()
      }
      Self::Wallet(wallet) => {
        println!("[subcommand.rs] Wallet subcommand");
        wallet.run(settings)
      }
    }
  }
}

#[derive(clap::ValueEnum, Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub enum OutputFormat {
  #[default]
  Json,
  Yaml,
  Minify,
}

pub trait Output: Send {
  fn print(&self, format: OutputFormat);
}

impl<T> Output for T
where
  T: Serialize + Send,
{
  fn print(&self, format: OutputFormat) {
    match format {
      OutputFormat::Json => serde_json::to_writer_pretty(io::stdout(), self).ok(),
      OutputFormat::Yaml => serde_yaml::to_writer(io::stdout(), self).ok(),
      OutputFormat::Minify => serde_json::to_writer(io::stdout(), self).ok(),
    };
    println!();
  }
}

pub(crate) type SubcommandResult = Result<Option<Box<dyn Output>>>;
