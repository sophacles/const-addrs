use ipnetwork::IpNetwork;

use net_macros::{ip, net};

fn main() {
    let a = net!("192.168.1.1/29");
    match a {
        IpNetwork::V4(net) => {
            assert_eq!(net, IpNetwork::new(ip!("192.168.1.1"), 24));
        }
        _ => panic!("should not be v6"),
    }
    let b = net!("2001:db8::32:23/64");
    match b {
        IpNetwork::V6(net) => {
            assert_eq!(net, IpNetwork::new(ip!("2001:db8::32:23"), 64));
        }
        _ => panic!("should not be v6"),
    }
}
