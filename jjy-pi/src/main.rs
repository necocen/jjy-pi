use std::thread;

use chrono::Utc;
use jjy_core::{get_next_second, get_signal};
use rppal::gpio::Gpio;

fn main() -> ! {
    let mut pin = Gpio::new()
        .expect("failed to initialize GPIO")
        .get(4)
        .expect("failed to get GPIO 4")
        .into_output();

    loop {
        let tz = chrono_tz::Asia::Tokyo;
        let now = Utc::now().with_timezone(&tz).naive_local();
        let (sleep_time, next_second) = get_next_second(now);
        let (signal_type, signal_value) = get_signal(next_second);
        thread::sleep(sleep_time);
        println!(
            "{} {signal_type:>4}: {signal_value}",
            next_second.format("%Y-%m-%d %H:%M:%S")
        );
        pin.set_high();
        thread::sleep(signal_value.to_duration());
        pin.set_low();
    }
}
