use pot_rpc::{self, PingsServiceServer, PingsRequest, Pings, Ping};
use std::time::{Duration, SystemTime};
use tonic::{transport::Server, Request, Response, Status};

#[derive(Debug, Default)]
pub struct PingsService {}

#[tonic::async_trait]
impl pot_rpc::PingsService for PingsService {
    async fn get_pings(&self, request: Request<PingsRequest>) -> Result<Response<Pings>, Status> {
        println!("Got a request: {:?}", request);

        let reply = Pings {
            ip: "1.2.3.4".to_owned(),
            pings: vec![
                Ping {
                    time: Some(SystemTime::now().into()),
                    duration: Some(Duration::from_millis(8).into()),
                },
                Ping {
                    time: Some((SystemTime::now() - Duration::from_secs(5)).into()),
                    duration: Some(Duration::from_millis(22).into()),
                },
                Ping {
                    time: Some((SystemTime::now() - Duration::from_secs(10)).into()),
                    duration: Some(Duration::from_millis(12).into()),
                }
            ]
            .into(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let pings_service = PingsService::default();

    Server::builder()
        .add_service(PingsServiceServer::new(pings_service))
        .serve(addr)
        .await?;

    Ok(())
}
