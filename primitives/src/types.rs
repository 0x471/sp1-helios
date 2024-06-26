use common::utils::hex_str_to_bytes;
use std::ops::Deref;

use ssz_rs::prelude::*;

pub use ssz_rs::prelude::{Bitvector, Vector};
use crate::utils::header_deserialize;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ByteVector<const N: usize> {
    inner: Vector<u8, N>,
}

impl<const N: usize> ByteVector<N> {
    pub fn as_slice(&self) -> &[u8] {
        self.inner.as_slice()
    }
}

impl<const N: usize> Deref for ByteVector<N> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.inner.as_slice()
    }
}

impl<const N: usize> TryFrom<Vec<u8>> for ByteVector<N> {
    type Error = eyre::Report;

    fn try_from(value: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        Ok(Self {
            inner: Vector::try_from(value).map_err(|(_, err)| err)?,
        })
    }
}

impl<const N: usize> TryFrom<&[u8]> for ByteVector<N> {
    type Error = eyre::Report;

    fn try_from(value: &[u8]) -> std::result::Result<Self, Self::Error> {
        Ok(Self {
            inner: Vector::try_from(value.to_vec()).map_err(|(_, err)| err)?,
        })
    }
}

impl<const N: usize> ssz_rs::Merkleized for ByteVector<N> {
    fn hash_tree_root(&mut self) -> std::result::Result<Node, MerkleizationError> {
        self.inner.hash_tree_root()
    }
}

impl<const N: usize> ssz_rs::Sized for ByteVector<N> {
    fn size_hint() -> usize {
        0
    }

    fn is_variable_size() -> bool {
        false
    }
}

impl<const N: usize> ssz_rs::Serialize for ByteVector<N> {
    fn serialize(&self, buffer: &mut Vec<u8>) -> std::result::Result<usize, SerializeError> {
        self.inner.serialize(buffer)
    }
}

impl<const N: usize> ssz_rs::Deserialize for ByteVector<N> {
    fn deserialize(encoding: &[u8]) -> std::result::Result<Self, DeserializeError>
    where
        Self: std::marker::Sized,
    {
        Ok(Self {
            inner: Vector::deserialize(encoding)?,
        })
    }
}

impl<const N: usize> ssz_rs::SimpleSerialize for ByteVector<N> {}

impl<'de, const N: usize> serde::Deserialize<'de> for ByteVector<N> {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let bytes: String = serde::Deserialize::deserialize(deserializer)?;
        let bytes = hex::decode(bytes.strip_prefix("0x").unwrap()).unwrap();
        Ok(Self {
            inner: bytes.to_vec().try_into().unwrap(),
        })
    }
}

impl<const N: usize> serde::Serialize for ByteVector<N> {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = hex::encode(self.inner.to_vec());
        let output = format!("0x{}", value);
        serializer.collect_str(&output)
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ByteList<const N: usize> {
    inner: List<u8, N>,
}

impl<const N: usize> ByteList<N> {
    pub fn as_slice(&self) -> &[u8] {
        self.inner.as_slice()
    }
}

impl<const N: usize> Deref for ByteList<N> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.inner.as_slice()
    }
}

impl<const N: usize> TryFrom<Vec<u8>> for ByteList<N> {
    type Error = eyre::Report;

    fn try_from(value: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        Ok(Self {
            inner: List::try_from(value).map_err(|(_, err)| err)?,
        })
    }
}

impl<const N: usize> TryFrom<&[u8]> for ByteList<N> {
    type Error = eyre::Report;

    fn try_from(value: &[u8]) -> std::result::Result<Self, Self::Error> {
        Ok(Self {
            inner: List::try_from(value.to_vec()).map_err(|(_, err)| err)?,
        })
    }
}

impl<const N: usize> ssz_rs::Merkleized for ByteList<N> {
    fn hash_tree_root(&mut self) -> std::result::Result<Node, MerkleizationError> {
        self.inner.hash_tree_root()
    }
}

impl<const N: usize> ssz_rs::Sized for ByteList<N> {
    fn size_hint() -> usize {
        0
    }

    fn is_variable_size() -> bool {
        false
    }
}

impl<const N: usize> ssz_rs::Serialize for ByteList<N> {
    fn serialize(&self, buffer: &mut Vec<u8>) -> std::result::Result<usize, SerializeError> {
        self.inner.serialize(buffer)
    }
}

impl<const N: usize> ssz_rs::Deserialize for ByteList<N> {
    fn deserialize(encoding: &[u8]) -> std::result::Result<Self, DeserializeError>
    where
        Self: std::marker::Sized,
    {
        Ok(Self {
            inner: List::deserialize(encoding)?,
        })
    }
}

impl<const N: usize> ssz_rs::SimpleSerialize for ByteList<N> {}

