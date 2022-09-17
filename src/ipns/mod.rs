use std::str::{from_utf8, Utf8Error};

use crate::{ipns_pb::IpnsEntry, revision::Revision, Name};

use chrono::{DateTime, ParseError, Utc};
use libipld::cbor::DagCborCodec;
use libipld::prelude::Codec;
use libipld::DagCbor;
use libp2p_core::identity::{Keypair, PublicKey};
use prost::Message;

pub fn revision_to_ipns_entry(
  revision: &Revision,
  signer: &Keypair,
) -> Result<IpnsEntry, IpnsError> {
  let value = revision.value().as_bytes().to_vec();
  let validity = revision.validity_string().as_bytes().to_vec();

  let duration = revision.validity().signed_duration_since(Utc::now());
  let ttl: u64 = duration.num_nanoseconds().unwrap_or(i64::MAX) as u64;

  let signature = create_v1_signature(&signer, &value, &validity)?;
  let data = v2_signature_data(
    revision.value(),
    &revision.validity_string(),
    revision.sequence(),
    ttl,
  )?;
  let signature_v2 = create_v2_signature(&signer, &data)?;
  let entry = IpnsEntry {
    value: Some(value),
    validity: Some(validity),
    sequence: Some(revision.sequence()),
    validity_type: Some(0),

    pub_key: None,
    signature: Some(signature),
    ttl: Some(ttl),
    signature_v2: Some(signature_v2),
    data: Some(data),
  };

  Ok(entry)
}

pub fn serialize_ipns_entry(entry: &IpnsEntry) -> Result<Vec<u8>, IpnsError> {
  let mut buf = Vec::new();
  buf.reserve(entry.encoded_len());
  entry.encode(&mut buf)?;
  Ok(buf)
}

pub fn deserialize_ipns_entry(entry_bytes: &[u8]) -> Result<IpnsEntry, IpnsError> {
  let entry = IpnsEntry::decode(entry_bytes)?;
  Ok(entry)
}

pub fn validate_ipns_entry(entry: &IpnsEntry, public_key: &PublicKey) -> Result<(), IpnsError> {
  if entry.signature_v2.is_some() && entry.data.is_some() {
    let sig = entry.signature_v2();
    let data = entry.data();
    validate_v2_signature(public_key, sig, data)?;
    validate_v2_data_matches_entry_data(entry)?;
    return Ok(());
  }

  validate_v1_signature(entry, public_key)
}

pub fn revision_from_ipns_entry(entry: &IpnsEntry, name: &Name) -> Result<Revision, IpnsError> {
  let value = from_utf8(entry.value())?;
  let validity_str = from_utf8(entry.validity())?;
  let validity = DateTime::parse_from_rfc3339(validity_str)?;

  let rev = Revision::new(name, value, validity.into(), entry.sequence());
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
) -> Result<Vec<u8>, IpnsError> {
  let data = SignatureV2Data {
    Value: value.to_string(),
    Validity: validity.to_string(),
    ValidityType: 0,
    Sequence: sequence,
    TTL: ttl,
  };
  let encoded = DagCborCodec.encode(&data)?.to_vec();
  Ok(encoded)
}

fn validate_v2_signature(public_key: &PublicKey, sig: &[u8], data: &[u8]) -> Result<(), IpnsError> {
  let mut msg = "ipns-signature:".as_bytes().to_vec();
  msg.extend_from_slice(data);
  if public_key.verify(&msg, sig) {
    Ok(())
  } else {
    Err(IpnsError::InvalidSignatureV2)
  }
}

fn validate_v2_data_matches_entry_data(entry: &IpnsEntry) -> Result<(), IpnsError> {
  if entry.data.is_none() {
    return Err(IpnsError::InvalidSignatureV2);
  }

  let data: SignatureV2Data = DagCborCodec.decode(entry.data())?;
  if entry.value() != data.Value.as_bytes()
    || entry.validity() != data.Validity.as_bytes()
    || entry.sequence() != data.Sequence
    || entry.ttl() != data.TTL
    || entry.validity_type != Some(data.ValidityType)
  {
    return Err(IpnsError::SignatureV2DataMismatch);
  }
  Ok(())
}

