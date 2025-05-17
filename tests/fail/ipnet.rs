use const_addrs::ipnet;

fn main() {
    let _ = ipnet!("192.168.1.1/300");
    let _ = ipnet!("2001:db8::32::23/129");
}
