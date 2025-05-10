use const_addrs::sock;

fn main() {
    let _ = sock!("192.168.11:300");
    let _ = sock!("2001:db8::32::23:22");
}
