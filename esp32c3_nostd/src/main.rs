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
use embedded_hal::spi::SpiDevice;
use embedded_hal::delay::DelayUs;
use scd4x::scd4x::Scd4x;
use embedded_graphics::{
    mono_font::MonoTextStyleBuilder,
    prelude::*,
    primitives::{Line, PrimitiveStyle},
    text::{Baseline, Text, TextStyleBuilder},
};
use epd_waveshare::{epd2in9b_v3::*, prelude::*, color::{TriColor, ColorType}, graphics};
use heapless::String;
use core::fmt::{Write as FmtWrite};
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

    let epd = Epd2in9b::new(
        &mut spi, 
        busy, 
        dc, 
        rst, 
        &mut delay,
        None
    ).expect("failing setting up epd");
   
    let mut display = Display2in9b::default();
    display.set_rotation(DisplayRotation::Rotate270);
    display.clear(TriColor::White).expect("failed clearing display");
   
    let mut display = Display {
        spi: spi,
        draw_target: display,
        epd: epd,
        delay: delay,
        display_text: String::new(),
    };

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
        display.draw(Data {
            co2: data.co2,
            temperature: data.temperature,
            humidity: data.humidity,
        }).expect("draw failed");

        info!("Sleeping");
        DelayUs::delay_ms(&mut delay, 60000);
    }
}

struct Data {
    pub co2: u16,
    pub temperature: f32,
    pub humidity: f32,
}

trait Buffer {
    fn buffer(&self) -> &[u8];
}

trait ChromaticBuffer {
    fn bw_buffer(&self) -> &[u8];
    fn chromatic_buffer(&self) -> &[u8];
}

impl<
        const WIDTH: u32,
        const HEIGHT: u32,
        const BWRBIT: bool,
        const BYTECOUNT: usize,
        COLOR: ColorType,
    > Buffer for graphics::Display<WIDTH, HEIGHT, BWRBIT, BYTECOUNT, COLOR> {
    
    fn buffer(&self) -> &[u8] {
        self.buffer()
    }

}

impl<
        const WIDTH: u32,
        const HEIGHT: u32,
        const BWRBIT: bool,
        const BYTECOUNT: usize,
    > ChromaticBuffer for graphics::Display<WIDTH, HEIGHT, BWRBIT, BYTECOUNT, TriColor> {
    
    fn bw_buffer(&self) -> &[u8] {
        self.bw_buffer()
    }

    fn chromatic_buffer(&self) -> &[u8] {
        self.chromatic_buffer()
    }
}

struct Display<SPI, EPD, DRAWTARGET, DELAY>
    where 
    SPI: SpiDevice,
    EPD: WaveshareThreeColorDisplayV2<SPI, DELAY>,
    DRAWTARGET: DrawTarget<Color = TriColor> + ChromaticBuffer,
    DELAY: DelayUs

{
    spi: SPI,
    epd: EPD,
    draw_target: DRAWTARGET,
    delay: DELAY,
    display_text: String<60>,
}

trait DisplayTheme {

    type Error;

    fn draw(&mut self, data: Data) -> Result<(), Self::Error>;
    fn draw_text(&mut self, text: &str) -> Result<(), Self::Error>;
}

impl<SPI, EPD, DRAWTARGET, DELAY> DisplayTheme for Display<SPI, EPD, DRAWTARGET, DELAY> 
    where 
    SPI: SpiDevice,
    EPD: WaveshareThreeColorDisplayV2<SPI, DELAY>,
    SPI: SpiDevice,
    DRAWTARGET: DrawTarget<Color = TriColor> + ChromaticBuffer,
    DELAY: DelayUs
{
    type Error = SPI::Error;

    fn draw(&mut self, data: Data) -> Result<(), Self::Error> {
        self.display_text.clear();
        write!(self.display_text, "CO2: {0} ppm | {1:#.2} °C | {2:#.2} %", data.co2, data.temperature, data.humidity).expect("Error occurred while trying to write in String");
        let _ = Line::new(Point::new(5, 50), Point::new(291, 50))
        .into_styled(PrimitiveStyle::with_stroke(TriColor::Chromatic, 4))
        .draw(&mut self.draw_target);
        draw_text(&mut self.draw_target, &self.display_text, 5, 10);
        draw_to_epd(&mut self.spi, &mut self.epd, &mut self.draw_target, &mut self.delay)?;
        Ok(())
    }

    fn draw_text(&mut self, text: &str) -> Result<(), Self::Error> {
        draw_text(&mut self.draw_target, text, 5, 10);
        draw_to_epd(&mut self.spi, &mut self.epd, &mut self.draw_target, &mut self.delay)?;
        Ok(())
    }
}

fn draw_to_epd<'a, SPI, EPD, DRAWTARGET, DELAY>(spi: &mut SPI, epd: &mut EPD, draw_target: &mut DRAWTARGET, delay: &mut DELAY) -> Result<(), SPI::Error>
where 
    SPI: SpiDevice,
    EPD: WaveshareThreeColorDisplayV2<SPI, DELAY>,
    DRAWTARGET: DrawTarget<Color = TriColor> + ChromaticBuffer,
    DELAY: DelayUs {
    info!("waking up display");
    epd.wake_up(spi, delay)?;
    
    epd.wait_until_idle(spi, delay)?;


    info!("updating display frame");
    epd.update_color_frame(spi, delay, draw_target.bw_buffer(), draw_target.chromatic_buffer())?;
    epd.display_frame(spi, delay)?;

    // Set the EPD to sleep
    epd.sleep(spi, delay)?;
    Ok(())
}



fn draw_text<DRAWTARGET>(display: &mut DRAWTARGET, text: &str, x: i32, y: i32) 
where
    DRAWTARGET: DrawTarget<Color = TriColor> {
    let style = MonoTextStyleBuilder::new()
        .font(&embedded_graphics::mono_font::ascii::FONT_8X13_BOLD)
        .text_color(TriColor::Black)
        .background_color(TriColor::White)
        .build();

    let text_style = TextStyleBuilder::new().baseline(Baseline::Top).build();

    let _ = Text::with_text_style(text, Point::new(x, y), style, text_style).draw(display);
}