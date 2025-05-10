use const_addrs::ip;

fn main() {
    let _ = ip!("192.168.11");
    let _ = ip!("2001:db8::32::23");
}
