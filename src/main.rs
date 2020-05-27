#![deny(warnings)]
use std::net::SocketAddr;
mod proxy;
pub fn main() {

    let in_addr: SocketAddr = ([127, 0, 0, 1], 3001).into();
    let out_addr: SocketAddr = ([127, 0, 0, 1], 3000).into();
    proxy::run(in_addr, out_addr);
}
