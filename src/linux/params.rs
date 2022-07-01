use std::net::{Ipv4Addr, Ipv6Addr};

/// Represents parameters for creating a new Tun/Tap device on Linux.
#[cfg(target_os = "linux")]
pub struct Params {
    pub name: Option<String>,
    pub flags: i16,
    pub persist: bool,
    pub up: bool,
    pub mtu: Option<i32>,
    pub owner: Option<i32>,
    pub group: Option<i32>,
    pub address: Option<Ipv4Addr>,
    pub destination: Option<Ipv4Addr>,
    pub broadcast: Option<Ipv4Addr>,
    pub netmask: Option<Ipv4Addr>,
    pub address_v6: Option<Ipv6Addr>,
    pub prefix_len_v6: u32,
}
