use smart_leds::{SmartLedsWrite, RGB8};
use ws2812_esp32_rmt_driver::Ws2812Esp32Rmt;

use crate::{CO2State, Environment};

pub struct LedDisplay {
    hw: Ws2812Esp32Rmt,
}

impl LedDisplay {
    pub fn init() -> Self {
        return Self {
            hw: Ws2812Esp32Rmt::new(0, 2).unwrap()
        }
    }

    pub fn update_status(&mut self, environment: &Environment) {
        let color = match environment.co2_state {
            CO2State::Good(_) => RGB8 { r: 0, g: 128, b: 0 },
            CO2State::Average(_) => RGB8 { r: 128, g: 109, b: 6 },
            CO2State::Bad(_) => RGB8 { r: 128, g: 0, b: 0 },
        };

        self.hw.write([color, color, color, color, color].into_iter()).unwrap();
    }
}