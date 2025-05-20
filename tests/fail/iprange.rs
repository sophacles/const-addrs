use const_addrs::iprange;

fn main() {
    let _ = iprange!("2001:db8::32:23", "2001:db8::32::ffff");
    let _ = iprange!("2001:db8::32::23", "2001:db8::32:ffff");
    let _ = iprange!("2001:db8::32::23", "2001:db8::32::ffff");
    let _ = iprange!("2001:db8::32:23", "10.0.0.2");
    let _ = iprange!("10.0.0.1", "2001:db8::32:23");
    let _ = iprange!("2001:db8::32:23", "10.0.0.1");
    let _ = iprange!("100.0.0", "10.0.0.255");
    let _ = iprange!("10.0.0.0", "10.0.0255");
}
