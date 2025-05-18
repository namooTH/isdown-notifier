#![feature(default_field_values)]
#![feature(random)]
use tokio::time::{sleep, Duration};

mod host;
mod config;
mod discord_webhook;
mod ping;

#[tokio::main]
async fn main() {
    let mut config = config::Config::default();
    config.load_config_from_file("config.toml");
    
    let mut pings: Vec<ping::Ping> = vec![];
    for host in config.hosts { pings.push(ping::Ping{ host: host, timeout: Duration::from_secs_f64(config.timeout), timeout_count: Default::default() }); }
    
    loop {

        sleep(Duration::from_secs(5)).await;
        for ping in pings.iter_mut() {
            let ping_result = ping.ping().await;

            match ping_result {
                Ok(_reply) => {
                    ping.reset_timeout_count();
                },
                Err(err) => {
                    match err {
                        
                        surge_ping::SurgeError::Timeout { seq: _ } => {
                            if ping.timeout_count < config.retry+1 {
                                ping.increment_timeout_count(1);
                            }
                            if ping.timeout_count == config.retry {
                                let discord_webhook = config.discord_webhook.as_mut();
                                if discord_webhook.is_some() {
                                    ..discord_webhook.unwrap().send(config.retry, &ping).await;
                                }
                            }

                        }

                        surge_ping::SurgeError::IOError(ref e) if e.kind() == std::io::ErrorKind::NetworkUnreachable => {
                            ping.reset_timeout_count();
                            println!("Network Unreachable.")
                        }
                        _ => println!("{:?}", err)

                    }

                }
            }
            
        }
    }

}