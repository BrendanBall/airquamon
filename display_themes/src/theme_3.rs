use crate::Theme;
use airquamon_domain::Data;
use core::fmt;
use core::fmt::Write;
use embedded_graphics::{
    mono_font::{
        iso_8859_1::{FONT_10X20, FONT_6X10},
        MonoTextStyleBuilder,
    },
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
    text::{Alignment, Text},
};
use embedded_layout::{
    layout::linear::{spacing::FixedMargin, LinearLayout},
    prelude::*,
    View,
};
use epd_waveshare::color::TriColor;
use heapless::String;
use u8g2_fonts::{fonts, U8g2TextStyle};

pub struct Theme3;

impl Theme3 {
    pub fn new() -> Self {
        Self
    }
}

struct Value<T> {
    bounds: Rectangle,
    value: T,
}

impl<T> View for Value<T> {
    #[inline]
    fn translate_impl(&mut self, by: Point) {
        // make sure you don't accidentally call `translate`!
        View::translate_mut(&mut self.bounds, by);
    }

    #[inline]
    fn bounds(&self) -> Rectangle {
        self.bounds
    }
}

struct CO2(u16);

impl Value<CO2> {
    fn new(value: CO2, position: Point, size: Size) -> Self {
        Self {
            bounds: Rectangle::new(position, size),
            value,
        }
    }
}

impl Drawable for Value<CO2> {
    type Color = TriColor;
    type Output = ();

    fn draw<D: DrawTarget<Color = TriColor>>(&self, display: &mut D) -> Result<(), D::Error> {
        // Create styles
        let border_style = PrimitiveStyle::with_stroke(TriColor::Black, 1);

        // Create a 1px border
        let border = self.bounds.into_styled(border_style);
        border.draw(display)?;

        let level_color = if self.value.0 > 800 {
            TriColor::Chromatic
        } else {
            TriColor::Black
        };

        let value_text_style = MonoTextStyleBuilder::new()
            .font(&FONT_10X20)
            .text_color(level_color)
            .background_color(TriColor::White)
            .build();

        let mut value_text: String<4> = String::new();
        write!(value_text, "{0}", self.value.0)
            .expect("Error occurred while trying to write in String");

        let text = Text::with_alignment(
            &value_text,
            Point::zero(),
            value_text_style,
            Alignment::Center,
        );

        let label_text_style = MonoTextStyleBuilder::new()
            .font(&FONT_6X10)
            .text_color(TriColor::Black)
            .background_color(TriColor::White)
            .build();
        let label_co2 =
            Text::with_alignment("CO2", Point::zero(), label_text_style, Alignment::Center);
        let label_ppm =
            Text::with_alignment("ppm", Point::zero(), label_text_style, Alignment::Center);

        let emoticons_character_style =
            U8g2TextStyle::new(fonts::u8g2_font_unifont_t_emoticons, level_color);

        let level_text = if self.value.0 > 800 {
            "\u{0055}"
        } else {
            "\u{0023}"
        };

        let label_level = Text::with_alignment(
            level_text,
            Point::zero(),
            emoticons_character_style,
            Alignment::Center,
        );

        LinearLayout::vertical(
            Chain::new(
                LinearLayout::horizontal(Chain::new(text).append(
                    LinearLayout::vertical(Chain::new(label_co2).append(label_ppm)).arrange(),
                ))
                .with_alignment(vertical::Center)
                .with_spacing(FixedMargin(4))
                .arrange()
                .align_to(&border, horizontal::Center, vertical::Center),
            )
            .append(label_level),
        )
        .with_alignment(horizontal::Center)
        .with_spacing(FixedMargin(4))
        .arrange()
        .align_to(&border, horizontal::Center, vertical::Center)
        .draw(display)?;

        Ok(())
    }
}

struct Temperature(f32);

impl Value<Temperature> {
    fn new(value: Temperature, position: Point, size: Size) -> Self {
        Self {
            bounds: Rectangle::new(position, size),
            value,
        }
    }
}

impl Drawable for Value<Temperature> {
    type Color = TriColor;
    type Output = ();

