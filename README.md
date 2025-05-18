# const-addrs [![Crates.io](https://img.shields.io/crates/v/const-addrs.svg)](https://crates.io/crates/const-addrs) ![License](https://img.shields.io/crates/l/const-addrs.svg) [![Documentation](https://docs.rs/const-addrs/badge.svg)](https://docs.rs/const-addrs/)

Build Status: [![Build Status](https://github.com/sophacles/const-addrs/actions/workflows/build-and-test.yml/badge.svg)](https://github.com/sophacles/const-addrs/actions/workflows/build-and-test.yml) [![Formatting Status](https://github.com/sophacles/const-addrs/actions/workflows/formatting.yml/badge.svg)](https://github.com/sophacles/const-addrs/actions/workflows/formatting.yml) [![Linting Status](https://github.com/sophacles/const-addrs/actions/workflows/linting.yml/badge.svg)](https://github.com/sophacles/const-addrs/actions/workflows/linting.yml)

A set of macros for creating networking types from a string literal.

Each of the macros will parse using the `FromStr` implementation for 
the appropriate type. The generated code will use a `const` constructor 
for the type, if one exists.

```rust
use std::net::Ipv4Addr;
use const_addrs::ip4;

let a = ip4!("192.168.1.1");
let b = Ipv4Addr::new(192,168,1,1);
assert_eq!(a, b);
```

And turns invalid strings into compile-time errors:
```
error: invalid IPv4 address syntax
  --> bad.rs:10:18
   |
10 |     let a = ip4!("192.1681.1");
   |                  ^^^^^^^^^^^^
```

There are macros for:

| Type                     | macro         |
| ------------------------ | ----------    |
| `std::net::IpAddr`       | `ip!`         |
| `std::net::Ipv4Addr`     | `ip4!`        |
| `std::net::Ipv6Addr`     | `ip6!`        |
| `std::net::SocketAddr`   | `sock!`       |
| `std::net::SocketAddrV4` | `sock4!`      |
| `std::net::SocketAddrV6` | `sock6!`      |
| `ipnet::IpNet`           | `ipnet!`      |
| `ipnet::IpAddrRange`     | `iprange!`    |
| `ipnet::Ipv4AddrRange`   | `iprange4!`   |
| `ipnet::Ipv6AddrRange`   | `iprange6!`   |
| `ipnet::IpSubnets`       | `ipsubnets!`  |
| `ipnet::Ipv4Subnets`     | `ipsubnets4!` |
| `ipnet::Ipv6Subnets`     | `ipsubnets6!` |
| `ipnetwork::IpNetwork`   | `net!`        |
| `ipnetwork::Ipv4Network` | `net4!`       |
| `ipnetwork::Ipv6Network` | `net6!`       |
| `macaddr::MacAddr`       | `mac!`        |
| `macaddr::MacAddr6`      | `mac6!`       |
| `macaddr::MacAddr8`      | `mac8!`       |


*Note*: using `ipnet::*` types requires you to have the 
[ipnet crate](https://crates.io/crates/ipnet) in your depdencies. These
types can be enabled with the `ipnet` feature.

*Note*: using `ipnetwork::*` types requires you to have the 
[ipnetwork crate](https://crates.io/crates/ipnetwork) in your depdencies. These
types can be enabled with the `ipnetwork` feature.

*Note*: using `macaddr::*` requires the 
[macaddr crate](https://crates.io/crates/macaddr) in your depdencies. These
types can be enabled with the `mac` feature.

When possible the expanded macro uses `const` constructors, allowing for simple
string representations of network types without the cost of runtime parsing.
