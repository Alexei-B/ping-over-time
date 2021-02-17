use pot_rpc::{PingsService, PingsServiceServer, Server};

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
