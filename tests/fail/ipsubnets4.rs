use const_addrs::ipsubnets4;

fn main() {
    let _ = ipsubnets4!("10.0.01", "10.0.1.0", 24);
    let _ = ipsubnets4!("10.0.0.1", "10.0.10", 24);
    let _ = ipsubnets4!("10.0.01", "10.0.10", 24);
    let _ = ipsubnets4!("10.0.0.1", "2001:db8::32:23", 24);
    let _ = ipsubnets4!("10.0.0.1", "10.0.10.0", 33);
    let _ = ipsubnets4!("10.0.0.1", "10.0.10.0", 260);
}
