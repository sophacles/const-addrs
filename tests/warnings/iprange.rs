use const_addrs::iprange;

fn main() {
    let _ = iprange!("2001:db8::32:ffff", "2001:db8::32:23");
    let _ = iprange!("10.128.0.0", "10.0.0.0");

    //failure so the output is compared
    let _ = iprange!("10128.0.0", "10.0.0.0");
}
