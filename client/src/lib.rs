use anyhow;
use miden_crypto::{
    hash::rpo::RpoDigest,
    merkle::{MmrPeaks, MmrProof},
};
mod http;
use async_trait::async_trait;
pub use http::*;

#[async_trait]
pub trait Client {
    async fn add_leaf(&self, leaf: RpoDigest) -> anyhow::Result<usize>;
    async fn get_peaks(&self, block_id: usize) -> anyhow::Result<MmrPeaks>;
    async fn get_proof(&self, block_id: usize, index: usize) -> anyhow::Result<MmrProof>;
}
