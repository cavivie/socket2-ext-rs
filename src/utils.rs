/// Make behavior look more like unix SO_BINDTODEVICE.
/// If create many sockets very frequently and need to bind,
/// it is recommanded to bind using pre-obtained IP directly.
#[cfg(target_os = "windows")]
pub fn win_ifname_to_addr(name: &str) -> std::io::Result<std::net::IpAddr> {
    macro_rules! ioerr {
        ($e:expr) => {
            std::io::Error::new(std::io::ErrorKind::NotFound, $e)
        };
    }

    let adapters = ipconfig::get_adapters().map_err(|e| ioerr!(e))?;
    adapters
        .iter()
        .find(|a| a.friendly_name() == name)
        .ok_or_else(|| ioerr!(format!("No such adapter name [{}]", name)))?
        .ip_addresses()
        .first()
        .ok_or_else(|| ioerr!(format!("No such ip address for name [{}]", name)))
        .copied()
}
