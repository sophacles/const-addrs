use ipnet::{IpSubnets, Ipv4Subnets, Ipv6Subnets};

use const_addrs::{ip4, ip6, ipsubnets};

fn main() {
    let a = ipsubnets!("192.168.1.1", "192.168.1.254", 30);
    match a {
        IpSubnets::V4(net) => {
            assert_eq!(
                net,
                Ipv4Subnets::new(ip4!("192.168.1.1"), ip4!("192.168.1.254"), 30)
            );
        }
        _ => panic!("should not be v6"),
    }
    let b = ipsubnets!("2001:db8::32:23", "2001:db8::32:ffff", 120);
    match b {
        IpSubnets::V6(net) => {
            assert_eq!(
                net,
                Ipv6Subnets::new(ip6!("2001:db8::32:23"), ip6!("2001:db8::32:ffff"), 120)
            );
        }
        _ => panic!("should not be v4"),
    }
}
