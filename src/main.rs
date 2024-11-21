use esp_idf_svc::hal::{
    gpio,
    prelude::Peripherals,
    task::block_on,
    timer::{TimerConfig, TimerDriver},
};
use log::*;
use std::{env, thread::sleep};

fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();
    env::set_var("RUST_BACKTRACE", "1");
    info!("Hello, world! Eyo");
    let peripherals = Peripherals::take()?;
    let mut gpio2_out = gpio::PinDriver::output(peripherals.pins.gpio2)?;
    let mut gpio19_out = gpio::PinDriver::output(peripherals.pins.gpio19)?;
    gpio19_out.set_high()?;
    let mut timer = TimerDriver::new(peripherals.timer00, &TimerConfig::new())?;
    let mut time: u64 = 0;
    timer.enable(true)?;
    block_on(async {
        loop {
            let duration = ((time as f64 / timer.tick_hz() as f64).sin() / 10.0) + 0.11;
            let duration_ticks = (duration * timer.tick_hz() as f64) as u64;
            info!(
                "Counter: {}, Duration: {}, Duration ticks: {}",
                time, duration, duration_ticks
            );
            info!("LED on");
            gpio2_out.set_high()?;
            gpio19_out.set_low()?;

            timer.delay(duration_ticks).await?;

            info!("LED off");
            gpio2_out.set_low()?;
            gpio19_out.set_high()?;

            timer.delay(duration_ticks).await?;
            time += duration_ticks * 2;
        }
    })
    // let mut time: f64 = 0.0;
    // loop {
    //     let duration = (time.sin() / 10.0) + 0.11;
    //     let duration_millis = (duration * 1000.0) as u64;
    //     info!("Time: {}", time);
    //     info!("Duration: {}", duration);
    //     info!("LED on");
    //     gpio2_out.set_high()?;
    //     gpio19_out.set_low()?;
    //     sleep(std::time::Duration::from_millis(duration_millis));
    //     info!("LED off");
    //     gpio2_out.set_low()?;
    //     gpio19_out.set_high()?;
    //     sleep(std::time::Duration::from_millis(duration_millis));
    //     time += duration;
    // }
}
