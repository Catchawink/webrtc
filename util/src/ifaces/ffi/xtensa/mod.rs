use esp_idf_svc::netif::{EspNetif, NetifStack};
use std::io::Error;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

use crate::ifaces::{Interface, Kind, NextHop};

fn get_wifi_interface() -> Option<Interface> {

    let netif = EspNetif::new(NetifStack::Sta).unwrap();
    let ip_info = netif.get_ip_info().unwrap();

    //let tcpip_adapter_if_mut = tcpip_adapter_get_ip_info(esp::TCPIP_ADAPTER_IF_A(0));
    //if tcpip_adapter_if_mut.is_null() {
    //    return None;
    //}
    //let tcpip_adapter_if = unsafe { &*tcpip_adapter_if_mut };

    let ip = IpAddr::from(ip_info.ip);
    let mask = Some(SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::try_from(ip_info.subnet.mask).unwrap(), 0)));
    let gw = IpAddr::from(ip_info.subnet.gateway);

    let addr = match ip {
        IpAddr::V4(addr) => SocketAddr::V4(SocketAddrV4::new(addr, 0)),
        IpAddr::V6(addr) => SocketAddr::V6(SocketAddrV6::new(addr, 0, 0, 0)),
    };

    //let mask = match netmask {
    //    IpAddr::V4(mask) => Some(SocketAddr::V4(SocketAddrV4::new(mask, 0))),
    //    IpAddr::V6(mask) => Some(SocketAddr::V6(SocketAddrV6::new(mask, 0, 0, 0))),
    //    _ => None,
    //};

    let hop = match gw {
        IpAddr::V4(gw) => Some(NextHop::Destination(SocketAddr::V4(SocketAddrV4::new(gw, 0)))),
        IpAddr::V6(gw) => Some(NextHop::Destination(SocketAddr::V6(SocketAddrV6::new(gw, 0, 0, 0)))),
        _ => None,
    };

    let kind = match ip {
        IpAddr::V4(_) => Kind::Ipv4,
        IpAddr::V6(_) => Kind::Ipv6,
    };

    Some(Interface {
        name: "WiFi".to_string(),
        kind,
        addr: Some(addr),
        mask,
        hop,
    })
}

/// Query the local system for all interface addresses.
pub fn ifaces() -> Result<Vec<Interface>, Error> {
    let mut ret = Vec::new();

    // Get the WiFi interface information
    if let Some(wifi_iface) = get_wifi_interface() {
        ret.push(wifi_iface);
    }

    // Add code to retrieve information about other interfaces (e.g., Ethernet, Bluetooth) if needed

    Ok(ret)
}