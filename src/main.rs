#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;
use cortex_m_semihosting::{hprintln, debug};
use stm32f4xx_hal::{delay::Delay, prelude::*, stm32};

#[entry]
fn main() -> ! {
    let _ = hprintln!("Hello, World!");
    
    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}
