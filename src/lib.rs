//! This crate provides some extensions and utils for socket2 operations.
//!
//! ## Example
//!
//! Cargo.toml:
//!
//! ```toml
//! [dependencies]
//! socket-ext = "0.1.0"
//! socket2 = "0.5.6"
//! ```
//!
//! main.rs:
//! ```
//! use socket2_ext::{AddressBinding, BindDeviceOption};
//!
//! let iface = "your/interface/name";
//! match socket2::Socket::new(socket2::Domain::IPV4, socket2::Type::DGRAM, None) {
//!     Err(e) => println!("create socket error: {:?}", e),
//!     Ok(socket) => {
//!         if let Err(e) = socket.bind_to_device(BindDeviceOption::v4(iface)) {
//!             println!("bind device error: {:?}", e);
//!         }
//!     }
//! }
//! ```
pub mod binds;
pub mod utils;

pub use binds::{AddressBinding, BindDeviceOption};

pub use socket2;
