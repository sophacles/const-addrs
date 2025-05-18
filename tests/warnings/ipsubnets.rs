use const_addrs::ipsubnets;

fn main() {
    let _ = ipsubnets!("2001:db8::32:ffff", "2001:db8::32:23", 64);
    let _ = ipsubnets!("10.128.0.0", "10.0.0.0", 24);

    //failure so the output is compared
    let _ = ipsubnets!("10128.0.0", "10.0.0.0", 24);
}
