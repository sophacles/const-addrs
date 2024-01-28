use std::net::SocketAddrV4;

use net_macros::{ip4, sock4};

fn main() {
    let a = sock4!("192.168.1.1:300");
    assert_eq!(a, SocketAddrV4::new(ip4!("192.168.1.1"), 300));
}
