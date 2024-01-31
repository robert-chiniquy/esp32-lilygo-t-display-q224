use std::error::Error;

use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{PrimitiveStyleBuilder, Rectangle};
use embedded_graphics::{
    mono_font::{ascii::FONT_8X13, MonoTextStyle},
    text::Text,
    Drawable,
};

use esp_idf_hal::prelude::*;

mod display;
mod menu;
mod util;

pub use display::Theme;
use menu::Menu;

fn main() -> Result<(), Box<dyn Error>> {
    esp_idf_sys::link_patches();

    // let peripherals = Peripherals::take().unwrap();
    // let rst = peripherals.pins.gpio23;
    // let dc = peripherals.pins.gpio16;
    // let sclk = peripherals.pins.gpio18;
    // let cs = peripherals.pins.gpio5;
    // let spi = peripherals.spi2;
    // let sdo = peripherals.pins.gpio19;

    let mut display = display::init_display(/*rst, cs, sdo*/)?;

    // let button = io.pins.gpio0.into_pull_up_input();

    util::draw_crab(&mut display)?;

    Text::new(
        "Goodnight Emma!",
        Point::new(2, 80),
        MonoTextStyle::new(&FONT_8X13, RgbColor::BLUE),
    )
    .draw(&mut display)
    .unwrap();

    // with i.e. FONT_8X13, char height is 13px
    // display is 135x240
    // line width in chars = 135 / 8 = 16. ... lets say 14, try to center it
    // 14 * 8 = 112, 135-112=23, 23/2=11
    // 5 lines for menu * 13 = 65px
    // that includes roughly 1 line for bottom button labels
    // 240 - 65 = 175
    // 175 / 13 = 12ish lines for output or status
    let menu: Menu<14, 4> = Menu::new(
        Point::new(11, 110),
        "select",
        "next..",
        embedded_graphics::mono_font::ascii::FONT_8X13,
    );

    menu.draw(&mut display);

    // loop {}

    Ok(())
}
