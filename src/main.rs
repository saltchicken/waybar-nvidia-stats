use nvml_wrapper::enum_wrappers::device::{Clock, TemperatureSensor};
use nvml_wrapper::error::NvmlError;
//use nvml_wrapper::{cuda_driver_version_major, cuda_driver_version_minor, Nvml};
use nvml_wrapper::Nvml;
//use pretty_bytes::converter::convert;

fn clock_tier(mut percentage: f64) -> String {
    percentage = (percentage * 100.0).round();
    if percentage < 30.0 {
        format!("{:>6}", percentage) // Green for low percentages
    } else if percentage < 70.0 {
        format!("{:>6}", percentage) // Yellow for medium percentages
    } else {
        format!("{:>6}", percentage) // Red for high percentages
    }
}

fn memory_tier(mut percentage: f64) -> String {
    percentage = (percentage * 100.0).round();
    if percentage < 30.0 {
        format!("{:>6}", percentage) // Green for low percentages
    } else if percentage < 70.0 {
        format!("{:>6}", percentage) // Yellow for medium percentages
    } else {
        format!("{:>6}", percentage) // Red for high percentages
    }
}

fn temperature_tier(temperature: u32) -> String {
    if temperature <= 45 {
        format!("{:>6}", temperature) // Green for low percentages
    } else if temperature <= 60 {
        format!("{:>6}", temperature) // Yellow for medium percentages
    } else {
        format!("{:>6}", temperature) // Red for high percentages
    }
}

fn main() -> Result<(), NvmlError> {
    let nvml = Nvml::init()?;

    //let cuda_version = nvml.sys_cuda_driver_version()?;

    // Grabbing the first device in the system, whichever one that is.
    // If you want to ensure you get the same physical device across reboots,
    // get devices via UUID or PCI bus IDs.
    let device = nvml.device_by_index(0)?;

    let temperature = device.temperature(TemperatureSensor::Gpu)?;
    let formatted_temperature = temperature_tier(temperature);

    let percentage_clock_used =
        device.clock_info(Clock::Graphics)? as f64 / device.max_clock_info(Clock::Graphics)? as f64;
    let formatted_percentage_clock_used = clock_tier(percentage_clock_used);

    let mem_info = device.memory_info()?;
    let percentage_mem_used = mem_info.used as f64 / mem_info.total as f64;
    let formatted_percentage_mem_used = memory_tier(percentage_mem_used);

    println!(
        //"{graphics_clock} {used_mem}/{total_mem} {temperature}",
        //"{percentage_clock_used} {percentage_mem_used} %{{F#FFFF00}}{temperature}%{{F-}}",
        "{percentage_clock_used}% CLK{percentage_mem_used}% VRAM{temperature}°C",
        //graphics_clock = graphics_clock,
        percentage_clock_used = formatted_percentage_clock_used,
        percentage_mem_used = formatted_percentage_mem_used,
        temperature = formatted_temperature,
        //used_mem = used_mem,
        //total_mem = total_mem,
    );
    Ok(())
}
