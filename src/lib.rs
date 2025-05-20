//! A set of macros for creating networking types from a string literal.
//!
//! ```rust
//! use std::net::{Ipv4Addr, IpAddr};
//! use const_addrs::ip;
//!
//! # fn main() {
//! let a = ip!("192.168.1.1");
//! let b = IpAddr::V4(Ipv4Addr::new(192,168,1,1));
//! assert_eq!(a, b);
//!  # }
//! ```
//!
//! Turning invalid strings into compile-time errors:
//! ```text
//! error: invalid IPv4 address syntax
//!   --> bad.rs:10:18
//!    |
//! 10 |     let a = ip4!("192.1681.1");
//!    |                  ^^^^^^^^^^^^
//! ```
//!
//! These macros will parse the string passed to them using its type's [`FromStr`] implementation.
//! See the documentation for each type for formatting details. The macro generated code will use
//! the `const` constructor(s) for the types, adding no runtime overhead.
//!
//! For example:
//! ```rust
//! # use std::net::SocketAddr;
//! # use const_addrs::sock;
//! #
//! # fn main() {
//! let val = sock!("192.168.1.1:500");
//! # // copied from below for doctest verification
//! # let val2 = ::std::net::SocketAddr::V4(::std::net::SocketAddrV4::new(
//! #     ::std::net::Ipv4Addr::new(192u8, 168u8, 1u8, 1u8),
//! #     500u16,
//! # ));
//! # assert_eq!(val, val2)
//! # }
//! ```
//! expands to:
//! ```rust
//! let val = ::std::net::SocketAddr::V4(::std::net::SocketAddrV4::new(
//!     ::std::net::Ipv4Addr::new(192u8, 168u8, 1u8, 1u8),
//!     500u16,
//! ));
//! ```
#![cfg_attr(feature = "document-features", doc = "\n## Features")]
#![cfg_attr(feature = "document-features", doc = document_features::document_features!())]
#![cfg_attr(docsrs, feature(doc_cfg))]

use proc_macro::TokenStream;

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};
use std::str::FromStr;

use proc_macro_error::{abort_call_site, emit_error, proc_macro_error};
use quote::quote;
use syn::{parse, LitStr};

// generate each proc_macro.
//
// the dummy value is used to make sure the emitted error is the only one generated when
// type::from_str errors (e.g `let ip = ip!("foo");` expanding to `let ip = ; `
//
// the closure in `$body` will generate tokens to call the type's const constructor
// which is used in the proc_macro itself.
macro_rules! make_macro {
    ($macro_name:ident: $ty:ty =? $dummy:expr; $generate_fn:ident => $body:expr; $tyname:expr; $feat:expr) => {
        #[cfg_attr(docsrs, doc(cfg(feature = $feat)))]
        #[doc = "generates [`"]
        #[doc = $tyname]
        #[doc = "`]"]
        #[proc_macro]
        #[proc_macro_error]
        pub fn $macro_name(input: TokenStream) -> TokenStream {
            let inval = match parse::<LitStr>(input) {
                Ok(ls) => ls,
                Err(e) => {
                    abort_call_site!(
                        e.to_string();
                        help = "Use the string literal for this type's FromStr impl";
                    );
                }
            };

            // try to parse type
            let val = match <$ty>::from_str(inval.value().as_str()) {
                // if good, turn it into the right token stream
                Ok(v) => v,
                Err(e) => {
                    // otherwise emit the parse error with the parser's error message
                    // as well as make the compile error show the invalid string
                    emit_error!(inval, e);
                    // return dummy value of the right type to squash subsequent errors
                    $dummy
                }
            };
            $generate_fn(val).into()
        }

        fn $generate_fn(input: $ty) -> proc_macro2::TokenStream {
            Some(input).map($body).unwrap()
        }
    };
    ($macro_name:ident: $ty:ty =? $dummy:expr; $generate_fn:ident => $body:expr) => {
        make_macro!($macro_name: $ty =? $dummy; $generate_fn => $body; stringify!($ty); "default");
    };
    ($macro_name:ident: $ty:ty =? $dummy:expr; $generate_fn:ident => $body:expr; $feat:expr) => {
        make_macro!($macro_name: $ty =? $dummy; $generate_fn => $body; stringify!($ty); $feat);
    }
}

