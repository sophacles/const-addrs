use ipnet::{IpAddrRange, Ipv4AddrRange, Ipv6AddrRange};

use const_addrs::{ip4, ip6, iprange};

fn main() {
    let a = iprange!("192.168.1.1", "192.168.1.254");
    match a {
        IpAddrRange::V4(net) => {
            assert_eq!(
                net,
                Ipv4AddrRange::new(ip4!("192.168.1.1"), ip4!("192.168.1.254"))
            );
        }
        _ => panic!("should not be v6"),
    }
    let b = iprange!("2001:db8::32:23", "2001:db8::32:ffff");
    match b {
        IpAddrRange::V6(net) => {
            assert_eq!(
                net,
                Ipv6AddrRange::new(ip6!("2001:db8::32:23"), ip6!("2001:db8::32:ffff"))
            );
        }
        _ => panic!("should not be v4"),
    }
}
