#![no_std]
#![no_main]

use esp_backtrace as _;
use hal::{
    clock::ClockControl, 
    peripherals::Peripherals, 
    spi::{Spi, SpiMode},
    gpio::IO,
    prelude::*, 
    Delay,
};
use epd_waveshare::{epd2in9bc::*, prelude::*};
use log::info;

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

    delay.delay_ms(100u16);

    epd.clear_frame(&mut spi, &mut delay).expect("Failed updating frame");
    epd
    .display_frame(&mut spi, &mut delay).expect("Failed displaying frame");

    delay.delay_ms(100u16);

    // Set the EPD to sleep
    epd.sleep(&mut spi, &mut delay).expect("Failed sleeping epd");

    loop {
        delay.delay_ms(15000u16);

        info!("Waiting");
  
    }
}
