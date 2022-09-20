use std::fmt::Display;

use crate::{error::ProtobufError, hash::Hasher};
use cid::Cid;
use libp2p_core::identity::{Keypair, PublicKey};
use multibase::Base;
use multihash::MultihashDigest;

use error_stack::{report, IntoReport, Result, ResultExt};

use crate::error::{InvalidCidString, InvalidMulticodecCode, NameError};

const LIBP2P_MULTICODEC: u64 = 0x72;

/// `Name` is a representation of an IPNS name identifier, which is also a public verification key.
///
/// `Name`s can be used to retrieve the latest published value from the w3name service
/// using [W3NameClient::resolve](crate::W3NameClient::resolve).
///
/// Note that `Name` contains only the public verification key and does not allow publishing
/// or updating records. To create or update a record, use the [WritableName] type instead.
///
/// To convert from a string representation of a name to a `Name` struct, use the [Name::parse](Self::parse) function.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Name(PublicKey);

impl Name {
  /// Parses a `Name` from the string form of a name identifier.
  ///
  /// ## Example
  ///
  /// ```rust
  /// # fn main() -> error_stack::Result<(), w3name::error::NameError> {
  /// use w3name::Name;
  ///
  /// let name_str = "k51qzi5uqu5dka3tmn6ipgsrq1u2bkuowdwlqcw0vibledypt1y9y5i8v8xwvu";
  /// let name = Name::parse(name_str)?;
  ///
  /// assert_eq!(name_str, &name.to_string());
  ///
  /// let invalid_name_str = "not a valid public key string";
  /// assert!(Name::parse(invalid_name_str).is_err());
  ///
  /// # Ok(())
  /// # }
  /// ```
  pub fn parse<S: AsRef<str>>(s: S) -> Result<Name, NameError> {
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

  /// Returns a reference to this `Name`'s [PublicKey].
  ///
  /// ## Example
  ///
  /// ```rust
  /// # fn main() -> error_stack::Result<(), w3name::error::NameError> {
  /// use w3name::Name;
  /// use libp2p_core::identity::PublicKey;
  ///
  /// let name = Name::parse("k51qzi5uqu5dka3tmn6ipgsrq1u2bkuowdwlqcw0vibledypt1y9y5i8v8xwvu")?;
  ///
  /// match name.public_key() {
  ///   &PublicKey::Ed25519(_) => println!("it's an ed25519 key, alright"),
  ///   _ => panic!("that's odd, I could have sworn that the key was ed25519..."),
  /// }
  ///
  /// # Ok(())
  /// # }
  ///
  /// ```
  pub fn public_key(&self) -> &PublicKey {
    &self.0
  }

  /// Returns this `Name` encoded as a [Cid], using the "identity" hash function to embed the key into the Cid itself.
  ///
  /// ## Example
  ///
  /// ```rust
  /// # fn main() -> error_stack::Result<(), w3name::error::NameError> {
  /// use w3name::Name;
  ///
  /// let name = Name::parse("k51qzi5uqu5dka3tmn6ipgsrq1u2bkuowdwlqcw0vibledypt1y9y5i8v8xwvu")?;
  ///
  /// let cid = name.to_cid();
  /// // Cid::to_string() returns a base32-encoded string, but w3name uses base36.
  ///
  /// let expected_cid_string = "bafzaajaiaejcbjdinwzcqwpdydtsxcfnvu2qak2zqpsss5zqqf5od54tk4ufkcf2";
  /// assert_eq!(&cid.to_string(), expected_cid_string);
  /// # Ok(())
  /// # }
  /// ```
  pub fn to_cid(&self) -> Cid {
    let key_bytes = self.0.to_protobuf_encoding();
    let hash = Hasher::Identity.digest(&key_bytes[..]);
    Cid::new_v1(LIBP2P_MULTICODEC, hash)
  }

  /// Returns a `Vec<u8>` containing the binary form of the [Cid] representing this `Name`.
  ///
  /// ## Example
  ///
  /// ```rust
  /// # fn main() -> error_stack::Result<(), w3name::error::NameError> {
  /// use w3name::Name;
  /// use cid::Cid;
  ///
  /// let name = Name::parse("k51qzi5uqu5dka3tmn6ipgsrq1u2bkuowdwlqcw0vibledypt1y9y5i8v8xwvu")?;
  ///
  /// let bytes = name.to_bytes();
  /// let cid = Cid::read_bytes(&bytes[..]).unwrap();
  ///
  /// assert_eq!(cid, name.to_cid());
  /// # Ok(())
  /// # }
  /// ```
  pub fn to_bytes(&self) -> Vec<u8> {
    self.to_cid().to_bytes()
  }

  /// Returns the public key in the "canonical" string format for name identifiers used by w3name.
  ///
  /// The returned string is a base36-encoded representation of [Name::to_cid()].
  /// This is the same format expected by [Name::parse()].
  ///
  /// ## Example
  ///
  /// ```rust
  /// # fn main() -> error_stack::Result<(), w3name::error::NameError> {
  /// use w3name::Name;
  ///
  /// let name_str = "k51qzi5uqu5dka3tmn6ipgsrq1u2bkuowdwlqcw0vibledypt1y9y5i8v8xwvu";
  /// let name = Name::parse(name_str)?;
  ///
  /// assert_eq!(name_str, &name.to_string());
  /// # Ok(())
  /// # }
  /// ```
  pub fn to_string(&self) -> String {
    self.to_cid().to_string_of_base(Base::Base36Lower).unwrap()
  }
}

impl Display for Name {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.to_string())
  }
}

