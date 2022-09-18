use std::{error::Error, fs, path::PathBuf, fmt::Display};

use clap::{Parser, Subcommand};
use error_stack::{Result, ResultExt, IntoReport};

use w3name::{Name, Revision, W3NameClient, WritableName};

#[derive(Parser)]
#[clap(version, about, long_about = None)]
struct Cli {
  #[clap(subcommand)]
  command: Commands,
}

#[derive(Subcommand)]
enum Commands {
  Resolve {
    #[clap(value_parser)]
    name: String,
  },

  Publish {
    #[clap(short, long, value_parser, value_name = "KEY_FILE")]
    key: PathBuf,

    #[clap(short, long, value_parser)]
    value: String,
  },

  Create {
    #[clap(short, long, value_parser)]
    output: Option<PathBuf>,
  },
}

#[tokio::main]
async fn main() {
  let cli = Cli::parse();

  use Commands::*;
  match &cli.command {
    Resolve { name } => {
      resolve(name).await.expect("resolve error");
    }

    Publish { key, value } => {
      publish(key, value).await.expect("publish error");
    }

    Create { output } => {
      create(output).expect("error creating name");
    }
  }
}

async fn resolve(name_str: &str) -> Result<(), CliError> {
  let client = W3NameClient::default();
  let name = Name::parse(name_str).change_context(CliError)?;
  let revision = client.resolve(&name).await.change_context(CliError)?;

  println!("{}", revision.value());
  Ok(())
}

fn create(output: &Option<PathBuf>) -> Result<(), CliError> {
  let name = WritableName::new();
  let output = output
    .clone()
    .unwrap_or_else(|| PathBuf::from(format!("{}.key", name.to_string())));

  let bytes = name.keypair().to_protobuf_encoding().report().change_context(CliError)?;
  fs::write(&output, bytes).report().change_context(CliError)?;
  println!("wrote new keypair to {}", output.display());
  Ok(())
}

async fn publish(key_file: &PathBuf, value: &str) -> Result<(), CliError> {
  let client = W3NameClient::default();
  let key_bytes = fs::read(key_file).report().change_context(CliError)?;
  let writable = WritableName::from_private_key(&key_bytes).change_context(CliError)?;

  // to avoid having to keep old revisions around, we first try to resolve and increment any existing records
  let new_revision = match client.resolve(&writable.to_name()).await {
    Ok(revision) => revision.increment(value),
    Err(_) => Revision::v0(&writable.to_name(), value),
  };

  client.publish(&writable, &new_revision).await.change_context(CliError)?;

  println!(
    "published new value for key {}: {}",
    writable.to_string(),
    value
  );
  Ok(())
}


#[derive(Debug)]
struct CliError;

impl Display for CliError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "error")
  }
}

impl Error for CliError {}