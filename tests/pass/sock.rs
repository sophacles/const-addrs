use std::net::{SocketAddr, SocketAddrV4, SocketAddrV6};

use const_addrs::{ip4, ip6, sock};

fn main() {
    let a = sock!("192.168.1.1:300");
    match a {
        SocketAddr::V4(sa) => {
            assert_eq!(sa, SocketAddrV4::new(ip4!("192.168.1.1"), 300));
        }
        _ => panic!("should not be v6!"),
    }

    let b = sock!("[2001:db8::32:23]:22");
    match b {
        SocketAddr::V6(sa) => {
            assert_eq!(sa, SocketAddrV6::new(ip6!("2001:db8::32:23"), 22, 0, 0));
        }
        _ => panic!("should not be v4!"),
    }
}
