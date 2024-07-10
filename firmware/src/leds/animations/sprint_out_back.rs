use esp_hal::{delay::Delay, prelude::*};
use smart_leds::{SmartLedsWrite, RGBW};

use crate::leds::controller::LedDriver;

use super::animation::Animation;

pub struct SprintOutBack {}

impl Animation for SprintOutBack {
    fn run(strip_colors: &mut [RGBW<u8>], driver: &mut LedDriver, delay: &Delay) {
        let led_count = strip_colors.len();

        for i in 0..led_count {
            let prev_index = if i > 0 { i - 1 } else { 0 };
            strip_colors[prev_index].r = 255;
            strip_colors[prev_index].g = 0;

            strip_colors[i].r = 0;
            strip_colors[i].g = 255;

            let it = strip_colors.iter().cloned();
            driver.ws2812.write(it).unwrap();
            log::info!("Waiting for delay...");
            delay.delay(10.millis());
        }

        for i in (0..led_count - 1).rev() {
            let prev_index = if i < led_count { i + 1 } else { led_count - 1 };
            strip_colors[prev_index].r = 255;
            strip_colors[prev_index].g = 0;

            strip_colors[i].r = 0;
            strip_colors[i].g = 255;

            driver.ws2812.write(strip_colors.iter().cloned()).unwrap();
            log::info!("Waiting for delay...");
            delay.delay(10.millis());
        }
    }
}
