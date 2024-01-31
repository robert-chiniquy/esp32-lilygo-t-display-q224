use std::error::Error;

use embedded_graphics;
use embedded_graphics::image::Image;
use tinybmp::*;

use super::*;

const BMP_DATA: &[u8; 16778] = include_bytes!("../ferris.bmp");

pub(crate) fn draw_crab(
    display: &mut impl DrawTarget<Color = embedded_graphics::pixelcolor::Rgb565>,
) -> Result<(), Box<dyn Error>> {
    let bmp = Bmp::from_slice(BMP_DATA).map_err(|e| format!("{e:?}"))?;
    Image::new(&bmp, Point::new(24, 0))
        .draw(display)
        .map_err(|_| Box::<dyn Error>::from("draw image"))?;
    Ok(())
}
