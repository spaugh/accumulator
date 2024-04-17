use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppState {
    pub mmr: Arc<Mutex<miden_crypto::merkle::Mmr>>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            mmr: Arc::new(Mutex::new(miden_crypto::merkle::Mmr::new())),
        }
    }
}
