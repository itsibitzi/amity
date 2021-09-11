//! The starter code slowly blinks the LED, and sets up
//! USB logging.

#![no_std]
#![no_main]

use teensy4_bsp as bsp;
use teensy4_panic as _;

mod devices;
mod logging;

const SLEEP_PERIOD: u32 = 100;

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = bsp::Peripherals::take().unwrap();
    let mut systick = bsp::SysTick::new(cortex_m::Peripherals::take().unwrap().SYST);
    let pins = bsp::t40::into_pins(p.iomuxc);

    let mut sd_card = devices::sd_card::SdCard::new(pins);

    // See the `logging` module docs for more info.
    assert!(logging::init().is_ok());

    loop {
        sd_card.poll();
        systick.delay(SLEEP_PERIOD);
        log::info!("{:?}", sd_card);
    }
}
