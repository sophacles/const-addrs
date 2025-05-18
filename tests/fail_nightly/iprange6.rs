use const_addrs::iprange6;

fn main() {
    let _ = iprange6!("2001:db8::32:23", "2001:db8::32::ffff");
    let _ = iprange6!("2001:db8::32::23", "2001:db8::32:ffff");
    let _ = iprange6!("2001:db8::32::23", "2001:db8::32::ffff");
    let _ = iprange6!("2001:db8::32:23", "10.0.0.2");
    let _ = iprange6!("10.0.0.1", "2001:db8::32:23");
}
