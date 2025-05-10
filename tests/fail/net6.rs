use const_addrs::net6;

fn main() {
    let _ = net6!("2001:db8::32::23/129");
    let _ = net6!("2001:db8::32:23/129");
}
