use std::{net::IpAddr, random::random, str::FromStr, time::Duration};
use surge_ping::{Client, Config, IcmpPacket, PingIdentifier, PingSequence, SurgeError, ICMP};

use crate::host::Host;
use crate::config;

pub struct Ping {
    pub host: Host,
    pub timeout: Duration,
    pub timeout_count: u8 = 0
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
            Ping{ host: Host { name: "Google DNS".to_string(), ip: IpAddr::from_str("8.8.8.8").unwrap() }, timeout: Duration::from_secs_f64(config.timeout), timeout_count: Default::default() },
            Ping{ host: Host { name: "Google DNS Backup".to_string(), ip: IpAddr::from_str("8.8.4.4").unwrap() }, timeout: Duration::from_secs_f64(config.timeout), timeout_count: Default::default() },
            Ping{ host: Host { name: "Cloudflare DNS".to_string(), ip: IpAddr::from_str("1.1.1.1").unwrap() }, timeout: Duration::from_secs_f64(config.timeout), timeout_count: Default::default() },
            Ping{ host: Host { name: "Cloudflare DNS Backup".to_string(), ip: IpAddr::from_str("1.0.0.1").unwrap() }, timeout: Duration::from_secs_f64(config.timeout), timeout_count: Default::default() }
        ];
        let mut is_successful = false; 
    
        for ping in pings.iter_mut() {
            let ping_result = ping.ping().await;
            if ping_result.is_ok() { is_successful = true }
        }
    
        if is_successful { Ok(()) }
        else { return Err(()) }
    }

    pub fn increment_timeout_count(&mut self, amount: u8) {
        self.timeout_count += amount;
    }

    pub fn reset_timeout_count(&mut self) {
        self.timeout_count = 0;
    }
}