use std::net::IpAddr;

#[derive(Debug)]
pub struct Host {
    pub name: String,
    pub ip: IpAddr,
    pub timeout_count: u16 = 0
}

impl Host {
    pub fn increment_timeout_count(&mut self, amount: u16) {
        self.timeout_count += amount;
    }

    pub fn reset_timeout_count(&mut self) {
        self.timeout_count = 0;
    }
}