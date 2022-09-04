use embedded_hal_0_2::blocking::delay::DelayMs;

use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::{gpio, ledc};

pub struct Alarm<T, C, P>
where T: ledc::HwTimer,
      C: ledc::HwChannel,
      P: gpio::OutputPin {
    ledc_timer: Option<T>,
    ledc_channel: Option<C>,
    pin: Option<P>
}

impl<T, C, P> Alarm<T, C, P> 
where T: ledc::HwTimer,
      C: ledc::HwChannel,
      P: gpio::OutputPin {
    pub fn init(ledc_timer: T, ledc_channel: C, pin: P) -> Self {
        let alarm = Self { 
            ledc_timer: Some(ledc_timer),
            ledc_channel: Some(ledc_channel),
            pin: Some(pin)
        };

        return alarm;
    }

    pub fn update_status(&mut self, co2: u16) {
        if co2 < 600 {
            return;
        }

        let notes = [ 659.25f32, 587.33, 369.99, 415.3, 554.37, 493.88, 293.66, 329.63, 493.88, 440.0, 277.18, 329.63, 440.0 ];
        let lengths = [ 1, 1, 2, 2, 1, 1, 2, 2, 1, 1, 2, 2, 6 ];

        for i in 0..13 {
            let config = ledc::config::TimerConfig::default().frequency(((notes[i]).round() as u32).into()).resolution(ledc::Resolution::Bits10);
            let timer = ledc::Timer::new(self.ledc_timer.take().unwrap(), &config).unwrap();
            let mut channel = ledc::Channel::new(self.ledc_channel.take().unwrap(), &timer, self.pin.take().unwrap()).unwrap();

            println!("Note {} freq = {}", i, notes[i]);
            channel.set_duty(255).unwrap();
            //channel.enable();
            FreeRtos.delay_ms(120 * lengths[i] as u32);
            let (ledc_channel_t, pin_t) = channel.release().unwrap();
            self.ledc_channel.replace(ledc_channel_t);
            self.pin.replace(pin_t);
            self.ledc_timer.replace(timer.release().unwrap());
        }
    }
}
