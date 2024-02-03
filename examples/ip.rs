use ipnetwork::{IpNetwork, Ipv4Network};
use macaddr::MacAddr6;
use net_macros::{ip, ip4, ip6, mac6, net, net4, sock, sock4, sock6};
use std::net::IpAddr;

fn test(ip: std::net::IpAddr) {
    println!("test! {}", ip);
}

fn main() {
    let it = ip4!("192.168.1.1");
    println!("it: {:?}", it);
    test(it.into());
    println!("it2: {}", ip4!("172.16.13.1"));

    let it6 = ip6!("2400:cb00::00:01");
    println!("it: {:?}", it6);

    let it = ip!("192.168.1.1");
    assert!(matches!(it, IpAddr::V4(_)));
    println!("it: {:?}", it);

    let it = ip!("2400:cb00::01");
    assert!(matches!(it, IpAddr::V6(_)));
    println!("it: {:?}", it);

    let it = sock6!("[2400:cb00::01]:500");
    assert_eq!(it.ip(), &ip6!("2400:cb00::01"));
    assert_eq!(it.port(), 500);
    println!("it: {:?}", it);

    let it = sock4!("192.168.1.1:500");
    assert_eq!(it.ip(), &ip4!("192.168.1.1"));
    assert_eq!(it.port(), 500);
    println!("it: {:?}", it);

    let it = sock!("[2400:cb00::01]:500");
    println!("it: {:?}", it);
    assert!(matches!(it.ip(), IpAddr::V6(_)));
    assert_eq!(it.port(), 500);

    let it = sock!("192.168.1.1:500");
    println!("it: {:?}", it);
    assert!(matches!(it.ip(), IpAddr::V4(_)));
    assert_eq!(it.port(), 500);

    let it = net!("192.168.1.1/24");
    println!("it: {:?} - {}", it, it);
    assert!(matches!(it, IpNetwork::V4(_)));
    assert_eq!(it.prefix(), 24);

    let it = net4!("192.168.1.1/24");
    println!("it: {:?} - {}", it, it);
    assert_eq!(it.prefix(), 24);
    assert_eq!(it, Ipv4Network::new(ip4!("192.168.1.1"), 24).unwrap());

    let it: IpNetwork = net4!("192.168.1.1/24").into();
    assert_eq!(it, IpNetwork::new(ip!("192.168.1.1"), 24).unwrap());

    let it = mac6!("DE:AD:BE:EF:DE:AD");
    let ma = MacAddr6::from([0xDE, 0xAD, 0xBE, 0xEF, 0xDE, 0xAD]);
    assert_eq!(it, ma);
    println!("ma: {:?}", ma);
}
