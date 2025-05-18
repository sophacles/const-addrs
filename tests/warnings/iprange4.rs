use const_addrs::iprange4;

fn main() {
    let _ = iprange4!("10.0.1.0", "10.0.0.0");

    // failure so output is compared
    let _ = iprange4!("10.0.10", "10.0.0.0");
}
