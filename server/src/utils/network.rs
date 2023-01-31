use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

pub fn all_interfaces(port: u16) -> SocketAddr {
    SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port))
}
