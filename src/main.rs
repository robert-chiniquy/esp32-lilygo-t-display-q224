use embedded_graphics::prelude::*;
use embedded_graphics::primitives::PrimitiveStyleBuilder;
use embedded_graphics::{
    mono_font::{ascii::FONT_8X13, MonoTextStyle},
    text::Text,
    Drawable,
};

use esp_idf_hal::gpio::PinDriver;
use esp_idf_hal::{delay::*, prelude::*};

mod display;
mod menu;
mod util;

pub use display::Theme;
use menu::Menu;

fn main() -> ! {
    esp_idf_sys::link_patches();
    esp_idf_logger::init().unwrap();

    let peripherals = Peripherals::take().unwrap();
    let rst = peripherals.pins.gpio23;
    let dc = peripherals.pins.gpio16;
    let sclk = peripherals.pins.gpio18;
    let cs = peripherals.pins.gpio5;
    let spi = peripherals.spi2;
    let sdo = peripherals.pins.gpio19;
    let backlight = peripherals.pins.gpio4;

    let mut display = display::init_display(rst, dc, sclk, cs, spi, sdo, backlight).unwrap();

    util::draw_crab(&mut display).unwrap();

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
    let mut menu: Menu<14, 4> = Menu::new(
        Point::new(11, 110),
        "select",
        "next..",
        embedded_graphics::mono_font::ascii::FONT_8X13,
    );

    let top_level = menu.new_menu("OPTIONS");
    let scan_wifi = menu.new_menu("scan wifi");
    let explode = menu.new_menu("explode");
    menu.set_submenus(top_level, &[scan_wifi, explode]);
    menu.set_current_menu(top_level);

    let l_button = PinDriver::input(peripherals.pins.gpio0).unwrap();
    let r_button = PinDriver::input(peripherals.pins.gpio35).unwrap();

    // let mut i = 3;
    loop {
        menu.draw(&mut display);
        FreeRtos::delay_ms(40_u32);

        if l_button.is_low() {
            menu.l_click();
            if let Some(cur) = menu.selected {
                if cur == scan_wifi {
                } else if cur == explode {
                    display
                        .clear(embedded_graphics::pixelcolor::Rgb565::RED)
                        .unwrap();
                }
            }
            log::info!("❤️");
        }

        if r_button.is_low() {
            menu.r_click();
            menu.cursor_next();
        }
    }
}
