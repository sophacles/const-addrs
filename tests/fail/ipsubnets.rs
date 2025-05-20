use const_addrs::ipsubnets;

fn main() {
    let _ = ipsubnets!("2001:db8::32:23", "2001:db8::32::ffff", 48);
    let _ = ipsubnets!("2001:db8::32::23", "2001:db8::32:ffff", 48);
    let _ = ipsubnets!("2001:db8::32::23", "2001:db8::32::ffff", 48);
    let _ = ipsubnets!("2001:db8::32:23", "10.0.0.2", 48);
    let _ = ipsubnets!("10.0.0.1", "2001:db8::32:23", 48);
    let _ = ipsubnets!("2001:db8::32:23", "10.0.0.1", 48);
    let _ = ipsubnets!("100.0.0", "10.0.0.255", 48);
    let _ = ipsubnets!("10.0.0.0", "10.0.0255", 24);
    let _ = ipsubnets!("2001:db8::32:23", "2001:db8::32:ffff", 129);
    let _ = ipsubnets!("2001:db8::32:23", "2001:db8::32:ffff", 260);

    let _ = ipsubnets!("10.0.0.1", "10.0.10.1", 33);
    let _ = ipsubnets!("10.0.0.1", "10.0.10.1", 260);
}
