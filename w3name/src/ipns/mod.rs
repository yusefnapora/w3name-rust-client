use crate::{
  error::{
    CborError, InvalidIpnsV1Signature, InvalidIpnsV2Signature, InvalidIpnsV2SignatureData,
    IpnsError, SigningError,
  },
  ipns_pb::IpnsEntry,
  Name, Revision,
};
use chrono::{DateTime, Utc};
use libp2p_core::identity::{Keypair, PublicKey};
use prost::Message;
use std::str::from_utf8;

use error_stack::{report, IntoReport, Result, ResultExt};

pub fn revision_to_ipns_entry(
  revision: &Revision,
  signer: &Keypair,
) -> Result<IpnsEntry, IpnsError> {
  let value = revision.value().as_bytes().to_vec();
  let validity = revision.validity_string().as_bytes().to_vec();

  let duration = revision.validity().signed_duration_since(Utc::now());
  let ttl: u64 = duration.num_nanoseconds().unwrap_or(i64::MAX) as u64;

  let signature = create_v1_signature(&signer, &value, &validity).change_context(IpnsError)?;
  let data = v2_signature_data(
    revision.value(),
    &revision.validity_string(),
    revision.sequence(),
    ttl,
  )
  .change_context(IpnsError)?;
  let signature_v2 = create_v2_signature(&signer, &data).change_context(IpnsError)?;
  let entry = IpnsEntry {
    value,
    validity,
    sequence: revision.sequence(),
    validity_type: 0,

    pub_key: vec![],
    signature,
    ttl,
    signature_v2,
    data,
  };

  Ok(entry)
}

pub fn serialize_ipns_entry(entry: &IpnsEntry) -> Result<Vec<u8>, IpnsError> {
  let mut buf = Vec::new();
  buf.reserve(entry.encoded_len());
  entry.encode(&mut buf).report().change_context(IpnsError)?;
  Ok(buf)
}

pub fn deserialize_ipns_entry(entry_bytes: &[u8]) -> Result<IpnsEntry, IpnsError> {
  let entry = IpnsEntry::decode(entry_bytes)
    .report()
    .change_context(IpnsError)?;
  Ok(entry)
}

pub fn validate_ipns_entry(entry: &IpnsEntry, public_key: &PublicKey) -> Result<(), IpnsError> {
  if !entry.signature_v2.is_empty() && !entry.data.is_empty() {
    validate_v2_signature(public_key, &entry.signature_v2, &entry.data).change_context(IpnsError)?;
    validate_v2_data_matches_entry_data(entry).change_context(IpnsError)?;

    return Ok(());
  }

  validate_v1_signature(entry, public_key).change_context(IpnsError)
}

pub fn revision_from_ipns_entry(entry: &IpnsEntry, name: &Name) -> Result<Revision, IpnsError> {
  let value = from_utf8(&entry.value)
    .report()
    .change_context(IpnsError)?;
  let validity_str = from_utf8(&entry.validity)
    .report()
    .change_context(IpnsError)?;
  let validity = DateTime::parse_from_rfc3339(validity_str)
    .report()
    .change_context(IpnsError)?;

  let rev = Revision::new(name, value, validity.into(), entry.sequence);
  Ok(rev)
}

fn v1_signature_data(value_bytes: &[u8], validity_bytes: &[u8]) -> Vec<u8> {
  let mut buf = value_bytes.to_vec();
  buf.extend("EOL".as_bytes()); // validity type (we only support Eol)
  buf.extend(validity_bytes);
  buf
}

fn v2_signature_data(
  value: &str,
  validity: &str,
  sequence: u64,
  ttl: u64,
) -> Result<Vec<u8>, CborError> {
  let data = SignatureV2Data {
    Value: value.as_bytes().to_vec(),
    Validity: validity.as_bytes().to_vec(),
    ValidityType: 0,
    Sequence: sequence,
    TTL: ttl,
  };
  let encoded = serde_cbor::to_vec(&data)
    .report()
    .change_context(CborError)?;

  Ok(encoded)
}

