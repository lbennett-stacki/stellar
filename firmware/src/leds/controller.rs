use esp_hal::{
    clock::Clocks,
    gpio::Io,
    peripherals::SPI2,
    spi::{master::Spi, FullDuplexMode, SpiMode},
};
use fugit::RateExtU32;
use smart_leds::{SmartLedsWrite, White, RGBA};
use ws2812_spi::{devices::Sk6812w, Ws2812};

pub struct LedDriver<'clocks> {
    pub ws2812: Ws2812<Spi<'clocks, SPI2, FullDuplexMode>, Sk6812w>,
}

impl LedDriver<'_> {
    pub fn new<'clocks>(
        spi_peripheral: &'clocks mut SPI2,
        io: Io,
        clocks: &'clocks Clocks,
    ) -> LedDriver<'clocks> {
        let mosi_pin = io.pins.gpio7;
        let clock_frequency = 3_200_000u32.Hz();
        let spi_mode = SpiMode::Mode0;

        let spi = Spi::new(spi_peripheral, clock_frequency, spi_mode, clocks).with_mosi(mosi_pin);

        let ws2812 = Ws2812::new_sk6812w(spi);

        LedDriver { ws2812 }
    }

    pub fn write<T, I>(&mut self, iterator: T)
    where
        T: IntoIterator<Item = I>,
        I: Into<RGBA<u8, White<u8>>>,
    {
        self.ws2812.write(iterator).unwrap()
    }
}
