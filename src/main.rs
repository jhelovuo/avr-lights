#![no_std]
#![no_main]

use embedded_hal::delay::DelayNs;
use panic_halt as _;

// Define core clock. This can be used in the rest of the project.
type CoreClock = atmega_hal::clock::MHz16;
type Delay = atmega_hal::delay::Delay<crate::CoreClock>;

// Below are examples of a delay helper functions
fn delay_ms(ms: u16) {
    Delay::new().delay_ms(u32::from(ms))
}

#[avr_device::entry]
fn main() -> ! {
    let dp = atmega_hal::Peripherals::take().unwrap();

    // Set Data Direction Register C to all pins output.
    dp.PORTC.ddrc.write(|w| unsafe {w.bits(0xFF)});
    
    let mut my_byte = 0x01_u8;

    loop {
        dp.PORTC.portc.write(|w| unsafe {w.bits(my_byte)});
        my_byte = my_byte.rotate_left(1);
        delay_ms(400);
    }
}