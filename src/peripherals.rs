use esp_idf_svc::hal::{gpio, prelude::Peripherals};

pub fn get_led() -> anyhow::Result<gpio::PinDriver<'static, gpio::Gpio2, gpio::Output>> {
    let peripherals = Peripherals::take().unwrap();
    let led = gpio::PinDriver::output(peripherals.pins.gpio2)?;
    Ok(led)
}
