use std::{net::IpAddr, random::random, time::Duration};
use surge_ping::{Client, Config, IcmpPacket, PingIdentifier, PingSequence, SurgeError, ICMP};

use crate::host::Host;

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

    pub fn increment_timeout_count(&mut self, amount: u8) {
        self.timeout_count += amount;
    }

    pub fn reset_timeout_count(&mut self) {
        self.timeout_count = 0;
    }
}