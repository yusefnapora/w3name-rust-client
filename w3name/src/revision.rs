use crate::{name::Name, error::CborError};
use chrono::{DateTime, Duration, SecondsFormat, Utc};
use error_stack::{Result, ResultExt, IntoReport};

/// A `Revision` represents a single value for a name record.
/// 
/// A `Revision` is essentially an IPNS entry without a signature, and it
/// contains all the information needed to construct an IPNS entry for signing.
/// 
/// Each `Revision` contains a sequence number, which must be incremented when publishing
/// updates to an existing `Revision`. To create the initial `Revision` (with sequence number == 0),
/// use [Revision::v0]. Subsequent `Revision`s are created by calling [increment](Revision::increment)
/// on an existing `Revision`.
#[derive(Debug, Eq, PartialEq)]
pub struct Revision {
  name: Name,
  value: String,
  sequence: u64,
  validity: DateTime<Utc>,
}

impl Revision {
  /// Creates a new `Revision`, specifying all fields.
  /// 
  /// Note that this is crate-only; users should use [Self::v0] or [Self:::increment]
  pub(crate) fn new<S: AsRef<str>>(
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

  /// Creates the initial `Revision` for the given [Name], with a sequence number of 0 and the default validity period (1 year).
  ///
  /// ## Example
  /// ```rust
  /// # fn main() -> error_stack::Result<(), w3name::error::NameError> {
  /// use w3name::{Name, Revision};
  /// 
  /// let name = Name::parse("k51qzi5uqu5dka3tmn6ipgsrq1u2bkuowdwlqcw0vibledypt1y9y5i8v8xwvu")?;
  /// let rev = Revision::v0(&name, "an initial value");
  /// 
  /// assert_eq!(&name, rev.name());
  /// assert_eq!(rev.value(), "an initial value");
  /// # Ok(())
  /// # }
  /// 
  /// 
  /// ```
  pub fn v0<S: AsRef<str>>(name: &Name, value: S) -> Revision {
    Revision {
        name: name.clone(),
        value: value.as_ref().to_string(),
        sequence: 0,
        validity: default_validity(),
    }
  }

  /// Creates the initial `Revision` for the given [Name], with an explicit validity period.
  /// 
  /// Note that `validity` is an end-of-life timestamp, not a duration.
  /// 
  /// ## Example
  /// 
  /// ```rust
  /// # fn main () -> error_stack::Result<(), w3name::error::NameError> {
  /// use w3name::{Name, Revision};
  /// use chrono::{Duration, Utc};
  /// 
  /// // set the expiration date to two weeks from now:
  /// let expiration_date = Utc::now().checked_add_signed(Duration::weeks(2)).unwrap();
  /// 
  /// let name = Name::parse("k51qzi5uqu5dka3tmn6ipgsrq1u2bkuowdwlqcw0vibledypt1y9y5i8v8xwvu")?;
  /// let rev = Revision::v0_with_validity(&name, "an initial value", expiration_date);
  ///
  /// assert_eq!(&name, rev.name());
  /// assert_eq!(rev.value(), "an initial value");
  /// assert_eq!(rev.validity(), &expiration_date);
  /// # Ok(())
  /// # }
  /// ```
  pub fn v0_with_validity<S: AsRef<str>>(
    name: &Name,
    value: S,
    validity: DateTime<Utc>,
  ) -> Revision {
    Revision::new(name, value, validity, 0)
  }

  /// Creates a new `Revision` with the given `value` and an incremented sequence number, using the default validity period (1 year).
  ///
  /// ## Example
  /// 
  /// ```rust
  /// # fn main() -> error_stack::Result<(), w3name::error::NameError> {
  /// use w3name::{Name, Revision};
  /// 
  /// let name = Name::parse("k51qzi5uqu5dka3tmn6ipgsrq1u2bkuowdwlqcw0vibledypt1y9y5i8v8xwvu")?;
  /// let rev = Revision::v0(&name, "an initial value");
  /// let rev2 = rev.increment("a new value");
  /// 
  /// assert_eq!(&name, rev.name());
  /// assert_eq!(&name, rev2.name());
  /// assert_eq!(rev.sequence(), 0);
  /// assert_eq!(rev2.sequence(), 1);
  /// assert_eq!(rev.value(), "an initial value");
  /// assert_eq!(rev2.value(), "a new value");
  /// 
  /// # Ok(())
  /// # }
  /// ```
  pub fn increment<S: AsRef<str>>(&self, value: S) -> Revision {
    Self::increment_with_validity(&self, value, default_validity())
  }

  /// Creates a new `Revision` with the given `value` and an incremented sequence number, with an explicit validity period.
  /// 
  /// Note that `validity` is an end-of-life timestamp, not a duration.
  pub fn increment_with_validity<S: AsRef<str>>(&self, value: S, validity: DateTime<Utc>) -> Revision {
    let sequence = self.sequence + 1;
    Revision {
      name: self.name.clone(),
      value: value.as_ref().to_string(),
      sequence,
      validity,
    }
  }

  /// Returns a reference to this `Revision`'s [Name].
  pub fn name(&self) -> &Name {
    &self.name
  }

  /// Returns a reference to this `Revision`'s value.
  pub fn value(&self) -> &str {
    &self.value
  }

  /// Returns this `Revision`'s sequence number.
  pub fn sequence(&self) -> u64 {
    self.sequence
  }

  /// Returns this `Revision`'s validity period (end of life date).
  pub fn validity(&self) -> &DateTime<Utc> {
    &self.validity
  }

  /// Returns this `Revision`'s validity period as a String, suitable for inclusion in an IPNS record.
  pub fn validity_string(&self) -> String {
    self.validity.to_rfc3339_opts(SecondsFormat::Nanos, true)
  }

  /// Encodes this `Revision` to a binary form, suitable for use with [Revision::decode].
  /// 
  /// Note that encoded `Revision`s are not signed and cannot be used directly as IPNS records.
  /// 
  /// ## Example
  /// 
  /// ```rust
  /// # fn main() -> error_stack::Result<(), w3name::error::CborError> {
  /// use w3name::{Name, Revision};
  /// 
  /// let name = Name::parse("k51qzi5uqu5dka3tmn6ipgsrq1u2bkuowdwlqcw0vibledypt1y9y5i8v8xwvu").unwrap();
  /// let rev = Revision::v0(&name, "an initial value"); 
  /// 
  /// let bytes = rev.encode()?;
  /// let rev2 = Revision::decode(&bytes)?;
  /// 
  /// assert_eq!(rev, rev2);
  /// 
  /// # Ok(())
  /// # }
  /// ```
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

  /// Decodes a `Revision` from a binary form as produced by [Revision::encode].
  /// 
  /// ## Example
  /// 
  /// ```rust
  /// # fn main() -> error_stack::Result<(), w3name::error::CborError> {
  /// use w3name::{Name, Revision};
  /// 
  /// let name = Name::parse("k51qzi5uqu5dka3tmn6ipgsrq1u2bkuowdwlqcw0vibledypt1y9y5i8v8xwvu").unwrap();
  /// let rev = Revision::v0(&name, "an initial value"); 
  /// 
  /// let bytes = rev.encode()?;
  /// let rev2 = Revision::decode(&bytes)?;
  /// 
  /// assert_eq!(rev, rev2);
  /// 
  /// # Ok(())
  /// # }
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