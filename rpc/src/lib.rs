use pings_rpc::pings_service_server;
pub use pings_rpc::pings_service_server::PingsServiceServer;
pub use pings_rpc::{Ping, Pings, PingsRequest};
use std::time::{Duration, SystemTime};
pub use tonic::{transport::Server, Request, Response, Status};

pub mod pings_rpc {
    tonic::include_proto!("pings");
}

#[derive(Debug, Default)]
pub struct PingsService {}

#[tonic::async_trait]
impl pings_service_server::PingsService for PingsService {
    async fn get_pings(&self, request: Request<PingsRequest>) -> Result<Response<Pings>, Status> {
        println!("Got a request: {:?}", request);

        let reply = Pings {
            ip: "0.0.0.0".into(),
            domain: "localhost".into(),
            pings: vec![Ping {
                time: Some(SystemTime::now().into()),
                duration: Some(Duration::from_millis(8).into()),
            }]
            .into(),
        };

        Ok(Response::new(reply))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
