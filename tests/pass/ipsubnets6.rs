use ipnet::Ipv6Subnets;

use const_addrs::{ip6, ipsubnets6};

fn main() {
    let b = ipsubnets6!("2001:db8::32:23", "2001:db8::32:ffff", 122);
    assert_eq!(
        b,
        Ipv6Subnets::new(ip6!("2001:db8::32:23"), ip6!("2001:db8::32:ffff"), 122)
    );
}
