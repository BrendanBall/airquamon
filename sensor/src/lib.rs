#![no_std]

use airquamon_domain::Data;

mod scd4x_sensor;
pub use scd4x_sensor::Scd4xSensor;

pub trait Sensor {
    type Error;

    fn measure(&mut self) -> Result<Data, Self::Error>;
}
