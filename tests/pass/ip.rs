use net_macros::ip;

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

fn main() {
    let a = ip!("192.168.1.1");
    match a {
        IpAddr::V4(a) => assert_eq!(a, Ipv4Addr::new(192, 168, 1, 1)),
        _ => panic!("should not be ipv6!"),
    }

    let b = ip!("2001:db8::32:23");
    match b {
        IpAddr::V6(b) => assert_eq!(
            b,
            Ipv6Addr::new(0x2001, 0x0db8, 0x0, 0x0, 0x0, 0x0, 0x32, 0x23)
        ),
        _ => panic!("should not be ipv4"),
    }
}
