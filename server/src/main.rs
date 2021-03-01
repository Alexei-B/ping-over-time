mod ping;
mod state;

use std::{io, sync::{Arc, RwLock}};
use std::time::{SystemTime};




use pot_rpc::{self, Ping, Pings, PingsRequest, PingsServiceServer};
use thiserror::Error;
use tonic::{transport::Server, Request, Response, Status};

// Errors that can occur while pinging a server
#[derive(Debug, Error)]
pub enum Error {
    #[error("error resolving hostname `{0}`")]
    Dns(String, #[source] Option<io::Error>),
}

#[derive(Debug)]
pub struct PingsService {
    state: state::PingState,
}

#[tonic::async_trait]
impl pot_rpc::PingsService for PingsService {
    async fn get_pings(&self, request: Request<PingsRequest>) -> Result<Response<Pings>, Status> {
        println!("Got a request: {:?}", request);
        let req = request.into_inner();

        let history = self.state.get(&req.address).await
            .map_err(|err| Status::invalid_argument(err.to_string()))?;

        let reply = Pings {
            ip: "216.58.204.14".to_owned(),
            pings: history.read().await.iter()
                .filter(|p|
                    req.since.is_none() ||
                    SystemTime::from(p.time.as_ref().unwrap().clone()) > SystemTime::from(req.since.as_ref().unwrap().clone()))
                .map(|p| p.clone())
                .collect(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let _pings: Arc<RwLock<Vec<Ping>>> = RwLock::new(Vec::<Ping>::new()).into();
    let pings_service = PingsService { state: Default::default() };

    Server::builder()
        .add_service(PingsServiceServer::new(pings_service))
        .serve(addr)
        .await?;

    Ok(())
}
