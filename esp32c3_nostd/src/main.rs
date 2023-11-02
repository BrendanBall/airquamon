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
use embedded_hal::delay::DelayUs;
use scd4x::scd4x::Scd4x;
use epd_waveshare::{epd2in9b_v3::{Display2in9b, Epd2in9b}, graphics::DisplayRotation};
use log::info;
use airquamon_domain::Data;
use display_themes::Theme2;
use epd_display::{Display, DisplayTheme};

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

    let epd = Epd2in9b::new(
        &mut spi, 
        busy, 
        dc, 
        rst, 
        &mut delay,
        None
    ).expect("failing setting up epd");
   
    let mut draw_target = Display2in9b::default();
    draw_target.set_rotation(DisplayRotation::Rotate270);
   
    let mut display = Display::new(
        spi, 
        epd,
        draw_target,
        delay,
        Theme2::new()
    );

    sensor.wake_up();
    // sensor.set_automatic_self_calibration(true).expect("failed enabling sensor automatic self calibration");
    sensor.stop_periodic_measurement().unwrap();
    sensor.reinit().unwrap();

    let serial = sensor.serial_number().unwrap();
    info!("Serial: {:#04x}", serial);

    loop {
        // sensor.wake_up();
        info!("Starting periodic measurement");
        sensor.start_periodic_measurement().unwrap();
        DelayUs::delay_ms(&mut delay, 5000);

        info!("Waiting for data ready");
        loop {
            match sensor.data_ready_status() {
                Ok(true) => break,
                Ok(false) => DelayUs::delay_ms(&mut delay, 100),
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
        display.draw(&Data {
            co2: data.co2,
            temperature: data.temperature,
            humidity: data.humidity,
        }).expect("draw failed");

        info!("Sleeping");
        DelayUs::delay_ms(&mut delay, 60000);
    }
}
