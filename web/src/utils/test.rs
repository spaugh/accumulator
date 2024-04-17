use anyhow::Context;
use async_trait::async_trait;
use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use http_body_util::BodyExt;
use miden_crypto::hash::rpo::{Rpo256, RpoDigest};
use miden_crypto::merkle::{MmrPeaks, MmrProof};
use serde_json;
use tower::ServiceExt;

use client::Client;

pub struct TestClient {
    router: axum::Router,
}

impl TestClient {
    pub fn new(router: axum::Router) -> Self {
        Self { router }
    }

    pub async fn add_data(&self, data: &str) -> anyhow::Result<usize> {
        let digest = Rpo256::hash(data.as_bytes());
        self.add_leaf(digest).await
    }
}

#[async_trait]
impl Client for TestClient {
    async fn add_leaf(&self, leaf: RpoDigest) -> anyhow::Result<usize> {
        let request = axum::http::Request::builder()
            .method(axum::http::Method::POST)
            .uri("/leaves")
            .body(leaf.to_string())
            .expect("request build failed");
        let response = self
            .router
            .clone()
            .oneshot(request)
            .await
            .expect("request failed");
        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        serde_json::from_slice(&body).with_context(|| "Failed to deserialize response")
    }

    async fn get_peaks(&self, block: usize) -> anyhow::Result<MmrPeaks> {
        let request = Request::builder()
            .method(http::Method::GET)
            .uri(format!("/blocks/{}/peaks", block))
            .body(Body::empty())
            .expect("request build failed");
        let response = self
            .router
            .clone()
            .oneshot(request)
            .await
            .expect("request failed");
        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        serde_json::from_slice(&body).with_context(|| "Failed to deserialize response")
    }

    async fn get_proof(&self, block: usize, index: usize) -> anyhow::Result<MmrProof> {
        let request = Request::builder()
            .method(http::Method::GET)
            .uri(format!("/blocks/{}/proofs/{}", block, index))
            .body(Body::empty())
            .expect("request build failed");
        let response = self
            .router
            .clone()
            .oneshot(request)
            .await
            .expect("request failed");
        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        serde_json::from_slice(&body).with_context(|| "Failed to deserialize response")
    }
}
