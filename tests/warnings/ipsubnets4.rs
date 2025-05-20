use const_addrs::ipsubnets4;

fn main() {
    let _ = ipsubnets4!("10.0.1.0", "10.0.0.0", 24);

    // failure so output is compared
    let _ = ipsubnets4!("10.0.10", "10.0.0.0", 24);
}
