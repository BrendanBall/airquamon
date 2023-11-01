use crate::Theme;
use airquamon_domain::Data;
use embedded_graphics::{
    mono_font::{MonoTextStyleBuilder, iso_8859_1::{FONT_6X10, FONT_10X20}},
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
    text::{Text, Alignment},
};
use embedded_layout::{
    layout::linear::{spacing::FixedMargin, LinearLayout},
    View,
    prelude::*
};
use epd_waveshare::color::TriColor;
use heapless::String;
use core::fmt::Write;
use core::fmt;

pub struct Theme2;

impl Theme2 {
    pub fn new() -> Self {
        Theme2
    }
}

struct Value<T, const TEXTSIZE: usize> {
    value_text: String<TEXTSIZE>,
    bounds: Rectangle,
    value: T,
}


impl<T, const TEXTSIZE: usize> View for Value<T, TEXTSIZE> {
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

impl Value<CO2, 4> {
    fn new(value: CO2, position: Point, size: Size) -> Self {
        let mut value_text: String<4> = String::new();
        write!(value_text, "{0}", value.0).expect("Error occurred while trying to write in String");
        Self {
            bounds: Rectangle::new(position, size),
            value_text,
            value,
        }
    }
}

impl<const TEXTSIZE: usize> Drawable for Value<CO2, TEXTSIZE> {
    type Color = TriColor;
    type Output = ();

    fn draw<D: DrawTarget<Color = TriColor>>(&self, display: &mut D) -> Result<(), D::Error> {
        // Create styles
        let border_style = PrimitiveStyle::with_stroke(TriColor::Black, 1);

        // Create a 1px border
        let border = self.bounds.into_styled(border_style);
        border.draw(display)?;

        let value_text_color = if self.value.0 > 800 {
            TriColor::Chromatic
        } else {
            TriColor::Black
        };
       
        let value_text_style = MonoTextStyleBuilder::new()
            .font(&FONT_10X20)
            .text_color(value_text_color)
            .background_color(TriColor::White)
            .build();
        let text = Text::with_alignment(&self.value_text, Point::zero(), value_text_style, Alignment::Center);

        let label_text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(TriColor::Black)
        .background_color(TriColor::White)
        .build();
        let label_co2 = Text::with_alignment("CO2", Point::zero(), label_text_style, Alignment::Center);
        let label_ppm = Text::with_alignment("ppm", Point::zero(), label_text_style, Alignment::Center);
    

        LinearLayout::horizontal(
            Chain::new(text)
                .append(LinearLayout::vertical(Chain::new(label_co2).append(label_ppm)).arrange())
        )
        .with_alignment(vertical::Center)
        .with_spacing(FixedMargin(4))
        .arrange()
        .align_to(&border, horizontal::Center, vertical::Center)
        .draw(display)?;

        Ok(())
    }
}

struct Temperature(f32);

impl Value<Temperature, 4> {
    fn new(value: Temperature, position: Point, size: Size) -> Self {
        let mut value_text: String<4> = String::new();
        write!(value_text, "{0:#.1}", value.0).expect("Error occurred while trying to write in String");
        Self {
            bounds: Rectangle::new(position, size),
            value_text,
            value,
        }
    }
}

impl<const TEXTSIZE: usize> Drawable for Value<Temperature, TEXTSIZE> {
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

        let text = Text::with_alignment(&self.value_text, Point::zero(), value_text_style, Alignment::Center);

        let label_text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(TriColor::Black)
        .background_color(TriColor::White)
        .build();
        let label_degrees = Text::with_alignment("°C", Point::zero(), label_text_style, Alignment::Center);
    

        LinearLayout::horizontal(
            Chain::new(text)
                .append(LinearLayout::vertical(Chain::new(label_degrees)).arrange())
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

impl Value<Humidity, 4> {
    fn new(value: Humidity, position: Point, size: Size) -> Self {
        let mut value_text: String<4> = String::new();
        write!(value_text, "{0:#.1}", value.0).expect("Error occurred while trying to write in String");
        Self {
            bounds: Rectangle::new(position, size),
            value_text,
            value,
        }
    }
}

impl<const TEXTSIZE: usize> Drawable for Value<Humidity, TEXTSIZE> {
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

        let text = Text::with_alignment(&self.value_text, Point::zero(), value_text_style, Alignment::Center);

        let label_text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(TriColor::Black)
        .background_color(TriColor::White)
        .build();
        let label_rh = Text::with_alignment("RH", Point::zero(), label_text_style, Alignment::Center);
        let label_percentage = Text::with_alignment("%", Point::zero(), label_text_style, Alignment::Center);
    

        LinearLayout::horizontal(
            Chain::new(text)
                .append(LinearLayout::vertical(Chain::new(label_rh).append(label_percentage)).arrange())
        )
        .with_alignment(vertical::Center)
        .with_spacing(FixedMargin(4))
        .arrange()
        .align_to(&border, horizontal::Center, vertical::Center)
        .draw(display)?;

        Ok(())
    }
}

impl Theme<TriColor> for Theme2
{
    fn draw<DRAWTARGET>(&mut self, data: &Data, display: &mut DRAWTARGET) -> Result<(), DRAWTARGET::Error> 
    where 
    DRAWTARGET: DrawTarget<Color = TriColor> + OriginDimensions,
    DRAWTARGET::Error: fmt::Debug
    {
        display.clear(TriColor::White)?;

        let display_area = display.bounding_box();

        let box_size = Size::new(80, 80); 

        let co2 = Value::<CO2, 4>::new(CO2(data.co2), Point::zero(), box_size);
        let temperature = Value::<Temperature, 4>::new(Temperature(data.temperature), Point::zero(), box_size);
        let humidity = Value::<Humidity, 4>::new(Humidity(data.humidity), Point::zero(), box_size);

        LinearLayout::horizontal(
            Chain::new(co2)
                .append(temperature)
                .append(humidity)
        )
        // .with_spacing(FixedMargin(4))
        .arrange()
        .align_to(&display_area, horizontal::Center, vertical::Center)
        .draw(display)?;
    
        Ok(())
    }
}
