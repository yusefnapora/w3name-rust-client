use std::io::Result;

fn main() -> Result<()> {
  #[cfg(feature = "protoc-src")]
  std::env::set_var("PROTOC", protobuf_src::protoc());

  prost_build::compile_protos(&["src/ipns/ipns_pb.proto"], &["src/ipns"])?;
  Ok(())
}
