use std::borrow::Cow;

use socket2::{Domain, Socket};

/// Options to bind device.
pub struct BindDeviceOption<'a> {
    /// Device name.
    name: Cow<'a, str>,
    /// Device socket domain.
    domain: Domain,
}

impl<'a> BindDeviceOption<'a> {
    pub fn v4(name: &'a str) -> Self {
        Self {
            name: Cow::Borrowed(name),
            domain: Domain::IPV4,
        }
    }

    pub fn v6(name: &'a str) -> Self {
        Self {
            name: Cow::Borrowed(name),
            domain: Domain::IPV6,
        }
    }
}

pub trait AddressBinding {
    /// Bind socket to device with name:
    ///
    /// Use name to find the adapter address and bind it on Windows.
    ///
    /// Call system builtin socket bind device method on non-Windows.
    fn bind_to_device(&self, opt: BindDeviceOption) -> std::io::Result<()>;
}

impl AddressBinding for Socket {
    #[cfg(not(target_os = "windows"))]
    #[allow(unreachable_code)]
    fn bind_to_device(&self, opt: BindDeviceOption) -> std::io::Result<()> {
        #[cfg(any(
            target_os = "ios",
            target_os = "macos",
            target_os = "tvos",
            target_os = "watchos"
        ))]
        {
            let ifindex = std::num::NonZeroU32::new(unsafe {
                libc::if_nametoindex(opt.name.as_ptr() as *const _)
            });
            match opt.domain {
                Domain::IPV4 => self.bind_device_by_index_v4(ifindex)?,
                Domain::IPV6 => self.bind_device_by_index_v6(ifindex)?,
                _ => {}
            }
            return Ok(());
        }

        #[cfg(any(target_os = "android", target_os = "fuchsia", target_os = "linux"))]
        {
            self.bind_device(Some(opt.name.as_bytes()))?;
            return Ok(());
        }

        #[cfg(feature = "log")]
        tracing::warn!(
            "bind device name [{}] on unsupported OS [{}]",
            opt.name,
            std::env::consts::OS,
        );
        Ok(())
    }

    #[cfg(target_os = "windows")]
    fn bind_to_device(&self, opt: BindDeviceOption) -> std::io::Result<()> {
        let ip = crate::utils::win_ifname_to_addr(&opt.name)?;

        #[cfg(feature = "log")]
        tracing::trace!(
            "bind adapter by name [{}] and ip [{}] on Windows",
            opt.name,
            ip,
        );

        let addr = std::net::SocketAddr::new(ip, 0);
        self.bind(&socket2::SockAddr::from(addr))?;
        Ok(())
    }
}

#[test]
fn test_bind_iface() {
    use crate::binds::{AddressBinding, BindDeviceOption};
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
