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

#[allow(dead_code)]
fn delay_us(us: u32) {
    Delay::new().delay_us(us)
}

#[avr_device::entry]
fn main() -> ! {
    let dp = atmega_hal::Peripherals::take().unwrap();

    dp.PORTC.ddrc.write(|w| unsafe {w.bits(0xFF)});

    //let pins = atmega_hal::pins!(dp);

    //let mut led = pins.pb7.into_output();
    
    let mut count: u8 = 0;

    loop {
        //led.toggle();
        dp.PORTC.portc.write(|w| unsafe {w.bits(count)});
        count = count.wrapping_add(1);
        delay_ms(100);
    }
}