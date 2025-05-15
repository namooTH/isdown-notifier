use dns_lookup::lookup_host;
use toml::{map::Map, Table, Value};
use webhook::client::WebhookClient;
use tokio::time::{sleep, Duration};
use std::{fs::File, io::{BufReader, Read}, net::IpAddr, ops::Index};

#[tokio::main]
async fn main() {
    let config_file = File::open("config.toml").unwrap_or_else(|error| panic!("Configuration File Is Not Presented.\n({:?})", error));
    let mut config_buffer = BufReader::new(config_file);
    let mut config_str = String::new();
    ..config_buffer.read_to_string(&mut config_str);
    
    let map_config = config_str.parse::<Table>().unwrap_or_else(|error| panic!("Error While Parsing The Config File.\n({:?})", error));
    let config = load_config(map_config);

    let hosts = config.hosts.as_slice();
    let discord_webhook = config.discord_webhook;

    loop {

        sleep(Duration::from_secs(5)).await;
        for host in hosts {
            let payload: [u8; 8] = [0; 8];
            let ping_result = surge_ping::ping(host.ip, &payload).await;
            
            match ping_result {
                Ok(reply) => (),
                Err(err) => {
                    let dc_wh = discord_webhook.as_ref();
                    if dc_wh.is_some() {
                        ..dc_wh.unwrap().send(|message| message
                            .username("IsDown Notifier Bot")
                            .embed(|embed| embed
                            .description(format!("## Error\n{:?}", err).as_str()))).await;
                    }
                }
            }
            
        }
    }

}

#[derive(Default)]
struct Config {
    timeout: f64,
    hosts: Vec<Host>,
    discord_webhook: Option<WebhookClient>
}

#[derive(Debug)]
struct Host {
    name: String,
    ip: IpAddr
}

fn load_config(map_config: Map<String, Value>) -> Config{
    let mut config = Config::default();
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
                                    config.hosts.push(Host{name: host.as_str().unwrap().to_string(), ip: *ip});
                                    continue 'host_loop;
                                }
                            }
                            config.hosts.push(Host{name: host.as_str().unwrap().to_string(), ip: *ips.index(0)});
                        }
                    
                    }
                }
            },
            "timeout" => {
                let timeout = map_config.get(key).unwrap();
                if timeout.is_integer() {
                    config.timeout = timeout.as_integer().unwrap() as f64;
                } else if timeout.is_float() {
                    config.timeout = timeout.as_float().unwrap();
                }
                    
            },
            "discord_webhook" => {
                let discord_webhook = map_config.get(key).unwrap();
                if discord_webhook.is_str() {
                    config.discord_webhook = Some(WebhookClient::new(discord_webhook.as_str().unwrap()));
                }
            },
            _ => {
                println!("Warning: Property '{key}' Is Not A Vaild Property.");
            }
        }
    }
    config
}