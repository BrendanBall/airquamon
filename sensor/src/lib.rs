#![no_std]

use airquamon_domain::Data;

mod mock_sensor;
mod scd4x_sensor;
pub use mock_sensor::MockSensor;
pub use scd4x_sensor::Scd4xSensor;

pub trait Sensor {
    type Error;

    fn measure(&mut self) -> Result<Data, Self::Error>;
}
