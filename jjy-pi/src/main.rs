use std::thread;

use chrono::Utc;
use jjy_core::{get_next_second, signal::Signal};
use rppal::gpio::Gpio;

fn main() -> ! {
    let tz = chrono_tz::Asia::Tokyo;
    let mut pin = Gpio::new()
        .expect("failed to initialize GPIO")
        .get(4)
        .expect("failed to get GPIO 4")
        .into_output();
    pin.set_low();

    loop {
        let now = Utc::now().with_timezone(&tz).naive_local();
        let next_second = get_next_second(now);
        let signal = Signal::from(next_second);
        thread::sleep((next_second - now).to_std().unwrap());
        println!(
            "{} {:>4}: {}",
            next_second.format("%Y-%m-%d %H:%M:%S"),
            signal.signal_type,
            signal.signal_value,
        );
        pin.set_high();
        thread::sleep(signal.signal_value.to_duration());
        pin.set_low();
    }
}
