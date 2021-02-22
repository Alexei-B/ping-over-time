use std::{
    collections::HashMap,
    net::SocketAddr,
    time::{Duration, Instant},
};

use tokio::sync::watch;
use tokio::task;

/// Tracks state of all the servers we are pinging to.
pub struct PingState {
    // TODO use a good concurrent hashmap
    servers: HashMap<String, ServerState>,
}

struct ServerState {
    receiver: watch::Receiver<() /*Some type*/>,
    task: task::JoinHandle<()>,
}

struct PingResponse {
    pub ips: Vec<SocketAddr>,
    pub start: Instant,
    pub duration: Duration,
}

impl PingState {
    // pub fn get(string: )
}
