use std::ops::Index;

use dns_lookup::lookup_host;
use toml::{map::Map, Value};
use webhook::client::WebhookClient;

use crate::host::Host;

#[derive(Default)]
pub struct Config {
    pub timeout: f64,
    pub hosts: Vec<Host>,
    pub discord_webhook: Option<WebhookClient>
}

impl Config {
    pub fn load_config(&mut self, map_config: Map<String, Value>) {
        for key in map_config.keys() {
            match key.as_str() {
                "hosts" => {
                    let hosts = map_config.get(key).unwrap();
                    if hosts.is_array() {
    
                        let hosts_iter = hosts.as_array().unwrap();
                        'host_loop: for host in hosts_iter {
                            if host.is_str() {
                                let ips = lookup_host(host.as_str().unwrap()).unwrap();
                                for ip in ips.as_slice() {
                                    if ip.is_ipv4() {
                                        self.hosts.push(Host{name: host.as_str().unwrap().to_string(), ip: *ip});
                                        continue 'host_loop;
                                    }
                                }
                                self.hosts.push(Host{name: host.as_str().unwrap().to_string(), ip: *ips.index(0)});
                            }
                        
                        }
                    }
                },
                "timeout" => {
                    let timeout = map_config.get(key).unwrap();
                    if timeout.is_integer() {
                        self.timeout = timeout.as_integer().unwrap() as f64;
                    } else if timeout.is_float() {
                        self.timeout = timeout.as_float().unwrap();
                    }
                        
                },
                "discord_webhook" => {
                    let discord_webhook = map_config.get(key).unwrap();
                    if discord_webhook.is_str() {
                        self.discord_webhook = Some(WebhookClient::new(discord_webhook.as_str().unwrap()));
                    }
                },
                _ => {
                    println!("Warning: Property '{key}' Is Not A Vaild Property.");
                }
            }
        }
    }
}