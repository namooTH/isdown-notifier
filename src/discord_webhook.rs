use formatify::{Formatify, PlaceholderFormatter};
use std::{collections::HashMap, time::{SystemTime, UNIX_EPOCH}};
use webhook::client::WebhookClient;
use gethostname::gethostname;

use crate::ping::Ping;

#[derive(Default)]
pub struct DiscordWebhook {
    pub client: Option<WebhookClient>,
    pub content: String,
    pub embed_content: String
}

impl DiscordWebhook {
    pub async fn send(&mut self, retry: u8, ping: &Ping) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        let client = self.client.as_ref();
        client.expect("No Client Found").send(|message| message
            .content(DiscordWebhook::format(&self.content, retry, ping).as_str())
            .embed(|embed| embed
            .description(DiscordWebhook::format(&self.embed_content, retry, ping).as_str())))
            .await
    }

    fn format(string: &str, retry: u8, ping: &Ping) -> String {
        let mut string_map: HashMap<&str, String> = HashMap::new();
        string_map.insert("unix_timestamp", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs().to_string());
        string_map.insert("name", ping.host.name.clone());
        string_map.insert("hostname", gethostname().into_string().unwrap());
        string_map.insert("ip", ping.host.ip.to_string());
        if ping.timeout_count > retry { string_map.insert("status", "Online".to_string()); }
        else { string_map.insert("status", "Offline".to_string()); }

        let formatter = Formatify::new();
        return formatter.replace_placeholders(&string_map, string);
    }
}