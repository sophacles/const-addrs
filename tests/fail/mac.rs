use const_addrs::mac;

fn main() {
    let a = mac!("ca:fe:ca:fe");
    let b = mac!("ca:fe:ca:fe:ca:fe:cafe");
}
