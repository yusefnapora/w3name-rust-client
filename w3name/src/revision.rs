use crate::name::Name;
use chrono::{DateTime, SecondsFormat, Utc};

#[derive(Debug, Eq, PartialEq)]
pub struct Revision {
  name: Name,
  value: String,
  sequence: u64,
  validity: DateTime<Utc>,
}

impl Revision {
  pub fn new<S: AsRef<str>>(
    name: &Name,
    value: S,
    validity: DateTime<Utc>,
    sequence: u64,
  ) -> Revision {
    let value = value.as_ref().to_string();
    let name = name.clone();
    Revision {
      name,
      value,
      sequence,
      validity,
    }
  }

  pub fn v0<S: AsRef<str>>(name: &Name, value: S, validity: DateTime<Utc>) -> Revision {
    Revision::new(name, value, validity, 0)
  }

  pub fn increment<S: AsRef<str>>(&self, value: S) -> Revision {
    let sequence = self.sequence + 1;
    Revision {
      name: self.name.clone(),
      value: value.as_ref().to_string(),
      sequence,
      validity: self.validity.clone(),
    }
  }

  pub fn name(&self) -> &Name {
    &self.name
  }

  pub fn value(&self) -> &str {
    &self.value
  }

  pub fn sequence(&self) -> u64 {
    self.sequence
  }

  pub fn validity(&self) -> &DateTime<Utc> {
    &self.validity
  }

  pub fn validity_string(&self) -> String {
    self.validity.to_rfc3339_opts(SecondsFormat::Nanos, true)
  }
}
