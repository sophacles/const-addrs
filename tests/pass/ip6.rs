use net_macros::ip6;
use std::net::Ipv6Addr;

fn main() {
    let a = ip6!("2001:db8::32:23");
    assert_eq!(
        a,
        Ipv6Addr::new(0x2001, 0x0db8, 0x0, 0x0, 0x0, 0x0, 0x32, 0x23)
    );
}
