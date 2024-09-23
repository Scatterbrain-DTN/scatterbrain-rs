use std::sync::Arc;
pub use std::{
    collections::{BTreeMap, BTreeSet},
    future::Future,
};

#[cfg(feature = "flutter")]
use flutter_rust_bridge::{frb, DartFnFuture};
pub use mdns_sd::{ServiceDaemon, ServiceEvent};
pub use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use tokio_util::sync::CancellationToken;

use crate::error::SbResult;

pub type HostRecords<'a> = tokio::sync::RwLockReadGuard<'a, BTreeMap<String, HostRecord>>;

struct ServiceScannerInner {
    devices: tokio::sync::RwLock<BTreeMap<String, HostRecord>>,
}

pub struct ServiceScanner {
    inner: std::sync::Arc<ServiceScannerInner>,
    handle: Option<CancellationToken>,
}

#[derive(Clone, Debug)]
pub struct HostRecord {
    pub name: String,
    pub addr: Vec<IpAddr>,
    pub port: u16,
}

#[cfg(feature = "flutter")]
impl ServiceScanner {
    pub async fn discover_devices(
        &mut self,
        cb: impl Fn(Vec<HostRecord>) -> DartFnFuture<()> + Send + Sync + 'static,
    ) -> SbResult<()> {
        self.discover_devices_impl(std::sync::Arc::new(cb)).await
    }

    async fn discover_devices_impl(
        &mut self,
        cb: std::sync::Arc<dyn Fn(Vec<HostRecord>) -> DartFnFuture<()> + Send + Sync + 'static>,
    ) -> SbResult<()> {
        let s = self.inner.clone();
        let c = CancellationToken::new();
        self.handle = Some(c.clone());
        tokio::spawn(async move {
            s.mdns_scan(
                |res| {
                    let cb = cb.clone();
                    async move {
                        cb(res.iter().map(|(_, v)| v.clone().into()).collect()).await;
                        Ok(())
                    }
                },
                c,
            )
            .await
        });
        Ok(())
    }

    pub async fn stop_scan(&mut self) {
        if let Some(handle) = self.handle.take() {
            handle.cancel()
        }
    }
}

impl ServiceScanner {
    #[cfg_attr(feature = "flutter", frb(sync))]
    pub fn new() -> Self {
        Self {
            inner: Arc::new(ServiceScannerInner {
                devices: tokio::sync::RwLock::new(BTreeMap::new()),
            }),
            handle: None,
        }
    }

    pub async fn mdns_scan<'b, F, Fut>(&'b mut self, cb: F) -> SbResult<()>
    where
        F: Fn(HostRecords<'b>) -> Fut,
        Fut: Future<Output = SbResult<()>>,
    {
        let c = CancellationToken::new();
        self.handle = Some(c.clone());
        self.inner.mdns_scan(cb, c).await
    }
}

impl ServiceScannerInner {
    async fn mdns_scan<'b, F, Fut>(&'b self, cb: F, token: CancellationToken) -> SbResult<()>
    where
        F: Fn(HostRecords<'b>) -> Fut,
        Fut: Future<Output = SbResult<()>>,
    {
        let mdns = ServiceDaemon::new()?;

        // Scatterbrain mdns service type
        let service_type = "_sbd._tcp.local.";
        let receiver = mdns.browse(service_type)?;
        while let Some(event) = tokio::select! {
           event =  receiver.recv_async() => {
              Some(event)
           }
           _ = token.cancelled() => {
                 None
            }
        } {
            match event {
                Ok(ServiceEvent::ServiceResolved(info)) => {
                    self.devices.write().await.insert(
                        info.get_fullname().to_owned(),
                        HostRecord {
                            name: info
                                .get_fullname()
                                .trim_end_matches(info.get_type())
                                .trim_end_matches(".")
                                .to_owned(),
                            addr: info
                                .get_addresses()
                                .into_iter()
                                .map(|v| v.clone())
                                .collect::<BTreeSet<_>>()
                                .into_iter()
                                .map(|v| v.into())
                                .collect(),
                            port: info.get_port(),
                        },
                    );
                }
                Ok(ServiceEvent::ServiceRemoved(_, fullname)) => {
                    self.devices.write().await.remove(&fullname);
                }
                _ => (),
            }
            cb(self.devices.read().await).await?;
        }
        Ok(())
    }
}
