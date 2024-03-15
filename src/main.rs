#![no_std]
#![no_main]

use defmt::info;
use {defmt_rtt as _, panic_probe as _};

#[cortex_m_rt::entry]
fn main() -> ! {
    info!("Hello, World!");

    loop {
        cortex_m::asm::wfi();
    }
}
