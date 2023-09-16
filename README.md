# Airquamon
Monitor air quality, starting with C02.

## Hardware
- [Sensirion SCD41](https://sensirion.com/products/catalog/SCD41)
- [ESP32-C3-DevKitC-02](https://docs.espressif.com/projects/esp-idf/en/latest/esp32c3/hw-reference/esp32c3/user-guide-devkitc-02.html)

## ESP32-C3-DevKitC-02
I2C can be used on any GPIO pins.
Currently using the following for Sensirion SCD41:
- I2C SCL (yellow) -> GPIO0
- I2C SDA (green) -> GPIO1
- GND (black) -> GND
- VDD (red) -> 3.3 V

## Resources
- https://github.com/Sensirion/arduino-ble-gadget/blob/master/documents/SCD4x_BLE_Gadget_Tutorial.md
- https://www.espressif.com/sites/default/files/documentation/esp32-c3_datasheet_en.pdf
- https://docs.rust-embedded.org/discovery/microbit/08-i2c/index.html
- https://docs.rs/esp32c3-hal/latest/esp32c3_hal/i2c/index.html

## Acknowledgements
- hauju for releasing https://github.com/hauju/scd4x-rs
- Everyone else involved in open source embedded Rust