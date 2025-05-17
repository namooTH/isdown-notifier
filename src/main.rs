use toml::Table;
use tokio::time::{sleep, Duration};
use std::{fs::File, io::{BufReader, Read}};

mod host;
mod config;

#[tokio::main]
async fn main() {
    let config_file = File::open("config.toml").unwrap_or_else(|error| panic!("Configuration File Is Not Presented.\n({:?})", error));
    let mut config_buffer = BufReader::new(config_file);
    let mut config_str = String::new();
    ..config_buffer.read_to_string(&mut config_str);
    
    let map_config = config_str.parse::<Table>().unwrap_or_else(|error| panic!("Error While Parsing The Config File.\n({:?})", error));
    let mut config = config::Config::default();
    config.load_config(map_config);

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