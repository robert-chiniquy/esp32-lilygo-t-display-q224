use std::error::Error;

use display_interface_spi::SPIInterface;
use embedded_graphics::{image::Image, prelude::*};
use esp_idf_hal::{
    delay::Ets,
    gpio::{AnyIOPin, Gpio5, PinDriver},
    prelude::*,
    spi::{config::Config, SpiDeviceDriver, SpiDriver, SpiDriverConfig},
};
use mipidsi::Builder;
use tinybmp::*;

fn main() -> Result<(), Box<dyn Error>> {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    let rst = PinDriver::output(peripherals.pins.gpio23)?;
    let dc = PinDriver::output(peripherals.pins.gpio16)?;
    let sclk = peripherals.pins.gpio18;
    let sdo = peripherals.pins.gpio19; // mosi renamed to sdo

    let mut backlight = PinDriver::output(peripherals.pins.gpio4)?;

    let _cs = PinDriver::output(peripherals.pins.gpio5)?;
    let _wr = PinDriver::output(peripherals.pins.gpio8)?;
    let _rd = PinDriver::output(peripherals.pins.gpio9)?;

    let spi = peripherals.spi2; // or spi3?

    let mut delay = Ets;

    let spi = SpiDriver::new(spi, sclk, sdo, None::<AnyIOPin>, &SpiDriverConfig::new())?;

    backlight.set_high()?;

    let cs = unsafe { Gpio5::new() };

    let spi = SpiDeviceDriver::new(spi, Some(cs), &Config::new())?;

    //     impl<SPI, DC> WriteOnlyDataCommand for SPIInterface<SPI, DC>
    // where
    //     SPI: SpiDevice,
    //     DC: OutputPin,
    let di = SPIInterface::new(spi, dc);

    // ### WARNING
    // The reset pin needs to be in *high* state in order for the display to operate.
    // If it wasn't provided the user needs to ensure this is the case.

    let mut display = Builder::st7789_pico1(di)
        .init(&mut delay, Some(rst))
        .map_err(|e| format!("{e:?}"))?;

    let bmp_data = include_bytes!("../ferris.bmp");
    let bmp = Bmp::from_slice(bmp_data).unwrap();

    Image::new(&bmp, Point::new(0, 0))
        .draw(&mut display)
        .map_err(|_| Box::<dyn Error>::from("draw image"))?;

    Ok(())
}
