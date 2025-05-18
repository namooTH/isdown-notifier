use std::net::IpAddr;

#[derive(Debug, Clone)]
pub struct Host {
    pub name: String,
    pub ip: IpAddr
}