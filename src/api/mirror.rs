pub use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use flutter_rust_bridge::frb;
#[frb(mirror(IpAddr), non_opaque)]
pub enum Ip {
    V4(#[frb(name = "ip")] Ipv4Addr),
    V6(#[frb(name = "ip")] Ipv6Addr),
}

#[frb(external)]
impl Ipv4Addr {
    pub const fn is_unspecified(&self) -> bool {}

    pub const fn is_loopback(&self) -> bool {}

    pub const fn is_multicast(&self) -> bool {}

    #[frb(sync)]
    pub fn to_string(&self) -> String {}
}

#[frb(external)]
impl Ipv6Addr {
    pub const fn is_unspecified(&self) -> bool {}

    pub const fn is_loopback(&self) -> bool {}

    pub const fn is_multicast(&self) -> bool {}

    #[frb(sync)]
    pub fn to_string(&self) -> String {}
}
