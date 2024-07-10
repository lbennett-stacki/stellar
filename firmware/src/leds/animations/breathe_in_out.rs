use esp_hal::{delay::Delay, prelude::*};
use smart_leds::{SmartLedsWrite, White, RGBW};

use crate::leds::controller::LedDriver;

use super::animation::Animation;

pub struct BreatheInOut {}

impl Animation for BreatheInOut {
    fn run(strip_colors: &mut [RGBW<u8>], driver: &mut LedDriver, delay: &Delay) {
        for a in 0..=255 {
            for color in strip_colors.iter_mut() {
                color.a = White(a);
            }

            log::info!("Writing with white {}", strip_colors[0].a.0);

            driver.ws2812.write(strip_colors.iter().cloned()).unwrap();
            log::info!("Waiting for delay...");
            delay.delay(10.millis());
        }

        for a in (1..255).rev() {
            for color in strip_colors.iter_mut() {
                color.a = White(a);
            }

            log::info!("Writing with white {}", strip_colors[0].a.0);

            driver.ws2812.write(strip_colors.iter().cloned()).unwrap();
            log::info!("Waiting for delay...");
            delay.delay(10.millis());
        }
    }
}
