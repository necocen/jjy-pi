mod signal_type;
mod signal_value;

use chrono::{Datelike as _, NaiveDateTime, Timelike as _};

use crate::utils::{bcd, parity};

use self::{signal_type::SignalType, signal_value::SignalValue};

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
            SignalType::PA(1) => parity(now.hour()).into(), // hourのパリティ
            SignalType::PA(2) => parity(now.minute()).into(), // minute のパリティ
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
