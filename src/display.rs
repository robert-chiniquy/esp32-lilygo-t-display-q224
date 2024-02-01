use super::*;

use display_interface_spi::SPIInterface;
use embedded_graphics;

// use embedded_graphics::primitives::Rectangle;
use esp_idf_hal;
use esp_idf_hal::delay::Ets;
use esp_idf_hal::gpio::AnyIOPin;
use esp_idf_hal::gpio::OutputPin;
use esp_idf_hal::gpio::PinDriver;
use esp_idf_hal::peripheral::Peripheral;
use esp_idf_hal::spi::config::Config;
use esp_idf_hal::spi::SpiDeviceDriver;
use esp_idf_hal::spi::SpiDriver;
use esp_idf_hal::spi::SpiDriverConfig;
use mipidsi::Builder;
use std::error::Error;

pub struct Theme {
    pub red: embedded_graphics::pixelcolor::Rgb565,
    pub green: embedded_graphics::pixelcolor::Rgb565,
    pub yellow: embedded_graphics::pixelcolor::Rgb565,
    pub blue: embedded_graphics::pixelcolor::Rgb565,
    pub magenta: embedded_graphics::pixelcolor::Rgb565,
    pub cyan: embedded_graphics::pixelcolor::Rgb565,
}

impl Default for Theme {
    // red: "#d50458"
    // green: "#00f3d7"
    // yellow: "#fefdc2"
    // blue: "#a5d5fe"
    // magenta: "#690759"
    // cyan: "#02c3fc"
    fn default() -> Self {
        Self {
            // ?
            red: embedded_graphics::pixelcolor::Rgb565::new(0xd5 / 8, 0x04 / 4, 0x58 / 8),
            green: embedded_graphics::pixelcolor::Rgb565::new(0x00 / 8, 0xf3 / 4, 0xd7 / 8),
            yellow: embedded_graphics::pixelcolor::Rgb565::new(0xfe / 8, 0xfd / 4, 0xc2 / 8),
            blue: embedded_graphics::pixelcolor::Rgb565::new(0xa5 / 8, 0xd5 / 4, 0xfe / 8),
            magenta: embedded_graphics::pixelcolor::Rgb565::new(0x69 / 8, 0x07 / 4, 0x59 / 8),
            cyan: embedded_graphics::pixelcolor::Rgb565::new(0x02 / 8, 0xc3 / 4, 0xfc / 8),
        }
    }
}

// TODO: To support palette rotation, either need to implement Model trait, composing over mipidsi::models::ST7789 to rotate the colors in transit, or, maybe could do TextRenderer
// - Also need to track a concept of display refresh speed so palette rotation can be suitably slow
//  - Might also want to play with update regions to dissociate with refresh ordering
// - could be fun to rotate only along one color axis at a time for subtlety

pub(crate) fn init_display<
    RST: OutputPin,
    DC: OutputPin,
    SCLK: OutputPin,
    CS: OutputPin,
    SPIPins: esp_idf_hal::spi::SpiAnyPins,
    SPI: Peripheral<P = SPIPins> + 'static,
    SDO: OutputPin,
    BL: OutputPin,
>(
    rst: RST,
    dc: DC,
    sclk: SCLK,
    cs: CS,
    spi: SPI,
    sdo: SDO,
    backlight: BL,
) -> Result<
    // impl DrawTarget<Color = embedded_graphics::pixelcolor::Rgb565>,
    mipidsi::Display<
        SPIInterface<
            SpiDeviceDriver<'static, SpiDriver<'static>>,
            PinDriver<'static, DC, esp_idf_hal::gpio::Output>,
        >,
        mipidsi::models::ST7789,
        PinDriver<'static, RST, esp_idf_hal::gpio::Output>,
    >,
    Box<dyn Error>,
> {
    let rst = PinDriver::output(rst)?;
    let dc = PinDriver::output(dc)?;

    let spi = SpiDriver::new(spi, sclk, sdo, None::<AnyIOPin>, &SpiDriverConfig::new())?;
    let spi = SpiDeviceDriver::new(spi, Some(cs), &Config::new())?;
    let di = SPIInterface::new(spi, dc);

    let mut backlight = PinDriver::output(backlight)?;
    backlight.set_high()?;

    let mut delay = Ets;
    let mut display = Builder::st7789_pico1(di)
        .init(&mut delay, Some(rst))
        .map_err(|e| format!("{e:?}"))?;
    display
        // .clipped(&Rectangle::new(Point::new(0, 80), Size::new(135, 200)))
        .clear(embedded_graphics::pixelcolor::Rgb565::BLACK)
        .map_err(|e| format!("{e:?}"))?;

    Ok(display)
}
