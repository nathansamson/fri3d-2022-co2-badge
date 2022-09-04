use embedded_hal_0_2::blocking::delay::DelayMs;

use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::{gpio, ledc};

struct AlarmBase<T, C, P>
where T: ledc::HwTimer + std::marker::Send + 'static,
      C: ledc::HwChannel + std::marker::Send + 'static,
      P: gpio::OutputPin + 'static {
    ledc_timer: Option<T>,
    ledc_channel: Option<C>,
    pin: Option<P>,
}

pub struct Alarm<T, C, P>
where T: ledc::HwTimer + std::marker::Send + 'static,
      C: ledc::HwChannel + std::marker::Send + 'static,
      P: gpio::OutputPin + 'static {
    alarm_base: std::sync::Arc<std::sync::Mutex<AlarmBase<T, C, P>>>
}

impl<T, C, P> Alarm<T, C, P> 
where T: ledc::HwTimer + std::marker::Send + 'static,
      C: ledc::HwChannel + std::marker::Send + 'static,
      P: gpio::OutputPin + 'static {
    pub fn init(ledc_timer: T, ledc_channel: C, pin: P) -> Self {
        let alarm_base = AlarmBase { 
            ledc_timer: Some(ledc_timer),
            ledc_channel: Some(ledc_channel),
            pin: Some(pin)
        };

        return Self { alarm_base: std::sync::Arc::new(std::sync::Mutex::new(alarm_base)) };
    }

    pub fn update_status(&mut self, co2: u16) {
        if co2 < 650 {
            return;
        }

        let alarm_base_mutex = std::sync::Arc::clone(&self.alarm_base);

        std::thread::spawn(move || {
            let alarm_base_lock = alarm_base_mutex.try_lock();

            if alarm_base_lock.is_err() {
                // Still playing a sound, do not interfere
                return;
            }

            let mut alarm_base  = alarm_base_lock.unwrap();

            let notes = [ 659.25f32, 587.33, 369.99, 415.3, 554.37, 493.88, 293.66, 329.63, 493.88, 440.0, 277.18, 329.63, 440.0 ];
            let lengths = [ 1, 1, 2, 2, 1, 1, 2, 2, 1, 1, 2, 2, 6 ];

            let mut ledc_timer_t = alarm_base.ledc_timer.take().unwrap();
            let mut ledc_channel_t = alarm_base.ledc_channel.take().unwrap();
            let mut pin_t = alarm_base.pin.take().unwrap();

            for _z in 0..20 { // play it 20 times which is approx a minute
                for i in 0..13 {
                    let config = ledc::config::TimerConfig::default().frequency(((notes[i]).round() as u32).into()).resolution(ledc::Resolution::Bits10);
                    let timer = ledc::Timer::new(ledc_timer_t, &config).unwrap();
                    let mut channel = ledc::Channel::new(ledc_channel_t, &timer, pin_t).unwrap();

                    println!("Note {} freq = {}", i, notes[i]);
                    channel.set_duty(255).unwrap();
                    //channel.enable();
                    FreeRtos.delay_ms(120 * lengths[i] as u32);
                    (ledc_channel_t, pin_t) = channel.release().unwrap();
                    ledc_timer_t = timer.release().unwrap();
                }
            }

            alarm_base.ledc_channel.replace(ledc_channel_t);
            alarm_base.pin.replace(pin_t);
            alarm_base.ledc_timer.replace(ledc_timer_t);
        });
    }
}
