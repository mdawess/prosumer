use std::net::IpAddr;

/// MQTT is a struct that represents a MQTT connection.
pub struct MQTT {
    /// The IP address of the MQTT broker.
    pub ip: IpAddr,
    /// The port of the MQTT broker.
    pub port: u16,
    /// The username of the MQTT broker.
    pub username: String,
    /// The password of the MQTT broker.
    pub password: String,
    /// The client ID of the MQTT broker.
    pub client_id: String,
}

// All topics available for DERs
// Currently configured for testing
pub enum Topic {
    test,
    hello
}