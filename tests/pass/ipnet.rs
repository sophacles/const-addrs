use ipnet::{IpNet, Ipv4Net, Ipv6Net};

use const_addrs::{ip4, ip6, ipnet};

fn main() {
    let a = ipnet!("192.168.1.1/24");
    match a {
        IpNet::V4(net) => {
            assert_eq!(net, Ipv4Net::new(ip4!("192.168.1.1"), 24).unwrap());
        }
        _ => panic!("should not be v6"),
    }
    let b = ipnet!("2001:db8::32:23/64");
    match b {
        IpNet::V6(net) => {
            assert_eq!(net, Ipv6Net::new(ip6!("2001:db8::32:23"), 64).unwrap());
        }
        _ => panic!("should not be v4"),
    }
}
