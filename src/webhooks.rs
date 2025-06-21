use crate::{config, ping::Ping};

pub async fn send_webhooks(config: &config::Config, ping: &Ping) {
    match &config.discord_webhook {
        Some(dc_wh) => { ..dc_wh.send(&config, &ping).await; }
        None => ()
    }
}