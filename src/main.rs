use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use embedded_hal_0_2::blocking::delay::DelayMs;

use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::serial;
use esp_idf_hal::prelude::*;

use nb::block;

use mh_z19c::MhZ19C;

#[cfg(feature = "alarm")]
mod alarm;

#[cfg(feature = "screen")]
mod screen;

pub enum CO2State {
    Good(u16),
    Average(u16),
    Bad(u16),
}

pub struct Environment {
    pub co2_state: CO2State,
}

fn main() {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;

    let config = serial::config::Config::default().baudrate(Hertz(9600));

    let serial: serial::Serial<serial::UART2, _, _> = serial::Serial::new(
        peripherals.uart2,
        serial::Pins {
            tx: pins.gpio26,
            rx: pins.gpio15,
            cts: None,
            rts: None,
        },
        config
    ).unwrap();

    #[cfg(feature = "alarm")]
    let mut my_alarm = alarm::Alarm::init(peripherals.ledc.timer0, peripherals.ledc.channel0, pins.gpio32);

    #[cfg(feature = "screen")]
    let mut display = screen::
        init_screen(peripherals.spi3, pins.gpio18, pins.gpio23, pins.gpio19, pins.gpio33.into_output().unwrap(), pins.gpio5.into_output().unwrap()).
        unwrap();

    let mut co2sensor = MhZ19C::new(serial);

    loop {
        let co2 = block!(co2sensor.read_co2_ppm()).unwrap();
        println!("CO2 value = {}ppm", co2);

        // Cutoff values taken from  https://www.euromate.com/group/nl/blogs/blog-heeft-de-co2-concentratie-invloed-op-de-overdracht-van-het-coronavirus/

        let co2_state = match co2 {
            co2 if co2 < 800 => CO2State::Good(co2),
            co2 if co2 < 1200 =>  CO2State::Average(co2),
            _ => CO2State::Bad(co2),
        };


        let environment = Environment {
            co2_state: co2_state
        };

        #[cfg(feature = "alarm")]
        {
            my_alarm.update_status(&environment);
        }

        #[cfg(feature = "screen")]
        {
            screen::update_screen(&mut display, &environment);
        }

        FreeRtos.delay_ms(5000 as u32);
    }
}
