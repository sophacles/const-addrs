use std::net::Ipv4Addr;

use const_addrs::ip4;

fn main() {
    let a = ip4!("192.168.1.1");
    assert_eq!(a, Ipv4Addr::new(192, 168, 1, 1));
}
