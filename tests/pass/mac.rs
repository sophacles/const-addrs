use macaddr::{MacAddr, MacAddr6, MacAddr8};

use const_addrs::mac;

fn main() {
    let a = mac!("c0:ff:ee:c0:ff:ee");
    match a {
        MacAddr::V6(net) => {
            assert_eq!(net, MacAddr6::from([0xc0, 0xff, 0xee, 0xc0, 0xff, 0xee]));
        }
        _ => panic!("should not be v8"),
    }
    let b = mac!("c0:ff:ee:c0:ff:ee:ca:fe");
    match b {
        MacAddr::V8(net) => {
            assert_eq!(
                net,
                MacAddr8::from([0xc0, 0xff, 0xee, 0xc0, 0xff, 0xee, 0xca, 0xfe])
            );
        }
        _ => panic!("should not be v6"),
    }
}
