use ipnetwork::{IpNetwork, Ipv4Network, Ipv6Network};

use net_macros::{ip4, ip6, net};

fn main() {
    let a = net!("192.168.1.1/24");
    match a {
        IpNetwork::V4(net) => {
            assert_eq!(net, Ipv4Network::new(ip4!("192.168.1.1"), 24).unwrap());
        }
        _ => panic!("should not be v6"),
    }
    let b = net!("2001:db8::32:23/64");
    match b {
        IpNetwork::V6(net) => {
            assert_eq!(net, Ipv6Network::new(ip6!("2001:db8::32:23"), 64).unwrap());
        }
        _ => panic!("should not be v6"),
    }
}
