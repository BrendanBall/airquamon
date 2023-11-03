#![no_std]
#![no_main]

use embedded_hal::blocking::spi::Write;
use embedded_hal::digital::v2::{InputPin, OutputPin};
use esp_backtrace as _;
use esp_println::println;
use hal::{
    clock::ClockControl,
    gpio::IO,
    peripherals::Peripherals,
    prelude::*,
    spi::{FullDuplexMode, Spi, SpiMode},
    Delay,
};
// use embedded_graphics::{
//     pixelcolor::BinaryColor::On as Black, prelude::*, primitives::{Line, PrimitiveStyle},
// };
use embedded_graphics::{
    mono_font::{ascii, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, Line, PrimitiveStyle},
    text::{Baseline, Text, TextStyleBuilder},
};

use log::{error, info};
use max7219_driver::MAX7219LedMat;

enum Error<T: Write<u8>> {
    EpdError(<T as Write<u8>>::Error),
}

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let mut system = peripherals.SYSTEM.split();
    let mut clocks = ClockControl::boot_defaults(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);

    // setup logger
    // To change the log_level change the env section in .config/cargo.toml
    // or remove it and set ESP_LOGLEVEL manually before running cargo run
    // this requires a clean rebuild because of https://github.com/rust-lang/cargo/issues/10358
    esp_println::logger::init_logger_from_env();
    info!("Logger is setup");
    println!("Hello world!");

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mosi = io.pins.gpio4;
    let sck = io.pins.gpio5;
    let cs = io.pins.gpio6.into_push_pull_output();

    let mut spi = Spi::new_no_cs_no_miso(
        peripherals.SPI2,
        sck,
        mosi,
        100u32.kHz(),
        SpiMode::Mode0,
        &mut system.peripheral_clock_control,
        &mut clocks,
    );

    info!("Connecting to MAX7219 display");
    let mut display: MAX7219LedMat<_, _, 256, 4> =
        MAX7219LedMat::new(spi, cs).expect("failed instantiating display");
    display.init_display(true);

    let txtstyle = MonoTextStyle::new(&ascii::FONT_6X10, BinaryColor::On);
    Text::new("Y", Point::new(0, 7), txtstyle)
        .draw(&mut display)
        .unwrap();

    loop {
        delay.delay_ms(5000u16);

        info!("Waiting");
    }
}
