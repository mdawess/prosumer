
extern crate paho_mqtt as mqtt;

pub fn main() {
    
    // 8883 is the encrypted TCP port
    let client = mqtt::AsyncClient::new("tcp://localhost:8883").unwrap_or_else(|err| {
        println!("Error creating the client: {:?}", err);
        std::process::exit(1);
    });

    // Does not handle error
    // Normally, the broker should be passed in rather than None
    client.connect(None);
    println!("Connected to the MQTT broker.");
}