impl<'de, const N: usize> serde::Deserialize<'de> for ByteList<N> {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let bytes: String = serde::Deserialize::deserialize(deserializer)?;
        let bytes = hex::decode(bytes.strip_prefix("0x").unwrap()).unwrap();
        Ok(Self {
            inner: bytes.to_vec().try_into().unwrap(),
        })
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct U64 {
    inner: u64,
}

impl U64 {
    pub fn as_u64(&self) -> u64 {
        self.inner
    }
}

impl From<U64> for u64 {
    fn from(value: U64) -> Self {
        value.inner
    }
}

impl From<u64> for U64 {
    fn from(value: u64) -> Self {
        Self { inner: value }
    }
}

impl ssz_rs::Merkleized for U64 {
    fn hash_tree_root(&mut self) -> std::result::Result<Node, MerkleizationError> {
        self.inner.hash_tree_root()
    }
}

impl ssz_rs::Sized for U64 {
    fn size_hint() -> usize {
        0
    }

    fn is_variable_size() -> bool {
        false
    }
}

impl ssz_rs::Serialize for U64 {
    fn serialize(&self, buffer: &mut Vec<u8>) -> std::result::Result<usize, SerializeError> {
        self.inner.serialize(buffer)
    }
}

impl ssz_rs::Deserialize for U64 {
    fn deserialize(encoding: &[u8]) -> std::result::Result<Self, DeserializeError>
    where
        Self: std::marker::Sized,
    {
        Ok(Self {
            inner: u64::deserialize(encoding)?,
        })
    }
}

impl ssz_rs::SimpleSerialize for U64 {}

impl<'de> serde::Deserialize<'de> for U64 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let val: String = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self {
            inner: val.parse().unwrap(),
        })
    }
}

impl serde::Serialize for U64 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let output = format!("{}", self.inner);
        serializer.collect_str(&output)
    }
}

pub type Bytes32 = ByteVector<32>;
pub type BLSPubKey = ByteVector<48>;
pub type BLSPubKeyUncompressed = ByteVector<96>;
pub type SignatureBytes = ByteVector<96>;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, SimpleSerialize)]
pub struct Header {
    pub slot: U64,
    pub proposer_index: U64,
    pub parent_root: Bytes32,
    pub state_root: Bytes32,
    pub body_root: Bytes32,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, SimpleSerialize)]
pub struct SyncCommittee {
    pub pubkeys: Vector<BLSPubKey, 512>,
    pub aggregate_pubkey: BLSPubKey,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, SimpleSerialize)]
pub struct SyncAggregate {
    pub sync_committee_bits: Bitvector<512>,
    pub sync_committee_signature: SignatureBytes,
}

#[derive(SimpleSerialize, Default, Debug)]
pub struct SigningData {
    pub object_root: Bytes32,
    pub domain: Bytes32,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ChainConfig {
    pub chain_id: u64,
    pub genesis_time: u64,
    #[serde(
        deserialize_with = "bytes_deserialize",
        serialize_with = "bytes_serialize"
    )]
    pub genesis_root: Vec<u8>,
}

impl From<&Update> for GenericUpdate {
    fn from(update: &Update) -> Self {
        Self {
            attested_header: update.attested_header.clone(),
            sync_aggregate: update.sync_aggregate.clone(),
            signature_slot: update.signature_slot.into(),
            next_sync_committee: Some(update.next_sync_committee.clone()),
            next_sync_committee_branch: Some(update.next_sync_committee_branch.clone()),
            finalized_header: Some(update.finalized_header.clone()),
            finality_branch: Some(update.finality_branch.clone()),
        }
    }
}
pub struct GenericUpdate {
    pub attested_header: Header,
    pub sync_aggregate: SyncAggregate,
    pub signature_slot: u64,
    pub next_sync_committee: Option<SyncCommittee>,
    pub next_sync_committee_branch: Option<Vec<Bytes32>>,
    pub finalized_header: Option<Header>,
    pub finality_branch: Option<Vec<Bytes32>>,
}

#[derive(Debug, Default)]
pub struct LightClientStore {
    pub finalized_header: Header,
    pub current_sync_committee: SyncCommittee,
    pub next_sync_committee: Option<SyncCommittee>,
    pub optimistic_header: Header,
    pub previous_max_active_participants: u64,
    pub current_max_active_participants: u64,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct Update {
    #[serde(deserialize_with = "header_deserialize")]
    pub attested_header: Header,
    pub next_sync_committee: SyncCommittee,
    pub next_sync_committee_branch: Vec<Bytes32>,
    #[serde(deserialize_with = "header_deserialize")]
    pub finalized_header: Header,
    pub finality_branch: Vec<Bytes32>,
    pub sync_aggregate: SyncAggregate,
    pub signature_slot: U64,
}
#[derive(SimpleSerialize, Default, Debug)]
pub struct ForkData {
    pub current_version: Vector<u8, 4>,
    pub genesis_validator_root: Bytes32,
}
