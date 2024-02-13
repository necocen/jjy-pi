use std::thread;

use chrono::Utc;
use jjy_core::get_next_signal;
use rppal::gpio::Gpio;

fn main() -> ! {
    let mut pin = Gpio::new()
        .expect("failed to initialize GPIO")
        .get(4)
        .expect("failed to get GPIO 4")
        .into_output();
    let tz = chrono_tz::Asia::Tokyo;

    loop {
        let now = Utc::now().with_timezone(&tz).naive_local();
        let (sleep_time, signal) = get_next_signal(now);
        thread::sleep(sleep_time);
        println!(
            "{} {:>4}: {}",
            Utc::now().with_timezone(&tz).format("%Y-%m-%d %H:%M:%S"),
            signal.signal_type,
            signal.signal_value,
        );
        pin.set_high();
        thread::sleep(signal.signal_value.to_duration());
        pin.set_low();
    }
}
