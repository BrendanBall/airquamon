#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_println::println;
use hal::{
    clock::ClockControl,
    gpio::IO,
    peripherals::Peripherals,
    prelude::*,
    spi::{Spi, SpiMode},
    Delay,
};
// use embedded_graphics::{
//     pixelcolor::BinaryColor::On as Black, prelude::*, primitives::{Line, PrimitiveStyle},
// };
use embedded_graphics::{
    mono_font::MonoTextStyleBuilder,
    prelude::*,
    primitives::{Circle, Line, PrimitiveStyle},
    text::{Baseline, Text, TextStyleBuilder},
};
use epd_waveshare::{color::*, epd2in9bc::*, prelude::*};

use log::info;

#[derive(Debug)]
enum Error {
    EpdError,
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
    let dc = io.pins.gpio7.into_push_pull_output();
    let rst = io.pins.gpio18.into_push_pull_output();
    let busy = io.pins.gpio19.into_pull_down_input();

    let mut spi = Spi::new_no_cs_no_miso(
        peripherals.SPI2,
        sck,
        mosi,
        100u32.kHz(),
        SpiMode::Mode0,
        &mut system.peripheral_clock_control,
        &mut clocks,
    );

    info!("Connecting to display");

    // Setup EPD
    let mut epd = Epd2in9bc::new(&mut spi, cs, busy, dc, rst, &mut delay).unwrap();

    let mut display = EpdDisplay::new(&mut spi, &mut epd, &mut delay);

    // update_display(&mut delay, "Airquamon!").unwrap();

    loop {
        delay.delay_ms(5000u16);

        info!("Waiting for data ready");
    }
}

trait Display<DELAY>
where
    DELAY: DelayMs<u8>,
{
    fn sleep(&self) -> Result<(), Error>;
    fn update_display(text: &str) -> Result<(), Error>;
}

struct EpdDisplay<SPI, CS, BUSY, DC, RST, DELAY> {
    epd: &mut WaveshareDisplay<SPI, CS, BUSY, DC, RST, DELAY>,
    spi: &mut SPI,
    delay: &mut DELAY,
}

impl Display for EpdDisplay<SPI, CS, BUSY, DC, RST, DELAY> {
    fn sleep(&self) -> Result<(), Error> {
        self.epd.sleep(&mut self.spi, &mut self.delay)?;
        Ok(())
    }

    fn update_display(&self, text: &str) -> Result<(), Error> {
        let mut mono_display = Display2in9bc::default();
        mono_display.set_rotation(DisplayRotation::Rotate0);
        draw_text(&mut mono_display, text, 5, 10);
        self.epd
            .update_frame(&mut spi, mono_display.buffer(), &mut delay)?;
        self.epd.display_frame(&mut spi, &mut delay)?;
        self.epd.sleep(&mut self.spi, &mut self.delay)?;
        Ok(())
    }
}

impl EpdDisplay<SPI, CS, BUSY, DC, RST, DELAY> {
    fn new(
        spi: &mut SPI,
        epd: &mut WaveshareDisplay<SPI, CS, BUSY, DC, RST, DELAY>,
        delay: &mut DELAY,
    ) {
        EpdDisplay { epd, spi, delay }
    }
}

// fn update_display(delay: &mut Delay,  text: &str) -> Result<(), Error> {
//     // Use display graphics from embedded-graphics
//     // This display is for the black/white pixels
//     let mut mono_display = Display2in9bc::default();

//     // Use embedded graphics for drawing
//     // A black line
//     // let _ = Line::new(Point::new(0, 120), Point::new(0, 200))
//     //     .into_styled(PrimitiveStyle::with_stroke(Black, 1))
//     //     .draw(&mut mono_display);

//     // // Use a second display for red/yellow
//     // let mut chromatic_display = Display2in9bc::default();

//     // // We use `Black` but it will be shown as red/yellow
//     // let _ = Line::new(Point::new(15, 120), Point::new(15, 200))
//     //     .into_styled(PrimitiveStyle::with_stroke(Black, 1))
//     //     .draw(&mut chromatic_display);

//     // // Display updated frame
//     // epd.update_color_frame(
//     //     &mut spi,
//     //     &mono_display.buffer(),
//     //     &chromatic_display.buffer()
//     // ).unwrap();
//     // epd.display_frame(&mut spi, &mut delay).unwrap();
//     mono_display.set_rotation(DisplayRotation::Rotate0);
//     draw_text(&mut mono_display, text, 5, 10);
//     epd.update_frame(&mut spi, mono_display.buffer(), &mut delay).unwrap();
//     epd
//         .display_frame(&mut spi, &mut delay)
//         .expect("display frame new graphics");

//     // Set the EPD to sleep
//     epd.sleep(&mut spi, &mut delay)?;
//     Ok(())
// }

fn draw_text(display: &mut Display2in9bc, text: &str, x: i32, y: i32) {
    let style = MonoTextStyleBuilder::new()
        .font(&embedded_graphics::mono_font::ascii::FONT_6X10)
        .text_color(Black)
        .background_color(White)
        .build();

    let text_style = TextStyleBuilder::new().baseline(Baseline::Top).build();

    let _ = Text::with_text_style(text, Point::new(x, y), style, text_style).draw(display);
}
