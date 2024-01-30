use std::error::Error;

use display_interface_spi::SPIInterface;
use embedded_graphics::{image::Image, prelude::*};
use esp_idf_hal::{
    delay::Ets,
    gpio::{AnyIOPin, PinDriver},
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
    let cs = peripherals.pins.gpio5;
    let spi = peripherals.spi2;
    let sdo = peripherals.pins.gpio19; // mosi renamed to sdo

    let spi = SpiDriver::new(spi, sclk, sdo, None::<AnyIOPin>, &SpiDriverConfig::new())?;

    let mut backlight = PinDriver::output(peripherals.pins.gpio4)?;
    backlight.set_high()?;

    let spi = SpiDeviceDriver::new(spi, Some(cs), &Config::new())?;

    let di = SPIInterface::new(spi, dc);

    let mut delay = Ets;
    let mut display = Builder::st7789_pico1(di)
        .init(&mut delay, Some(rst))
        .map_err(|e| format!("{e:?}"))?;

    let bmp_data = include_bytes!("../ferris.bmp");
    let bmp = Bmp::from_slice(bmp_data).map_err(|e| format!("{e:?}"))?;

    Image::new(&bmp, Point::new(0, 0))
        .draw(&mut display)
        .map_err(|_| Box::<dyn Error>::from("draw image"))?;

    Ok(())
}
