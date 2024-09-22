pub use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use flutter_rust_bridge::frb;

#[frb(mirror(Ipv4Addr))]
#[derive(Clone, Debug)]
pub struct _Ipv4Addr {
    octets: [u8; 4],
}

#[frb(mirror(Ipv6Addr))]
#[derive(Clone, Debug)]
pub struct _Ipv6Addr {
    octets: [u8; 16],
}

#[frb(mirror(IpAddr, Ipv6Addr, Ipv4Addr))]
#[derive(Clone, Debug)]
pub enum _IpAddr {
    V4(_Ipv4Addr),
    V6(_Ipv6Addr),
}

#[frb(external)]
impl IpAddr {
    pub const fn is_unspecified(&self) -> bool {}

    pub const fn is_loopback(&self) -> bool {}

    pub const fn is_multicast(&self) -> bool {}

    pub const fn is_ipv4(&self) -> bool {}

    pub const fn is_ipv6(&self) -> bool {}

    pub const fn to_canonical(&self) -> IpAddr {}

    #[frb(sync)]
    pub fn to_string(&self) -> String {}
}
