// Agnostic API for DERs (Distributed Energy Resources)

// A DER owned/controlled by a prosumer
// A DER can subscribe to any given number of topics
// available for the specific DER type
// @todo: Add a way to configure the DER connection
pub struct Device {
    device_id: String,
    device_state: DeviceState,
    pub name: String,
    pub device_type: DeviceType,
    pub connected: bool
}

// A DER can be in one of the following states
enum DeviceState {
    // DER is currently producing energy
    Producing,
    // DER is currently consuming energy
    Consuming,
    // DER is currently idle
    Idle,
}

// DERs currently (or planned to be supported)
pub enum DeviceType {
    SolarPanel,
    WindTurbine,
}

// All topics available for DERs
// Currently configured for testing
const TOPICS: &[&str] = &[
    "energy_produced",
    "energy_available"
];

// QoS for each available topic
/// Both are set as 1 to allow the messages to be sent
/// > 1 time increasing reliability without adding significant
/// additional overhead
const QoS: &[i32] = &[
    1,
    1
];

impl DeviceType {
    fn get_topics(&self) -> Vec<String> {
        match self {
            DeviceType::SolarPanel => vec![TOPICS[0].to_string(), TOPICS[1].to_string()],
            DeviceType::WindTurbine => vec![TOPICS[0].to_string(), TOPICS[1].to_string()]
        }
    }
}

// Specifying the topics which each device can subscribe to
impl Device {
    fn new(device_id: String, device_state: DeviceState, name: String, device_type: DeviceType) -> Device {
        Device {
            device_id,
            device_state,
            name,
            device_type,
            connected: false
        }
    }
}

