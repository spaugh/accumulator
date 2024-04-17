use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{response::Json, routing::post, Router};

use crate::state::AppState;
use miden_crypto::hash::rpo::RpoDigest;

async fn add_leaf(State(state): State<AppState>, body: String) -> impl IntoResponse {
    let mut mmr = state.mmr.lock().unwrap();
    let index = (*mmr).forest();
    (*mmr).add(body.try_into().unwrap());
    Json(index)
}

async fn get_peaks(State(state): State<AppState>, Path(block): Path<usize>) -> impl IntoResponse {
    let mmr = state.mmr.lock().unwrap();
    let peaks = (*mmr).peaks(block).unwrap();
    Json(peaks)
}

async fn get_proof(
    State(state): State<AppState>,
    Path((block, index)): Path<(usize, usize)>,
) -> impl IntoResponse {
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
        .route("/leaves", post(add_leaf))
        .route("/blocks/:id/peaks", get(get_peaks))
        .route("/blocks/:id/proofs/:index", get(get_proof))
        .with_state(AppState::new())
}

#[cfg(test)]
mod tests {
    use crate::utils::TestClient;

    use super::*;
    use client::Client;
    use miden_crypto::hash::rpo::Rpo256;

    #[tokio::test]
    async fn test_leaves() {
        let client = TestClient::new(app());
        assert_eq!(client.add_data("hello").await.unwrap(), 0);
        assert_eq!(client.add_data("goodbye").await.unwrap(), 1);
    }

    #[tokio::test]
    async fn test_peaks() {
        let client = TestClient::new(app());
        let block = client.add_data("a").await.unwrap() + 1;

        let peaks = client.get_peaks(block).await.unwrap();
        assert_eq!(peaks.num_leaves(), 1);
        assert_eq!(peaks.num_peaks(), 1);
    }

    #[tokio::test]
    async fn test_proofs() {
        let client = TestClient::new(app());
        let leaf = client.add_data("1").await.unwrap();
        let block = leaf + 1;
        let peaks = client.get_peaks(block).await.unwrap();
        let proof = client.get_proof(block, leaf).await.unwrap();

        assert_eq!(proof.peak_index(), 0);
        assert_eq!(proof.relative_pos(), 0);
        assert!(peaks.verify(Rpo256::hash("1".as_bytes()), proof));
    }
}
