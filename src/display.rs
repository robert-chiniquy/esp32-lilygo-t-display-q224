use super::*;

use display_interface_spi::SPIInterface;
use embedded_graphics;

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
            red: embedded_graphics::pixelcolor::Rgb565::new(0xd5, 0x04, 0x58),
            green: embedded_graphics::pixelcolor::Rgb565::new(0x00, 0xf3, 0xd7),
            yellow: embedded_graphics::pixelcolor::Rgb565::new(0xfe, 0xfd, 0xc2),
            blue: embedded_graphics::pixelcolor::Rgb565::new(0xa5, 0xd5, 0xfe),
            magenta: embedded_graphics::pixelcolor::Rgb565::new(0x69, 0x07, 0x59),
            cyan: embedded_graphics::pixelcolor::Rgb565::new(0x02, 0xc3, 0xfc),
        }
    }
}

// TODO: To support palette rotation, either need to implement Model trait, composing over mipidsi::models::ST7789 to rotate the colors in transit, or, maybe could do TextRenderer
// - Also need to track a concept of display refresh speed so palette rotation can be suitably slow
//  - Might also want to play with update regions to dissociate with refresh ordering
// - could be fun to rotate only along one color axis at a time for subtlety

pub(crate) fn init_display(// rst: RST,
    // cs: impl Peripheral<P = impl OutputPin> + 'static,
    // sdo: impl Peripheral<P = impl OutputPin> + 'static,
) -> Result<
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
>
// where
//     RST: Peripheral<P = dyn OutputPin> + 'static,
{
    // FIXME: parameterize GPIO pin mapping
    let peripherals = Peripherals::take().unwrap();
    let rst = PinDriver::output(peripherals.pins.gpio23)?;
    let dc = PinDriver::output(peripherals.pins.gpio16)?;
    let sclk = peripherals.pins.gpio18;
    let cs = peripherals.pins.gpio5;
    let spi = peripherals.spi2;
    let sdo = peripherals.pins.gpio19;

    // let peripherals = Peripherals::take().unwrap();
    // let rst = PinDriver::output(rst)?;
    // let dc = PinDriver::output(peripherals.pins.gpio16)?;
    // let sclk = peripherals.pins.gpio18;
    // // let cs = peripherals.pins.gpio5;
    // let spi = peripherals.spi2;
    // // let sdo = peripherals.pins.gpio19;

    let spi = SpiDriver::new(spi, sclk, sdo, None::<AnyIOPin>, &SpiDriverConfig::new())?;
    let spi = SpiDeviceDriver::new(spi, Some(cs), &Config::new())?;
    let di = SPIInterface::new(spi, dc);

    let mut backlight = PinDriver::output(peripherals.pins.gpio4)?;
    backlight.set_high()?;

    let mut delay = Ets;
    let mut display = Builder::st7789_pico1(di)
        .init(&mut delay, Some(rst))
        .map_err(|e| format!("{e:?}"))?;
    display
        .clear(embedded_graphics::pixelcolor::Rgb565::BLACK)
        .map_err(|e| format!("{e:?}"))?;

    Ok(display)
}
