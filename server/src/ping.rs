use std::{
    io,
    net::IpAddr,
    str::FromStr,
    time::{Duration, Instant},
};

use tokio::task;

use crate::Error;

pub struct Pinger {
    // todo steal code from https://docs.rs/surge-ping/0.1.8/src/surge_ping/ping.rs.html#58-65
}

pub struct Response {
    ips: IpAddr,
    time: Instant,
    duration: Duration,
}

impl Pinger {
    pub async fn new(addr: String) -> Result<Self, Error> {
        let ip_addr = resolve_address(addr).await?;

        todo!()
    }
}

async fn resolve_address(addr: String) -> Result<IpAddr, Error> {
    if let Ok(addr) = IpAddr::from_str(&addr) {
        return Ok(addr);
    }

    // arbitrarily choose first IP resolved. TODO is this the right behaviour?
    task::spawn_blocking(move || match dns_lookup::lookup_host(&addr) {
        Ok(results) => results
            .into_iter()
            .next()
            .ok_or(Error::Dns(addr, None)),
        Err(err) => Err(Error::Dns(addr, Some(err))),
    })
    .await
    .expect("blocking task cancelled")
}
