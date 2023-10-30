#![no_std]

use airquamon_domain::Data;
use embedded_graphics::prelude::*;

mod theme_1;
pub use theme_1::Theme1;

pub trait Theme<COLOR>
where 
COLOR: PixelColor,
{
    fn draw<DRAWTARGET: DrawTarget<Color = COLOR>>(&mut self, data: &Data, display: &mut DRAWTARGET) -> Result<(), DRAWTARGET::Error>;
}