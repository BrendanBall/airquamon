use crate::Sensor;
use airquamon_domain::Data;
use embedded_hal::{delay::DelayUs, i2c::I2c};
use scd4x::{Error, Scd4x};

pub struct Scd4xSensor<I2C, DELAY> {
    scd4x: Scd4x<I2C, DELAY>,
    delay: DELAY,
}

impl<I2C, DELAY> Scd4xSensor<I2C, DELAY>
where
    I2C: I2c,
    DELAY: DelayUs + Copy,
{
    pub fn new(i2c: I2C, delay: DELAY) -> Self {
        Self {
            scd4x: Scd4x::new(i2c, delay),
            delay,
        }
    }
}

impl<I2C, DELAY> Sensor for Scd4xSensor<I2C, DELAY>
where
    I2C: I2c,
    DELAY: DelayUs,
{
    type Error = Error<I2C::Error>;

    fn measure(&mut self) -> Result<Data, Self::Error> {
        self.scd4x.wake_up();
        self.scd4x.reinit()?;
        self.scd4x.start_periodic_measurement()?;
        self.delay.delay_ms(5000);
        loop {
            match self.scd4x.data_ready_status() {
                Ok(true) => break,
                Ok(false) => {
                    self.delay.delay_ms(100);
                    Ok(())
                }
                Err(e) => Err(e),
            }?;
        }
        let data = self.scd4x.measurement()?;
        self.scd4x.stop_periodic_measurement()?;
        Ok(Data {
            co2: data.co2,
            temperature: data.temperature,
            humidity: data.humidity,
        })
    }
}
