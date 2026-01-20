//! Benchmark data types for TCS vs BCS comparison
//!
//! These types mirror the Tapedrive schema and are used to compare
//! serialization performance across different formats.

use serde::{Deserialize, Serialize};
use wincode_derive::{SchemaRead, SchemaWrite};

/// Node roles in the network
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize, SchemaRead, SchemaWrite)]
#[repr(u32)]
pub enum NodeRole {
    #[default]
    Storage = 1,
    Validator = 2,
    Light = 3,
}

/// Status of a storage operation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize, SchemaRead, SchemaWrite)]
#[repr(u32)]
pub enum BlobStatus {
    #[default]
    Pending = 1,
    Stored = 2,
    Verified = 3,
    Failed = 4,
}

/// A simple hash wrapper (32 bytes)
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, SchemaRead, SchemaWrite)]
pub struct Hash {
    pub data: [u8; 32],
}

/// Block header - representative of blockchain header data
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, SchemaRead, SchemaWrite)]
pub struct BlockHeader {
    pub height: u64,
    pub prev_hash: [u8; 32],
    pub merkle_root: [u8; 32],
    pub state_root: [u8; 32],
    pub timestamp: u64,
    pub epoch: u64,
}

/// A slice of erasure-coded data
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, SchemaRead, SchemaWrite)]
pub struct Slice {
    pub slice_index: u32,
    pub data: Vec<u8>,
    pub hash: [u8; 32],
}

/// Metadata for a stored blob
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, SchemaRead, SchemaWrite)]
pub struct BlobMetadata {
    pub blob_id: [u8; 32],
    pub size: u64,
    pub timestamp: u64,
    pub status: BlobStatus,
    pub epoch: u64,
}

/// 64-byte signature (split into two 32-byte arrays for serde compatibility)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SchemaRead, SchemaWrite)]
pub struct Signature {
    pub part1: [u8; 32],
    pub part2: [u8; 32],
}

impl Default for Signature {
    fn default() -> Self {
        Signature {
            part1: [0u8; 32],
            part2: [0u8; 32],
        }
    }
}

/// A transaction in the network
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, SchemaRead, SchemaWrite)]
pub struct Transaction {
    pub tx_hash: [u8; 32],
    pub nonce: u64,
    pub payload: Vec<u8>,
    pub signature: Signature,
    pub sender_role: NodeRole,
}

/// Spool sync request between nodes
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, SchemaRead, SchemaWrite)]
pub struct SpoolSyncRequest {
    pub spool_index: u32,
    pub from_epoch: u64,
    pub to_epoch: u64,
    pub node_id: [u8; 32],
}

/// A batch of transactions (for testing larger payloads)
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, SchemaRead, SchemaWrite)]
pub struct TransactionBatch {
    pub transactions: Vec<Transaction>,
    pub batch_id: u64,
    pub timestamp: u64,
}

/// Generate random test data
pub mod generators {
    use super::*;
    use rand::RngCore;

    pub fn random_bytes_32(rng: &mut impl RngCore) -> [u8; 32] {
        let mut arr = [0u8; 32];
        rng.fill_bytes(&mut arr);
        arr
    }

    pub fn random_signature(rng: &mut impl RngCore) -> Signature {
        Signature {
            part1: random_bytes_32(rng),
            part2: random_bytes_32(rng),
        }
    }

    pub fn random_block_header(rng: &mut impl RngCore) -> BlockHeader {
        BlockHeader {
            height: rng.next_u64(),
            prev_hash: random_bytes_32(rng),
            merkle_root: random_bytes_32(rng),
            state_root: random_bytes_32(rng),
            timestamp: rng.next_u64(),
            epoch: rng.next_u64(),
        }
    }

    pub fn random_transaction(rng: &mut impl RngCore, payload_size: usize) -> Transaction {
        let mut payload = vec![0u8; payload_size];
        rng.fill_bytes(&mut payload);

        Transaction {
            tx_hash: random_bytes_32(rng),
            nonce: rng.next_u64(),
            payload,
            signature: random_signature(rng),
            sender_role: match rng.next_u32() % 3 {
                0 => NodeRole::Storage,
                1 => NodeRole::Validator,
                _ => NodeRole::Light,
            },
        }
    }

    pub fn random_slice(rng: &mut impl RngCore, data_size: usize) -> Slice {
        let mut data = vec![0u8; data_size];
        rng.fill_bytes(&mut data);

        Slice {
            slice_index: rng.next_u32(),
            data,
            hash: random_bytes_32(rng),
        }
    }

    pub fn random_blob_metadata(rng: &mut impl RngCore) -> BlobMetadata {
        BlobMetadata {
            blob_id: random_bytes_32(rng),
            size: rng.next_u64(),
            timestamp: rng.next_u64(),
            status: match rng.next_u32() % 4 {
                0 => BlobStatus::Pending,
                1 => BlobStatus::Stored,
                2 => BlobStatus::Verified,
                _ => BlobStatus::Failed,
            },
            epoch: rng.next_u64(),
        }
    }

    pub fn random_spool_sync_request(rng: &mut impl RngCore) -> SpoolSyncRequest {
        SpoolSyncRequest {
            spool_index: rng.next_u32() % 1024,
            from_epoch: rng.next_u64(),
            to_epoch: rng.next_u64(),
            node_id: random_bytes_32(rng),
        }
    }

    pub fn random_transaction_batch(
        rng: &mut impl RngCore,
        tx_count: usize,
        payload_size: usize,
    ) -> TransactionBatch {
        TransactionBatch {
            transactions: (0..tx_count)
                .map(|_| random_transaction(rng, payload_size))
                .collect(),
            batch_id: rng.next_u64(),
            timestamp: rng.next_u64(),
        }
    }
}
