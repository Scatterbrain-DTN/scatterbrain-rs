use std::sync::Arc;
pub use std::{
    collections::{BTreeMap, BTreeSet},
    future::Future,
};

#[cfg(feature = "flutter")]
use flutter_rust_bridge::BaseAsyncRuntime;
#[cfg(feature = "flutter")]
use flutter_rust_bridge::{frb, DartFnFuture, JoinHandle};
pub use mdns_sd::{ServiceDaemon, ServiceEvent};
pub use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use tokio_util::sync::CancellationToken;

#[cfg(feature = "flutter")]
use super::error::Error;
use crate::error::SbResult;

pub type HostRecords<'a> = tokio::sync::RwLockReadGuard<'a, BTreeMap<String, HostRecord>>;

struct ServiceScannerInner {
    devices: tokio::sync::RwLock<BTreeMap<String, HostRecord>>,
}
#[cfg(feature = "flutter")]
struct CancelationHandle {
    token: CancellationToken,
    handle: Option<JoinHandle<Result<(), Error>>>,
}

pub struct ServiceScanner {
    inner: std::sync::Arc<ServiceScannerInner>,
    #[cfg(feature = "flutter")]
    handle: Option<CancelationHandle>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "flutter", frb(opaque))]
#[cfg(not(feature = "flutter"))]
pub struct HostRecord {
    pub name: String,
    pub addr: BTreeSet<IpAddr>,
    pub port: u16,
}

#[cfg_attr(feature = "flutter", frb(opaque))]
#[cfg(feature = "flutter")]
#[derive(Clone, Debug)]
pub struct HostRecord {
    pub(crate) name: String,
    pub(crate) addr: BTreeSet<IpAddr>,
    pub(crate) port: u16,
}

#[cfg(feature = "flutter")]
impl HostRecord {
    #[frb(sync)]
    pub fn get_port(&self) -> u16 {
        self.port
    }

    #[frb(sync)]
    pub fn get_addrs(&self) -> Vec<IpAddr> {
        self.addr.iter().cloned().collect()
    }

    #[frb(sync)]
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

#[cfg(feature = "flutter")]
#[allow(async_fn_in_trait)]
pub trait ServiceScannerLike {
    async fn discover_devices(
        &mut self,
        cb: impl Fn(Vec<HostRecord>) -> DartFnFuture<()> + Send + Sync + 'static,
    ) -> anyhow::Result<()>;

    #[frb(sync)]
    fn scan_nonblock(
        &mut self,
        cb: impl Fn(Vec<HostRecord>) -> DartFnFuture<()> + Send + Sync + 'static,
    ) -> anyhow::Result<()>;

    async fn stop_scan(&mut self) -> anyhow::Result<()>;
}

#[cfg(feature = "flutter")]
impl ServiceScannerLike for ServiceScanner {
    async fn discover_devices(
        &mut self,
        cb: impl Fn(Vec<HostRecord>) -> DartFnFuture<()> + Send + Sync + 'static,
    ) -> anyhow::Result<()> {
        self.discover_devices_impl(std::sync::Arc::new(cb)).await?;
        Ok(())
    }

    #[frb(sync)]
    fn scan_nonblock(
        &mut self,
        cb: impl Fn(Vec<HostRecord>) -> DartFnFuture<()> + Send + Sync + 'static,
    ) -> anyhow::Result<()> {
        self.discover_devices_nonblock(std::sync::Arc::new(cb))
    }

    async fn stop_scan(&mut self) -> anyhow::Result<()> {
        if let Some(handle) = self.handle.take() {
            handle.token.cancel();
            if let Some(join) = handle.handle {
                join.await??;
            }
        }
        Ok(())
    }
}

#[cfg(feature = "flutter")]
impl ServiceScanner {
    fn discover_devices_nonblock(
        &mut self,
        cb: std::sync::Arc<dyn Fn(Vec<HostRecord>) -> DartFnFuture<()> + Send + Sync + 'static>,
    ) -> anyhow::Result<()> {
        if self.handle.is_none() {
            let s = self.inner.clone();
            let c = CancellationToken::new();
            let c2 = c.clone();
            let task = crate::api::frb::FLUTTER_RUST_BRIDGE_HANDLER
                .async_runtime()
                .spawn(async move {
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

            self.handle = Some(CancelationHandle {
                token: c2,
                handle: Some(task),
            });
        }
        Ok(())
    }

    async fn discover_devices_impl(
        &mut self,
        cb: std::sync::Arc<dyn Fn(Vec<HostRecord>) -> DartFnFuture<()> + Send + Sync + 'static>,
    ) -> anyhow::Result<()> {
        self.stop_scan().await?;
        let s = self.inner.clone();
        let c = CancellationToken::new();
        let c2 = c.clone();
        let task = tokio::spawn(async move {
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

        self.handle = Some(CancelationHandle {
            token: c2,
            handle: Some(task),
        });
        Ok(())
    }
}

impl ServiceScanner {
    #[cfg_attr(feature = "flutter", frb(sync))]
    pub fn new() -> Self {
        Self {
            inner: Arc::new(ServiceScannerInner {
                devices: tokio::sync::RwLock::new(BTreeMap::new()),
            }),
            #[cfg(feature = "flutter")]
            handle: None,
        }
    }

    pub async fn mdns_scan<'b, F, Fut>(&'b mut self, cb: F) -> SbResult<()>
    where
        F: Fn(HostRecords<'b>) -> Fut,
        Fut: Future<Output = SbResult<()>>,
    {
        let c = CancellationToken::new();

        #[cfg(feature = "flutter")]
        {
            self.handle = Some(CancelationHandle {
                token: c.clone(),
                handle: None,
            });
        }
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
