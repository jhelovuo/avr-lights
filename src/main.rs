#![no_std]
#![no_main]

use embedded_hal::delay::DelayNs;
use panic_halt as _;

// Define core clock. This can be used in the rest of the project.
type CoreClock = atmega_hal::clock::MHz16;
type Delay = atmega_hal::delay::Delay<crate::CoreClock>;

// Below are examples of a delay helper functions
#[allow(dead_code)]
fn delay_ms(ms: u16) {
    Delay::new().delay_ms(u32::from(ms))
}

fn delay_us(us: u32) {
    Delay::new().delay_us(us)
}

#[avr_device::entry]
fn main() -> ! {
    let dp = atmega_hal::Peripherals::take().unwrap();

    // Set Data Direction Register C to all pins output.
    dp.PORTC.ddrc.write(|w| unsafe {w.bits(0xFF)});
    
    let led_pwm_values :[u8;8] = [0 , 2, 4, 8, 16, 32, 64, 128];

    loop {
        //let mut pins_out: u8 = 0;
        for i in u8::MIN ..= u8::MAX {
            let pins_out = led_pwm_values.iter().fold(0, |mut out, limit| {
                out >>= 1; 
                if i < *limit {
                    out |= 0b1000_0000;
                }
                out
            });
            dp.PORTC.portc.write(|w| unsafe {w.bits(pins_out)});
            delay_us(10);
        }
    }
}