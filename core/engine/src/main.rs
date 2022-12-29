
fn main() {

    let allowed_devices: [&str; 2] = ["Solar Panels", "Wind Turbines"];

    for device in allowed_devices.iter() {
        println!("Device: {}", device);
    }
}
