#![no_std]

#[derive(Copy, Clone)]
pub struct Data {
    pub co2: u16,
    pub temperature: f32,
    pub humidity: f32,
}
