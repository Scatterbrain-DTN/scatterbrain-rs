pub use std::net::{Ipv4Addr, Ipv6Addr};

use flutter_rust_bridge::frb;

#[frb(mirror(Ipv4Addr))]
#[derive(Clone, Debug)]
pub struct _Ipv4Addr {
    pub octets: [u8; 4],
}

#[frb(mirror(Ipv6Addr))]
#[derive(Clone, Debug)]
pub struct _Ipv6Addr {
    pub octets: [u8; 16],
}

#[frb(mirror(IpAddr, Ipv4Addr, Ipv6Addr))]
#[derive(Clone, Debug)]
pub enum _IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
