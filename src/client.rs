use reqwest::{Client, Url};
use governor::{RateLimiter, Quota, state::{NotKeyed, InMemoryState}, clock::DefaultClock};
use nonzero_ext::nonzero;

use crate::{revision::Revision, name::Name};

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

  pub async fn publish(&self, revision: Revision) -> Result<(), ServiceError> {
    let mut url = self.endpoint.clone();
    url.set_path(revision.name().to_string().as_str());

    self.limiter.until_ready().await;
    // self.http.post(url);

    // TODO: need to create IPNS record body

    todo!()
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

pub enum ServiceError {
  GenericError(String)
}