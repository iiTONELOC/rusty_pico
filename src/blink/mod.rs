use embedded_hal::digital::v2::OutputPin;
use hal::gpio::bank0::Gpio22;
use rp2040_hal as hal;

#[doc = "This function alternates the pin between high and low every 1/2 second."]
///
/// Attach an LED to GPIO22 and you will see it blink.
///
/// GP22 was selected because it does not interfere with UART, I2C, SPI, etc.
///
/// For more information see: https://datasheets.raspberrypi.com/picow/PicoW-A4-Pinout.pdf
pub fn pin_gpio22(
    led_pin: &mut hal::gpio::Pin<Gpio22, hal::gpio::Output<hal::gpio::PushPull>>,
    delay: &mut cortex_m::delay::Delay,
) {
    led_pin.set_high().unwrap();
    delay.delay_ms(500);
    led_pin.set_low().unwrap();
    delay.delay_ms(500);
}
