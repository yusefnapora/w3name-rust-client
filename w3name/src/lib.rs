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
