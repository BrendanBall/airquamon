use crate::Sensor;
use airquamon_domain::Data;
use embedded_hal::{delay::DelayUs, i2c::I2c};
use scd4x::{Error, Scd4x};

pub struct MockSensor {
    data: Data,
}

impl MockSensor {
    pub fn new(co2: u16, temperature: f32, humidity: f32) -> Self {
        Self {
            data: Data {
                co2,
                temperature,
                humidity,
            },
        }
    }
}

impl Sensor for MockSensor {
    type Error = ();

    fn measure(&mut self) -> Result<Data, Self::Error> {
        Ok(self.data)
    }
}
