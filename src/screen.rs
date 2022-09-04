use esp_idf_sys::EspError;

use esp_idf_hal::{spi, gpio};
use esp_idf_hal::prelude::*;

use embedded_graphics::{
    prelude::*,
    pixelcolor::Rgb565,
    mono_font::MonoTextStyle,
    text::{Alignment, Text},
};

use profont::{PROFONT_14_POINT, PROFONT_18_POINT, PROFONT_24_POINT};

use crate::{CO2State, Environment};

pub fn init_screen<SCLK, SDO, SDI, DC, CS>(spi: spi::SPI3, sclk: SCLK, sdo: SDO, sdi: SDI, dc: DC, cs: CS) -> Result<
    mipidsi::Display<display_interface_spi::SPIInterface<spi::Master<spi::SPI3, SCLK, SDO, SDI, gpio::Gpio5<esp_idf_hal::gpio::Unknown>>, DC, CS>, 
    mipidsi::NoPin, mipidsi::models::ST7789>, EspError
> 
where SCLK: esp_idf_hal::gpio::Pin + esp_idf_hal::gpio::OutputPin, 
      SDO: esp_idf_hal::gpio::Pin + esp_idf_hal::gpio::OutputPin, 
      SDI: esp_idf_hal::gpio::OutputPin + esp_idf_hal::gpio::InputPin, 
      DC: esp_idf_hal::gpio::Pin + esp_idf_hal::gpio::OutputPin + embedded_hal_0_2::digital::v2::OutputPin,
      CS: esp_idf_hal::gpio::Pin + esp_idf_hal::gpio::OutputPin + embedded_hal_0_2::digital::v2::OutputPin
{
    let config = <spi::config::Config as Default>::default().baudrate(32.MHz().into()).data_mode(embedded_hal::spi::MODE_0);
    
    let spi = spi::Master::<spi::SPI3, _, _, _, gpio::Gpio5<gpio::Unknown>>::new(
        spi,
        spi::Pins {
            sclk: sclk,
            sdo: sdo,
            sdi: Some(sdi),
            cs: None,
        },
        config,
    ).unwrap();

    let di = display_interface_spi::SPIInterface::new(spi, dc, cs);

    let mut display = mipidsi::Display::st7789_without_rst(di);
    let mut delay = esp_idf_hal::delay::Ets;

    display.init(
        &mut delay,
        mipidsi::DisplayOptions::default()
    ).unwrap();

    return Ok(display);
}

pub fn update_screen<D, PE>(display: &mut D, environment: &Environment)
where
    PE: std::fmt::Debug,
    D: DrawTarget<Color = Rgb565, Error = mipidsi::Error<PE>> {

    let co2 = environment.co2();

    let state_color = match environment.co2_state {
        CO2State::Good(_) => Rgb565::GREEN,
        CO2State::Average(_) => Rgb565::YELLOW,
        CO2State::Bad(_) => Rgb565::RED,
    };

    let text_co2 = format!("{}", co2);
    let text_ppm = "ppm";
    let text_temp = format!("{:.1}°C", environment.temp);

    display.clear(Rgb565::BLACK).unwrap();

    let character_style_small = MonoTextStyle::new(&PROFONT_14_POINT, state_color);
    let character_style = MonoTextStyle::new(&PROFONT_24_POINT, state_color);
    let character_style_temp = MonoTextStyle::new(&PROFONT_18_POINT, Rgb565::WHITE);

    Text::with_alignment(
        &text_co2,
        display.bounding_box().center(),
        character_style,
        Alignment::Center,
    )
    .draw(display).unwrap();

    Text::with_alignment(
        &text_ppm,
        display.bounding_box().center() + Point::new(0, 16),
        character_style_small,
        Alignment::Center,
    )
    .draw(display).unwrap();

    Text::with_alignment(
        &text_temp,
        display.bounding_box().bottom_right().unwrap() + Point::new(-12, -12),
        character_style_temp,
        Alignment::Right,
    )
    .draw(display).unwrap();

    // return Ok(());
}