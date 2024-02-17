use macaddr::MacAddr6;

use const_addrs::mac6;

fn main() {
    let a = mac6!("c0:ff:ee:c0:ff:ee");
    assert_eq!(a, MacAddr6::from([0xc0, 0xff, 0xee, 0xc0, 0xff, 0xee]));
}
