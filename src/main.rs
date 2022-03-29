use std::hash::Hasher;
use std::hash::BuildHasher;
use ahash::RandomState;

fn main() {
    let hash_builder = RandomState::with_seeds(
        0xbb8c484891ec6c86,
        0x0522a25ae9c769f9,
        0xeed2797b9571bc75,
        0x4feb29c1fbbd59d0,
    );

    let mut hasher = hash_builder.build_hasher();
    hasher.write_u64(0);
    let v = hasher.finish();
    println!("hash: {}", v);
}
