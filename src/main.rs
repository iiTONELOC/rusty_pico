#![no_std]
#![no_main]

pub mod blink;
pub mod pico;

use self::{blink::pin_gpio22, pico::init_pico};

use cortex_m_rt::entry;
use defmt_rtt as _;

#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

#[entry]
fn main() -> ! {
    // initialize the Pico board and get the pins and delay objects
    // the init logic is from the rp2040-hal crate
    let (pins, mut delay) = init_pico();

    // *************** NOTE ***************
    // An individual GPIO can be assigned from the pins object.
    // This particular example uses GPIO22 and sets it to a push-pull output.
    // We pass pin 22 to the blink::gpio22 function. Which is defined in our blink/mod.rs file.
    // The simple function alternates the pin between high and low every 1/2 second.
    //
    // Attach an LED to GPIO22 and you will see it blink.

    let mut led_pin = pins.gpio22.into_push_pull_output();

    loop {
        pin_gpio22(&mut led_pin, &mut delay);
    }
}
