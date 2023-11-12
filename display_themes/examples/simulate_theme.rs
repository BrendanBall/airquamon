use airquamon_domain::Data;
use clap::Parser;
use display_themes::Theme;
#[cfg(feature = "theme1")]
use display_themes::Theme1 as ThemeImpl;
#[cfg(feature = "theme2")]
use display_themes::Theme2 as ThemeImpl;
#[cfg(feature = "theme3")]
use display_themes::Theme3 as ThemeImpl;
use embedded_graphics::prelude::*;
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};
use epd_waveshare::color::TriColor;

/// Simulate theme 2
#[derive(Parser, Debug)]
#[command(long_about = None)]
struct Args {
    /// CO2 in ppm
    #[arg(short, long, default_value_t = 400)]
    co2: u16,

    /// Temperature in °C
    #[arg(short, long, default_value_t = 23.5)]
    temperature: f32,

    /// Relative humidity in %
    #[arg(short = 'r', long, default_value_t = 60.5)]
    humidity: f32,
}

fn main() -> Result<(), core::convert::Infallible> {
    let args = Args::parse();

    let mut display = SimulatorDisplay::<TriColor>::new(Size::new(296, 128));

    let data = Data {
        co2: args.co2,
        temperature: args.temperature,
        humidity: args.humidity,
    };

    let mut theme = ThemeImpl::new();
    theme.draw(&data, &mut display)?;

    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    Window::new("Airquamon Simulator", &output_settings).show_static(&display);

    Ok(())
}