    fn draw<D: DrawTarget<Color = TriColor>>(&self, display: &mut D) -> Result<(), D::Error> {
        // Create styles
        let border_style = PrimitiveStyle::with_stroke(TriColor::Black, 1);

        // Create a 1px border
        let border = self.bounds.into_styled(border_style);
        border.draw(display)?;

        let value_text_style = MonoTextStyleBuilder::new()
            .font(&FONT_10X20)
            .text_color(TriColor::Black)
            .background_color(TriColor::White)
            .build();

        let mut value_text: String<4> = String::new();
        write!(value_text, "{0:#.1}", self.value.0)
            .expect("Error occurred while trying to write in String");

        let text = Text::with_alignment(
            &value_text,
            Point::zero(),
            value_text_style,
            Alignment::Center,
        );

        let label_text_style = MonoTextStyleBuilder::new()
            .font(&FONT_6X10)
            .text_color(TriColor::Black)
            .background_color(TriColor::White)
            .build();
        let label_degrees =
            Text::with_alignment("°C", Point::zero(), label_text_style, Alignment::Center);

        LinearLayout::horizontal(
            Chain::new(text).append(LinearLayout::vertical(Chain::new(label_degrees)).arrange()),
        )
        .with_alignment(vertical::Center)
        .with_spacing(FixedMargin(4))
        .arrange()
        .align_to(&border, horizontal::Center, vertical::Center)
        .draw(display)?;

        Ok(())
    }
}

struct Humidity(f32);

impl Value<Humidity> {
    fn new(value: Humidity, position: Point, size: Size) -> Self {
        Self {
            bounds: Rectangle::new(position, size),
            value,
        }
    }
}

impl Drawable for Value<Humidity> {
    type Color = TriColor;
    type Output = ();

    fn draw<D: DrawTarget<Color = TriColor>>(&self, display: &mut D) -> Result<(), D::Error> {
        // Create styles
        let border_style = PrimitiveStyle::with_stroke(TriColor::Black, 1);

        // Create a 1px border
        let border = self.bounds.into_styled(border_style);
        border.draw(display)?;

        let value_text_style = MonoTextStyleBuilder::new()
            .font(&FONT_10X20)
            .text_color(TriColor::Black)
            .background_color(TriColor::White)
            .build();

        let mut value_text: String<4> = String::new();
        write!(value_text, "{0:#.1}", self.value.0)
            .expect("Error occurred while trying to write in String");

        let text = Text::with_alignment(
            &value_text,
            Point::zero(),
            value_text_style,
            Alignment::Center,
        );

        let label_text_style = MonoTextStyleBuilder::new()
            .font(&FONT_6X10)
            .text_color(TriColor::Black)
            .background_color(TriColor::White)
            .build();
        let label_rh =
            Text::with_alignment("RH", Point::zero(), label_text_style, Alignment::Center);
        let label_percentage =
            Text::with_alignment("%", Point::zero(), label_text_style, Alignment::Center);

        LinearLayout::horizontal(Chain::new(text).append(
            LinearLayout::vertical(Chain::new(label_rh).append(label_percentage)).arrange(),
        ))
        .with_alignment(vertical::Center)
        .with_spacing(FixedMargin(4))
        .arrange()
        .align_to(&border, horizontal::Center, vertical::Center)
        .draw(display)?;

        Ok(())
    }
}

impl Theme<TriColor> for Theme3 {
    fn draw<DRAWTARGET>(
        &mut self,
        data: &Data,
        display: &mut DRAWTARGET,
    ) -> Result<(), DRAWTARGET::Error>
    where
        DRAWTARGET: DrawTarget<Color = TriColor> + OriginDimensions,
        DRAWTARGET::Error: fmt::Debug,
    {
        display.clear(TriColor::White)?;

        let display_area = display.bounding_box();

        let box_size = Size::new(80, 80);

        let co2 = Value::<CO2>::new(CO2(data.co2), Point::zero(), box_size);
        let temperature =
            Value::<Temperature>::new(Temperature(data.temperature), Point::zero(), box_size);
        let humidity = Value::<Humidity>::new(Humidity(data.humidity), Point::zero(), box_size);

        LinearLayout::horizontal(Chain::new(co2).append(temperature).append(humidity))
            // .with_spacing(FixedMargin(4))
            .arrange()
            .align_to(&display_area, horizontal::Center, vertical::Center)
            .draw(display)?;

        Ok(())
    }
}
