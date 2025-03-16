use std::net::Ipv6Addr;

use log::info;
use pnet::ipnetwork::IpNetwork;

pub fn get_ipv6() -> Result<String, String> {
    let ret: Vec<Ipv6Addr> = pnet::datalink::interfaces()
        .iter()
        .map(|x| {
            info!("checking device: {}", x.name);
            x
        })
        .map(|interface| &interface.ips)
        .flatten()
        .filter_map(|x| match x {
            IpNetwork::V6(ip) => Some(ip),
            _ => None,
        })
        .map(|x| {
            info!("get ipv6 network: {}/{}", x.ip(), x.prefix());
            x
        })
        .filter_map(|x| {
            let ip = x.ip().to_string().parse::<Ipv6Addr>().ok()?;
            if ip.is_loopback() {
                return None;
            }
            if ip.is_unicast_link_local() {
                return None;
            }
            Some(ip)
        })
        .collect();

    if ret.len() == 0 {
        return Err("no IPv6 public addr found".to_string());
    }
    let ret = ret[0].to_string();
    info!("found public ipv6 addr: {:?}", ret);
    Ok(ret)
}
