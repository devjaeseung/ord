use {
  super::*,
  clap::builder::styling::{AnsiColor, Effects, Styles},
};

#[derive(Debug, Parser)]
#[command(
  version,
  styles = Styles::styled()
    .header(AnsiColor::Green.on_default() | Effects::BOLD)
    .usage(AnsiColor::Green.on_default() | Effects::BOLD)
    .literal(AnsiColor::Blue.on_default() | Effects::BOLD)
    .placeholder(AnsiColor::Cyan.on_default()))
]
pub(crate) struct Arguments {
  #[command(flatten)]
  pub(crate) options: Options,
  #[command(subcommand)]
  pub(crate) subcommand: Subcommand,
}

impl Arguments {
  pub(crate) fn run(self) -> SnafuResult<Option<Box<dyn subcommand::Output>>> {

    println!("[arguments.rs] Running with options: {:?}", self.options);
    println!("[arguments.rs] Subcommand: {:?}", self.subcommand);

    let mut env: BTreeMap<String, String> = BTreeMap::new();

    for (variable, value) in env::vars_os() {
      let Some(variable) = variable.to_str() else {
        continue;
      };

      let Some(key) = variable.strip_prefix("ORD_") else {
        continue;
      };

      env.insert(
        key.into(),
        value
          .into_string()
          .map_err(|value| SnafuError::EnvVarUnicode {
            backtrace: Backtrace::capture(),
            value,
            variable: variable.into(),
          })?,
      );
    }

    Ok(self.subcommand.run(Settings::load(self.options)?)?)
    // let settings = Settings::load(self.options)?;
    // println!("[arguments.rs] Loaded settings: {:?}", settings);
    //
    // let result = self.subcommand.run(settings)?;
    // println!("[arguments.rs] Subcommand result: {:?}", result);
    //
    // Ok(result)
  }
}
