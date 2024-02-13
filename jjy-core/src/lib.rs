use std::time::Duration;

use chrono::{Datelike as _, NaiveDateTime, Timelike as _};
use signal::{SignalType, SignalValue};
use utils::bcd;

mod signal;
mod utils;

/// 次の秒と、それまでの時間を返す
pub fn get_next_second(now: NaiveDateTime) -> (Duration, NaiveDateTime) {
    // 現在時刻の秒以下を切り捨てて次の秒を求める
    // うるう秒がある場合、最大2秒後になる（MM:60がある場合にMM:59に1秒加えるとMM+1:00まで飛ぶため）
    // FIXME: うるう秒の考慮はこの動作で問題ないのか？
    let nanos = now.timestamp_subsec_nanos();
    let last_second = now - Duration::from_nanos(nanos as u64);
    let next_second = last_second + Duration::from_secs(1);
    ((next_second - now).to_std().unwrap(), next_second)
}

pub fn get_signal(now: NaiveDateTime) -> (SignalType, SignalValue) {
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
    (signal_type, signal_value)
}
