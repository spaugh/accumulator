use anyhow;
use async_trait::async_trait;
use miden_crypto::{
    hash::rpo::RpoDigest,
    merkle::{MmrPeaks, MmrProof},
};
use reqwest;

pub struct HttpClient {
    client: reqwest::Client,
    base_uri: String,
}

impl HttpClient {
    pub fn new(base_uri: String) -> Self {
        HttpClient {
            client: reqwest::Client::new(),
            base_uri,
        }
    }
}

#[async_trait]
impl crate::Client for HttpClient {
    async fn add_leaf(&self, leaf: RpoDigest) -> anyhow::Result<usize> {
        let uri = format!("{}/leaves", self.base_uri);
        let response = self
            .client
            .post(uri)
            .body(leaf.to_string())
            .send()
            .await?
            .error_for_status()?
            .json::<usize>()
            .await?;
        Ok(response)
    }

    async fn get_peaks(&self, block_id: usize) -> anyhow::Result<MmrPeaks> {
        let uri = format!("{}/blocks/{}/peaks", self.base_uri, block_id);
        let response = self
            .client
            .get(uri)
            .send()
            .await?
            .error_for_status()?
            .json::<MmrPeaks>()
            .await?;
        Ok(response)
    }

    async fn get_proof(&self, block_id: usize, index: usize) -> anyhow::Result<MmrProof> {
        let response = self
            .client
            .get(format!(
                "{}/blocks/{}/proofs/{}",
                self.base_uri, block_id, index
            ))
            .send()
            .await?
            .error_for_status()?
            .json::<MmrProof>()
            .await?;
        Ok(response)
    }
}
