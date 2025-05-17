use ipnet::Ipv4Net;

use const_addrs::{ip4, net4};

fn main() {
    let a = ipnet4!("192.168.1.1/24");
    assert_eq!(a, Ipv4Net::new(ip4!("192.168.1.1"), 24).unwrap());
}
