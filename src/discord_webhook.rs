use formatify::{Formatify, PlaceholderFormatter};
use std::{collections::HashMap, time::{SystemTime, UNIX_EPOCH}};
use webhook::client::WebhookClient;

#[derive(Default)]
pub struct DiscordWebhook {
    pub client: Option<WebhookClient>,
    pub content: String,
    pub embed_content: String
}

impl DiscordWebhook {
    pub async fn send(&mut self) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        let client = self.client.as_ref();
        client.expect("No Client Found").send(|message| message
            .content(DiscordWebhook::format(&self.content).as_str())
            .embed(|embed| embed
            .description(DiscordWebhook::format(&self.embed_content).as_str())))
            .await
    }

    fn format(string: &str) -> String {
        let mut string_map: HashMap<&str, String> = HashMap::new();
        string_map.insert("unix_timestamp", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs().to_string());
        
        let formatter = Formatify::new();
        return formatter.replace_placeholders(&string_map, string);
    }
}