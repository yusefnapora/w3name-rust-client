use std::f64::consts::E;

use reqwest::{Client, Url, Response};
use governor::{RateLimiter, Quota, state::{NotKeyed, InMemoryState}, clock::DefaultClock};
use nonzero_ext::nonzero;

use crate::{revision::Revision, name::Name, ipns::{revision_to_ipns_entry, IpnsError, serialize_ipns_entry, deserialize_ipns_entry, revision_from_ipns_entry, validate_ipns_entry}, WritableName};

const DEFAULT_ENDPOINT: &str = "https://name.web3.storage";
const RATE_LIMIT_REQUESTS: u32 = 30;

pub struct W3NameClient {
  endpoint: Url,
  http: Client,
  limiter: RateLimiter<NotKeyed, InMemoryState, DefaultClock>,
}

impl W3NameClient {
  pub fn new(endpoint: Url) -> Self {
    W3NameClient { 
      endpoint: endpoint,
      http: Client::new(),
      limiter: RateLimiter::direct(Quota::per_second(nonzero!(RATE_LIMIT_REQUESTS)))
    }
  }

  pub async fn publish(&self, name: &WritableName, revision: &Revision) -> Result<(), ServiceError> {
    let mut url = self.endpoint.clone();
    url.set_path(format!("name/{}", name.to_string()).as_str());

    let entry = revision_to_ipns_entry(revision, name.keypair())?;
    let encoded = serialize_ipns_entry(&entry)?;
    let body = base64::encode(encoded);

    self.limiter.until_ready().await;
    
    let res = self.http.post(url)
      .body(body)
      .send()
      .await?;

    if res.status().is_success() {
      Ok(())
    } else {
      Err(error_from_response(res).await)
    }
  }

  pub async fn resolve(&self, name: Name) -> Result<Revision, ServiceError> {
    let mut url = self.endpoint.clone();
    url.set_path(format!("name/{}", name.to_string()).as_str());

    self.limiter.until_ready().await;
    let res = self.http.get(url)
      .send()
      .await?;

    parse_resolve_response(res).await
  }
}

impl Default for W3NameClient {
  fn default() -> Self {
    let url = Url::parse(DEFAULT_ENDPOINT).unwrap();
    Self::new(url)
  }
}

async fn parse_resolve_response(res: Response) -> Result<Revision, ServiceError> {
  let r = res.json::<ResolveResponse>().await?;
  let entry_bytes = base64::decode(r.record).map_err(|_| ServiceError::GenericError("unable to base64 decode record in response".to_string()))?;
  let entry = deserialize_ipns_entry(&entry_bytes)?;
  validate_ipns_entry(&entry)?;

  let revision = revision_from_ipns_entry(&entry)?;
  Ok(revision)
}

#[derive(Debug, serde::Deserialize)]
struct APIErrorResponse {
  message: String
}

#[derive(Debug, serde::Deserialize)]
struct ResolveResponse {
  record: String
}

#[derive(Debug)]
pub enum ServiceError {
  GenericError(String),

  APIError(String),
  RequestError(reqwest::Error),
  Ipns(IpnsError),
}

impl From<reqwest::Error> for ServiceError {
  fn from(e: reqwest::Error) -> Self {
    ServiceError::RequestError(e)
  }
}

impl From<IpnsError> for ServiceError {
  fn from(e: IpnsError) -> Self {
    ServiceError::Ipns(e)
  }
}

async fn error_from_response(res: Response) -> ServiceError {
  match res.json::<APIErrorResponse>().await {
    Ok(json) => ServiceError::APIError(json.message),
    Err(e) => ServiceError::GenericError(format!("unexpected response from API, unable to parse error message: {e}"))
  }
}