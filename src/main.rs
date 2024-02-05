use std::{fmt::Display, thread, time::Duration};

use chrono::{Datelike, NaiveDateTime, Timelike as _, Utc};
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

/// 次の秒と、それまでの時間を返す
fn get_next_second(now: NaiveDateTime) -> (Duration, NaiveDateTime) {
    // 現在時刻の秒以下を切り捨てて次の秒を求める
    // うるう秒がある場合、最大2秒後になる（MM:60がある場合にMM:59に1秒加えるとMM+1:00まで飛ぶため）
    // FIXME: うるう秒の考慮はこの動作で問題ないのか？
    let nanos = now.timestamp_subsec_nanos();
    let last_second = now - Duration::from_nanos(nanos as u64);
    let next_second = last_second + Duration::from_secs(1);
    ((next_second - now).to_std().unwrap(), next_second)
}

fn get_signal(now: NaiveDateTime) -> (SignalType, SignalValue) {
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

// nのBCD表現のi桁目の値を返す
fn bcd(n: u32, i: u8) -> u8 {
    let k = i >> 2;
    let l = 10u32.pow(k as u32);
    if ((n / l % 10) as u8) & (1 << (i & 3)) == 0 {
        0
    } else {
        1
    }
}

/// 送信する信号の値。マーカー、0、1の3種類。
#[derive(Debug, Clone, Copy)]
enum SignalValue {
    Marker,
    Zero,
    One,
}

impl SignalValue {
    pub fn to_duration(&self) -> Duration {
        match self {
            SignalValue::Marker => Duration::from_millis(200),
            SignalValue::Zero => Duration::from_millis(800),
            SignalValue::One => Duration::from_millis(500),
        }
    }
}

impl Display for SignalValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SignalValue::Marker => "M".fmt(f),
            SignalValue::Zero => "0".fmt(f),
            SignalValue::One => "1".fmt(f),
        }
    }
}

impl From<u8> for SignalValue {
    fn from(n: u8) -> Self {
        match n {
            0 => SignalValue::Zero,
            1 => SignalValue::One,
            _ => panic!("unexpected signal value: {n}"),
        }
    }
}

/// 送信する信号の種別。Hour, Minute, Day, Year, DayOfWeekはBCD符号で送信するので、下から数えた桁数を指定する。
#[derive(Debug, Clone, Copy)]
enum SignalType {
    Marker,        // マーカー(M)
    Position(u8),  // ポジションマーカー(P1, P2, P3, P4, P5, P0)
    Hour(u8),      // 時間
    Minute(u8),    // 分
    Day(u8),       // 通算日
    Year(u8),      // 年（下2桁）
    DayOfWeek(u8), // 曜日
    LS(u8),        // うるう秒情報(LS1, LS2)
    PA(u8),        // パリティ情報(PA1, PA2)
    SU(u8),        // 予備／サマータイム情報(SU1, SU2)
    #[allow(dead_code)]
    ST(u8), // 停波予告情報(ST1, ST2, ST3, ST4, ST5, ST6)
    Zero,          // 固定値0
}

impl Display for SignalType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bcd: [u8; 10] = [1, 2, 4, 8, 10, 20, 40, 80, 100, 200];
        let s = match self {
            SignalType::Marker => format!("M"),
            SignalType::Position(p) => format!("P{p}"),
            SignalType::Hour(h) => format!("{}h", bcd[*h as usize]),
            SignalType::Minute(m) => format!("{}m", bcd[*m as usize]),
            SignalType::Day(d) => format!("{}d", bcd[*d as usize]),
            SignalType::Year(y) => format!("{}y", bcd[*y as usize]),
            SignalType::DayOfWeek(w) => format!("{}w", bcd[*w as usize]),
            SignalType::LS(l) => format!("LS{l}"),
            SignalType::PA(p) => format!("PA{p}"),
            SignalType::SU(s) => format!("SU{s}"),
            SignalType::ST(s) => format!("ST{s}"),
            SignalType::Zero => format!("0"),
        };
        f.pad(s.as_str())
    }
}

impl From<u8> for SignalType {
    fn from(n: u8) -> Self {
        // 通常時のみ対応（呼び出し符号は未対応）
        // TODO: うるう秒は未対応（60秒がある場合でも59秒にP0を送信する）
        match n {
            0 => SignalType::Marker,
            1..=3 => SignalType::Minute(7 - n), // 40m, 20m, 10m
            4 => SignalType::Zero,
            5..=8 => SignalType::Minute(8 - n), // 8m, 4m, 2m, 1m
            9 => SignalType::Position(1),
            10..=11 => SignalType::Zero,
            12..=13 => SignalType::Hour(17 - n), // 20h, 10h
            14 => SignalType::Zero,
            15..=18 => SignalType::Hour(18 - n), // 8h, 4h, 2h, 1h
            19 => SignalType::Position(2),
            20..=21 => SignalType::Zero,
            22..=23 => SignalType::Day(31 - n), // 200d, 100d
            24 => SignalType::Zero,
            25..=28 => SignalType::Day(32 - n), // 80d, 40d, 20d, 10d
            29 => SignalType::Position(3),
            30..=33 => SignalType::Day(33 - n), // 8d, 4d, 2d, 1d
            34..=35 => SignalType::Zero,
            36..=37 => SignalType::PA(n - 35), // PA1, PA2
            38 => SignalType::SU(1),
            39 => SignalType::Position(4),
            40 => SignalType::SU(2),
            41..=48 => SignalType::Year(48 - n), // 80y, 40y, 20y, 10y, 8y, 4y, 2y, 1y
            49 => SignalType::Position(5),
            50..=52 => SignalType::DayOfWeek(52 - n), // 4w, 2w, 1w
            53..=54 => SignalType::LS(n - 52),        // LS1, LS2
            55..=58 => SignalType::Zero,
            59 => SignalType::Position(0),
            _ => SignalType::Zero,
        }
    }
}
