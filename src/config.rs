use std::{fs::File, io::{BufReader, Read}, ops::Index};

use dns_lookup::lookup_host;
use toml::{map::Map, Table, Value};
use webhook::client::WebhookClient;

use crate::host::Host;
use crate::discord_webhook::DiscordWebhook;

#[derive(Default)]
pub struct Config {
    pub timeout: f64,
    pub hosts: Vec<Host>,
    pub discord_webhook: Option<DiscordWebhook>
}

impl Config {
    pub fn load_config_from_file(&mut self, path: &str) {
        let config_file = File::open(path).unwrap_or_else(|error| panic!("Configuration Is Not A Vaild Path.\n({:?})", error));
        let mut config_buffer = BufReader::new(config_file);
        let mut config_str = String::new();
        ..config_buffer.read_to_string(&mut config_str);
        
        self.load_config_from_map(config_str.parse::<Table>()
                                 .unwrap_or_else(|error| panic!("Error While Parsing The Config File.\n({:?})", error)));
    }

    pub fn load_config_from_map(&mut self, map_config: Map<String, Value>) {
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
                                        self.hosts.push(Host{name: host.as_str().unwrap().to_string(), ip: *ip, timeout_count: Default::default()});
                                        continue 'host_loop;
                                    }
                                }
                                self.hosts.push(Host{name: host.as_str().unwrap().to_string(), ip: *ips.index(0), timeout_count: Default::default()});
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

                "discord" => {
                    self.discord_webhook = Some(DiscordWebhook{
                        client: Default::default(),
                        content: "At <t:%(unix_timestamp)>".to_string(),
                        embed_content: "## Error\n%(error_message)".to_string()});
                        
                    let discord = map_config.get(key).unwrap().as_table().unwrap();
                    for value in discord {
                        match value.0.as_str() {
                            "webhook" => {
                                if value.1.is_str() {
                                    let webhook = self.discord_webhook.as_mut();
                                    webhook.unwrap().client = Some(WebhookClient::new(value.1.as_str().unwrap()))
                                }
                                else { println!("Warning: Property '{:}' in '{:}' Contains Invaild Datatype. (Expected: String)", value.0, key.as_str()); }
                            }
                            "content" => {
                                if value.1.is_str() {
                                    let webhook = self.discord_webhook.as_mut();
                                    webhook.unwrap().content = value.1.as_str().unwrap().to_string()
                                }
                                else { println!("Warning: Property '{:}' in '{:}' Contains Invaild Datatype. (Expected: String)", value.0, key.as_str()); }
                            }
                            "embed_content" => {
                                if value.1.is_str() {
                                    let webhook = self.discord_webhook.as_mut();
                                    webhook.unwrap().embed_content = value.1.as_str().unwrap().to_string()                              
                                }
                                else { println!("Warning: Property '{:}' in '{:}' Contains Invaild Datatype. (Expected: String)", value.0, key.as_str()); }
                            }
                            _ => ()
                        }
                    }
                },

                _ => {
                    println!("Warning: Property '{key}' Is Not A Vaild Property.");
                }
            }
        }
    }
}