//! A client library for the w3name distributed naming service.
//! 
//! This library contains an HTTP client for the [w3name service](https://github.com/web3-storage/w3name),
//! an implementation of the [IPNS](https://docs.ipfs.tech/concepts/ipns/) decentralized naming protocol.
//! 
//! w3name allows permissionless creation and publication of name records, which are signed with a private key
//! that corresponds to a public key embedded in the name identifier.
//! 
//! Name identifiers are the string form of a public key, encoded as a [CID](https://docs.ipfs.tech/concepts/content-addressing/).
//! They look something like this: `k51qzi5uqu5dka3tmn6ipgsrq1u2bkuowdwlqcw0vibledypt1y9y5i8v8xwvu`
//! 
//! The `w3name` crate provides a few types for working with names and name records:
//! - [Name] is a representation of a name identifier. It contains the public verification key.
//!   `Name`s can be used to fetch and verify the latest published value for a name record.
//! - [WritableName] contains a private key that can be used to sign and publish name records.
//! - [Revision] represents an unsigned name record. It contains a string value and some metadata (sequence number, expiration date, etc).
//! 
//! The [W3NameClient] type provides a [reqwest](https://docs.rs/reqwest/latest/reqwest/)-based HTTP client
//! for the w3name service. Using the client, you can [resolve](W3NameClient::resolve) the value for a [Name] and/or
//! [publish](W3NameClient::publish) a new [Revision] for a [WritableName].
//! 
//! Note that the client requires a [tokio](https://tokio.rs) runtime, as it uses the async reqwest implementation.
//! For a real-world example of using the client, see [w3name-cli](https://crates.io/crates/w3name-cli).
//! 
//! ## Errors
//! 
//! This crate uses the [error-stack](https://docs.rs/error-stack/latest/error_stack/) library for error handling,
//! so all `Err` branches of `Result`s return a `Report<E>`, where `Report` is an 
//! [error-stack `Report` struct](https://docs.rs/error-stack/latest/error_stack/struct.Report.html) and `E` is
//! one of the types defined in this crate's [error] module. 
//! 
//! If you don't care about the full report, you can get the error instance out of the `Report` using 
//! [`Report::current_context()`](https://docs.rs/error-stack/latest/error_stack/struct.Report.html#method.current_context).

mod client;
pub mod error;
mod ipns;
mod name;
mod revision;

// Include the `ipns_pb` module, which is generated from ipns/ipns_pb.proto.
mod ipns_pb {
  include!(concat!(env!("OUT_DIR"), "/ipns_pb.rs"));
}

pub use client::W3NameClient;
pub use name::{Name, WritableName};
pub use revision::Revision;
