use cpal::Device;
use cpal::traits::{DeviceTrait, HostTrait};

pub fn get_or_default_input(device_name: Option<String>) -> anyhow::Result<Device> {
    let host = cpal::default_host();
    let target = device_name
        .clone()
        .unwrap_or_else(|| host.default_input_device().unwrap().name().unwrap());

    let mut device: Option<Device> = None;
    let input_devices = host.input_devices().unwrap();
    for in_device in input_devices {
        if in_device.name().is_ok_and(|name| name == target) {
            device = Some(in_device);
        }
    }
    if device.is_none() {
        return Err(anyhow::anyhow!("No target device found"));
    }
    let device = device.unwrap();
    Ok(device)
}

pub fn get_or_default_output(device_name: Option<String>) -> anyhow::Result<Device> {
    let host = cpal::default_host();
    let target = device_name
        .clone()
        .unwrap_or_else(|| host.default_output_device().unwrap().name().unwrap());

    let mut device: Option<Device> = None;
    let output_devices = host.output_devices().unwrap();
    for out_device in output_devices {
        if out_device.name().is_ok_and(|name| name == target) {
            device = Some(out_device);
        }
    }
    if device.is_none() {
        return Err(anyhow::anyhow!("No target device found"));
    }
    let device = device.unwrap();
    Ok(device)
}

pub fn get_available() -> String {
    for host in cpal::available_hosts() {
        tracing::debug!("Available host: {:?}", host);
    }

    let host = cpal::default_host();
    let mut device_names: Vec<String> = Vec::new();
    let default_device = host
        .default_input_device()
        .expect("No default input device")
        .name()
        .expect("Default input device has no name...");
    let input_devices = host.input_devices().expect("No input devices found");
    for in_device in input_devices {
        let d_name = in_device.name().expect("Device has no name...");
        let d_cfg = in_device
            .default_input_config()
            .expect("Device has no default input config...");
        let d_sampling_rate = d_cfg.sample_rate().0;
        let d_ch = d_cfg.channels();

        let mut d = format!(" * {}({}ch, {}hz)", d_name, d_ch, d_sampling_rate);
        if d_name == default_device {
            d.push_str(" [default]");
        }
        device_names.push(d);
    }
    device_names.join("\n")
}