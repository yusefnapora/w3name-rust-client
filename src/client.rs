use std::f64::consts::E;

use reqwest::{Client, Url, Response};
use governor::{RateLimiter, Quota, state::{NotKeyed, InMemoryState}, clock::DefaultClock};
use nonzero_ext::nonzero;

use crate::{revision::Revision, name::Name, ipns::{revision_to_ipns_entry, IpnsError, serialize_ipns_entry}, WritableName};

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
    url.set_path(revision.name().to_string().as_str());

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

  pub async fn resolve(name: Name) -> Result<Revision, ServiceError> {
    todo!()
  }
}

impl Default for W3NameClient {
  fn default() -> Self {
    let url = Url::parse(DEFAULT_ENDPOINT).unwrap();
    Self::new(url)
  }
}

#[derive(Debug, serde::Deserialize)]
struct APIErrorResponse {
  message: String
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