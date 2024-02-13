mod signal_type;
mod signal_value;

use chrono::{Datelike as _, NaiveDateTime, Timelike as _};

use crate::utils::{bcd, parity};

use self::{signal_type::SignalType, signal_value::SignalValue};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;
    use SignalValue::*;

    #[test]
    fn test_signal_from_date() {
        // https://jjy.nict.go.jp/jjy/trans/timecode1.html で例示されているもの
        let now =
            NaiveDateTime::parse_from_str("2004-04-01 17:25:00", "%Y-%m-%d %H:%M:%S").unwrap();

        let expectations = [
            // 00-09
            (SignalType::Marker, Marker),
            (SignalType::Minute(6), Zero),
            (SignalType::Minute(5), One),
            (SignalType::Minute(4), Zero),
            (SignalType::Zero, Zero),
            (SignalType::Minute(3), Zero),
            (SignalType::Minute(2), One),
            (SignalType::Minute(1), Zero),
            (SignalType::Minute(0), One),
            (SignalType::Position(1), Marker),
            // 10-19
            (SignalType::Zero, Zero),
            (SignalType::Zero, Zero),
            (SignalType::Hour(5), Zero),
            (SignalType::Hour(4), One),
            (SignalType::Zero, Zero),
            (SignalType::Hour(3), Zero),
            (SignalType::Hour(2), One),
            (SignalType::Hour(1), One),
            (SignalType::Hour(0), One),
            (SignalType::Position(2), Marker),
            // 20-29
            (SignalType::Zero, Zero),
            (SignalType::Zero, Zero),
            (SignalType::Day(9), Zero),
            (SignalType::Day(8), Zero),
            (SignalType::Zero, Zero),
            (SignalType::Day(7), One),
            (SignalType::Day(6), Zero),
            (SignalType::Day(5), Zero),
            (SignalType::Day(4), One),
            (SignalType::Position(3), Marker),
            // 30-39
            (SignalType::Day(3), Zero),
            (SignalType::Day(2), Zero),
            (SignalType::Day(1), One),
            (SignalType::Day(0), Zero),
            (SignalType::Zero, Zero),
            (SignalType::Zero, Zero),
            (SignalType::PA(1), Zero),
            (SignalType::PA(2), One),
            (SignalType::SU(1), Zero),
            (SignalType::Position(4), Marker),
            // 40-49
            (SignalType::SU(2), Zero),
            (SignalType::Year(7), Zero),
            (SignalType::Year(6), Zero),
            (SignalType::Year(5), Zero),
            (SignalType::Year(4), Zero),
            (SignalType::Year(3), Zero),
            (SignalType::Year(2), One),
            (SignalType::Year(1), Zero),
            (SignalType::Year(0), Zero),
            (SignalType::Position(5), Marker),
            // 50-59
            (SignalType::DayOfWeek(2), One),
            (SignalType::DayOfWeek(1), Zero),
            (SignalType::DayOfWeek(0), Zero),
            (SignalType::LS(1), Zero),
            (SignalType::LS(2), Zero),
            (SignalType::Zero, Zero),
            (SignalType::Zero, Zero),
            (SignalType::Zero, Zero),
            (SignalType::Zero, Zero),
            (SignalType::Position(0), Marker),
        ];

        for (i, (signal_type, signal_value)) in expectations.iter().enumerate() {
            let signal = Signal::from(now + Duration::seconds(i as i64));
            assert_eq!(signal.signal_type, *signal_type, "2004-04-01 17:25:{i:02}");
            assert_eq!(
                signal.signal_value, *signal_value,
                "2004-04-01 17:25:{i:02} Type: {:?}",
                *signal_type
            );
        }
    }
}
