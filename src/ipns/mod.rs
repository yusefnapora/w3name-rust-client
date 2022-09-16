use libp2p_core::identity::Keypair;

use crate::{ipns_pb::IpnsEntry, revision::Revision, name::WritableName};
use std::collections::BTreeMap;
use libipld::ipld::Ipld;
use libipld::prelude::Codec;
use libipld::cbor::DagCborCodec;

pub fn revision_to_ipns_entry(revision: &Revision, signer: Keypair) -> Result<IpnsEntry, IpnsError> {
  let value = revision.value().as_bytes().to_vec();
  let validity = revision.validity().as_bytes().to_vec();
  let ttl: u64 = 0; // TODO: set based on expiration time

  let signature = create_v1_signature(&signer, &value, &validity)?;
  let signature_v2 = create_v2_signature(&signer, revision.value(), revision.validity(), revision.sequence(), ttl)?;
  let entry = IpnsEntry {
    value: Some(value),
    validity: Some(validity),
    sequence: Some(revision.sequence()),
    validity_type: Some(0),

    pub_key: None,
    signature: Some(signature),
    ttl: Some(ttl),
    signature_v2: Some(signature_v2),
    data: None,
  };

  Ok(entry)
}


fn create_v1_signature(signer: &Keypair, value_bytes: &[u8], validity_bytes: &[u8]) -> Result<Vec<u8>, IpnsError> {
  let mut buf = value_bytes.to_vec();
  buf.extend("EOL".as_bytes()); // validity type (we only support Eol)
  buf.extend(validity_bytes);
  signer.sign(&buf).map_err(|_| IpnsError::SigningError)
}

fn create_v2_signature(signer: &Keypair, value: &str, validity: &str, sequence: u64, ttl: u64) -> Result<Vec<u8>, IpnsError> {
  let mut data = BTreeMap::new();
  data.insert("Value".to_string(), Ipld::String(value.to_string()));
  data.insert("Validity".to_string(), Ipld::String(validity.to_string()));
  data.insert("ValidityType".to_string(), Ipld::Integer(0));
  data.insert("Sequence".to_string(), Ipld::Integer(sequence as i128));
  data.insert("TTL".to_string(), Ipld::Integer(ttl as i128));

  let encoded = DagCborCodec.encode(&data).map_err(|_| IpnsError::CborEncodingError)?.to_vec();
  signer.sign(&encoded).map_err(|_| IpnsError::SigningError)
}

pub enum IpnsError {
  SigningError,
  CborEncodingError,
}