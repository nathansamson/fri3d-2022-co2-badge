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