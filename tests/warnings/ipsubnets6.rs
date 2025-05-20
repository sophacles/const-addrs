use const_addrs::ipsubnets6;

fn main() {
    let _ = ipsubnets6!("2001:db8::32:ffff", "2001:db8::32:23", 64);

    // bad one just to fail the compilation and compare output
    let _ = ipsubnets6!("2001:db8::32::ffff", "2001:db8::32:23", 64);
}
