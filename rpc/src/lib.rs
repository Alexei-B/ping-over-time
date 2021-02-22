pub use pings_rpc::pings_service_server::{PingsService, PingsServiceServer};
pub use pings_rpc::pings_service_client::{PingsServiceClient};
pub use pings_rpc::{Ping, Pings, PingsRequest};

pub mod pings_rpc {
    tonic::include_proto!("pings");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
