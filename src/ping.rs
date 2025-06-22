use std::{net::IpAddr, random::random, str::FromStr, time::Duration};
use surge_ping::{Client, Config, IcmpPacket, PingIdentifier, PingSequence, SurgeError, ICMP};

use crate::{config, host::Host, screen::{self, Screen}, webhooks};

pub struct Ping {
    pub host: Host,
    pub timeout: Duration,
    pub timeout_count: u8,
    pub online: bool,
    pub screen: Option<Screen>
}

impl Ping {
    pub async fn ping(&self) -> Result<(IcmpPacket, Duration), SurgeError> {
        let config = match self.host.ip {
            IpAddr::V4(_) => Config::default(),
            IpAddr::V6(_) => Config::builder().kind(ICMP::V6).build(),
        };
        let client = Client::new(&config).unwrap();
        
        let mut pinger = client.pinger(self.host.ip, PingIdentifier(random())).await;
        pinger.timeout(self.timeout);

        pinger.ping(PingSequence(0), &[0; 1]).await
    }

    pub async fn check_internet(config: &config::Config) -> Result<(), ()> {
        let mut pings: [Ping; 4] = [
            Ping{ host: Host { name: "Google DNS".to_string(), ip: IpAddr::from_str("8.8.8.8").unwrap() }, timeout: Duration::from_secs_f64(config.timeout), timeout_count: 0, online: true, screen: None },
            Ping{ host: Host { name: "Google DNS Backup".to_string(), ip: IpAddr::from_str("8.8.4.4").unwrap() }, timeout: Duration::from_secs_f64(config.timeout), timeout_count: 0, online: true, screen: None },
            Ping{ host: Host { name: "Cloudflare DNS".to_string(), ip: IpAddr::from_str("1.1.1.1").unwrap() }, timeout: Duration::from_secs_f64(config.timeout), timeout_count: 0, online: true, screen: None },
            Ping{ host: Host { name: "Cloudflare DNS Backup".to_string(), ip: IpAddr::from_str("1.0.0.1").unwrap() }, timeout: Duration::from_secs_f64(config.timeout), timeout_count: 0, online: true, screen: None }
        ];

        for ping in pings.iter_mut() {
            let ping_result = ping.ping().await;
            if ping_result.is_ok() { return Ok(()) }
        }
    
        return Err(())
    }

    pub async fn update_status(&mut self, config: &config::Config) {
        let commands = config.execute_on_offline.get(&self.host.name);
        
        if self.timeout_count < config.retry {
            self.online = true;
            if self.screen.is_some() {
                screen::kill(&self.screen.as_ref().unwrap().get_full_name());
                self.screen = None;
            }
        }
        else {
            self.online = false;
            if commands.is_some() { self.screen = screen::create(&self.host.name, commands.unwrap()); }
        }

        webhooks::send_webhooks(config, &self).await;
    }

    pub fn increment_timeout_count(&mut self, amount: u8) {
        self.timeout_count += amount;
    }

    pub fn reset_timeout_count(&mut self) {
        self.timeout_count = 0;
    }
}