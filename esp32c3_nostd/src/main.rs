#![no_std]
#![no_main]

use esp_backtrace as _;
use esp32c3_hal::{
    clock::ClockControl, 
    peripherals::Peripherals, 
    i2c::I2C,
    spi::{
        master::{Spi, SpiBusController},
        SpiMode,
    },
    gpio::IO,
    prelude::*, 
    Delay,
};
use scd4x::scd4x::Scd4x;
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
    let i2c_scl = io.pins.gpio0;
    let i2c_sda = io.pins.gpio1;

    let i2c = I2C::new(
        peripherals.I2C0,
        i2c_sda,
        i2c_scl,
        100u32.kHz(),
        &clocks,
    );

    info!("Connecting to sensor");
    let mut sensor = Scd4x::new(i2c, delay);

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

    epd.sleep(&mut spi, &mut delay).expect("Failed sleeping epd");


    sensor.wake_up();
    // sensor.set_automatic_self_calibration(true).expect("failed enabling sensor automatic self calibration");
    sensor.stop_periodic_measurement().unwrap();
    sensor.reinit().unwrap();

    let serial = sensor.serial_number().unwrap();
    info!("Serial: {:#04x}", serial);

    // info!("Starting periodic measurement");
    // sensor.start_periodic_measurement().unwrap();

    // info!("Waiting for first measurement... (5 sec)");
    // delay.delay_ms(5000u16);

    let mut display_text: String<20> = String::new();


    loop {
        // sensor.wake_up();
        info!("Starting periodic measurement");
        sensor.start_periodic_measurement().unwrap();
        delay.delay_ms(5000u16);

        info!("Waiting for data ready");
        loop {
            match sensor.data_ready_status() {
                Ok(true) => break,
                Ok(false) => delay.delay_ms(100u16),
                Err(e) => {
                    panic!("Failed to poll for data ready: {:?}", e);
                },
            }
        }

        info!("Reading sensor data");
        let data = match sensor.measurement() {
            Ok(v) => v,
            Err(e) => {
                panic!("Failed to read measurement: {:?}", e);
            },
        };

        info!(
            "CO2: {0}, Temperature: {1:#.2} °C, Humidity: {2:#.2} RH",
            data.co2, data.temperature, data.humidity
        );

        info!("Stop sensor periodic measurement");
        // sensor.power_down().expect("failed powering down sensor");
        sensor.stop_periodic_measurement().expect("failed to stop sensor periodic measurement");


        info!("updating display");
        display_text.clear();
        write!(display_text, "C02: {}", data.co2).expect("Error occurred while trying to write in String");
        let mut mono_display = Display2in9b::default();

        mono_display.set_rotation(DisplayRotation::Rotate270);
        draw_text(&mut mono_display, &display_text.as_str(), 5, 10);
    
        info!("waking up display");
        epd.wake_up(&mut spi, &mut delay).expect("Failed waking up epd");
        
        epd.wait_until_idle(&mut spi, &mut delay).expect("Failed waiting until idle");


        info!("updating display frame");
        epd.update_frame(&mut spi, mono_display.buffer(), &mut delay).expect("Failed updating frame");
        epd
            .display_frame(&mut spi, &mut delay).expect("Failed displaying frame");
    
        // Set the EPD to sleep
        epd.sleep(&mut spi, &mut delay).expect("Failed sleeping epd");
        info!("Sleeping");
        delay.delay_ms(30000u16);
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