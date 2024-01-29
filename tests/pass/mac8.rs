use macaddr::MacAddr8;

use net_macros::mac8;

fn main() {
    let a = mac8!("c0:ff:ee:c0:ff:ee:ca:fe");
    assert_eq!(
        a,
        MacAddr8::from([0xc0, 0xff, 0xee, 0xc0, 0xff, 0xee, 0xca, 0xfe])
    );
}
