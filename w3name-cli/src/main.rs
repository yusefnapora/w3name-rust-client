use clap::{Parser, Subcommand};

mod resolve;

use crate::resolve::resolve;

#[derive(Parser)]
#[clap(version, about, long_about = None)]
struct Cli {
  #[clap(subcommand)]
  command: Commands
}

#[derive(Subcommand)]
enum Commands {
  Resolve {

    #[clap(value_parser)]
    name: String,
  },

  // TODO: publish
}

#[tokio::main]
async fn main() {
  let cli = Cli::parse();
  match &cli.command {
    Commands::Resolve { name } => {
      let value = resolve(name).await.expect("resolve error");
      println!("{}", value);
    },

    // TODO: publish
  }
}
