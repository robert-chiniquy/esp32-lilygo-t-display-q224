use std::error::Error;

use display_interface_spi::SPIInterface;
use embedded_graphics::{image::Image, prelude::*};
use esp_idf_hal::{delay::Ets, prelude::*, spi::config::Config};

use tinybmp::*;

mod display;

fn main() -> Result<(), Box<dyn Error>> {
    esp_idf_sys::link_patches();

    let mut display = display::init_display()?;

    let bmp_data = include_bytes!("../ferris.bmp"); // 86 × 64
    let bmp = Bmp::from_slice(bmp_data).map_err(|e| format!("{e:?}"))?;

    // display is 135x240
    // (135-86)/2=24
    Image::new(&bmp, Point::new(24, 0))
        .draw(&mut display)
        .map_err(|_| Box::<dyn Error>::from("draw image"))?;

    Ok(())
}
