use error_stack::{report, IntoReport, Report, Result, ResultExt};
use governor::{
  clock::DefaultClock,
  state::{InMemoryState, NotKeyed},
  Quota, RateLimiter,
};
use nonzero_ext::nonzero;
use reqwest::{Client, Response, Url};

use crate::{
  error::{APIError, ClientError, HttpError, UnexpectedAPIResponse},
  ipns::{
    deserialize_ipns_entry, revision_from_ipns_entry, revision_to_ipns_entry, serialize_ipns_entry,
    validate_ipns_entry,
  },
  Name, Revision, WritableName,
};

const DEFAULT_ENDPOINT: &str = "https://name.web3.storage";
const RATE_LIMIT_REQUESTS: u32 = 30;

pub struct W3NameClient {
  endpoint: Url,
  http: Client,
  limiter: RateLimiter<NotKeyed, InMemoryState, DefaultClock>,
}

impl W3NameClient {
  pub fn new(endpoint: Url) -> Self {
    let http = Client::new();
    let limiter = RateLimiter::direct(Quota::per_second(nonzero!(RATE_LIMIT_REQUESTS)));
    W3NameClient {
      endpoint,
      http,
      limiter,
    }
  }

  pub async fn publish(&self, name: &WritableName, revision: &Revision) -> Result<(), ClientError> {
    let mut url = self.endpoint.clone();
    url.set_path(format!("name/{}", name.to_string()).as_str());

    let entry = revision_to_ipns_entry(revision, name.keypair()).change_context(ClientError)?;
    let encoded = serialize_ipns_entry(&entry).change_context(ClientError)?;
    let body = base64::encode(encoded);

    self.limiter.until_ready().await;

    let res = self
      .http
      .post(url)
      .body(body)
      .send()
      .await
      .report()
      .change_context(HttpError)
      .change_context(ClientError)?;

    if res.status().is_success() {
      Ok(())
    } else {
      Err(error_from_response(res).await)
    }
  }

  pub async fn resolve(&self, name: &Name) -> Result<Revision, ClientError> {
    let mut url = self.endpoint.clone();
    url.set_path(format!("name/{}", name.to_string()).as_str());

    self.limiter.until_ready().await;
    let res = self
      .http
      .get(url)
      .send()
      .await
      .report()
      .change_context(HttpError)
      .change_context(ClientError)?;

    parse_resolve_response(&name, res).await
  }
}

impl Default for W3NameClient {
  fn default() -> Self {
    let url = Url::parse(DEFAULT_ENDPOINT).unwrap();
    Self::new(url)
  }
}

async fn parse_resolve_response(name: &Name, res: Response) -> Result<Revision, ClientError> {
  let r = res
    .json::<ResolveResponse>()
    .await
    .report()
    .change_context(ClientError)?;
  let entry_bytes = base64::decode(r.record)
    .report()
    .change_context(ClientError)?;
  let entry = deserialize_ipns_entry(&entry_bytes).change_context(ClientError)?;
  validate_ipns_entry(&entry, name.public_key()).change_context(ClientError)?;

  let revision = revision_from_ipns_entry(&entry, name).change_context(ClientError)?;
  Ok(revision)
}

#[derive(Debug, serde::Deserialize)]
struct APIErrorResponse {
  message: String,
}

#[derive(Debug, serde::Deserialize)]
struct ResolveResponse {
  record: String,
}

async fn error_from_response(res: Response) -> Report<ClientError> {
  match res.json::<APIErrorResponse>().await {
    Ok(json) => report!(APIError(json.message)).change_context(ClientError),
    Err(e) => report!(e)
      .change_context(UnexpectedAPIResponse)
      .change_context(ClientError),
  }
}
