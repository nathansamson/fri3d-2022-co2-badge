use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use embedded_hal_0_2::blocking::delay::DelayMs;

use embedded_hal_0_2::adc::OneShot;

use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::serial;
use esp_idf_hal::prelude::*;

use nb::block;

use mh_z19c::MhZ19C;
use lis2dh12::Accelerometer;

#[cfg(feature = "alarm")]
mod alarm;

#[cfg(feature = "leds")]
mod leds;

#[cfg(feature = "screen")]
mod screen;

pub enum CO2State {
    Good(u16),
    Average(u16),
    Bad(u16),
}

pub struct Environment {
    pub co2_state: CO2State,
    pub temp: f32,
}

impl Environment {
    pub fn co2(&self) -> u16 {
        return match self.co2_state {
            CO2State::Good(co2) => co2,
            CO2State::Average(co2) => co2,
            CO2State::Bad(co2) => co2,
        }
    }
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

    let i2c = esp_idf_hal::i2c::Master::new(peripherals.i2c0, esp_idf_hal::i2c::MasterPins {
        sda: pins.gpio21,
        scl: pins.gpio22
    }, esp_idf_hal::i2c::config::MasterConfig::default()).unwrap();
    let mut accel_sensor = lis2dh12::Lis2dh12::new(i2c, lis2dh12::SlaveAddr::Default).unwrap();
    accel_sensor.enable_axis((true, true, true)).unwrap();
    accel_sensor.enable_temp(true).unwrap();
    accel_sensor.set_odr(lis2dh12::Odr::Hz1).unwrap();
    accel_sensor.set_mode(lis2dh12::Mode::LowPower).unwrap();

    let mut screen_on = true;

    // Setting the interrupt pins on the acceloremeter can enable
    // or diable the backlight of the screen (via some magic in the wiring)
    accel_sensor.enable_i2_ia2(screen_on).unwrap();
    accel_sensor.set_int_polarity(screen_on).unwrap();


    // Debug output, can probably safely be disavled.
    let mut s = String::new();
    accel_sensor.dump_regs(&mut s).unwrap();
    println!("{}", s);

    #[cfg(feature = "alarm")]
    let mut my_alarm = alarm::Alarm::init(peripherals.ledc.timer0, peripherals.ledc.channel0, pins.gpio32);

    #[cfg(feature = "leds")]
    let mut my_leds = leds::LedDisplay::init();

    #[cfg(feature = "screen")]
    let mut display = screen::
        init_screen(peripherals.spi3, pins.gpio18, pins.gpio23, pins.gpio19, pins.gpio33.into_output().unwrap(), pins.gpio5.into_output().unwrap()).
        unwrap();

    let mut co2sensor = MhZ19C::new(serial);

    let fw_version = block!(co2sensor.get_firmware_version()).unwrap();
    println!("It seems we are running on FW version '{}' today", fw_version);

    let current_accel = accel_sensor.accel_norm().unwrap();
    let accel_temp = accel_sensor.get_temp_outf().unwrap();
    println!("Badge is oriented x = {}  y = {}  z = {}, accel temp = {}°C", current_accel.x, current_accel.y, current_accel.z, accel_temp);

    let mut touch0 = pins.gpio27.into_analog_atten_0db().unwrap();
    let mut powered_adc2 = esp_idf_hal::adc::PoweredAdc::new(
        peripherals.adc2,
        esp_idf_hal::adc::config::Config::new().calibration(true),
    ).unwrap();
    // First read gives a higher value in my tests so delaying this a bit seems to help to not
    // immediatly switch state
    powered_adc2.read(&mut touch0).unwrap();
    FreeRtos.delay_ms(200 as u32);

    loop {
        let touch0_value = powered_adc2.read(&mut touch0).unwrap();
        println!("TOUCH 0 = {}", touch0_value);
        if touch0_value > 800 {
            screen_on = !screen_on;
            accel_sensor.enable_i2_ia2(screen_on).unwrap();
            accel_sensor.set_int_polarity(screen_on).unwrap();
        }

        let (co2, temp) = block!(co2sensor.read_co2_ppm_and_temp_celcius()).unwrap();
        // Cutoff values taken from  https://www.euromate.com/group/nl/blogs/blog-heeft-de-co2-concentratie-invloed-op-de-overdracht-van-het-coronavirus/

        let co2_state = match co2 {
            co2 if co2 < 800 => CO2State::Good(co2),
            co2 if co2 < 1200 =>  CO2State::Average(co2),
            _ => CO2State::Bad(co2),
        };

        let environment = Environment {
            co2_state: co2_state,
            temp: temp,
        };

        println!("CO2 value = {}ppm & temp = {:.2}° Celcius", environment.co2(), environment.temp);

        #[cfg(feature = "alarm")]
        {
            my_alarm.update_status(&environment);
        }

        #[cfg(feature = "leds")]
        {
            my_leds.update_status(&environment);
        }

        #[cfg(feature = "screen")]
        {
            if screen_on {
                screen::update_screen(&mut display, &environment);
            }
        }

        let current_accel = accel_sensor.accel_norm().unwrap();
        let accel_temp = accel_sensor.get_temp_outf().unwrap();
        println!("Badge is oriented x = {}  y = {}  z = {}, accel temp = {}°C", current_accel.x, current_accel.y, current_accel.z, accel_temp);

        FreeRtos.delay_ms(5000 as u32);
    }
}
