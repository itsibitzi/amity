use core::fmt;
use embedded_hal::digital::v2::OutputPin;
use teensy4_bsp::t40::{P13, P9};
use teensy4_bsp::{
    hal::gpio::{Input, Output, GPIO},
    t40::Pins,
};

pub struct SdCard {
    led_pin: GPIO<P13, Output>,
    det_pin: GPIO<P9, Input>,
    pub det: bool,
}

impl SdCard {
    pub fn new(pins: Pins) -> SdCard {
        let led = teensy4_bsp::configure_led(pins.p13);
        let sd_card_in = GPIO::new(pins.p9);

        SdCard {
            led_pin: led,
            det_pin: sd_card_in,
            det: false,
        }
    }

    pub fn poll(&mut self) {
        if self.det_pin.is_set() {
            self.det = true;
            self.led_pin.set_high().unwrap();
        } else {
            self.det = false;
            self.led_pin.set_low().unwrap();
        }
    }
}

impl fmt::Debug for SdCard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SdCard {{ det: {} }}", self.det)
    }
}