/// `WritableName` represnts a public/private keypair that can be used to sign name records for publication.
///
/// You can use a `WritableName` to publish a value to the w3name service using [W3NameClient::publish()](crate::W3NameClient::publish).
///
#[derive(Clone, Debug)]
pub struct WritableName(Keypair);

impl WritableName {
  /// Creates a new `WritableName` by generating an ed25519 keypair.
  pub fn new() -> WritableName {
    let kp = Keypair::generate_ed25519();
    WritableName(kp)
  }

  /// Decodes a `WritableName` from a binary encoding of a keypair as produced by [encode](Self::encode).
  ///
  /// ## Example
  ///
  /// ```rust
  /// # fn main() -> error_stack::Result<(), w3name::error::ProtobufError> {
  ///
  /// use w3name::WritableName;
  ///
  /// let w = WritableName::new();
  /// let bytes = w.encode()?;
  /// let w2 = WritableName::decode(&bytes)?;
  ///
  /// assert_eq!(w, w2);
  ///
  /// # Ok(())
  /// # }
  /// ```
  pub fn decode(key_bytes: &[u8]) -> Result<WritableName, ProtobufError> {
    let mut kb = key_bytes.to_vec(); // from_protobuf_encoding takes &mut, so clone instead of requiring the same
    let kp = Keypair::from_protobuf_encoding(&mut kb)
      .report()
      .change_context(ProtobufError)?;
    Ok(WritableName(kp))
  }

  /// Encodes a `WritableName` into a binary representation, suitable for [decode](Self::decode).
  ///
  /// ## Example
  ///
  /// ```rust
  /// # fn main() -> error_stack::Result<(), w3name::error::ProtobufError> {
  /// use w3name::WritableName;
  ///
  /// let w = WritableName::new();
  /// let bytes = w.encode()?;
  /// let w2 = WritableName::decode(&bytes)?;
  ///
  /// assert_eq!(w, w2);
  /// # Ok(())
  /// # }
  /// ```
  pub fn encode(&self) -> Result<Vec<u8>, ProtobufError> {
    self
      .keypair()
      .to_protobuf_encoding()
      .report()
      .change_context(ProtobufError)
  }

  /// Returns a reference to this `WritableName`'s underlying [Keypair].
  ///
  /// ## Example
  ///
  /// ```rust
  /// use w3name::WritableName;
  /// use libp2p_core::identity::Keypair;
  ///
  /// let w = WritableName::new();
  /// match w.keypair() {
  ///   &Keypair::Ed25519(_) => println!("it's an ed25519 keypair!"),
  ///   _ => panic!("only ed25519 keys are supported, so this shouldn't happen..."),
  /// }
  /// ```
  pub fn keypair(&self) -> &Keypair {
    &self.0
  }

  /// Returns a `Name` that represents the public half of this `WritableName`'s keypair.
  ///
  /// ## Example
  ///
  /// ```rust
  /// use w3name::WritableName;
  ///
  /// let w = WritableName::new();
  /// let n = w.to_name();
  ///
  /// assert_eq!(&w.keypair().public(), n.public_key());
  /// ```
  pub fn to_name(&self) -> Name {
    Name(self.0.public())
  }

  /// Convenience wrapper around `Self::to_name().to_cid()` that returns the Cid form of the **public** portion of this `WritableName`'s keypair.
  ///
  /// Please note that this does not encode the private key.
  /// If you want to save the `WritableName`, use [encode](Self::encode).
  ///
  /// ## Example
  ///
  /// ```rust
  /// use w3name::WritableName;
  ///
  /// let w = WritableName::new();
  /// let n = w.to_name();
  ///
  /// assert_eq!(w.to_cid(), n.to_cid());
  /// ```
  pub fn to_cid(&self) -> Cid {
    self.to_name().to_cid()
  }

  /// Convenience wrapper around `Self::to_name().to_string()` that returns a string encoding of the public key (aka the "name identifier").
  ///
  /// Please note that this does not encode the private key.
  /// If you want to save the `WritableName`, use [encode](Self::encode).
  ///
  /// ## Example
  ///
  /// ```rust
  /// use w3name::WritableName;
  ///
  /// let w = WritableName::new();
  /// let n = w.to_name();
  ///
  /// assert_eq!(w.to_string(), n.to_string());
  pub fn to_string(&self) -> String {
    self.to_name().to_string()
  }
}

impl Display for WritableName {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.to_string())
  }
}

// since Keypair doesn't implement PartialEq, we can't derive it for
// WritableName and need to implement manually.
impl PartialEq for WritableName {
  fn eq(&self, other: &Self) -> bool {
    use Keypair::Ed25519;

    match (self.keypair(), other.keypair()) {
      (Ed25519(our_key), Ed25519(their_key)) => {
        // encode both keys to bytes and return true if they're identical
        let our_key_bytes = our_key.encode();
        let their_key_bytes = their_key.encode();
        our_key_bytes == their_key_bytes
      }

      // we only support Ed25519 keys, so if we have anything else, return false
      _ => false,
    }
  }
}

impl Eq for WritableName {}

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
    let name = WritableName::decode(&private_key).unwrap();
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
