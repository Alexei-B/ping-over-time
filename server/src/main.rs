mod ping;
mod state;

use std::{io, ops::Deref, sync::{Arc, RwLock}};
use std::time::{Duration, SystemTime};

use std::net::IpAddr;
use tokio::{task, time::Instant};
use winping::{Buffer, AsyncPinger};
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
    pings: Arc<RwLock<Vec<Ping>>>
}

#[tonic::async_trait]
impl pot_rpc::PingsService for PingsService {
    async fn get_pings(&self, request: Request<PingsRequest>) -> Result<Response<Pings>, Status> {
        println!("Got a request: {:?}", request);
        let req = request.into_inner();

        let reply = Pings {
            ip: "216.58.204.14".to_owned(),
            pings: self.pings.read().unwrap().iter()
                .filter(|p|
                    req.since.is_none() ||
                    SystemTime::from(p.time.as_ref().unwrap().clone()) > SystemTime::from(req.since.as_ref().unwrap().clone()))
                .map(|p| p.clone())
                .collect(),
        };

        Ok(Response::new(reply))
    }
}

async fn ping(addr: &str, pings: Arc<RwLock<Vec<Ping>>>) {
    let start = Instant::now();
    let mut intervals = 1;

    loop {
        let pings = pings.clone();
        let dst = addr
            .parse::<IpAddr>()
            .expect("Could not parse IP Address");

        task::spawn(async move {
            let buffer = Buffer::new();
            let pinger = AsyncPinger::new();
            let time = SystemTime::now();

            match pinger.send(dst, buffer).await.result {
                Err(err) => {
                    println!("{}", err);
                    let mut p = pings.write().unwrap();
                    p.push(Ping {
                        time: Some(time.into()),
                        duration: Some(Duration::from_secs(30).into())
                    })
                },
                Ok(rtt) => {
                    let mut p = pings.write().unwrap();
                    p.push(Ping {
                        time: Some(time.into()),
                        duration: Some(Duration::from_millis(rtt as u64).into())
                    })
                }
            };
        });

        tokio::time::sleep_until(start + Duration::from_secs(intervals)).await;
        intervals += 1;
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let pings: Arc<RwLock<Vec<Ping>>> = RwLock::new(Vec::<Ping>::new()).into();
    let pings_service = PingsService { pings: pings.clone() };

    task::spawn(async {
        ping("216.58.204.14", pings).await;
    });

    Server::builder()
        .add_service(PingsServiceServer::new(pings_service))
        .serve(addr)
        .await?;

    Ok(())
}
