use crate::config::Config;
use crate::error::FetchInfoError;
use crate::filtered_values;
use crate::system::{InfoField, InfoGroup, InfoResult, InfoValue};
use crate::util::{PROJECT_VERSION, ToOptionString};
use socket2::{Domain, Protocol, Socket, Type};
use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, ToSocketAddrs};
use std::time::Duration;

const IPV4: IpAddr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
const IPV6: IpAddr = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0));
const TIMEOUT: Duration = Duration::from_secs(5);

pub fn get_public_ip(
    _languages_func: fn(&str) -> &str,
    fields: &[InfoField],
    config: &Config,
) -> Result<InfoResult, FetchInfoError> {
    let ipv4 =
        if fields.contains(&InfoField::PublicIpv4) || fields.contains(&InfoField::PublicIpAny) {
            http_get_request(
                config.parameters.public_ip.ipv4_domain,
                config.parameters.public_ip.ipv4_port,
                config.parameters.public_ip.ipv4_path,
                IPV4,
            )
        } else {
            None
        };
    let ipv6 = if fields.contains(&InfoField::PublicIpv6)
        || (fields.contains(&InfoField::PublicIpAny) && ipv4.is_none())
    {
        http_get_request(
            config.parameters.public_ip.ipv6_domain,
            config.parameters.public_ip.ipv6_port,
            config.parameters.public_ip.ipv6_path,
            IPV6,
        )
    } else {
        None
    };

    Ok(InfoResult::Single(InfoGroup {
        values: filtered_values!(
            fields,
            [
                (InfoField::PublicIpAny, ipv4.as_deref().or(ipv6.as_deref())),
                (InfoField::PublicIpv4, ipv4),
                (InfoField::PublicIpv6, ipv6),
            ]
        ),
    }))
}

fn http_get_request(domain: &str, port: u16, path: &str, ip: IpAddr) -> Option<String> {
    let local_addr = SocketAddr::new(ip, 0);
    let addresses = (domain, port).to_socket_addrs().ok()?;
    for address in addresses {
        let socket = match Socket::new(
            if address.is_ipv4() {
                Domain::IPV4
            } else {
                Domain::IPV6
            },
            Type::STREAM,
            Some(Protocol::TCP),
        ) {
            Ok(socket) => socket,
            Err(_) => continue,
        };

        if socket.bind(&local_addr.into()).is_err() || socket.connect(&address.into()).is_err() {
            continue;
        }

        let mut stream: std::net::TcpStream = socket.into();
        stream.set_read_timeout(Some(TIMEOUT)).ok();
        stream.set_write_timeout(Some(TIMEOUT)).ok();

        let request = format!(
            "GET {path} HTTP/1.1\r\nHost: {domain}\r\nUser-Agent: AFetch/{PROJECT_VERSION}\r\nAccept: */*\r\nConnection: close\r\n\r\n"
        );
        stream.write_all(request.as_bytes()).ok()?;

        let mut buffer = Vec::new();
        stream.read_to_end(&mut buffer).ok()?;

        let response = String::from_utf8_lossy(&buffer);
        if let Some(body_start) = response.find("\r\n\r\n") {
            return Some(response[(body_start + 4)..].trim().to_owned());
        }
    }

    None
}
