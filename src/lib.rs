mod client;
mod ipns;
mod name;
mod revision;

// Include the `ipns_pb` module, which is generated from ipns/ipns_pb.proto.
mod ipns_pb {
  include!(concat!(env!("OUT_DIR"), "/ipns_pb.rs"));
}

pub use client::{ServiceError, W3NameClient};
pub use name::{Name, NameError, WritableName};
pub use revision::Revision;