fn validate_v2_signature(
  public_key: &PublicKey,
  sig: &[u8],
  data: &[u8],
) -> Result<(), InvalidIpnsV2Signature> {
  let mut msg = "ipns-signature:".as_bytes().to_vec();
  msg.extend_from_slice(data);
  if public_key.verify(&msg, sig) {
    Ok(())
  } else {
    Err(report!(InvalidIpnsV2Signature))
  }
}

fn validate_v2_data_matches_entry_data(
  entry: &IpnsEntry,
) -> Result<(), InvalidIpnsV2SignatureData> {
  if entry.data.is_empty() {
    return Err(report!(InvalidIpnsV2SignatureData));
  }

  let data: SignatureV2Data = serde_cbor::from_slice(&entry.data[..])
    .report()
    .change_context(InvalidIpnsV2SignatureData)?;
  if entry.value != data.Value
    || entry.validity != data.Validity
    || entry.sequence != data.Sequence
    || entry.ttl != data.TTL
    || entry.validity_type != data.ValidityType
  {
    Err(report!(InvalidIpnsV2SignatureData))
  } else {
    Ok(())
  }
}

fn validate_v1_signature(
  entry: &IpnsEntry,
  public_key: &PublicKey,
) -> Result<(), InvalidIpnsV1Signature> {
  let data = v1_signature_data(&entry.value, &entry.validity);
  if public_key.verify(&data, &entry.signature) {
    Ok(())
  } else {
    Err(report!(InvalidIpnsV1Signature))
  }
}

fn create_v1_signature(
  signer: &Keypair,
  value_bytes: &[u8],
  validity_bytes: &[u8],
) -> Result<Vec<u8>, SigningError> {
  let msg = v1_signature_data(value_bytes, validity_bytes);
  let sig = signer.sign(&msg).report().change_context(SigningError)?;
  Ok(sig)
}

fn create_v2_signature(signer: &Keypair, sig_data: &[u8]) -> Result<Vec<u8>, SigningError> {
  let mut msg = "ipns-signature:".as_bytes().to_vec();
  msg.extend_from_slice(sig_data);
  let sig = signer.sign(&msg).report().change_context(SigningError)?;
  Ok(sig)
}

#[allow(non_snake_case)]
#[derive(serde::Serialize, serde::Deserialize)]
struct SignatureV2Data {
  #[serde(with = "serde_bytes")]
  Value: Vec<u8>,
  #[serde(with = "serde_bytes")]
  Validity: Vec<u8>,
  ValidityType: i32,
  Sequence: u64,
  TTL: u64,
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::WritableName;
  use chrono::Duration;

  #[test]
  fn to_ipns() {
    let name = WritableName::new();
    let value = "such value. much wow".to_string();
    let validity = Utc::now().checked_add_signed(Duration::weeks(52)).unwrap();
    let rev = Revision::v0_with_validity(&name.to_name(), &value, validity);
    assert_eq!(rev.sequence(), 0);
    assert_eq!(rev.name(), &name.to_name());
    assert_eq!(rev.value(), &value);
    assert_eq!(rev.validity(), &validity);

    let entry = revision_to_ipns_entry(&rev, name.keypair()).unwrap();
    assert_eq!(rev.sequence(), entry.sequence);
    assert_eq!(rev.value().as_bytes(), &entry.value);
    assert_eq!(rev.validity_string().as_bytes(), &entry.validity);
  }

  #[test]
  fn round_trip() {
    let name = WritableName::new();
    let value = "such value. much wow".to_string();
    let rev = Revision::v0(&name.to_name(), &value);

    let entry = revision_to_ipns_entry(&rev, name.keypair()).unwrap();

    validate_ipns_entry(&entry, &name.keypair().public()).unwrap();

    let rev2 = revision_from_ipns_entry(&entry, &name.to_name()).unwrap();
    assert_eq!(rev, rev2);
  }
}
