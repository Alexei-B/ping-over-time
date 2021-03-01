use std::{net::IpAddr, str::FromStr, sync::Weak, time::SystemTime};

use tokio::{sync::RwLock, task::{self}, time::{Instant, Duration}};
use pot_rpc::{self, Ping};
use winping::{AsyncPinger, Buffer};

use crate::Error;

pub async fn ping(addr: String, state: Weak<RwLock<Vec<Ping>>>) -> Result<(), Error> {
    let start = Instant::now();
    let mut intervals = 1;

    let pinger = AsyncPinger::new();

    let dst = resolve_address(addr).await?;

    tokio::spawn(async move {
        while let Some(state) = state.upgrade() {
            let buffer = Buffer::new();
            let time = SystemTime::now();

            let pinger = pinger.clone();

            tokio::spawn(async move {
                match pinger.send(dst, buffer).await.result {
                    Err(err) => {
                        log::error!("{}", err);
                        state.write().await.push(Ping {
                            time: Some(time.into()),
                            duration: Some(Duration::from_secs(30).into())
                        });
                    },
                    Ok(rtt) => {
                        state.write().await.push(Ping {
                            time: Some(time.into()),
                            duration: Some(Duration::from_millis(rtt as u64).into())
                        });
                    }
                };
            });

            tokio::time::sleep_until(start + Duration::from_secs(intervals)).await;
            intervals += 1;
        }
    });

    Ok(())
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
