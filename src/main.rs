#![feature(default_field_values)]
use tokio::time::{sleep, Duration};

mod host;
mod config;
mod discord_webhook;

#[tokio::main]
async fn main() {
    let mut config = config::Config::default();
    config.load_config_from_file("config.toml");
    let hosts = config.hosts.as_mut_slice();

    loop {

        sleep(Duration::from_secs(5)).await;
        for host in hosts.iter_mut() {
            let payload: [u8; 8] = [0; 8];
            let ping_result = surge_ping::ping(host.ip, &payload).await;

            match ping_result {
                Ok(reply) => {
                    host.reset_timeout_count();
                },
                Err(err) => {
                    match err {
                        
                        surge_ping::SurgeError::Timeout { seq } => {
                            if host.timeout_count < 6 {
                                host.increment_timeout_count(1);
                            }
                            if host.timeout_count == 5 {
                                let discord_webhook = config.discord_webhook.as_mut();
                                if discord_webhook.is_some() {
                                    ..discord_webhook.unwrap().send().await;
                                }
                            }
   
                        }

                        surge_ping::SurgeError::IOError(ref e) if e.kind() == std::io::ErrorKind::NetworkUnreachable => {
                            host.reset_timeout_count();
                            println!("Network Unreachable.")
                        }
                        _ => println!("{:?}", err)

                    }

                }
            }
            
        }
    }

}