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

#[proc_macro]
#[proc_macro_error]
pub fn ip(input: TokenStream) -> TokenStream {
    let ToAddr { inval } = parse_macro_input!(input as ToAddr);

    match IpAddr::from_str(inval.value().as_str()) {
        Ok(addr) => match addr {
            IpAddr::V4(ip) => {
                let inner = ip4_tokens(Some(ip));
                quote! { ::std::net::IpAddr::V4(#inner) }.into()
            }
            IpAddr::V6(ip) => {
                let inner = ip6_tokens(Some(ip));
                quote! { ::std::net::IpAddr::V6(#inner) }.into()
            }
        },
        Err(e) => {
            emit_error!(inval, e);
            let ip_tok = ip4_tokens(None);
            quote! { ::std::net::IpAddr::V4(#ip_tok, 0) }.into()
        }
    }
}

#[proc_macro]
#[proc_macro_error]
pub fn ip4(input: TokenStream) -> TokenStream {
    let ToAddr { inval } = parse_macro_input!(input as ToAddr);

    match Ipv4Addr::from_str(inval.value().as_str()) {
        Ok(ip) => ip4_tokens(Some(ip)).into(),
        Err(e) => {
            emit_error!(inval, e.to_string());
            ip4_tokens(None).into()
        }
    }
}

#[proc_macro]
#[proc_macro_error]
pub fn ip6(input: TokenStream) -> TokenStream {
    let ToAddr { inval } = parse_macro_input!(input as ToAddr);

    match Ipv6Addr::from_str(inval.value().as_str()) {
        Ok(ip) => ip6_tokens(Some(ip)).into(),
        Err(e) => {
            emit_error!(inval, e);
            ip6_tokens(None).into()
        }
    }
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

#[proc_macro]
#[proc_macro_error]
pub fn sock(input: TokenStream) -> TokenStream {
    let ToAddr { inval } = parse_macro_input!(input as ToAddr);

    match SocketAddr::from_str(inval.value().as_str()) {
        Ok(sock) => match sock {
            SocketAddr::V4(sock) => {
                let sock_tok = sock4_tokens(Some(sock));
                quote! { ::std::net::SocketAddr::V4(#sock_tok) }.into()
            }
            SocketAddr::V6(sock) => {
                let sock_tok = sock6_tokens(Some(sock));
                quote! { ::std::net::SocketAddr::V6(#sock_tok) }.into()
            }
        },
        Err(e) => {
            emit_error!(inval, e);
            let sock_tok = sock4_tokens(None);
            quote! { ::std::net::SocketAddr::V4(#sock_tok) }.into()
        }
    }
}

#[proc_macro]
#[proc_macro_error]
pub fn sock4(input: TokenStream) -> TokenStream {
    let ToAddr { inval } = parse_macro_input!(input as ToAddr);

    match SocketAddrV4::from_str(inval.value().as_str()) {
        Ok(sock) => sock4_tokens(Some(sock)).into(),
        Err(e) => {
            emit_error!(inval, e);
            sock4_tokens(None).into()
        }
    }
}

#[proc_macro]
#[proc_macro_error]
pub fn sock6(input: TokenStream) -> TokenStream {
    let ToAddr { inval } = parse_macro_input!(input as ToAddr);

    match SocketAddrV6::from_str(inval.value().as_str()) {
        Ok(sock) => sock6_tokens(Some(sock)).into(),
        Err(e) => {
            emit_error!(inval, e);
            sock6_tokens(None).into()
        }
    }
}

#[cfg(feature = "ipnet")]
use ipnetwork::{IpNetwork, Ipv4Network, Ipv6Network};

#[cfg(feature = "ipnet")]
fn net4_tokens(net: Option<Ipv4Network>) -> proc_macro2::TokenStream {
    let net = net.unwrap_or(Ipv4Network::new(Ipv4Addr::UNSPECIFIED, 0).unwrap());
    let ip = ip4_tokens(Some(net.ip()));
    let prefix = net.prefix();
    quote! { ipnetwork::Ipv4Network::new(#ip, #prefix).unwrap() }
}

#[cfg(feature = "ipnet")]
fn net6_tokens(net: Option<Ipv6Network>) -> proc_macro2::TokenStream {
    let net = net.unwrap_or(Ipv6Network::new(Ipv6Addr::UNSPECIFIED, 0).unwrap());
    let ip = ip6_tokens(Some(net.ip()));
    let prefix = net.prefix();
    quote! { ipnetwork::Ipv6Network::new(#ip, #prefix).unwrap() }
}

#[cfg(feature = "ipnet")]
#[proc_macro]
#[proc_macro_error]
pub fn net(input: TokenStream) -> TokenStream {
    let ToAddr { inval } = parse_macro_input!(input as ToAddr);

    match IpNetwork::from_str(inval.value().as_str()) {
        Ok(sock) => match sock {
            IpNetwork::V4(net) => {
                let net_tok = net4_tokens(Some(net));
                quote! { ipnetwork::IpNetwork::V4(#net_tok) }.into()
            }
            IpNetwork::V6(net) => {
                let net_tok = net6_tokens(Some(net));
                quote! { ipnetwork::IpNetwork::V6(#net_tok) }.into()
            }
        },
        Err(e) => {
            emit_error!(inval, e);
            let net_tok = net4_tokens(None);
            quote! { ipnetwork::IpNetwork::V4(#net_tok) }.into()
        }
    }
}

#[cfg(feature = "ipnet")]
#[proc_macro]
#[proc_macro_error]
pub fn net4(input: TokenStream) -> TokenStream {
    let ToAddr { inval } = parse_macro_input!(input as ToAddr);

    match Ipv4Network::from_str(inval.value().as_str()) {
        Ok(net) => net4_tokens(Some(net)).into(),
        Err(e) => {
            emit_error!(inval, e);
            net4_tokens(None).into()
        }
    }
}

#[cfg(feature = "ipnet")]
#[proc_macro]
#[proc_macro_error]
pub fn net6(input: TokenStream) -> TokenStream {
    let ToAddr { inval } = parse_macro_input!(input as ToAddr);

    match Ipv6Network::from_str(inval.value().as_str()) {
        Ok(net) => net6_tokens(Some(net)).into(),
        Err(e) => {
            emit_error!(inval, e);
            net6_tokens(None).into()
        }
    }
}
