use ipnet::Ipv4AddrRange;

use const_addrs::{ip4, iprange4};

fn main() {
    let a = iprange4!("192.168.1.1", "192.168.1.254");
    assert_eq!(
        a,
        Ipv4AddrRange::new(ip4!("192.168.1.1"), ip4!("192.168.1.254"))
    );
}
