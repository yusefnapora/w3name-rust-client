mod client;
mod ipns;
mod name;
mod revision;

// Include the `ipns_pb` module, which is generated from ipns/ipns_pb.proto.
pub mod ipns_pb {
    include!(concat!(env!("OUT_DIR"), "/ipns_pb.rs"));
}


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
