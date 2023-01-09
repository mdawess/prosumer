use std::net::IpAddr;
use crate::der::Device;

// A user of the system
// A prosumer can own any number of DERs
// E.g. Solar panel, small wind turbine, etc.
// Auth is optional if on local ip address
pub struct Prosumer {
    client_id: u32,
    name: String,
    email: String,
    auth: bool,
    ip: IpAddr,
    port: u16,
    pub devices: Vec<Device>
}