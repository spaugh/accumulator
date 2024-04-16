use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppState {
    pub data: Arc<Mutex<Vec<u8>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(Vec::new())),
        }
    }
}