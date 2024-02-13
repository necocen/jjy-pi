use std::{
    fmt::{Display, Formatter, Result},
    time::Duration,
};

/// 送信する信号の値。マーカー、0、1の3種類。
#[derive(Debug, Clone, Copy)]
pub enum SignalValue {
    Marker,
    Zero,
    One,
}

impl SignalValue {
    pub fn to_duration(self) -> Duration {
        match self {
            SignalValue::Marker => Duration::from_millis(200),
            SignalValue::Zero => Duration::from_millis(800),
            SignalValue::One => Duration::from_millis(500),
        }
    }
}

impl Display for SignalValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
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
