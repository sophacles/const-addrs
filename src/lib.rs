#![feature(proc_macro_diagnostic)]

use proc_macro::TokenStream;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};
use std::str::FromStr;

use quote::{quote, quote_spanned};

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

fn ip4_tokens(addr: Ipv4Addr) -> proc_macro2::TokenStream {
    let [a, b, c, d] = addr.octets();

    quote! { ::std::net::Ipv4Addr::new(#a, #b, #c, #d) }
}

fn ip6_tokens(addr: Ipv6Addr) -> proc_macro2::TokenStream {
    let [a, b, c, d, e, f, g, h] = addr.segments();

    quote! { ::std::net::Ipv6Addr::new(#a, #b, #c, #d, #e, #f, #g, #h) }
}

#[proc_macro]
pub fn ip4(input: TokenStream) -> TokenStream {
    let ToAddr { inval } = parse_macro_input!(input as ToAddr);

    match Ipv4Addr::from_str(inval.value().as_str()) {
        Ok(ip) => ip4_tokens(ip).into(),
        Err(e) => {
            inval.span().unwrap().error(format!("{}", e)).emit();
            quote_spanned!(inval.span() => ::std::net::Ipv4Addr::UNSPECIFIED).into()
        }
    }
}

#[proc_macro]
pub fn ip6(input: TokenStream) -> TokenStream {
    let ToAddr { inval } = parse_macro_input!(input as ToAddr);

    match Ipv6Addr::from_str(inval.value().as_str()) {
        Ok(ip) => ip6_tokens(ip).into(),
        Err(e) => {
            inval.span().unwrap().error(format!("{}", e)).emit();
            quote_spanned!(inval.span() => ::std::net::Ipv6Addr::UNSPECIFIED).into()
        }
    }
}

#[proc_macro]
pub fn ip(input: TokenStream) -> TokenStream {
    let ToAddr { inval } = parse_macro_input!(input as ToAddr);

    match IpAddr::from_str(inval.value().as_str()) {
        Ok(addr) => match addr {
            IpAddr::V4(ip) => {
                let inner = ip4_tokens(ip);
                quote!(::std::net::IpAddr::V4(#inner)).into()
            }
            IpAddr::V6(ip) => {
                let inner = ip6_tokens(ip);
                quote!(::std::net::IpAddr::V6(#inner)).into()
            }
        },
        Err(e) => {
            inval.span().unwrap().error(format!("{}", e)).emit();
            let ip_tok = ip4_tokens(Ipv4Addr::UNSPECIFIED);
            quote_spanned!(inval.span() => ::std::net::IpAddr::V4(#ip_tok)).into()
        }
    }
}

fn sock4_tokens(addr: SocketAddrV4) -> proc_macro2::TokenStream {
    let ip = ip4_tokens(*addr.ip());
    let port = addr.port();
    quote!(::std::net::SocketAddrV4::new(#ip, #port)).into()
}

fn sock6_tokens(addr: SocketAddrV6) -> proc_macro2::TokenStream {
    let ip = ip6_tokens(*addr.ip());
    let port = addr.port();
    quote!(::std::net::SocketAddrV6::new(#ip, #port, 0, 0)).into()
}

#[proc_macro]
pub fn sock(input: TokenStream) -> TokenStream {
    let ToAddr { inval } = parse_macro_input!(input as ToAddr);

    match SocketAddr::from_str(inval.value().as_str()) {
        Ok(sock) => match sock {
            SocketAddr::V4(sock) => {
                let sock_tok = sock4_tokens(sock);
                quote!(::std::net::SocketAddr::V4(#sock_tok)).into()
            }
            SocketAddr::V6(sock) => {
                let sock_tok = sock6_tokens(sock);
                quote!(::std::net::SocketAddr::V6(#sock_tok)).into()
            }
        },
        Err(e) => {
            inval.span().unwrap().error(format!("{}", e)).emit();
            let sock_tok = sock4_tokens(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0));
            quote_spanned!(inval.span() => ::std::net::SocketAddr::V4(sock_tock)).into()
        }
    }
}

#[proc_macro]
pub fn sock4(input: TokenStream) -> TokenStream {
    let ToAddr { inval } = parse_macro_input!(input as ToAddr);

    match SocketAddrV4::from_str(inval.value().as_str()) {
        Ok(sock) => sock4_tokens(sock).into(),
        Err(e) => {
            inval.span().unwrap().error(format!("{}", e)).emit();
            let sock_tok = sock4_tokens(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0));
            quote_spanned!(inval.span() => #sock_tok).into()
        }
    }
}

#[proc_macro]
pub fn sock6(input: TokenStream) -> TokenStream {
    let ToAddr { inval } = parse_macro_input!(input as ToAddr);

    match SocketAddrV6::from_str(inval.value().as_str()) {
        Ok(sock) => sock6_tokens(sock).into(),
        Err(e) => {
            inval.span().unwrap().error(format!("{}", e)).emit();
            let sock_tok = sock6_tokens(SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, 0, 0, 0));
            quote_spanned!(inval.span() => #sock_tok).into()
        }
    }
}
