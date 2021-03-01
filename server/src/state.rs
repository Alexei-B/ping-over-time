use std::sync::Arc;

use dashmap::DashMap;
use tokio::sync::RwLock;

use pot_rpc::Ping;

use crate::{Error, ping::ping};

/// Tracks state of all the servers we are pinging.
#[derive(Debug, Default)]
pub struct PingState {
    servers: DashMap<String, ServerState>,
}

#[derive(Debug)]
struct ServerState {
    history: Arc<RwLock<Vec<Ping>>>,
}

impl PingState {
    pub async fn get(&self, addr: &str) -> Result<Arc<RwLock<Vec<Ping>>>, Error> {
        if let Some(state) = self.servers.get(addr) {
            return Ok(state.history.clone());
        }

        let history = Default::default();
        ping(addr.to_owned(), Arc::downgrade(&history)).await?;
        self.servers.insert(addr.to_owned(), ServerState {
            history: history.clone()
        });

        Ok(history)
    }
}
