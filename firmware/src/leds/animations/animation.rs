use esp_hal::delay::Delay;
use smart_leds::RGBW;

use crate::leds::controller::LedDriver;

pub trait Animation {
    fn run(strip_colors: &mut [RGBW<u8>], ws: &mut LedDriver, delay: &Delay);
}
