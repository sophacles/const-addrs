use const_addrs::iprange4;

fn main() {
    let _ = iprange4!("10.0.01", "10.0.1.0");
    let _ = iprange4!("10.0.0.1", "10.0.10");
    let _ = iprange4!("10.0.01", "10.0.10");
    let _ = iprange4!("10.0.0.1", "2001:db8::32:23");
}
