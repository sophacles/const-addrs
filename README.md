# net-macros

A set of macros for creating networking types from a string literal.

```rust
use std::net::Ipv4Addr;
use net_macros::ip4;

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

| Type                     | macro      |
| ------------------------ | ---------- |
| `std::net::IpAddr`       | `ip!`      |
| `std::net::Ipv4Addr`     | `ip4!`     |
| `std::net::Ipv6Addr`     | `ip6!`     |
| `std::net::SocketAddr`   | `sock!`    |
| `std::net::SocketAddrV4` | `sock4!`   |
| `std::net::SocketAddrV6` | `sock6!`   |
| `ipnetwork::IpNetwork`   | `net!`     |
| `ipnetwork::Ipv4Network` | `net4!`    |
| `ipnetwork::Ipv6Network` | `net6!`    |
| `macaddr::MacAddr`       | `mac!`     |
| `macaddr::MacAddr6`      | `mac6!`    |
| `macaddr::MacAddr8`      | `mac8!`    |

Each of the macros will parse using the `FromStr` implementation for 
the type, refer to the type documentaton for format details.

Note: using `ipnetwork::*` types requires you to have the 
[ipnetwork crate](https://crates.io/crates/ipnetwork) in your depdencies. These
types can be turned off by disabling the `ipnet` feature (which is on by
default)

Note: using `macaddr::*` requires the 
[macaddr crate](https://crates.io/crates/macaddr) in your depdencies. These
types can be turned off by disabling the `mac` feature (which is on by default)

In all cases the expanded macro uses `const` constructors, allowing for simple
string representations of network types without the cost of runtime parsing.
