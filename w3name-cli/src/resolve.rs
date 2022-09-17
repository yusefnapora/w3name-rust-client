use std::error::Error;

use w3name::{Name, W3NameClient};

pub async fn resolve(name_str: &str) -> Result<String, Box<dyn Error>> {
  let client = W3NameClient::default();
  let name = Name::parse(name_str)?;
  let revision = client.resolve(&name).await?;
  Ok(revision.value().to_string())
}