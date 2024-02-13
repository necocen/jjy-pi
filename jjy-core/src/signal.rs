mod signal_type;
mod signal_value;

use chrono::{Datelike as _, NaiveDateTime, Timelike as _};
pub use signal_type::SignalType;
pub use signal_value::SignalValue;

use crate::utils::bcd;

#[derive(Debug, Clone, Copy)]
pub struct Signal {
    pub signal_type: SignalType,
    pub signal_value: SignalValue,
}

impl From<NaiveDateTime> for Signal {
    fn from(now: NaiveDateTime) -> Self {
        let signal_type = SignalType::from(now.second() as u8);
        let signal_value = match signal_type {
            SignalType::Marker | SignalType::Position(_) => SignalValue::Marker,
            SignalType::Hour(h) => bcd(now.hour(), h).into(),
            SignalType::Minute(m) => bcd(now.minute(), m).into(),
            SignalType::Day(d) => bcd(now.ordinal(), d).into(),
            SignalType::Year(y) => bcd(now.year() as u32 % 100, y).into(),
            SignalType::DayOfWeek(w) => bcd(now.weekday().num_days_from_sunday(), w).into(),
            SignalType::LS(_) => SignalValue::Zero, // うるう秒は未対応
            SignalType::PA(1) => ((bcd(now.hour(), 5)
                + bcd(now.hour(), 4)
                + bcd(now.hour(), 3)
                + bcd(now.hour(), 2)
                + bcd(now.hour(), 1)
                + bcd(now.hour(), 0))
                % 2)
            .into(), // 20h + 10h + 8h + 4h + 2h + 1h のパリティ
            SignalType::PA(2) => ((bcd(now.minute(), 6)
                + bcd(now.minute(), 5)
                + bcd(now.minute(), 4)
                + bcd(now.minute(), 3)
                + bcd(now.minute(), 2)
                + bcd(now.minute(), 1)
                + bcd(now.minute(), 0))
                % 2)
            .into(), // 40m + 20m + 10m + 8m + 4m + 2m + 1m のパリティ
            SignalType::SU(_) => SignalValue::Zero,
            SignalType::ST(_) => SignalValue::Zero,
            SignalType::Zero => SignalValue::Zero,
            _ => panic!("unexpected signal type: {:?}", signal_type),
        };
        Signal {
            signal_type,
            signal_value,
        }
    }
}
