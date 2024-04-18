use anyhow;
use miden_crypto::{
    hash::rpo::{Rpo256, RpoDigest},
    merkle::{MmrPeaks, MmrProof},
};
mod http;
mod memory;
use async_trait::async_trait;
pub use http::*;
pub use memory::*;

#[async_trait]
pub trait Client {
    async fn add_data(&self, data: &str) -> anyhow::Result<usize> {
        let digest = Rpo256::hash(data.as_bytes());
        self.add_leaf(digest).await
    }
    async fn verify(&self, data: &str, index: usize) -> anyhow::Result<bool> {
        let digest = Rpo256::hash(data.as_bytes());
        let proof = self.get_proof(index + 1, index).await?;
        let peaks = self.get_peaks(index + 1).await.unwrap();
        let verified = peaks.verify(digest, proof);
        Ok(verified)
    }
    async fn add_leaf(&self, leaf: RpoDigest) -> anyhow::Result<usize>;
    async fn get_peaks(&self, block_id: usize) -> anyhow::Result<MmrPeaks>;
    async fn get_proof(&self, block_id: usize, index: usize) -> anyhow::Result<MmrProof>;
}
