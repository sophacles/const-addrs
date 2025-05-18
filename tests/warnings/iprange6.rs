use const_addrs::iprange6;

fn main() {
    let _ = iprange6!("2001:db8::32:ffff", "2001:db8::32:23");

    // bad one just to fail the compilation and compare output
    let _ = iprange6!("2001:db8::32::ffff", "2001:db8::32:23");
}
