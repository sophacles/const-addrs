use ipnet::Ipv6AddrRange;

use const_addrs::{ip6, iprange6};

fn main() {
    let b = iprange6!("2001:db8::32:23", "2001:db8::32:ffff");
    assert_eq!(
        b,
        Ipv6AddrRange::new(ip6!("2001:db8::32:23"), ip6!("2001:db8::32:ffff"))
    );
}
