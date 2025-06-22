use formatify::{Formatify, PlaceholderFormatter};
use std::{collections::HashMap, time::{SystemTime, UNIX_EPOCH}};
use webhook::client::WebhookClient;
use gethostname::gethostname;

use crate::{config::Config, ping::Ping, screen};

#[derive(Default)]
pub struct DiscordWebhook {
    pub client: Option<WebhookClient>,
    pub content: String,
    pub embed_content: String
}

impl DiscordWebhook {
    pub async fn send(&self, config: &Config, ping: &Ping) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        let client = self.client.as_ref();
        client.expect("No Client Found").send(|message| message
            .content(DiscordWebhook::format(&self.content, config, ping).as_str())
            .embed(|embed| embed
            .description(DiscordWebhook::format(&self.embed_content,  config, ping).as_str())))
            .await
    }

    fn format(string: &str, config: &Config, ping: &Ping) -> String {
        let mut string_map: HashMap<&str, String> = HashMap::new();
        string_map.insert("unix_timestamp", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs().to_string());
        string_map.insert("name", ping.host.name.clone());
        string_map.insert("hostname", gethostname().into_string().unwrap());
        string_map.insert("ip", ping.host.ip.to_string());
        if ping.online { string_map.insert("status", "Online".to_string()); }
        else { string_map.insert("status", "Offline".to_string()); }
        let commands = config.execute_on_offline.get(&ping.host.name);
        if commands.is_some() { string_map.insert("commands", commands.unwrap().join("\n")); }
        else { string_map.insert("commands", "NONE".to_string()); }
        let screens: Vec<String> = screen::list().iter().map(|screen| screen.get_full_name()).collect();
        if screens.len() > 0{ string_map.insert("screens", screens.join("\n")); }
        else { string_map.insert("screens", "NONE".to_string()); }
        
        let formatter = Formatify::new();
        return formatter.replace_placeholders(&string_map, string);
    }
}