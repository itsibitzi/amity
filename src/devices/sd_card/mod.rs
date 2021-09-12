use core::fmt;
use teensy4_bsp::t40::P9;
use teensy4_bsp::{
    hal::gpio::{Input, GPIO},
    t40::Pins,
};

use self::sdmmc::command::SDMMCCommand;
use self::sdmmc::driver::SDMMCDriver;

mod sdmmc;

pub struct SdCard {
    det_pin: GPIO<P9, Input>,
    driver: SDMMCDriver,
    pub det: bool,
}

impl SdCard {
    pub fn new(pins: Pins) -> SdCard {
        SdCard {
            driver: SDMMCDriver::new(),
            det_pin: GPIO::new(pins.p9),
            det: false,
        }
    }

    pub fn poll(&mut self) {
        self.driver.command(SDMMCCommand::GO_IDLE_STATE, 2);
        if self.det_pin.is_set() {
            self.det = true;
        } else {
            self.det = false;
        }
    }
}

impl fmt::Debug for SdCard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SdCard {{ det: {} }}", self.det)
    }
}
