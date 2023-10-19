#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_println::println;
use hal::{
    clock::ClockControl, 
    peripherals::Peripherals, 
    spi::{Spi, SpiMode, FullDuplexMode},
    gpio::IO,
    prelude::*, 
    Delay,
};
use embedded_hal::blocking::spi::Write;
use embedded_hal::digital::v2::{OutputPin, InputPin};
use embedded_graphics::{
    mono_font::MonoTextStyleBuilder,
    prelude::*,
    primitives::{Circle, Line, PrimitiveStyle},
    text::{Baseline, Text, TextStyleBuilder},
};
use epd_waveshare::{epd2in9bc::*, prelude::*, color::*};
use heapless::String;
use core::fmt::{self, Write as FmtWrite};
use log::{info, error};

enum Error<T: Write<u8>> {
    EpdError(<T as Write<u8>>::Error)
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
    let mut epd = Epd2in9bc::new(
        &mut spi, 
        cs, 
        busy, 
        dc, 
        rst, 
        &mut delay
    ).expect("failing setting up epd");

    info!("epd is busy: {}", epd.is_busy());
    for i in 0..10 {
        info!("epd is busy: {}", epd.is_busy());
        delay.delay_ms(1000u16);
    }

    let mut mono_display = Display2in9bc::default();

    mono_display.set_rotation(DisplayRotation::Rotate90);
    draw_text(&mut mono_display, "Hello", 5, 10);

    // epd.wake_up(&mut spi, &mut delay).expect("Failed waking up epd");

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
        delay.delay_ms(15000u16);

        let mut mono_display = Display2in9bc::default();

        mono_display.set_rotation(DisplayRotation::Rotate270);
        draw_text(&mut mono_display, counter_text.as_str(), 5, 10);
    
        info!("waking up display");
        epd.wake_up(&mut spi, &mut delay).expect("Failed waking up epd");
        
        info!("epd is busy: {}", epd.is_busy());
        loop {
            if !epd.is_busy() {
                break;
            }
            info!("epd is still busy");
            delay.delay_ms(1000u16);
        }

        info!("updating display frame");
        epd.update_frame(&mut spi, mono_display.buffer(), &mut delay).expect("Failed updating frame");
        epd
            .display_frame(&mut spi, &mut delay).expect("Failed displaying frame");
    
        // Set the EPD to sleep
        epd.sleep(&mut spi, &mut delay).expect("Failed sleeping epd");
        counter += 1;
    }
}


fn draw_text(display: &mut Display2in9bc, text: &str, x: i32, y: i32) {
    let style = MonoTextStyleBuilder::new()
        .font(&embedded_graphics::mono_font::ascii::FONT_9X15_BOLD)
        .text_color(Black)
        .background_color(White)
        .build();

    let text_style = TextStyleBuilder::new().baseline(Baseline::Top).build();

    let _ = Text::with_text_style(text, Point::new(x, y), style, text_style).draw(display);
}