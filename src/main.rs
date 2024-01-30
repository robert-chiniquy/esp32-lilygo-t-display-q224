use display_interface::prelude::*;
use display_interface_spi::*;
use esp_idf_svc::hal::{gpio::PinDriver, prelude::*, *};
// use mipidsi::Error;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    esp_idf_svc::sys::link_patches();

    let peripherals = Peripherals::take().unwrap();

    let mut backlight = PinDriver::output(peripherals.pins.gpio4)?;

    let dc = PinDriver::output(peripherals.pins.gpio16)?;
    let mut cs = PinDriver::output(peripherals.pins.gpio5)?;
    let mut rst = PinDriver::output(peripherals.pins.gpio23)?;
    let wr = PinDriver::output(peripherals.pins.gpio8)?;
    let mut rd = PinDriver::output(peripherals.pins.gpio9)?;

    backlight.set_high()?;

    // set to low to enable display
    cs.set_low()?;

    // set to high when not in use
    rd.set_high()?;

    let spi = peripherals.spi3; // I don't know if I should use SPI2 or 3
    let sclk = peripherals.pins.gpio18;
    // --
    let sdo = peripherals.pins.gpio19; // mosi?

    let di = SPIInterface::new(
        // Found this example here: https://github.com/ivmarkov/rust-esp32-std-demo/blob/50ab64ed4bf51f02807864458e096c595b94cc1c/src/main.rs#L1045-L1071
        spi::SpiDeviceDriver::new_single(
            spi,
            sclk,
            sdo,
            Option::<gpio::AnyIOPin>::None,
            Option::<gpio::AnyIOPin>::None,
            // Some(cs),
            &spi::SpiDriverConfig::new(),
            &spi::SpiConfig::new(),
        )?,
        dc,
    );

    // ### WARNING
    // The reset pin needs to be in *high* state in order for the display to operate.
    // If it wasn't provided the user needs to ensure this is the case.
    rst.set_high().expect(".");

    let mut display = mipidsi::Builder::st7789(di)
        .init(&mut delay::Ets, Some(rst))
        .unwrap();

    // // // have tried various ways of drawing anything to screen but LCD always black
    // // display.clear(RgbColor::RED).unwrap();
    loop {}
}
