// Agnostic API for DERs (Distributed Energy Resources)
use mqtt::Topic;

// A user of the system
// A prosumer can own any number of DERs
// E.g. Solar panel, small wind turbine, etc.
// Auth is optional if on local ip address
pub struct Prosumer {
    name: String,
    email: String,
    auth: bool,
    pub devices: Vec<Device>
}

// A DER owned/controlled by a prosumer
// A DER can subscribe to any given number of topics
// available for the specific DER type
pub struct Device {
    device_id: String,
    device_state: DeviceState,
    device_config: DeviceConfig,
    pub name: String,
    pub device_type: DeviceType,
    pub topics: Topics,
    pub connected: bool
}

// DERs currently (or planned to be supported)
enum DeviceType {
    solar_panel,
    wind_turbine,
}

// Specifying the topics which each device can subscribe to
// Currently configured for testing
impl Topic for DeviceType {
    fn get_topic(&self) -> String {
        match self {
            DeviceType::solar_panel => (Topic::test, Topic::hello),
            DeviceType::wind_turbine => (Topic::test, Topic::hello),
        }
    }
}