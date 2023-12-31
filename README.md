# Airquamon
Monitor air quality, starting with C02.
This project currently uses the ESP32-C3 MCU and is implemented in Rust (no_std).
It periodically reads the sensor data (CO2, temperature, humidity) and displays it on an e-ink display.
See [Display Themes](display_themes/README.md) for more info on the display themes.

## Building and Flashing
To build:

```sh
cd esp32c3_nostd
cargo build --release
```

To flash:
```sh
cd esp32c3_nostd
cargo run --release
```

## Hardware
- [Sensirion SCD41](https://sensirion.com/products/catalog/SCD41)
- [ESP32-C3-DevKitC-02](https://docs.espressif.com/projects/esp-idf/en/latest/esp32c3/hw-reference/esp32c3/user-guide-devkitc-02.html)
- [2.9inch E-Paper E-Ink Display Module (B)](https://www.waveshare.com/product/displays/e-paper/epaper-2/2.9inch-e-paper-module-b.htm)

## ESP32-C3-DevKitC-02
I2C and SPI2 can be used on any GPIO pins.

### Sensirion SCD41
Connections:
- I2C SCL (yellow) -> GPIO0
- I2C SDA (green) -> GPIO1
- GND (black) -> GND
- VDD (red) -> 3.3 V

### 2.9inch E-Paper E-Ink Display Module (B), 296×128, Red / Black / White, SPI
https://www.waveshare.com/product/displays/e-paper/epaper-2/2.9inch-e-paper-module-b.htm

- VCC:	3.3V/5V
- GND:	Ground
- DIN:	SPI MOSI pin
- CLK:	SPI SCK pin
- CS:	SPI chip selection, low active
- DC:	Data / Command selection (high for data, low for command)
- RST:	External reset, low active
- BUSY:	Busy status output, low active

Connections:
- VCC -> 3.3 V
- GND -> GND
- DIN (MOSI) -> GPIO4
- CLK (SCK) -> GPIO5
- CS -> GPIO6
- DC (MISO) -> GPIO7
- RST -> GPIO18
- BUSY -> GPIO19

## Resources
- https://github.com/Sensirion/arduino-ble-gadget/blob/master/documents/SCD4x_BLE_Gadget_Tutorial.md
- https://www.espressif.com/sites/default/files/documentation/esp32-c3_datasheet_en.pdf
- https://docs.rust-embedded.org/discovery/microbit/08-i2c/index.html
- https://docs.rs/esp32c3-hal/latest/esp32c3_hal/i2c/index.html
- https://www.allaboutcircuits.com/technical-articles/spi-serial-peripheral-interface
- https://github.com/waveshareteam/Pico_ePaper_Code
- https://projects.raspberrypi.org/en/projects/button-switch-scratch-pi/1
- https://microcontrollerslab.com/push-button-esp32-gpio-digital-input/

## Acknowledgements
- hauju for releasing https://github.com/hauju/scd4x-rs
- caemor for https://github.com/caemor/epd-waveshare
- Everyone else involved in open source embedded Rust