use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    // printing to serial console
    esp_idf_svc::log::EspLogger::initialize_default();
    
    let peripherals = Peripherals::take().unwrap();
    let mut led = PinDriver::output(peripherals.pins.gpio4)?;

    log::info!("Hello world!");
    loop {
        led.set_high()?;
        // we are sleeping here to make sure the watchdog isn't triggered
        FreeRtos::delay_ms(1000);
        log::info!("LED ON");

        led.set_low()?;
        FreeRtos::delay_ms(1000);
        log::info!("LED OFF");
    }
}
