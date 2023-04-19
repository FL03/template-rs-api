/*
   Appellation: server <module>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use std::net::{IpAddr, SocketAddr};

pub struct Server {
    addr: SocketAddr,
}

impl Server {
    pub fn new(addr: SocketAddr) -> Self {
        Self { addr }
    }
    pub fn address(&self) -> SocketAddr {
        self.addr
    }
    pub fn host(&self) -> IpAddr {
        self.address().ip()
    }
    pub fn port(&self) -> u16 {
        self.address().port()
    }
    pub fn set(&mut self, addr: SocketAddr) {
        self.addr = addr;
    }
}
