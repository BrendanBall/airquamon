use embedded_graphics::prelude::*;
use embedded_graphics_simulator::{SimulatorDisplay, Window, OutputSettingsBuilder};
use epd_waveshare::color::TriColor;
use display_themes::{Theme, Theme2};
use airquamon_domain::Data;


fn main() -> Result<(), core::convert::Infallible> {
    let mut display = SimulatorDisplay::<TriColor>::new(Size::new(296, 128));

    let data = Data{
        co2: 900,
        temperature: 20.59,
        humidity: 57.42,
    };

    let mut theme = Theme2::new();
    theme.draw(&data, &mut display).unwrap();

    let output_settings = OutputSettingsBuilder::new()
        .scale(2)
        .build();
    Window::new("Airquamon Simulator", &output_settings).show_static(&display);

    Ok(())
}
