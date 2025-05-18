use ipnet::Ipv6Net;

use const_addrs::{ip6, ipnet6};

fn main() {
    let a = ipnet6!("2001:db8::32:23/63");
    assert_eq!(a, Ipv6Net::new(ip6!("2001:db8::32:23"), 63).unwrap());
}
