use cid::Cid;
use libp2p_core::identity::{Keypair, PublicKey};
use multibase::Base;
use multihash::derive::Multihash;
use multihash::MultihashDigest;

// we need to rename Result here, because the multihash derive macro gets confused otherwise
use error_stack::{report, IntoReport, Result as ResultStack, ResultExt};

use crate::error::{InvalidCidString, InvalidMulticodecCode, NameError};

const LIBP2P_MULTICODEC: u64 = 0x72;

/// Name is an IPNS key ID.
///
/// Names can be used to retrieve the latest published value from the W3name service
/// using [W3NameClient](crate::client::W3NameClient).
///
/// Note that `Name` contains only the public verification key and does not allow publishing
/// or updating records. To create or update a record, use the [WritableName] type instead.
///
/// To convert from a string representation of a name to a `Name` struct, use the [Name::parse](Self::parse) function.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Name(PublicKey);

impl Name {
  pub fn parse<S: AsRef<str>>(s: S) -> ResultStack<Name, NameError> {
    let res = Cid::try_from(s.as_ref());
    let c = res
      .map_err(|_| InvalidCidString)
      .report()
      .change_context(NameError)?;
    if c.codec() != LIBP2P_MULTICODEC {
      return Err(report!(InvalidMulticodecCode).change_context(NameError));
    }

    let key_bytes = c.hash().digest();
    let pk = PublicKey::from_protobuf_encoding(key_bytes)
      .report()
      .change_context(NameError)?;
    Ok(Name(pk))
  }

  pub fn public_key(&self) -> &PublicKey {
    &self.0
  }

  pub fn to_cid(&self) -> Cid {
    let key_bytes = self.0.to_protobuf_encoding();
    let hash = Hasher::Identity.digest(&key_bytes[..]);
    Cid::new_v1(LIBP2P_MULTICODEC, hash)
  }

  pub fn to_bytes(&self) -> Vec<u8> {
    self.to_cid().to_bytes()
  }

  pub fn to_string(&self) -> String {
    self.to_cid().to_string_of_base(Base::Base36Lower).unwrap()
  }
}

#[derive(Clone, Debug)]
pub struct WritableName(Keypair);

impl WritableName {
  pub fn new() -> WritableName {
    let kp = Keypair::generate_ed25519();
    WritableName(kp)
  }

  pub fn from_private_key(key_bytes: &[u8]) -> ResultStack<WritableName, NameError> {
    let mut kb = key_bytes.to_vec(); // from_protobuf_encoding takes &mut, so clone instead of requiring the same
    let kp = Keypair::from_protobuf_encoding(&mut kb)
      .report()
      .change_context(NameError)?;
    Ok(WritableName(kp))
  }

  pub fn keypair(&self) -> &Keypair {
    &self.0
  }

  pub fn to_name(&self) -> Name {
    Name(self.0.public())
  }

  pub fn to_cid(&self) -> Cid {
    self.to_name().to_cid()
  }

  pub fn to_bytes(&self) -> Vec<u8> {
    self.to_name().to_bytes()
  }

  pub fn to_string(&self) -> String {
    self.to_name().to_string()
  }
}

#[derive(Clone, Copy, Debug, Eq, Multihash, PartialEq)]
#[mh(alloc_size = 64)]
enum Hasher {
  #[mh(code = 0x0, hasher = multihash::IdentityHasher::<64>)]
  Identity,
}

#[cfg(test)]
mod tests {
  use std::str::FromStr;

  use super::*;
  use base64;

  #[test]
  fn create_writable_name() {
    let name = WritableName::new();

    let cid = Cid::from_str(name.to_string().as_str()).unwrap();
    assert_eq!(cid.codec(), LIBP2P_MULTICODEC);
    assert_eq!(cid.hash().code(), 0x0); // identity hash code
    assert_eq!(cid, name.to_cid());
  }

  #[test]
  fn writable_name_from_private_key() {
    let name_str = "k51qzi5uqu5dkgso0xihmnkn1sthxgs3nilzmofwy29jrplwdtk6sc14x9f2zv";
    let private_key_base64 = "CAESQI8NcJgBK+9qfSBz/ZiXNuw4OJkUTn4jWZvd3Sj8W6GLq900cwz32d6ylbqBl81WRgM6QvSEXMwGlEODgEkXCes=";
    let private_key = base64::decode(private_key_base64).unwrap();
    let name = WritableName::from_private_key(&private_key).unwrap();
    assert_eq!(name.to_string(), name_str);
  }

  #[test]
  fn parse_name() {
    let name_str = "k51qzi5uqu5dl2hq2hm5m29sdq1lum0kb0lmyqsowicmrmxzxywwgxhy6ymrdv";
    let name = Name::parse(name_str).expect("parse error");
    assert_eq!(name_str, name.to_string());

    // it fails to parse a CIDv0
    let invalid_cidv0 = "QmPFpDRC87jTdSYxjnEZUTjJuYF5yLRWxir3DzJ1XiVZ3t";
    assert!(Name::parse(invalid_cidv0).is_err());

    // it fails to parse a non libp2p-key codec name
    let invalid = "k2jmtxx8tc9pv6b9sj5wm71mheawu849x2bzkjuecpwizjwjeufiadl6";
    assert!(Name::parse(invalid).is_err());
  }
}
