use multihash::derive::Multihash;

/// Hasher is a custom Multihash "code table" with just the identity hash function enabled.
#[derive(Clone, Copy, Debug, Eq, Multihash, PartialEq)]
#[mh(alloc_size = 64)]
pub enum Hasher {
  #[mh(code = 0x0, hasher = multihash::IdentityHasher::<64>)]
  Identity,
}
