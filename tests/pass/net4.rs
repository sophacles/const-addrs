use ipnetwork::Ipv4Network;

use const_addrs::{ip4, net4};

fn main() {
    let a = net4!("192.168.1.1/24");
    assert_eq!(a, Ipv4Network::new(ip4!("192.168.1.1"), 24).unwrap());
}
