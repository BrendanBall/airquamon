#![no_std]
#![no_main]

use core::cell::RefCell;
use critical_section::Mutex;
use display_themes::Theme2;
use epd_display::{Display, DisplayTheme};
use epd_waveshare::{
    epd2in9b_v3::{Display2in9b, Epd2in9b},
    graphics::DisplayRotation,
};
use esp32c3_hal::{
    clock::ClockControl,
    gpio::{Event, Gpio3, Gpio9, Input, PullDown, PullUp, IO},
    i2c::I2C,
    interrupt,
    peripherals::{self, Peripherals},
    prelude::*,
    riscv,
    spi::{
        master::{Spi, SpiBusController},
        SpiMode,
    },
    Delay,
};
use esp_backtrace as _;
use log::info;
use sensor::{MockSensor, Scd4xSensor, Sensor};

static BOOT_BUTTON: Mutex<RefCell<Option<Gpio9<Input<PullDown>>>>> = Mutex::new(RefCell::new(None));
static BUTTON: Mutex<RefCell<Option<Gpio3<Input<PullUp>>>>> = Mutex::new(RefCell::new(None));

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

    let i2c = I2C::new(peripherals.I2C0, i2c_sda, i2c_scl, 100u32.kHz(), &clocks);

    info!("Connecting to sensor");
    let mut sensor = Scd4xSensor::new(i2c, delay);
    // let mut sensor = MockSensor::new(500, 19f32, 69f32);

    let mosi = io.pins.gpio4;
    let sck = io.pins.gpio5;
    let cs = io.pins.gpio6.into_push_pull_output();
    let dc = io.pins.gpio7.into_push_pull_output();
    let rst = io.pins.gpio18.into_push_pull_output();
    let busy = io.pins.gpio19.into_pull_down_input();

    let mut boot_button = io.pins.gpio9.into_pull_down_input();
    boot_button.listen(Event::FallingEdge);

    let mut button = io.pins.gpio3.into_pull_up_input();
    button.listen(Event::RisingEdge);

    critical_section::with(|cs| BOOT_BUTTON.borrow_ref_mut(cs).replace(boot_button));
    critical_section::with(|cs| BUTTON.borrow_ref_mut(cs).replace(button));

    interrupt::enable(peripherals::Interrupt::GPIO, interrupt::Priority::Priority3).unwrap();

    unsafe {
        riscv::interrupt::enable();
    }

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

    let epd =
        Epd2in9b::new(&mut spi, busy, dc, rst, &mut delay, None).expect("failing setting up epd");

    let mut draw_target = Display2in9b::default();
    draw_target.set_rotation(DisplayRotation::Rotate270);

    let mut display = Display::new(spi, epd, draw_target, delay, Theme2::new());

    loop {
        let data = sensor.measure().expect("failed reading sensor");

        info!(
            "CO2: {0}, Temperature: {1:#.2} °C, Humidity: {2:#.2} RH",
            data.co2, data.temperature, data.humidity
        );

        info!("updating display");
        display.draw(&data).expect("draw failed");

        info!("Sleeping");
        delay.delay_ms(60000u32);
    }
}

#[interrupt]
fn GPIO() {
    critical_section::with(|cs| {
        info!("Button was pressed");
        BOOT_BUTTON
            .borrow_ref_mut(cs)
            .as_mut()
            .unwrap()
            .clear_interrupt();
        BUTTON
            .borrow_ref_mut(cs)
            .as_mut()
            .unwrap()
            .clear_interrupt();
    });
}
