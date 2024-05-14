mod peripherals;

use log::*;
use std::thread::sleep;

fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();
    info!("Hello, world! Eyo");
    let mut led = peripherals::get_led()?;
    let mut time: f64 = 0.0;
    loop {
        let duration = (time.sin() / 10.0) + 0.11;
        let duration_millis = (duration * 1000.0) as u64;
        info!("Time: {}", time);
        info!("Duration: {}", duration);
        info!("LED on");
        led.set_high()?;
        sleep(std::time::Duration::from_millis(duration_millis));
        info!("LED off");
        led.set_low()?;
        sleep(std::time::Duration::from_millis(duration_millis));
        time += duration;
    }
}
