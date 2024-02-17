use const_addrs::sock;

fn main() {
    let a = sock!("192.168.11:300");
    let b = sock!("2001:db8::32::23:22");
}
