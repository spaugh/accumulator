use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{
    extract::Extension,
    response::Json,
    routing::post,
    Router,
};
use serde_json::json;
use std::sync::{Arc, Mutex};
use tower_http::trace::TraceLayer;

use miden_crypto::merkle::Mmr;

use miden_crypto::{
    hash::rpo::{Rpo256, RpoDigest},
    Felt, Word, EMPTY_WORD, ZERO,
};

#[derive(Clone)]
pub struct AppState {
    mmr: Arc<Mutex<Mmr>>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            mmr: Arc::new(Mutex::new(Mmr::new())),
        }
    }
}

pub struct Witness {
    pub data: Vec<i32>,
}

const fn int_to_node(value: u64) -> RpoDigest {
    RpoDigest::new([Felt::new(value), ZERO, ZERO, ZERO])
}

async fn add(State(state): State<AppState>) -> impl IntoResponse {
    let mut mmr = state.mmr.lock().unwrap();
    let index = (*mmr).forest();
    (*mmr).add(int_to_node(0));
    Json(index)
}

async fn get_peaks(State(state): State<AppState>, Path(block): Path<usize>) -> impl IntoResponse {
    let mmr = state.mmr.lock().unwrap();
    let peaks = (*mmr).peaks(block).unwrap();
    Json(peaks)
}

async fn get_proof(State(state): State<AppState>, Path((block, index)): Path<(usize, usize)>) -> impl IntoResponse {
    let mmr = state.mmr.lock().unwrap();
    let proof = (*mmr).open(index, block).unwrap();
    Json(proof)
}

fn app() -> Router {
    /* 
    I'm being pretty loose with the term "block" here. I mostly just needed another term for the state of the mmr at the time
    that's a bit more intuitive than "forest". That said, if this were to turn into a production service, you probably would want
    the concept of a block, so that you can more easily update old proofs. There are a number of clever techniques to doing this
    sort of batching but, for now, we just have a new "block" for every leaf.
     */
    Router::new()
        .route("/leaves", post(add))
        .route("/blocks/:id/peaks", get(get_peaks))
        .route("/blocks/:id/proofs/:index", get(get_proof))
        .with_state(AppState::new())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
    };
    use miden_crypto::merkle::{MmrPeaks, MmrProof};
    use tower::ServiceExt;
    use serde_json;
    use http_body_util::BodyExt;

    async fn add_leaf(app: &Router) -> usize {
        let request = Request::builder()
            .method(http::Method::POST)
            .uri("/leaves")
            .body(Body::empty())
            .expect("request build failed");
        let response = app.clone().oneshot(request).await.expect("request failed");
        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        serde_json::from_slice(&body).expect("Failed to deserialize response")
    }

    async fn get_peaks(app: &Router, block: usize) -> MmrPeaks {
        let request = Request::builder()
            .method(http::Method::GET)
            .uri(format!("/blocks/{}/peaks", block))
            .body(Body::empty())
            .expect("request build failed");
        let response = app.clone().oneshot(request).await.expect("request failed");
        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        serde_json::from_slice(&body).expect("Failed to deserialize response")
    }

    async fn get_proof(app: &Router, block: usize, index: usize) -> MmrProof {
        let request = Request::builder()
            .method(http::Method::GET)
            .uri(format!("/blocks/{}/proofs/{}", block, index))
            .body(Body::empty())
            .expect("request build failed");
        let response = app.clone().oneshot(request).await.expect("request failed");
        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        serde_json::from_slice(&body).expect("Failed to deserialize response")
    }

    #[tokio::test]
    async fn test_leaves() {
        let app = app();
        assert_eq!(add_leaf(&app).await, 0);
        assert_eq!(add_leaf(&app).await, 1);
    }

    #[tokio::test]
    async fn test_peaks() {
        let app = app();
        let _index = add_leaf(&app).await;

        let peaks = get_peaks(&app, 1).await;
        assert_eq!(peaks.num_leaves(), 1);
        assert_eq!(peaks.num_peaks(), 1);
    }

    #[tokio::test]
    async fn test_proofs() {
        let app = app();
        let index = add_leaf(&app).await;
        let proof = get_proof(&app, 1, index).await;

        assert_eq!(proof.peak_index(), 0);
        assert_eq!(proof.relative_pos(), 0);

        panic!("not done");
    }
}
