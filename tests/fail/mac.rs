use const_addrs::mac;

fn main() {
    let _ = mac!("ca:fe:ca:fe");
    let _ = mac!("ca:fe:ca:fe:ca:fe:cafe");
}
