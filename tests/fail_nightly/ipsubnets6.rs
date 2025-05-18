use const_addrs::ipsubnets6;

fn main() {
    let _ = ipsubnets6!("2001:db8::32::23", "2001:db8::32:ffff", 24);
    let _ = ipsubnets6!("2001:db8::32:23", "2001:db8::32::ffff", 24);
    let _ = ipsubnets6!("2001:db8::32::23", "2001:db8::32::ffff", 24);
    let _ = ipsubnets6!("2001:db8::32:23", "10.0.0.1", 24);
    let _ = ipsubnets6!("10.0.0.1", "2001:db8::32:ffff", 129);
    let _ = ipsubnets6!("2001:db8::32:23", "2001:db8::32:ffff", 129);
    let _ = ipsubnets6!("2001:db8::32:23", "2001:db8::32:ffff", 260);
}
