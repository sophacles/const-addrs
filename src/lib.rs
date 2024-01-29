use proc_macro::TokenStream;

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};
use std::str::FromStr;

use proc_macro_error::{abort_call_site, emit_error, proc_macro_error};
use quote::quote;
use syn::{parse, LitStr};

macro_rules! parse_type {
    ($input:ident, $fn:ident, $ty:ty) => {{
        let inval = match parse::<LitStr>($input) {
            Ok(ls) => ls,
            Err(e) => {
                abort_call_site!(
                    e.to_string();
                    help = "Use the string literal for this type's FromStr impl";
                );
            }
        };

        // try to parse type
        match <$ty>::from_str(inval.value().as_str()) {
            // if good, turn it into the right token stream
            Ok(v) => $fn(Some(v)).into(),
            Err(e) => {
                // otherwise emit the parse error with the parser's error message
                // as well as make the compile error show the invalid string
                emit_error!(inval, e);
                // return dummy value of the right type to squash subsequent errors
                $fn(None).into()
            }
        }
    }};
}

// IP types

fn ip4_tokens(addr: Option<Ipv4Addr>) -> proc_macro2::TokenStream {
    let addr = addr.unwrap_or(Ipv4Addr::UNSPECIFIED);
    let octets = addr.octets();

    quote! { ::std::net::Ipv4Addr::new(#(#octets),*) }
}

fn ip6_tokens(addr: Option<Ipv6Addr>) -> proc_macro2::TokenStream {
    let addr = addr.unwrap_or(Ipv6Addr::UNSPECIFIED);
    let segments = addr.segments();

    quote! { ::std::net::Ipv6Addr::new(#(#segments),*) }
}

fn ipaddr_tokens(addr: Option<IpAddr>) -> proc_macro2::TokenStream {
    let addr = addr.unwrap_or(IpAddr::V4(Ipv4Addr::UNSPECIFIED));
    match addr {
        IpAddr::V4(ip) => {
            let inner = ip4_tokens(Some(ip));
            quote! { ::std::net::IpAddr::V4(#inner) }
        }
        IpAddr::V6(ip) => {
            let inner = ip6_tokens(Some(ip));
            quote! { ::std::net::IpAddr::V6(#inner) }
        }
    }
}

#[proc_macro]
#[proc_macro_error]
pub fn ip(input: TokenStream) -> TokenStream {
    parse_type!(input, ipaddr_tokens, IpAddr)
}

#[proc_macro]
#[proc_macro_error]
pub fn ip4(input: TokenStream) -> TokenStream {
    parse_type!(input, ip4_tokens, Ipv4Addr)
}

#[proc_macro]
#[proc_macro_error]
pub fn ip6(input: TokenStream) -> TokenStream {
    parse_type!(input, ip6_tokens, Ipv6Addr)
}

// SocketAddr types

fn sock4_tokens(addr: Option<SocketAddrV4>) -> proc_macro2::TokenStream {
    let addr = addr.unwrap_or(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0));
    let ip = ip4_tokens(Some(*addr.ip()));
    let port = addr.port();

    quote! { ::std::net::SocketAddrV4::new(#ip, #port) }
}

fn sock6_tokens(addr: Option<SocketAddrV6>) -> proc_macro2::TokenStream {
    let addr = addr.unwrap_or(SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, 0, 0, 0));
    let ip = ip6_tokens(Some(*addr.ip()));
    let port = addr.port();

    quote! { ::std::net::SocketAddrV6::new(#ip, #port, 0, 0) }
}

fn sockaddr_tokens(addr: Option<SocketAddr>) -> proc_macro2::TokenStream {
    let addr = addr.unwrap_or(SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0)));
    match addr {
        SocketAddr::V4(sock) => {
            let inner = sock4_tokens(Some(sock));
            quote! { ::std::net::SocketAddr::V4(#inner) }
        }
        SocketAddr::V6(sock) => {
            let inner = sock6_tokens(Some(sock));
            quote! { ::std::net::SocketAddr::V6(#inner) }
        }
    }
}

#[proc_macro]
#[proc_macro_error]
pub fn sock(input: TokenStream) -> TokenStream {
    parse_type!(input, sockaddr_tokens, SocketAddr)
}

#[proc_macro]
#[proc_macro_error]
pub fn sock4(input: TokenStream) -> TokenStream {
    parse_type!(input, sock4_tokens, SocketAddrV4)
}

#[proc_macro]
#[proc_macro_error]
pub fn sock6(input: TokenStream) -> TokenStream {
    parse_type!(input, sock6_tokens, SocketAddrV6)
}

// IpNetwork types

cfg_if::cfg_if! {
    if #[cfg(feature = "ipnet")] {
        use ipnetwork::{IpNetwork, Ipv4Network, Ipv6Network};

        fn net4_tokens(net: Option<Ipv4Network>) -> proc_macro2::TokenStream {
            let net = net.unwrap_or(Ipv4Network::new(Ipv4Addr::UNSPECIFIED, 0).unwrap());
            let ip = ip4_tokens(Some(net.ip()));
            let prefix = net.prefix();

            quote! { ipnetwork::Ipv4Network::new(#ip, #prefix).unwrap() }
        }

        fn net6_tokens(net: Option<Ipv6Network>) -> proc_macro2::TokenStream {
            let net = net.unwrap_or(Ipv6Network::new(Ipv6Addr::UNSPECIFIED, 0).unwrap());
            let ip = ip6_tokens(Some(net.ip()));
            let prefix = net.prefix();

            quote! { ipnetwork::Ipv6Network::new(#ip, #prefix).unwrap() }
        }

        fn net_tokens(net: Option<IpNetwork>) -> proc_macro2::TokenStream {
            let net = net.unwrap_or(IpNetwork::V4(
                Ipv4Network::new(Ipv4Addr::UNSPECIFIED, 0).unwrap(),
            ));
            match net {
                IpNetwork::V4(net) => {
                    let inner = net4_tokens(Some(net));
                    quote! { ipnetwork::IpNetwork::V4(#inner) }
                }
                IpNetwork::V6(net) => {
                    let inner = net6_tokens(Some(net));
                    quote! { ipnetwork::IpNetwork::V6(#inner) }
                }
            }
        }

        #[proc_macro]
        #[proc_macro_error]
        pub fn net(input: TokenStream) -> TokenStream {
            parse_type!(input, net_tokens, IpNetwork)
        }

        #[proc_macro]
        #[proc_macro_error]
        pub fn net4(input: TokenStream) -> TokenStream {
            parse_type!(input, net4_tokens, Ipv4Network)
        }

        #[proc_macro]
        #[proc_macro_error]
        pub fn net6(input: TokenStream) -> TokenStream {
            parse_type!(input, net6_tokens, Ipv6Network)
        }
    }
}

// MacAddr types

cfg_if::cfg_if! {
    if #[cfg(feature = "mac")] {
        use macaddr::{MacAddr, MacAddr6, MacAddr8};

        fn mac6_tokens(addr: Option<MacAddr6>) -> proc_macro2::TokenStream {
            let addr = addr.unwrap_or(MacAddr6::from([0x00; 6]));
            let bytes = addr.into_array();

            quote! { macaddr::MacAddr6::new(#(#bytes),*) }
        }

        fn mac8_tokens(addr: Option<MacAddr8>) -> proc_macro2::TokenStream {
            let addr = addr.unwrap_or(MacAddr8::from([0x00; 8]));
            let bytes = addr.into_array();

            quote! { macaddr::MacAddr8::new(#(#bytes),*) }
        }

        fn mac_tokens(addr: Option<MacAddr>) -> proc_macro2::TokenStream {
            let addr = addr.unwrap_or(MacAddr::V6(MacAddr6::from([0x00; 6])));
            match addr {
                MacAddr::V6(addr) => {
                    let inner = mac6_tokens(Some(addr));
                    quote! { macaddr::MacAddr::V6(#inner) }
                }
                MacAddr::V8(addr) => {
                    let inner = mac8_tokens(Some(addr));
                    quote! { macaddr::MacAddr::V8(#inner) }
                }
            }
        }

        #[proc_macro]
        #[proc_macro_error]
        pub fn mac(input: TokenStream) -> TokenStream {
            parse_type!(input, mac_tokens, MacAddr)
        }

        #[proc_macro]
        #[proc_macro_error]
        pub fn mac6(input: TokenStream) -> TokenStream {
            parse_type!(input, mac6_tokens, MacAddr6)
        }

        #[proc_macro]
        #[proc_macro_error]
        pub fn mac8(input: TokenStream) -> TokenStream {
            parse_type!(input, mac8_tokens, MacAddr8)
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn compilation() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/fail/bad_tt.rs");

        t.compile_fail("tests/fail/ip.rs");
        t.compile_fail("tests/fail/ip4.rs");
        t.compile_fail("tests/fail/ip6.rs");
        t.compile_fail("tests/fail/sock.rs");
        t.compile_fail("tests/fail/sock4.rs");
        t.compile_fail("tests/fail/sock6.rs");
        t.compile_fail("tests/fail/net.rs");
        t.compile_fail("tests/fail/net4.rs");
        t.compile_fail("tests/fail/net6.rs");
        t.compile_fail("tests/fail/mac.rs");
        t.compile_fail("tests/fail/mac6.rs");
        t.compile_fail("tests/fail/mac8.rs");

        t.pass("tests/pass/ip.rs");
        t.pass("tests/pass/ip4.rs");
        t.pass("tests/pass/ip6.rs");
        t.pass("tests/pass/sock.rs");
        t.pass("tests/pass/sock4.rs");
        t.pass("tests/pass/sock6.rs");
        t.pass("tests/pass/net.rs");
        t.pass("tests/pass/net4.rs");
        t.pass("tests/pass/net6.rs");
        t.pass("tests/pass/mac.rs");
        t.pass("tests/pass/mac6.rs");
        t.pass("tests/pass/mac8.rs");
    }
}
