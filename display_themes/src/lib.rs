#![no_std]

use airquamon_domain::Data;
use core::fmt;
use embedded_graphics::prelude::*;

mod theme_1;
pub use theme_1::Theme1;

mod theme_2;
pub use theme_2::Theme2;

mod theme_3;
pub use theme_3::Theme3;

pub trait Theme<COLOR>
where
    COLOR: PixelColor,
{
    fn draw<DRAWTARGET>(
        &mut self,
        data: &Data,
        display: &mut DRAWTARGET,
    ) -> Result<(), DRAWTARGET::Error>
    where
        DRAWTARGET: DrawTarget<Color = COLOR> + OriginDimensions,
        DRAWTARGET::Error: fmt::Debug;
}
