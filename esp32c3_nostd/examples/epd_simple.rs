#![no_std]
#![no_main]

use esp_backtrace as _;

use esp32c3_hal::{
    clock::ClockControl, 
    peripherals::Peripherals, 
    spi::{
        master::{Spi, SpiBusController},
        SpiMode,
    },
    gpio::IO,
    prelude::*, 
    Delay,
};
use embedded_graphics::{
    mono_font::MonoTextStyleBuilder,
    prelude::*,
    text::{Baseline, Text, TextStyleBuilder},
};
use epd_waveshare::{epd2in9b_v3::*, prelude::*, color::Color};
use heapless::String;
use core::fmt::{self, Write as FmtWrite};
use log::{info, error};


#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);

    // setup logger
    // To change the log_level change the env section in .config/cargo.toml
    // or remove it and set ESP_LOGLEVEL manually before running cargo run
    // this requires a clean rebuild because of https://github.com/rust-lang/cargo/issues/10358
    esp_println::logger::init_logger_from_env();
    info!("Logger is setup");

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mosi = io.pins.gpio4;
    let sck = io.pins.gpio5;
    let cs = io.pins.gpio6.into_push_pull_output();
    let dc = io.pins.gpio7.into_push_pull_output();
    let rst = io.pins.gpio18.into_push_pull_output();
    let busy = io.pins.gpio19.into_pull_down_input();

    let spi_controller = SpiBusController::from_spi(Spi::new_no_cs_no_miso(
        peripherals.SPI2,
        sck,
        mosi,
        100u32.kHz(),
        SpiMode::Mode0,
        &clocks,
    ));
    let mut spi = spi_controller.add_device(cs);

    info!("Connecting to display");

    // Setup EPD
    let mut epd = Epd2in9b::new(
        &mut spi, 
        busy, 
        dc, 
        rst, 
        &mut delay,
        None
    ).expect("failing setting up epd");


    let mut mono_display = Display2in9b::default();

    mono_display.set_rotation(DisplayRotation::Rotate270);
    draw_text(&mut mono_display, "Hello", 5, 10);

    // epd.wake_up(&mut spi, &mut delay).expect("Failed waking up epd");

    epd.wait_until_idle(&mut spi, &mut delay).expect("Failed waiting until idle");

    epd.update_frame(&mut spi, mono_display.buffer(), &mut delay).expect("Failed updating frame");
    epd
        .display_frame(&mut spi, &mut delay).expect("Failed displaying frame");

    // Set the EPD to sleep
    epd.sleep(&mut spi, &mut delay).expect("Failed sleeping epd");


    let mut counter = 0;
    let mut counter_text: String<20> = String::new();

    loop {
        counter_text.clear();
        // fmt::write(&mut counter_text, format_args!("Counter: {}", counter)).expect("Error occurred while trying to write in String");
        write!(counter_text, "Counter: {counter}").expect("Error occurred while trying to write in String");
        info!("Sleeping");
        delay.delay_ms(30000u16);

        let mut mono_display = Display2in9b::default();

        mono_display.set_rotation(DisplayRotation::Rotate270);
        draw_text(&mut mono_display, counter_text.as_str(), 5, 10);
    
        info!("waking up display");
        epd.wake_up(&mut spi, &mut delay).expect("Failed waking up epd");
        
        epd.wait_until_idle(&mut spi, &mut delay).expect("Failed waiting until idle");


        info!("updating display frame");
        epd.update_frame(&mut spi, mono_display.buffer(), &mut delay).expect("Failed updating frame");
        epd
            .display_frame(&mut spi, &mut delay).expect("Failed displaying frame");
    
        // Set the EPD to sleep
        epd.sleep(&mut spi, &mut delay).expect("Failed sleeping epd");
        counter += 1;
    }
}


fn draw_text(display: &mut Display2in9b, text: &str, x: i32, y: i32) {
    let style = MonoTextStyleBuilder::new()
        .font(&embedded_graphics::mono_font::ascii::FONT_8X13_BOLD)
        .text_color(Color::Black)
        .background_color(Color::White)
        .build();

    let text_style = TextStyleBuilder::new().baseline(Baseline::Top).build();

    let _ = Text::with_text_style(text, Point::new(x, y), style, text_style).draw(display);
}