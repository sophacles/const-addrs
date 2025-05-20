use ipnet::Ipv4Subnets;

use const_addrs::{ip4, ipsubnets4};

fn main() {
    let a = ipsubnets4!("192.168.1.1", "192.168.1.254", 30);
    assert_eq!(
        a,
        Ipv4Subnets::new(ip4!("192.168.1.1"), ip4!("192.168.1.254"), 30)
    );
}
