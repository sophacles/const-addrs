use std::net::SocketAddrV6;

use net_macros::{ip6, sock6};

fn main() {
    let a = sock6!("[2001:db8::32:23]:22");
    assert_eq!(a, SocketAddrV6::new(ip6!("2001:db8::32:23"), 22, 0, 0));
}
