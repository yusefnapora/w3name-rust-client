use crate::{name::Name, error::CborError};
use chrono::{DateTime, Duration, SecondsFormat, Utc};
use error_stack::{Result, ResultExt, IntoReport};

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

  pub fn v0<S: AsRef<str>>(name: &Name, value: S) -> Revision {
    Revision::new(name, value, default_validity(), 0)
  }

  pub fn v0_with_validity<S: AsRef<str>>(
    name: &Name,
    value: S,
    validity: DateTime<Utc>,
  ) -> Revision {
    Revision::new(name, value, validity, 0)
  }

  pub fn increment<S: AsRef<str>>(&self, value: S) -> Revision {
    let sequence = self.sequence + 1;
    Revision {
      name: self.name.clone(),
      value: value.as_ref().to_string(),
      sequence,
      validity: default_validity(),
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

  pub fn encode(&self) -> Result<Vec<u8>, CborError> {
    let data = RevisionCbor {
      name: self.name.to_string(),
      value: self.value.clone(),
      sequence: self.sequence,
      validity: self.validity_string(),
    };
    let bytes = serde_cbor::to_vec(&data).report().change_context(CborError)?;
    Ok(bytes)
  }

  pub fn decode(bytes: &[u8]) -> Result<Revision, CborError> {
    let data: RevisionCbor = serde_cbor::from_slice(bytes).report().change_context(CborError)?;
    let name = Name::parse(data.name).change_context(CborError)?;
    let validity = DateTime::parse_from_rfc3339(&data.validity).report().change_context(CborError)?;

    let rev = Revision {
        name,
        value: data.value,
        sequence: data.sequence,
        validity: validity.into(),
    };

    Ok(rev)
  }
}

fn default_validity() -> DateTime<Utc> {
  Utc::now().checked_add_signed(Duration::weeks(52)).unwrap()
}


#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct RevisionCbor {
  name: String,
  value: String,
  sequence: u64,
  validity: String,
}


#[cfg(test)]
mod tests {
  use crate::WritableName;

use super::*;

  fn make_test_revision(value: &str) -> Revision {
    let w = WritableName::new();
    Revision::v0(&w.to_name(), value)
  }

  #[test]
  fn serde_roundtrip() {
    let rev = make_test_revision("it's a test");

    let rev_bytes = rev.encode().expect("encoding error");
    let rev2 = Revision::decode(&rev_bytes).expect("decode error");

    assert_eq!(rev, rev2);
  }

}