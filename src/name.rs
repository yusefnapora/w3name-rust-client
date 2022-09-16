use libp2p_core::identity::{Keypair, PublicKey, ed25519};
use multihash::derive::Multihash;
use multihash::MultihashDigest;
use multibase::Base;
use cid::Cid;

const LIBP2P_MULTICODEC: u64 = 0x72;

/// Name is an IPNS key ID.
///
/// Names can be used to retrieve the latest published value from the W3name service
/// using the {@link resolve} function.
///
/// Note that `Name` contains only the public verification key and does not allow publishing
/// or updating records. To create or update a record, use the `WritableName` type instead.
///
/// To convert from a string representation of a name to a `Name` object use the [Name::parse] function.

#[derive(Clone, Debug, PartialEq)]
pub struct Name(PublicKey);

impl Name {
  pub fn parse<S: AsRef<str>>(s: S) -> Result<Name, NameError> {
    let c = Cid::try_from(s.as_ref()).map_err(|_| NameError::InvalidCidString)?;
    if c.codec() != LIBP2P_MULTICODEC {
      return Err(NameError::InvalidMulticodec);
    }

    let key_bytes = c.hash().digest();
    let ed_pk = ed25519::PublicKey::decode(key_bytes).map_err(|_| NameError::InvalidKey)?;
    let pk = PublicKey::Ed25519(ed_pk);
    Ok(Name(pk))
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
    self.to_cid().to_string_of_base(Base::Base36Upper).unwrap()
  }
}

#[derive(Clone, Debug)]
pub struct WritableName(Keypair);

impl WritableName {
  pub fn from_private_key_bytes(key_bytes: &mut [u8]) -> Result<WritableName, NameError> {
    let kp = ed25519::Keypair::decode(key_bytes).map_err(|_| NameError::InvalidKey)?;
    Ok(WritableName(Keypair::Ed25519(kp)))
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

impl WritableName {
  pub fn new() -> WritableName {
    let kp = Keypair::generate_ed25519();
    WritableName(kp)
  }
}

pub enum NameError {
  InvalidCidString,
  InvalidMulticodec,
  InvalidKey,
}

#[derive(Clone, Copy, Debug, Eq, Multihash, PartialEq)]
#[mh(alloc_size = 64)]
enum Hasher {
  #[mh(code = 0x0, hasher = multihash::IdentityHasher::<64>)]
  Identity
}
