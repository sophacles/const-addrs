use proc_macro::TokenStream;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};
use std::str::FromStr;

use proc_macro_error::{emit_error, proc_macro_error};
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, LitStr};

struct ToAddr {
    inval: LitStr,
}

impl Parse for ToAddr {
    fn parse(input: ParseStream) -> Result<Self> {
        let inval = input.parse::<LitStr>()?;
        Ok(ToAddr { inval })
    }
}

macro_rules! parse_type {
    ($input:ident, $fn:ident, $ty:ty) => {{
        let ToAddr { inval } = parse_macro_input!($input as ToAddr);

        match <$ty>::from_str(inval.value().as_str()) {
            Ok(v) => $fn(Some(v)).into(),
            Err(e) => {
                emit_error!(inval, e);
                $fn(None).into()
            }
        }
    }};
}

fn ip4_tokens(addr: Option<Ipv4Addr>) -> proc_macro2::TokenStream {
    let addr = addr.unwrap_or(Ipv4Addr::UNSPECIFIED);
    let [a, b, c, d] = addr.octets();

    quote! { ::std::net::Ipv4Addr::new(#a, #b, #c, #d) }
}

fn ip6_tokens(addr: Option<Ipv6Addr>) -> proc_macro2::TokenStream {
    let addr = addr.unwrap_or(Ipv6Addr::UNSPECIFIED);
    let [a, b, c, d, e, f, g, h] = addr.segments();

    quote! { ::std::net::Ipv6Addr::new(#a, #b, #c, #d, #e, #f, #g, #h) }
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
            let sock_tok = sock4_tokens(Some(sock));
            quote! { ::std::net::SocketAddr::V4(#sock_tok) }
        }
        SocketAddr::V6(sock) => {
            let sock_tok = sock6_tokens(Some(sock));
            quote! { ::std::net::SocketAddr::V6(#sock_tok) }
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
                    let net_tok = net4_tokens(Some(net));
                    quote! { ipnetwork::IpNetwork::V4(#net_tok) }
                }
                IpNetwork::V6(net) => {
                    let net_tok = net6_tokens(Some(net));
                    quote! { ipnetwork::IpNetwork::V6(#net_tok) }
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