// IP types

make_macro! {
    ip: IpAddr =? IpAddr::V4(Ipv4Addr::UNSPECIFIED);
    ipaddr_tokens => |input| match input {
        IpAddr::V4(ip) => {
            let inner = ip4_tokens(ip);
            quote! { ::std::net::IpAddr::V4(#inner) }
        }
        IpAddr::V6(ip) => {
            let inner = ip6_tokens(ip);
            quote! { ::std::net::IpAddr::V6(#inner) }
        }
    }
}

make_macro! {
    ip4: Ipv4Addr =? Ipv4Addr::UNSPECIFIED;
    ip4_tokens => |input| {
        let octets = input.octets();
        quote! { ::std::net::Ipv4Addr::new(#(#octets),*) }
    }
}

make_macro! {
    ip6: Ipv6Addr =? Ipv6Addr::UNSPECIFIED;
    ip6_tokens => |input| {
        let segments = input.segments();
        quote! { ::std::net::Ipv6Addr::new(#(#segments),*) }
    }
}

// SocketAddr types

make_macro! {
    sock: SocketAddr =? SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0));
    sockaddr_tokens => |addr| {
        match addr {
            SocketAddr::V4(sock) => {
                let inner = sock4_tokens(sock);
                quote! { ::std::net::SocketAddr::V4(#inner) }
            }
            SocketAddr::V6(sock) => {
                let inner = sock6_tokens(sock);
                quote! { ::std::net::SocketAddr::V6(#inner) }
            }
        }
    }
}

make_macro! {
    sock4: SocketAddrV4 =? SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0);
    sock4_tokens => |input| {
        let ip = ip4_tokens(*input.ip());
        let port = input.port();
        quote! { ::std::net::SocketAddrV4::new(#ip, #port) }
    }
}

make_macro! {
    sock6: SocketAddrV6 =? SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, 0, 0, 0);
    sock6_tokens => |addr| {
        let ip = ip6_tokens(*addr.ip());
        let port = addr.port();
        quote! { ::std::net::SocketAddrV6::new(#ip, #port, 0, 0) }
    }
}

// IpNetwork types

cfg_if::cfg_if! {
    if #[cfg(feature = "ipnetwork")] {
        use ipnetwork::{IpNetwork, Ipv4Network, Ipv6Network};

        make_macro!{
            net: IpNetwork =? IpNetwork::V4(Ipv4Network::new(Ipv4Addr::UNSPECIFIED, 0).unwrap());
            net_tokens => |net| match net {
                IpNetwork::V4(net) => {
                    let inner = net4_tokens(net);
                    quote! { ipnetwork::IpNetwork::V4(#inner) }
                }
                IpNetwork::V6(net) => {
                    let inner = net6_tokens(net);
                    quote! { ipnetwork::IpNetwork::V6(#inner) }
                }
            };
            "ipnetwork"
        }

        make_macro!{
            net4: Ipv4Network =? Ipv4Network::new(Ipv4Addr::UNSPECIFIED, 0).unwrap();
            net4_tokens => |net| {
                let ip = ip4_tokens(net.ip());
                let prefix = net.prefix();
                quote! { ipnetwork::Ipv4Network::new_checked(#ip, #prefix).unwrap() }
            };
            "ipnetwork"
        }


        make_macro!{
            net6: Ipv6Network =? Ipv6Network::new(Ipv6Addr::UNSPECIFIED, 0).unwrap();
            net6_tokens => |net| {
                let ip = ip6_tokens(net.ip());
                let prefix = net.prefix();
                quote! { ipnetwork::Ipv6Network::new_checked(#ip, #prefix).unwrap() }
            };
            "ipnetwork"
        }
    }
}

// ipnet types
cfg_if::cfg_if! {
    if #[cfg(feature = "ipnet")] {
        use ipnet::{
            IpNet, Ipv4Net, Ipv6Net,
        };
        use proc_macro_error::emit_warning;

        make_macro!{
            ipnet: IpNet =? IpNet::V4(Ipv4Net::new(Ipv4Addr::UNSPECIFIED, 0).unwrap());
            ipnet_tokens => |net| match net {
                IpNet::V4(net) => {
                    let inner = ipnet4_tokens(net);
                    quote! { ipnet::IpNet::V4(#inner) }
                }
                IpNet::V6(net) => {
                    let inner = ipnet6_tokens(net);
                    quote! { ipnet::IpNet::V6(#inner) }
                }
            };
            "ipnetwork"
        }

        make_macro!{
            ipnet4: Ipv4Net =? Ipv4Net::new(Ipv4Addr::UNSPECIFIED, 0).unwrap();
            ipnet4_tokens => |net| {
                let ip = ip4_tokens(net.addr());
                let prefix = net.prefix_len();
                quote! { ipnet::Ipv4Net::new_assert(#ip, #prefix) }
            };
            "ipnetwork"
        }


        make_macro!{
            ipnet6: Ipv6Net =? Ipv6Net::new(Ipv6Addr::UNSPECIFIED, 0).unwrap();
            ipnet6_tokens => |net| {
                let ip = ip6_tokens(net.addr());
                let prefix = net.prefix_len();
                quote! { ipnet::Ipv6Net::new_assert(#ip, #prefix) }
            };
            "ipnetwork"
        }


        struct RangeArgs {
            ip1: IpAddr,
            ip2: IpAddr,
            span1: proc_macro2::Span,
            span2: proc_macro2::Span,
            cancel: bool,
        }

        impl syn::parse::Parse for RangeArgs {
            fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
                let ip1_lit = input.parse::<syn::LitStr>()?;
                input.parse::<syn::Token![,]>()?;
                let ip2_lit = input.parse::<LitStr>()?;

                let span1 = ip1_lit.span();
                let span2 = ip2_lit.span();

                let mut cancel= false;
                let ip1 = IpAddr::from_str(ip1_lit.value().as_str()).unwrap_or_else(|e| {
                    emit_error!(ip1_lit, e);
                    cancel= true;
                    // return dummy value of the right type to squash subsequent errors
                    IpAddr::V4(Ipv4Addr::UNSPECIFIED)
                });
                let ip2 = IpAddr::from_str(ip2_lit.value().as_str()).unwrap_or_else(|e| {
                    emit_error!(ip2_lit, e);
                    cancel= true;
                    // return dummy value of the right type to squash subsequent errors
                    IpAddr::V4(Ipv4Addr::UNSPECIFIED)
                });

                Ok(Self{ip1, ip2, span1, span2, cancel})
            }
        }

        impl RangeArgs {
            fn range_span(&self) -> Option<proc_macro2::Span> {
               self.span1.join(self.span2)
            }

            fn v4_tokens(&self) -> proc_macro2::TokenStream  {
                if self.cancel {
                    let ip1_tok = ip4_tokens(Ipv4Addr::UNSPECIFIED);
                    let ip2_tok = ip4_tokens(Ipv4Addr::UNSPECIFIED);
                    return quote! { #ip1_tok, #ip2_tok }
                }
                let mut errored: bool = false;
                let ip1_tok = match self.ip1 {
                    IpAddr::V4(val) => ip4_tokens(val),
                    IpAddr::V6(_) => {
                        emit_error!(self.span1, "Addr is IPv6 but IPv4 is expected");
                        errored = true;
                        ip4_tokens(Ipv4Addr::UNSPECIFIED)
                    }
                };

                let ip2_tok = match self.ip2 {
                    IpAddr::V4(val) => ip4_tokens(val),
                    IpAddr::V6(_) => {
                        emit_error!(self.span2, "Addr is IPv6 but IPv4 is expected");
                        errored = true;
                        ip4_tokens(Ipv4Addr::UNSPECIFIED)
                    }
                };

                if  !errored && self.ip1 >= self.ip2 {
                    if let Some(span) = self.range_span() {
                        emit_warning!(
                            span,
                            "This range will yeild no values: IP1 >= IP2";
                            help="This iterator requires the lower IP value first in it's arguments list."

                        );
                    }
                }

                quote! { #ip1_tok, #ip2_tok }
            }

            fn v6_tokens(&self) -> proc_macro2::TokenStream  {
                if self.cancel{
                    let ip1_tok = ip6_tokens(Ipv6Addr::UNSPECIFIED);
                    let ip2_tok = ip6_tokens(Ipv6Addr::UNSPECIFIED);
                    return quote! { #ip1_tok, #ip2_tok }
                }
                let mut errored: bool = false;
                let ip1_tok = match self.ip1 {
                    IpAddr::V6(val) => ip6_tokens(val),
                    IpAddr::V4(_) => {
                        emit_error!(self.span1, "Addr is IPv4 but IPv6 is expected");
                        errored = true;
                        ip6_tokens(Ipv6Addr::UNSPECIFIED)
                    }
                };

                let ip2_tok = match self.ip2 {
                    IpAddr::V6(val) => ip6_tokens(val),
                    IpAddr::V4(_) => {
                        emit_error!(self.span2, "Addr is IPv4 but IPv6 is expected");
                        errored = true;
                        ip6_tokens(Ipv6Addr::UNSPECIFIED)
                    }
                };

                if !errored && self.ip1 >= self.ip2 {
                    if let Some(span) = self.range_span() {
                        emit_warning!(
                            span,
                            "This range will yeild no values: IP1 >= IP2";
                            help="This iterator requires the lower IP value first in it's arguments list."
                        );
                    }
                }

                quote! { #ip1_tok, #ip2_tok }
            }
        }

        struct SubnetArgs {
            range: RangeArgs,
            min_prefix_len: u8,
            span: proc_macro2::Span
        }

        impl syn::parse::Parse for SubnetArgs {
            fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
                let range = input.parse::<RangeArgs>()?;
                input.parse::<syn::Token![,]>()?;
                let min_prefix_len_lit = input.parse::<syn::LitInt>()?;
                let span = min_prefix_len_lit.span();

                let min_prefix_len = match min_prefix_len_lit.base10_parse::<u8>() {
                    Ok(v) => v,
                    Err(e) => {
                        emit_error!(min_prefix_len_lit, e);
                        0
                    }
                };
                Ok(Self{range, min_prefix_len, span})
            }
        }

        impl SubnetArgs {
            fn v4_tokens(&self) -> proc_macro2::TokenStream {
                let inner = self.range.v4_tokens();
                if self.min_prefix_len > 32 {
                    emit_error!(self.span, "Minimum prefix must be 32 or less");
                }
                let size = self.min_prefix_len;
                quote! { #inner , #size }
            }

            fn v6_tokens(&self) -> proc_macro2::TokenStream {
                let inner = self.range.v6_tokens();
                if self.min_prefix_len > 128 {
                    emit_error!(self.span, "Minimum prefix must be 128 or less");
                }
                let size = self.min_prefix_len;
                quote! { #inner , #size }
            }
        }

        #[cfg_attr(docsrs, doc(cfg(feature = "ipnet")))]
        #[doc = "generates [`Ipv4AddrRange`](ipnet::Ipv4AddrRange)"]
        #[proc_macro]
        #[proc_macro_error]
        pub fn iprange4(input: TokenStream) -> TokenStream {
            let args = syn::parse_macro_input!(input as RangeArgs);
            let inner = args.v4_tokens();
            quote! { ::ipnet::Ipv4AddrRange::new(#inner) }.into()
        }

        #[cfg_attr(docsrs, doc(cfg(feature = "ipnet")))]
        #[doc = "generates [`Ipv6AddrRange`](ipnet::Ipv6AddrRange)"]
        #[proc_macro]
        #[proc_macro_error]
        pub fn iprange6(input: TokenStream) -> TokenStream {
            let args = syn::parse_macro_input!(input as RangeArgs);
            let inner = args.v6_tokens();
            quote! { ::ipnet::Ipv6AddrRange::new(#inner) }.into()
        }

        #[cfg_attr(docsrs, doc(cfg(feature = "ipnet")))]
        #[doc = "generates [`IpAddrRange`](ipnet::IpAddrRange)"]
        #[proc_macro]
        #[proc_macro_error]
        pub fn iprange(input: TokenStream) -> TokenStream {
            let args = syn::parse_macro_input!(input as RangeArgs);
            match args.ip1 {
                IpAddr::V4(_) => {
                    let inner = args.v4_tokens();
                    quote! { ::ipnet::IpAddrRange::V4(::ipnet::Ipv4AddrRange::new(#inner)) }.into()
                }
                IpAddr::V6(_) => {
                    let inner = args.v6_tokens();
                    quote! { ::ipnet::IpAddrRange::V6(::ipnet::Ipv6AddrRange::new(#inner)) }.into()
                }
            }
        }

        #[cfg_attr(docsrs, doc(cfg(feature = "ipnet")))]
        #[doc = "generates [`Ipv4Subnets`](ipnet::Ipv4Subnets)"]
        #[proc_macro]
        #[proc_macro_error]
        pub fn ipsubnets4(input: TokenStream) -> TokenStream {
            let args = syn::parse_macro_input!(input as SubnetArgs);
            let inner = args.v4_tokens();
            quote! { ::ipnet::Ipv4Subnets::new(#inner) }.into()
        }

        #[cfg_attr(docsrs, doc(cfg(feature = "ipnet")))]
        #[doc = "generates [`Ipv6Subnets`](ipnet::Ipv6Subnets)"]
        #[proc_macro]
        #[proc_macro_error]
        pub fn ipsubnets6(input: TokenStream) -> TokenStream {
            let args = syn::parse_macro_input!(input as SubnetArgs);
            let inner = args.v6_tokens();
            quote! { ::ipnet::Ipv6Subnets::new(#inner) }.into()
        }

        #[cfg_attr(docsrs, doc(cfg(feature = "ipnet")))]
        #[doc = "generates [`IpSubnets`](ipnet::IpSubnets)"]
        #[proc_macro]
        #[proc_macro_error]
        pub fn ipsubnets(input: TokenStream) -> TokenStream {
            let args = syn::parse_macro_input!(input as SubnetArgs);
            match args.range.ip1 {
                IpAddr::V4(_) => {
                    let inner = args.v4_tokens();
                    quote! { ::ipnet::IpSubnets::V4(::ipnet::Ipv4Subnets::new(#inner)) }.into()
                }
                IpAddr::V6(_) => {
                    let inner = args.v6_tokens();
                    quote! { ::ipnet::IpSubnets::V6(::ipnet::Ipv6Subnets::new(#inner)) }.into()
                }
            }
        }

    }
}

// MacAddr types

cfg_if::cfg_if! {
    if #[cfg(feature = "macaddr")] {
        use macaddr::{MacAddr, MacAddr6, MacAddr8};

        make_macro!{
            mac: MacAddr =? MacAddr::V6(MacAddr6::from([0x00; 6]));
            mac_tokens => |addr| match addr {
                MacAddr::V6(addr) => {
                    let inner = mac6_tokens(addr);
                    quote! { macaddr::MacAddr::V6(#inner) }
                }
                MacAddr::V8(addr) => {
                    let inner = mac8_tokens(addr);
                    quote! { macaddr::MacAddr::V8(#inner) }
                }
            };
            "macaddr"
        }

        make_macro! {
            mac6: MacAddr6 =? MacAddr6::from([0x00; 6]);
            mac6_tokens => |addr| {
                let bytes = addr.into_array();
                quote! { macaddr::MacAddr6::new(#(#bytes),*) }
            };
            "macaddr"
        }

        make_macro!{
            mac8: MacAddr8 =? MacAddr8::from([0x00; 8]);
            mac8_tokens => |addr| {
                let bytes = addr.into_array();
                quote! { macaddr::MacAddr8::new(#(#bytes),*) }
            };
            "macaddr"
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn base_types() {
        let t = trybuild::TestCases::new();

        t.compile_fail("tests/fail/ip.rs");
        t.compile_fail("tests/fail/ip4.rs");
        t.compile_fail("tests/fail/ip6.rs");
        t.compile_fail("tests/fail/sock.rs");
        t.compile_fail("tests/fail/sock4.rs");
        t.compile_fail("tests/fail/sock6.rs");

        t.pass("tests/pass/ip.rs");
        t.pass("tests/pass/ip4.rs");
        t.pass("tests/pass/ip6.rs");
        t.pass("tests/pass/sock.rs");
        t.pass("tests/pass/sock4.rs");
        t.pass("tests/pass/sock6.rs");
    }

    #[cfg(feature = "ipnetwork")]
    #[test]
    fn ipnetwork_types() {
        let t = trybuild::TestCases::new();

        t.compile_fail("tests/fail/net.rs");
        t.compile_fail("tests/fail/net4.rs");
        t.compile_fail("tests/fail/net6.rs");

        t.pass("tests/pass/net.rs");
        t.pass("tests/pass/net4.rs");
        t.pass("tests/pass/net6.rs");
    }

    #[cfg(feature = "ipnet")]
    #[allow(unexpected_cfgs)]
    #[test]
    fn ipnet_types() {
        let t = trybuild::TestCases::new();

        t.compile_fail("tests/fail/ipnet.rs");
        t.compile_fail("tests/fail/ipnet4.rs");
        t.compile_fail("tests/fail/ipnet6.rs");

        // The error output between stable and nightly differs
        // and has an effect on these tests. Therefor there are
        // two different versions for stable and nightly
        if cfg!(nightly) {
            t.compile_fail("tests/fail_nightly/iprange4.rs");
            t.compile_fail("tests/fail_nightly/iprange6.rs");
            t.compile_fail("tests/fail_nightly/iprange.rs");
            t.compile_fail("tests/fail_nightly/ipsubnets4.rs");
            t.compile_fail("tests/fail_nightly/ipsubnets6.rs");
            t.compile_fail("tests/fail_nightly/ipsubnets.rs");
        } else {
            t.compile_fail("tests/fail/iprange4.rs");
            t.compile_fail("tests/fail/iprange6.rs");
            t.compile_fail("tests/fail/iprange.rs");
            t.compile_fail("tests/fail/ipsubnets4.rs");
            t.compile_fail("tests/fail/ipsubnets6.rs");
            t.compile_fail("tests/fail/ipsubnets.rs");
        }

        // Nightly supports warnings, so test those out
        if cfg!(nightly) {
            t.compile_fail("tests/warnings/iprange4.rs");
            t.compile_fail("tests/warnings/iprange6.rs");
            t.compile_fail("tests/warnings/iprange.rs");
            t.compile_fail("tests/warnings/ipsubnets4.rs");
            t.compile_fail("tests/warnings/ipsubnets6.rs");
            t.compile_fail("tests/warnings/ipsubnets.rs");
        }

        t.pass("tests/pass/ipnet.rs");
        t.pass("tests/pass/ipnet4.rs");
        t.pass("tests/pass/ipnet6.rs");
        t.pass("tests/pass/iprange.rs");
        t.pass("tests/pass/iprange4.rs");
        t.pass("tests/pass/iprange6.rs");
        t.pass("tests/pass/ipsubnets.rs");
        t.pass("tests/pass/ipsubnets4.rs");
        t.pass("tests/pass/ipsubnets6.rs");
    }

    #[cfg(feature = "macaddr")]
    #[test]
    fn macaddr_types() {
        let t = trybuild::TestCases::new();

        t.compile_fail("tests/fail/mac.rs");
        t.compile_fail("tests/fail/mac6.rs");
        t.compile_fail("tests/fail/mac8.rs");

        t.pass("tests/pass/mac.rs");
        t.pass("tests/pass/mac6.rs");
        t.pass("tests/pass/mac8.rs");
    }
}
