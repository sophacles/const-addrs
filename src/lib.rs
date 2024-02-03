//! A set of macros for creating networking types from a string literal.
//!
//! ```rust
//! use std::net::Ipv4Addr;
//! use net_macros::ip4;
//!
//! # fn main() {
//! let a = ip4!("192.168.1.1");
//! let b = Ipv4Addr::new(192,168,1,1);
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
//! the `const` constructor(s) for stdlib and macaddr types, adding no runtime overhead.
//!
//! For example:
//! ```rust
//! # use std::net::Ipv4Addr;
//! # use net_macros::ip;
//!
//! # fn main() {
//! let val = ip!("192.168.1.1");
//! # }
//! ```
//! expands to:
//! ```rust
//! let val = ::std::net::SocketAddr::V4(::std::net::SocketAddrV4::new(
//!     ::std::net::Ipv4Addr::new(192u8, 168u8, 1u8, 1u8),
//!     500u16,
//! ));
//! ```
//!
//! [`ipnetwork`] types do not provide a const constructor and return a
//! [`Result`]. These are unwrapped in the generated code, since they are
//! validated when the macro runs.
//!
//! For example:
//! ```rust
//! # use std::net::Ipv4Addr;
//! # use net_macros::net;
//!
//! # fn main() {
//! let val = net!("192.168.1.1/24");
//! # }
//! ```
//! expands to:
//! ```rust
//! let val = ipnetwork::IpNetwork::V4(
//!     ipnetwork::Ipv4Network::new(::std::net::Ipv4Addr::new(192u8, 168u8, 1u8, 1u8), 24u8)
//!         .unwrap(),
//! );
//! ```
#![cfg_attr(feature = "document-features", doc = "\n## Features")]
#![cfg_attr(feature = "document-features", doc = document_features::document_features!())]

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
    ($macro_name:ident: $ty:ty =? $dummy:expr; $fn_name:ident => $body:expr; $tyname:expr) => {
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
            match <$ty>::from_str(inval.value().as_str()) {
                // if good, turn it into the right token stream
                Ok(v) => $fn_name(Some(v)).into(),
                Err(e) => {
                    // otherwise emit the parse error with the parser's error message
                    // as well as make the compile error show the invalid string
                    emit_error!(inval, e);
                    // return dummy value of the right type to squash subsequent errors
                    $fn_name(None).into()
                }
            }
        }

        fn $fn_name(input: Option<$ty>) -> proc_macro2::TokenStream {
            input.or(Some($dummy)).map($body).unwrap()
        }
    };
    ($macro_name:ident: $ty:ty =? $dummy:expr; $fn_name:ident => $body:expr) => {
        make_macro!($macro_name: $ty =? $dummy; $fn_name => $body; stringify!($ty));
    }
}

// IP types

make_macro! {
    ip: IpAddr =? IpAddr::V4(Ipv4Addr::UNSPECIFIED);
    ipaddr_tokens => |input| match input {
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
                let inner = sock4_tokens(Some(sock));
                quote! { ::std::net::SocketAddr::V4(#inner) }
            }
            SocketAddr::V6(sock) => {
                let inner = sock6_tokens(Some(sock));
                quote! { ::std::net::SocketAddr::V6(#inner) }
            }
        }
    }
}

make_macro! {
    sock4: SocketAddrV4 =? SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0);
    sock4_tokens => |input| {
        let ip = ip4_tokens(Some(*input.ip()));
        let port = input.port();
        quote! { ::std::net::SocketAddrV4::new(#ip, #port) }
    }
}

make_macro! {
    sock6: SocketAddrV6 =? SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, 0, 0, 0);
    sock6_tokens => |addr| {
        let ip = ip6_tokens(Some(*addr.ip()));
        let port = addr.port();
        quote! { ::std::net::SocketAddrV6::new(#ip, #port, 0, 0) }
    }
}

// IpNetwork types

cfg_if::cfg_if! {
    if #[cfg(feature = "ipnet")] {
        use ipnetwork::{IpNetwork, Ipv4Network, Ipv6Network};

        make_macro!{
            net: IpNetwork =? IpNetwork::V4(Ipv4Network::new(Ipv4Addr::UNSPECIFIED, 0).unwrap());
            net_tokens => |net| match net {
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

        make_macro!{
            net4: Ipv4Network =? Ipv4Network::new(Ipv4Addr::UNSPECIFIED, 0).unwrap();
            net4_tokens => |net| {
                let ip = ip4_tokens(Some(net.ip()));
                let prefix = net.prefix();
                quote! { ipnetwork::Ipv4Network::new(#ip, #prefix).unwrap() }
            }
        }


        make_macro!{
            net6: Ipv6Network =? Ipv6Network::new(Ipv6Addr::UNSPECIFIED, 0).unwrap();
            net6_tokens => |net| {
                let ip = ip6_tokens(Some(net.ip()));
                let prefix = net.prefix();
                quote! { ipnetwork::Ipv6Network::new(#ip, #prefix).unwrap() }
            }
        }
    }
}

// MacAddr types

cfg_if::cfg_if! {
    if #[cfg(feature = "mac")] {
        use macaddr::{MacAddr, MacAddr6, MacAddr8};

        make_macro!{
            mac: MacAddr =? MacAddr::V6(MacAddr6::from([0x00; 6]));
            mac_tokens => |addr| match addr {
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

        make_macro! {
            mac6: MacAddr6 =? MacAddr6::from([0x00; 6]);
            mac6_tokens => |addr| {
                let bytes = addr.into_array();
                quote! { macaddr::MacAddr6::new(#(#bytes),*) }
            }
        }

        make_macro!{
            mac8: MacAddr8 =? MacAddr8::from([0x00; 8]);
            mac8_tokens => |addr| {
                let bytes = addr.into_array();
                quote! { macaddr::MacAddr8::new(#(#bytes),*) }
            }
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
