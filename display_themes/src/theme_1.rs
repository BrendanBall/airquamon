use crate::Theme;
use airquamon_domain::Data;
use core::fmt;
use core::fmt::Write;
use embedded_graphics::{
    mono_font::{iso_8859_1::FONT_8X13_BOLD, MonoTextStyleBuilder},
    prelude::*,
    primitives::{Line, PrimitiveStyle},
    text::{Baseline, Text, TextStyleBuilder},
};
use epd_waveshare::color::TriColor;
use heapless::String;

pub struct Theme1 {
    display_text: String<60>,
}

impl Theme1 {
    pub fn new() -> Self {
        Theme1 {
            display_text: String::new(),
        }
    }
}

impl Theme<TriColor> for Theme1 {
    fn draw<DRAWTARGET>(
        &mut self,
        data: &Data,
        display: &mut DRAWTARGET,
    ) -> Result<(), DRAWTARGET::Error>
    where
        DRAWTARGET: DrawTarget<Color = TriColor> + OriginDimensions,
        DRAWTARGET::Error: fmt::Debug,
    {
        self.display_text.clear();
        write!(
            self.display_text,
            "CO2: {0} ppm | {1:#.2} °C | {2:#.2} %",
            data.co2, data.temperature, data.humidity
        )
        .expect("Error occurred while trying to write in String");
        let _ = Line::new(
            Point::new(5, display.size().height as i32 / 2),
            Point::new(
                display.size().width as i32 - 5,
                display.size().height as i32 / 2,
            ),
        )
        .into_styled(PrimitiveStyle::with_stroke(TriColor::Chromatic, 4))
        .draw(display);
        draw_text(display, &self.display_text, 5, 10)?;
        Ok(())
    }
}

fn draw_text<DRAWTARGET>(
    display: &mut DRAWTARGET,
    text: &str,
    x: i32,
    y: i32,
) -> Result<(), DRAWTARGET::Error>
where
    DRAWTARGET: DrawTarget<Color = TriColor>,
{
    let style = MonoTextStyleBuilder::new()
        .font(&FONT_8X13_BOLD)
        .text_color(TriColor::Black)
        .background_color(TriColor::White)
        .build();

    let text_style = TextStyleBuilder::new().baseline(Baseline::Top).build();

    Text::with_text_style(text, Point::new(x, y), style, text_style).draw(display)?;
    Ok(())
}
