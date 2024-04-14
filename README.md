# socket2-ext-rs

An socket2 extension contains utilities for handling networking sockets with a maximal amount of configuration possible intended.

[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]
[![Build Status][actions-badge]][actions-url]

[crates-badge]: https://img.shields.io/crates/v/socket2-ext.svg
[crates-url]: https://crates.io/crates/socket2-ext
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/cavivie/socket2-ext-rs/blob/master/LICENSE
[actions-badge]: https://github.com/cavivie/socket2-ext-rs/workflows/CI/badge.svg
[actions-url]: https://github.com/cavivie/socket2-ext-rs/actions?query=workflow%3ACI+branch%3Amain

## Example

Bind a socket to a specific device on Unix/Windows host platform.

```rust
use socket2_ext::binds::{AddressBinding, BindDeviceOption};

fn main() {
    let iface = "your/interface/name";
    match socket2::Socket::new(socket2::Domain::IPV4, socket2::Type::DGRAM, None) {
        Err(e) => println!("create socket error: {:?}", e),
        Ok(socket) => {
            if let Err(e) = socket.bind_to_device(BindDeviceOption::v4(iface)) {
                println!("bind device error: {:?}", e);
            }
        }
    }
}
```

## License

This project is licensed under the [MIT license](LICENSE).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion
in the work by you shall be dual licensed as above, without any additional terms or conditions.
