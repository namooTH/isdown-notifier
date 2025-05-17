use tokio::time::{sleep, Duration};

mod host;
mod config;

#[tokio::main]
async fn main() {
    let mut config = config::Config::default();
    config.load_config_from_file("config.toml");

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