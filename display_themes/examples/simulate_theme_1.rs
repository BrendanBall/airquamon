use airquamon_domain::Data;
use display_themes::{Theme, Theme1};
use embedded_graphics::prelude::*;
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};
use epd_waveshare::color::TriColor;

fn main() -> Result<(), core::convert::Infallible> {
    let mut display = SimulatorDisplay::<TriColor>::new(Size::new(296, 128));

    let data = Data {
        co2: 459,
        temperature: 20.59,
        humidity: 57.42,
    };

    let mut theme1 = Theme1::new();
    theme1.draw(&data, &mut display).unwrap();

    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    Window::new("Airquamon Simulator", &output_settings).show_static(&display);

    Ok(())
}
