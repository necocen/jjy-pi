use std::fmt::{Display, Result};

/// 送信する信号の種別。Hour, Minute, Day, Year, DayOfWeekはBCD符号で送信するので、下から数えた桁数を指定する。
#[derive(Debug, Clone, Copy)]
pub enum SignalType {
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        let bcd: [u8; 10] = [1, 2, 4, 8, 10, 20, 40, 80, 100, 200];
        let s = match self {
            SignalType::Marker => "M".to_string(),
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
            SignalType::Zero => "0".to_string(),
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
