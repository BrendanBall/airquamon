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
use epd_waveshare::{epd2in9b_v3::*, prelude::*};
use log::info;


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


    epd.wait_until_idle(&mut spi, &mut delay).expect("Failed waiting until idle");

    epd.clear_frame(&mut spi, &mut delay).expect("Failed clearing frame");
    epd
        .display_frame(&mut spi, &mut delay).expect("Failed displaying frame");

    // Set the EPD to sleep
    epd.sleep(&mut spi, &mut delay).expect("Failed sleeping epd");

    loop {
        info!("Sleeping");
        delay.delay_ms(30000u16);
    }
}
