#![no_std]

use airquamon_domain::Data;
use core::fmt;
use display_themes::Theme;
use embedded_graphics::prelude::*;
use embedded_hal::delay::DelayUs;
use embedded_hal::spi::SpiDevice;
use epd_waveshare::{graphics, prelude::*};
use log::info;

pub trait ChromaticBuffer {
    fn bw_buffer(&self) -> &[u8];
    fn chromatic_buffer(&self) -> &[u8];
}

impl<const WIDTH: u32, const HEIGHT: u32, const BWRBIT: bool, const BYTECOUNT: usize>
    ChromaticBuffer for graphics::Display<WIDTH, HEIGHT, BWRBIT, BYTECOUNT, TriColor>
{
    fn bw_buffer(&self) -> &[u8] {
        self.bw_buffer()
    }

    fn chromatic_buffer(&self) -> &[u8] {
        self.chromatic_buffer()
    }
}

pub struct Display<SPI, EPD, DRAWTARGET, DELAY, THEME>
where
    SPI: SpiDevice,
    EPD: WaveshareThreeColorDisplayV2<SPI, DELAY>,
    DRAWTARGET: DrawTarget<Color = TriColor> + ChromaticBuffer,
    DRAWTARGET::Error: fmt::Debug,
    DELAY: DelayUs,
    THEME: Theme<TriColor>,
{
    spi: SPI,
    epd: EPD,
    draw_target: DRAWTARGET,
    delay: DELAY,
    theme: THEME,
}

impl<SPI, EPD, DRAWTARGET, DELAY, THEME> Display<SPI, EPD, DRAWTARGET, DELAY, THEME>
where
    SPI: SpiDevice,
    EPD: WaveshareThreeColorDisplayV2<SPI, DELAY>,
    DRAWTARGET: DrawTarget<Color = TriColor> + ChromaticBuffer,
    DRAWTARGET::Error: fmt::Debug,
    DELAY: DelayUs,
    THEME: Theme<TriColor>,
{
    pub fn new(spi: SPI, epd: EPD, draw_target: DRAWTARGET, delay: DELAY, theme: THEME) -> Self {
        Self {
            spi,
            draw_target,
            epd,
            delay,
            theme,
        }
    }
}

pub trait DisplayTheme {
    type Error;

    fn draw(&mut self, data: &Data) -> Result<(), Self::Error>;
}

impl<SPI, EPD, DRAWTARGET, DELAY, THEME> DisplayTheme
    for Display<SPI, EPD, DRAWTARGET, DELAY, THEME>
where
    SPI: SpiDevice,
    EPD: WaveshareThreeColorDisplayV2<SPI, DELAY>,
    SPI: SpiDevice,
    DRAWTARGET: DrawTarget<Color = TriColor> + ChromaticBuffer + OriginDimensions,
    DRAWTARGET::Error: fmt::Debug,
    DELAY: DelayUs,
    THEME: Theme<TriColor>,
{
    type Error = SPI::Error;

    fn draw(&mut self, data: &Data) -> Result<(), Self::Error> {
        let _ = self.theme.draw(data, &mut self.draw_target);
        draw_to_epd(
            &mut self.spi,
            &mut self.epd,
            &mut self.draw_target,
            &mut self.delay,
        )?;
        Ok(())
    }
}

fn draw_to_epd<'a, SPI, EPD, BUFFER, DELAY>(
    spi: &mut SPI,
    epd: &mut EPD,
    buffer: &mut BUFFER,
    delay: &mut DELAY,
) -> Result<(), SPI::Error>
where
    SPI: SpiDevice,
    EPD: WaveshareThreeColorDisplayV2<SPI, DELAY>,
    BUFFER: ChromaticBuffer,
    DELAY: DelayUs,
{
    info!("waking up display");
    epd.wake_up(spi, delay)?;

    epd.wait_until_idle(spi, delay)?;

    info!("updating display frame");
    epd.update_color_frame(spi, delay, buffer.bw_buffer(), buffer.chromatic_buffer())?;
    epd.display_frame(spi, delay)?;

    // Set the EPD to sleep
    epd.sleep(spi, delay)?;
    Ok(())
}
