use std::sync::RwLock;

use anyhow::{self, Context};
use async_trait::async_trait;
use miden_crypto::{
    hash::rpo::RpoDigest,
    merkle::{Mmr, MmrPeaks, MmrProof},
};

pub struct InMemoryClient {
    mmr: RwLock<Mmr>, 
}

impl InMemoryClient {
    pub fn new() -> Self {
        InMemoryClient {
            mmr: RwLock::new(Mmr::new()) 
        }
    }
}

#[async_trait]
impl crate::Client for InMemoryClient {
    async fn add_leaf(&self, leaf: RpoDigest) -> anyhow::Result<usize> {
        let index = self.mmr.read().unwrap().forest();
        self.mmr.write().unwrap().add(leaf);
        Ok(index)
    }

    async fn get_peaks(&self, block_id: usize) -> anyhow::Result<MmrPeaks> {
        self.mmr.read().unwrap().peaks(block_id).with_context(|| "Failed to get peaks")
    }

    async fn get_proof(&self, block_id: usize, index: usize) -> anyhow::Result<MmrProof> {
        self.mmr.read().unwrap().open(index, block_id).with_context(|| "Failed to get proof")
    }
}
