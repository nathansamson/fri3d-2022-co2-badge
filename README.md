# Fri3d 2022 CO2 Badge

## Hardware

* [Fri3d 2022 badge](https://github.com/Fri3dCamp/badge-2020)

With the help of P5 and P8 SMD headers [61000413321, 61000513321]( https://www.we-online.com/catalog/en/PHD_2_54_SMT_SOCKET_HEADER_6100XXXXX21) and a 10µF 0805 capacitor, you can add the [MH-Z19C sensor]( https://www.reichelt.com/be/nl/infrarood-co2-sensor-mh-z19c-pin-header-rm-2-54-co2-mh-z19c-ph-p297320.html). Alternatively, there is also room for the [Molex 
53261-0771](https://www.molex.com/molex/products/part-detail/pcb_headers/0532610771) SMD connector on P9 so you can use the [wire mounted variant of the MH-Z19C]( https://www.tinytronics.nl/shop/en/sensors/air/gas/winsen-mh-z19c-co2-sensor-with-cable)

Note that this sensor will only work on 5V so USB power is required. It connects the CO2's serial port to IO26 and IO15 of the ESP32.

## Required Software

To build the software to flash you need to install the esp32 rust compiler. Up to date instructions can be found in the [Rust on ESP32 book](https://esp-rs.github.io/book/dependencies/index.html).

## Build & Flsah

* `cargo build` (the first time this will take a while)
* `espflash  /dev/ttyUSB0 target/xtensa-esp32-espidf/debug/fri3d-2022-co2-badge` (the `espflash` utility should be installed with the required dependencies)

## Optional features

This program cam be built with optional features.
You can customize your own build to create the perfect use-case for you.

To build with extra features change the build command from `cargo build` to `cargo build --features "X Y"`

A list of implemented features

* `screen`: Displays the current Co2 values on the badges display. Probably increases power usage dramatically.
   If the screen switch (the one on the right) is set to auto the screen's backlight can be disabled if you want to save power.
   Just touch the 0 touch with your fingers for a couple of seconds and the screen will switch off (or on again).

   Note: I assume this saves energy but I am not sure. Even if it doesn't it can help reduce light output in darker rooms.
* `alarm`: Plays a tone over the "speaker" when the CO02 values go over a healthy value. Not recommended for home use as it is rather annoying. Serves more as an example how to use the buzzer on the badge. 
* `leds`: Displays the status (Good, average & bad) with 3 colors (Green, yellow, red) on the 5 LEDs of the badge.