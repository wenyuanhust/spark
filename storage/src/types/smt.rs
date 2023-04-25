use ethereum_types::H160;
use rocksdb::DBVector;
use smt_rocksdb_store::default_store::DefaultStoreMultiTree;
use sparse_merkle_tree::{blake2b::Blake2bHasher, traits::Value, SparseMerkleTree, H256};

pub type DefaultStoreMultiSMT<'a, T, W> =
    SparseMerkleTree<Blake2bHasher, LeafValue, DefaultStoreMultiTree<'a, T, W>>;

pub type Address = H160;
pub type Amount = u128;
pub type Delegator = H160;
pub type Epoch = u64;
pub type Proof = Vec<u8>;
pub type Root = H256;
pub type Staker = H160;

// define SMT value
#[derive(Default, Clone, PartialEq, Eq)]
pub struct LeafValue(pub [u8; 32]);
impl Value for LeafValue {
    fn to_h256(&self) -> H256 {
        self.0.into()
    }

    fn zero() -> Self {
        Self([0u8; 32])
    }
}

impl From<DBVector> for LeafValue {
    fn from(vec: DBVector) -> Self {
        LeafValue(vec.as_ref().try_into().expect("stored value is 32 bytes"))
    }
}

impl AsRef<[u8]> for LeafValue {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}
