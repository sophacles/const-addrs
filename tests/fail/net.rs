use const_addrs::net;

fn main() {
    let a = net!("192.168.1.1/300");
    let b = net!("2001:db8::32::23/129");
}
