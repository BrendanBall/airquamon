#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_println::println;
use hal::{
    clock::ClockControl, 
    peripherals::Peripherals, 
    i2c::I2C,
    gpio::IO,
    prelude::*, 
    Delay,
};
use scd4x::scd4x::Scd4x;
use log::info;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let mut system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);

    // setup logger
    // To change the log_level change the env section in .config/cargo.toml
    // or remove it and set ESP_LOGLEVEL manually before running cargo run
    // this requires a clean rebuild because of https://github.com/rust-lang/cargo/issues/10358
    esp_println::logger::init_logger_from_env();
    info!("Logger is setup");
    println!("Hello world!");

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let i2c_scl = io.pins.gpio0;
    let i2c_sda = io.pins.gpio1;

    let i2c = I2C::new(
        peripherals.I2C0,
        i2c_sda,
        i2c_scl,
        100u32.kHz(),
        &mut system.peripheral_clock_control,
        &clocks,
    );

    info!("Connecting to sensor");
    let mut sensor = Scd4x::new(i2c, delay);


    sensor.wake_up();
    sensor.stop_periodic_measurement().unwrap();
    sensor.reinit().unwrap();

    let serial = sensor.serial_number().unwrap();
    info!("Serial: {:#04x}", serial);

    info!("Starting periodic measurement");
    sensor.start_periodic_measurement().unwrap();

    info!("Waiting for first measurement... (5 sec)");

    loop {
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

        println!(
            "CO2: {0}, Temperature: {1:#.2} °C, Humidity: {2:#.2} RH",
            data.co2, data.temperature, data.humidity
        );
    }
}
