#![no_std]
#![no_main]

mod leds;

use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl, delay::Delay, gpio::Io, peripherals::Peripherals, prelude::*,
    system::SystemControl,
};
use leds::{
    animations::{
        animation::Animation, breathe_in_out::BreatheInOut, sprint_out_back::SprintOutBack,
    },
    controller::LedDriver,
};
use smart_leds::{White, RGBW};

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);

    let clocks = ClockControl::max(system.clock_control).freeze();
    let delay = Delay::new(&clocks);

    esp_println::logger::init_logger_from_env();

    let timer = esp_hal::timer::systimer::SystemTimer::new(peripherals.SYSTIMER).alarm0;
    let _init = esp_wifi::initialize(
        esp_wifi::EspWifiInitFor::Wifi,
        timer,
        esp_hal::rng::Rng::new(peripherals.RNG),
        peripherals.RADIO_CLK,
        &clocks,
    )
    .unwrap();

    log::info!("Init GPIO");
    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);

    let color = RGBW {
        r: 255,
        g: 0,
        b: 0,
        a: White(0),
    };

    let led_count = 60;
    const MAX_LED_COUNT: usize = 120;

    let mut colors = [color; MAX_LED_COUNT];
    let strip_colors = &mut colors[..led_count];

    let mut driver = LedDriver::new(&mut peripherals.SPI2, io, &clocks);

    loop {
        SprintOutBack::run(strip_colors, &mut driver, &delay);
        BreatheInOut::run(strip_colors, &mut driver, &delay);
    }
}
