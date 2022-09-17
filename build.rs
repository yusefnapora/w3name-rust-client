use std::io::Result;
fn main() -> Result<()> {
  prost_build::compile_protos(&["src/ipns/ipns_pb.proto"], &["src/ipns"])?;
  Ok(())
}
