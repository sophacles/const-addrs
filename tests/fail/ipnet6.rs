use const_addrs::ipnet6;

fn main() {
    let _ = ipnet6!("2001:db8::32::23/129");
    let _ = ipnet6!("2001:db8::32:23/129");
}
