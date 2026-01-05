use crate::config::Config;
use crate::error::FetchInfoError;
use crate::filtered_values;
use crate::system::{InfoField, InfoGroup, InfoResult, InfoValue};
use crate::util::{ToOptionString, convert_to_readable_unity};
use std::net::IpAddr;
use sysinfo::Networks;

pub fn get_networks(
    _languages_func: fn(&str) -> &str,
    fields: &[InfoField],
    config: &Config,
) -> Result<InfoResult, FetchInfoError> {
    let mut networks_info: Vec<InfoGroup> = Vec::new();

    for (name, network) in Networks::new_with_refreshed_list().list() {
        if config
            .parameters
            .networks
            .exclude
            .iter()
            .any(|ignore| name.starts_with(ignore))
            || config
                .parameters
                .networks
                .include
                .as_ref()
                .is_some_and(|include| !include.iter().any(|include| name.starts_with(include)))
        {
            continue;
        }

        if config.parameters.networks.ignore_loopback
            && network.ip_networks().iter().any(|ip| ip.addr.is_loopback())
        {
            continue;
        }

        let first_ip = match network.ip_networks().first() {
            Some(ip) => Some(ip.to_string()),
            None if config.parameters.networks.assigned_only => continue,
            None => None,
        };
        let first_ipv4 = network.ip_networks().iter().find_map(|ip| match ip.addr {
            IpAddr::V4(v4) => Some((v4.is_private(), v4.to_string())),
            _ => None,
        });
        let first_ipv6 = network.ip_networks().iter().find(|ip| ip.addr.is_ipv6());

        if config.parameters.networks.private_only && matches!(first_ipv4, Some((false, _))) {
            continue;
        }

        let first_ipv4: Option<String> = first_ipv4.map(|tuple| tuple.1);
        networks_info.push(InfoGroup {
            values: filtered_values!(
                fields,
                [
                    (InfoField::NetworkName, name.as_str()),
                    (InfoField::NetworkFirstIp, first_ipv4.as_deref()),
                    (
                        InfoField::NetworkPreferFirstIpv4,
                        first_ipv4.as_deref().or(first_ip.as_deref())
                    ),
                    (
                        InfoField::NetworkPreferFirstIpv6,
                        first_ipv6.map(|ip| ip.to_string()).or(first_ip)
                    ),
                    (
                        InfoField::NetworkAllIp,
                        if network.ip_networks().is_empty() {
                            None
                        } else {
                            Some(
                                network
                                    .ip_networks()
                                    .iter()
                                    .map(|ip| ip.to_string())
                                    .collect::<Vec<_>>()
                                    .join(" "),
                            )
                        }
                    ),
                    (
                        InfoField::NetworkMacAddress,
                        network.mac_address().to_string()
                    ),
                    (
                        InfoField::NetworkMaximumTransferUnit,
                        network.mtu().to_string()
                    ),
                    (
                        InfoField::NetworkErrorsOnReceived,
                        network.total_errors_on_received().to_string()
                    ),
                    (
                        InfoField::NetworkErrorsOnTransmitted,
                        network.total_errors_on_transmitted().to_string()
                    ),
                    (
                        InfoField::NetworkPacketsReceived,
                        network.total_packets_received().to_string()
                    ),
                    (
                        InfoField::NetworkPacketsTransmitted,
                        network.total_packets_transmitted().to_string()
                    ),
                    (
                        InfoField::NetworkReceived,
                        convert_to_readable_unity(network.total_received() as f64)
                    ),
                    (
                        InfoField::NetworkTransmitted,
                        convert_to_readable_unity(network.total_transmitted() as f64)
                    )
                ]
            ),
        });
    }

    Ok(InfoResult::Several(networks_info))
}
