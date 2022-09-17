use libp2p_core::identity::{Keypair, PublicKey};
use prost::Message;

use crate::{ipns_pb::IpnsEntry, revision::Revision};
use std::collections::BTreeMap;
use libipld::ipld::Ipld;
use libipld::prelude::Codec;
use libipld::cbor::DagCborCodec;

pub fn revision_to_ipns_entry(revision: &Revision, signer: &Keypair) -> Result<IpnsEntry, IpnsError> {
  let value = revision.value().as_bytes().to_vec();
  let validity = revision.validity().as_bytes().to_vec();
  let ttl: u64 = 0; // TODO: set based on expiration time

  let signature = create_v1_signature(&signer, &value, &validity)?;
  let data = v2_signature_data(revision.value(), revision.validity(), revision.sequence(), ttl)?;
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

  todo!("validate v1 signature if v2 sig is missing")
}

pub fn revision_from_ipns_entry(_entry: &IpnsEntry) -> Result<Revision, IpnsError> {
  todo!()
}

fn v1_signature_data(value_bytes: &[u8], validity_bytes: &[u8]) -> Vec<u8> {
  let mut buf = value_bytes.to_vec();
  buf.extend("EOL".as_bytes()); // validity type (we only support Eol)
  buf.extend(validity_bytes);
  buf
}

fn v2_signature_data(value: &str, validity: &str, sequence: u64, ttl: u64) -> Result<Vec<u8>, IpnsError> {
  let mut data = BTreeMap::new();
  data.insert("Value".to_string(), Ipld::String(value.to_string()));
  data.insert("Validity".to_string(), Ipld::String(validity.to_string()));
  data.insert("ValidityType".to_string(), Ipld::Integer(0));
  data.insert("Sequence".to_string(), Ipld::Integer(sequence as i128));
  data.insert("TTL".to_string(), Ipld::Integer(ttl as i128));

  let encoded = DagCborCodec.encode(&data)?.to_vec();
  Ok(encoded)
}

fn validate_v2_signature(public_key: &PublicKey, sig: &[u8], data: &[u8]) -> Result<(), IpnsError> {
  todo!()
}

fn validate_v2_data_matches_entry_data(entry: &IpnsEntry) -> Result<(), IpnsError> {
  todo!()
}

fn create_v1_signature(signer: &Keypair, value_bytes: &[u8], validity_bytes: &[u8]) -> Result<Vec<u8>, IpnsError> {
  let msg = v1_signature_data(value_bytes, validity_bytes);
  let sig = signer.sign(&msg)?;
  Ok(sig)
}

fn create_v2_signature(signer: &Keypair, sig_data: &[u8]) -> Result<Vec<u8>, IpnsError> {
  let mut msg = "ipns-signature:".as_bytes().to_vec();
  msg.extend(sig_data);
  let sig = signer.sign(&msg)?;
  Ok(sig)
}

#[derive(Debug)]
pub enum IpnsError {
  SigningError(libp2p_core::identity::error::SigningError),
  CborEncodingError(libipld::error::Error),
  ProtobufEncodingError(prost::EncodeError),
  ProtobufDecodingError(prost::DecodeError),
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