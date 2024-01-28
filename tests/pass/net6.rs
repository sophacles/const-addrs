use ipnetwork::Ipv6Network;

use net_macros::{ip6, net6};

fn main() {
    let a = net6!("2001:db8::32:23/63");
    assert_eq!(a, Ipv6Network::new(ip6!("2001:db8::32:23"), 63).unwrap());
}
