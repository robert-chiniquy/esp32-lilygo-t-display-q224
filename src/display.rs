use super::*;

use display_interface_spi::SPIInterface;
use embedded_graphics;
use esp_idf_hal;
use esp_idf_hal::delay::Ets;
use esp_idf_hal::gpio::AnyIOPin;
use esp_idf_hal::gpio::PinDriver;
use esp_idf_hal::spi::config::Config;
use esp_idf_hal::spi::SpiDeviceDriver;
use esp_idf_hal::spi::SpiDriver;
use esp_idf_hal::spi::SpiDriverConfig;
use mipidsi::Builder;
use std::error::Error;

pub(crate) fn init_display() -> Result<
    // impl DrawTarget<Color = embedded_graphics::pixelcolor::Rgb565>,
    mipidsi::Display<
        SPIInterface<
            SpiDeviceDriver<'static, SpiDriver<'static>>,
            PinDriver<'static, esp_idf_hal::gpio::Gpio16, esp_idf_hal::gpio::Output>,
        >,
        mipidsi::models::ST7789,
        PinDriver<'static, esp_idf_hal::gpio::Gpio23, esp_idf_hal::gpio::Output>,
    >,
    Box<dyn Error>,
> {
    let peripherals = Peripherals::take().unwrap();
    let rst = PinDriver::output(peripherals.pins.gpio23)?;
    let dc = PinDriver::output(peripherals.pins.gpio16)?;
    let sclk = peripherals.pins.gpio18;
    let cs = peripherals.pins.gpio5;
    let spi = peripherals.spi2;
    let sdo = peripherals.pins.gpio19;
    let spi = SpiDriver::new(spi, sclk, sdo, None::<AnyIOPin>, &SpiDriverConfig::new())?;
    let mut backlight = PinDriver::output(peripherals.pins.gpio4)?;
    backlight.set_high()?;
    let spi = SpiDeviceDriver::new(spi, Some(cs), &Config::new())?;
    let di = SPIInterface::new(spi, dc);
    let mut delay = Ets;
    let mut display = Builder::st7789_pico1(di)
        .init(&mut delay, Some(rst))
        .map_err(|e| format!("{e:?}"))?;
    display
        .clear(embedded_graphics::pixelcolor::Rgb565::BLACK)
        .map_err(|e| format!("{e:?}"))?;

    // display.set_tearing_effect(tearing_effect)
    Ok(display)
}
