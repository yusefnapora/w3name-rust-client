use std::{error::Error, fmt::Display, fs, path::PathBuf, process::exit};

use clap::{Parser, Subcommand};
use error_stack::{IntoReport, Result, ResultExt, Report};

use w3name::{Name, Revision, W3NameClient, WritableName, error::{ClientError, APIError}};

#[derive(Parser)]
#[clap(name = "w3name", version, about, long_about = None)]
/// A tool for creating verifiable names in a web3 world.
struct Cli {
  #[clap(subcommand)]
  command: Commands,
}

#[derive(Subcommand)]
enum Commands {
  /// Lookup the current value for a name record.
  Resolve {
    /// The name identifier, e.g. "k51qzi5uqu5dka3tmn6ipgsrq1u2bkuowdwlqcw0vibledypt1y9y5i8v8xwvu"
    #[clap(value_parser)]
    name: String,
  },

  /// Publish a new value for a name, signed with the name's private key.
  Publish {
    /// Path to a key file (see the `create` command to make one).
    #[clap(short, long, value_parser, value_name = "KEY_FILE")]
    key: PathBuf,

    /// The value to publish.
    #[clap(short, long, value_parser)]
    value: String,
  },

  /// Create a new public/private keypair and save it to disk.
  Create {
    /// Filename to write the key to.
    ///
    /// If not given, will write to a file named `<name>.key`,
    /// where `<name>` is the string form of the public key.
    #[clap(short, long, value_parser)]
    output: Option<PathBuf>,
  },
}

#[tokio::main]
async fn main() {
  let cli = Cli::parse();

  use Commands::*;
  let res = match &cli.command {
    Resolve { name } => {
      resolve(name).await
    }

    Publish { key, value } => {
      publish(key, value).await
    }

    Create { output } => {
      create(output)
    }
  };

  if let Err(err_report) = res {
    eprintln!("{err_report:?}");
    exit(1);
  }
}

async fn resolve(name_str: &str) -> Result<(), CliError> {
  let client = W3NameClient::default();
  let name = Name::parse(name_str).change_context(CliError)?;
  match client.resolve(&name).await {
    Ok(revision) => {
      println!("{}", revision.value());
      Ok(())
    }

    Err(err_report) => {
      if is_404(&err_report) {
        eprintln!("no record found for key {}", name_str);
        Ok(())
      } else {
        Err(err_report.change_context(CliError))
      }
    },
  }
}

fn create(output: &Option<PathBuf>) -> Result<(), CliError> {
  let name = WritableName::new();
  let output = output
    .clone()
    .unwrap_or_else(|| PathBuf::from(format!("{}.key", name.to_string())));

  let bytes = name
    .keypair()
    .to_protobuf_encoding()
    .report()
    .change_context(CliError)?;
  fs::write(&output, bytes)
    .report()
    .change_context(CliError)?;
  println!("wrote new keypair to {}", output.display());
  Ok(())
}

async fn publish(key_file: &PathBuf, value: &str) -> Result<(), CliError> {
  let client = W3NameClient::default();
  let key_bytes = fs::read(key_file).report().change_context(CliError)?;
  let writable = WritableName::decode(&key_bytes).change_context(CliError)?;

  // to avoid having to keep old revisions around, we first try to resolve and increment any existing records
  let new_revision = match client.resolve(&writable.to_name()).await {
    Ok(revision) => revision.increment(value),

    // If the API returned a 404, create the initial (v0) Revision.
    // Bail out for all other errors 
    Err(err_report) => { 
      if is_404(&err_report) {
        Revision::v0(&writable.to_name(), value)
      } else {
        return Err(err_report.change_context(CliError))
      }
    },
  };

  client
    .publish(&writable, &new_revision)
    .await
    .change_context(CliError)?;

  println!(
    "published new value for key {}: {}",
    writable.to_string(),
    value
  );
  Ok(())
}


/// Returns true if the error report contains an [APIError] with a 404 status
fn is_404(report: &Report<ClientError>) -> bool {
  let maybe_api_err: Option<&APIError> = report.downcast_ref();
  if let Some(err) = maybe_api_err {
    err.status_code == 404
  } else {
    false
  }
}


#[derive(Debug)]
struct CliError;

impl Display for CliError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "something went wrong")
  }
}

impl Error for CliError {}
