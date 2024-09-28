use crate::ui::Device;

pub fn create_devices() -> Vec<Device> {
    let mut devices: Vec<Device> = Vec::new();
    for i in 0..8 {
        devices.push(Device {
            port: i,
            selected: false,
            signal: true,
            sn: "00123456789ABCDEF".into(),
            state: "OFF".into(),
            version: "Version 1.0.0".into(),
        });
    }
    devices
}