fn validate_v1_signature(entry: &IpnsEntry, public_key: &PublicKey) -> Result<(), IpnsError> {
  let data = v1_signature_data(entry.value(), entry.validity());
  let sig = entry.signature();
  if public_key.verify(&data, sig) {
    Ok(())
  } else {
    Err(IpnsError::InvalidSignatureV1)
  }
}

fn create_v1_signature(
  signer: &Keypair,
  value_bytes: &[u8],
  validity_bytes: &[u8],
) -> Result<Vec<u8>, IpnsError> {
  let msg = v1_signature_data(value_bytes, validity_bytes);
  let sig = signer.sign(&msg)?;
  Ok(sig)
}

fn create_v2_signature(signer: &Keypair, sig_data: &[u8]) -> Result<Vec<u8>, IpnsError> {
  let mut msg = "ipns-signature:".as_bytes().to_vec();
  msg.extend_from_slice(sig_data);
  let sig = signer.sign(&msg)?;
  Ok(sig)
}

#[derive(Debug)]
pub enum IpnsError {
  SigningError(libp2p_core::identity::error::SigningError),
  CborEncodingError(libipld::error::Error),
  ProtobufEncodingError(prost::EncodeError),
  ProtobufDecodingError(prost::DecodeError),
  InvalidSignatureV1,
  InvalidSignatureV2,
  SignatureV2DataMismatch,

  InvalidUtf8(Utf8Error),
  InvalidDateString(ParseError),
}

impl From<libipld::error::Error> for IpnsError {
  fn from(e: libipld::error::Error) -> Self {
    IpnsError::CborEncodingError(e)
  }
}

impl From<libp2p_core::identity::error::SigningError> for IpnsError {
  fn from(e: libp2p_core::identity::error::SigningError) -> Self {
    IpnsError::SigningError(e)
  }
}

impl From<prost::EncodeError> for IpnsError {
  fn from(e: prost::EncodeError) -> Self {
    IpnsError::ProtobufEncodingError(e)
  }
}

impl From<prost::DecodeError> for IpnsError {
  fn from(e: prost::DecodeError) -> Self {
    IpnsError::ProtobufDecodingError(e)
  }
}

impl From<Utf8Error> for IpnsError {
  fn from(e: Utf8Error) -> Self {
    IpnsError::InvalidUtf8(e)
  }
}

impl From<ParseError> for IpnsError {
  fn from(e: ParseError) -> Self {
    IpnsError::InvalidDateString(e)
  }
}

#[allow(non_snake_case)]
#[derive(DagCbor)]
struct SignatureV2Data {
  Value: String,
  Validity: String,
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
    let rev = Revision::v0(&name.to_name(), &value, validity);
    assert_eq!(rev.sequence(), 0);
    assert_eq!(rev.name(), &name.to_name());
    assert_eq!(rev.value(), &value);
    assert_eq!(rev.validity(), &validity);

    let entry = revision_to_ipns_entry(&rev, name.keypair()).unwrap();
    assert_eq!(rev.sequence(), entry.sequence());
    assert_eq!(rev.value().as_bytes(), entry.value());
    assert_eq!(rev.validity_string().as_bytes(), entry.validity());
  }

  #[test]
  fn round_trip() {
    let name = WritableName::new();
    let value = "such value. much wow".to_string();
    let validity = Utc::now().checked_add_signed(Duration::weeks(52)).unwrap();
    let rev = Revision::v0(&name.to_name(), &value, validity);

    let entry = revision_to_ipns_entry(&rev, name.keypair()).unwrap();

    validate_ipns_entry(&entry, &name.keypair().public()).unwrap();

    let rev2 = revision_from_ipns_entry(&entry, &name.to_name()).unwrap();
    assert_eq!(rev, rev2);
  }
}
