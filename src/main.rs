#![feature(default_field_values)]
#![feature(random)]
#![feature(int_from_ascii)]

use ping::Ping;
use tokio::time::{sleep, Duration};

mod host;
mod config;
mod discord_webhook;
mod webhooks;
mod ping;
mod screen;

#[tokio::main]
async fn main() {
    let mut config = config::Config::default();
    config.load_config_from_file("config.toml");

    let mut pings: Vec<ping::Ping> = vec![];
    for host in config.hosts.iter() { pings.push(ping::Ping{ host: host.clone(), timeout: Duration::from_secs_f64(config.timeout), timeout_count: 0, online: true, screen: None }); }
    
    let mut is_network_available = true;

    loop {
        sleep(Duration::from_secs_f64(config.delay)).await;

        if Ping::check_internet(&config).await.is_ok() {
            is_network_available = true;

            for ping in pings.iter_mut() {
                let ping_result = ping.ping().await;
    
                match ping_result {
                    Ok(_reply) => {
                        let timeout_count: u8 = ping.timeout_count.clone();
                        ping.reset_timeout_count();

                        if timeout_count >= config.retry && !ping.online {
                            ping.update_status(&config).await;
                        }
                    },
                    Err(err) => {
                        match err {
                            surge_ping::SurgeError::Timeout { seq: _ } => {
                                if ping.timeout_count < config.retry+1 {
                                    ping.increment_timeout_count(1);
                                }
                                if ping.timeout_count == config.retry && ping.online {
                                    ping.update_status(&config).await;
                                }
                            }
    
                            surge_ping::SurgeError::IOError(ref e) if e.kind() == std::io::ErrorKind::NetworkUnreachable => {
                                ping.reset_timeout_count();
                            }

                            _ => println!("{:?}", err)
    
                        }
                    }
                }
                
            }

        } else if is_network_available {
            println!("Network Unreachable.");
            for ping in pings.iter_mut() { ping.reset_timeout_count(); }
            is_network_available = false;
        }
    }

}