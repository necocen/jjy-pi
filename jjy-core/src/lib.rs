use std::time::Duration;

use chrono::NaiveDateTime;

pub mod signal;
mod utils;

/// 次の秒を返す
pub fn get_next_second(now: NaiveDateTime) -> NaiveDateTime {
    // 現在時刻の秒以下を切り捨てて次の秒を求める
    // うるう秒がある場合、最大2秒後になる（MM:60がある場合にMM:59に1秒加えるとMM+1:00まで飛ぶため）
    // FIXME: うるう秒の考慮はこの動作で問題ないのか？
    let nanos = now.timestamp_subsec_nanos();
    let last_second = now - Duration::from_nanos(nanos as u64);
    last_second + Duration::from_secs(1)
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use super::*;

    #[test]
    fn test_get_next_second() {
        let now = NaiveDate::from_ymd_opt(2016, 7, 8)
            .unwrap()
            .and_hms_nano_opt(9, 10, 11, 123_456_789)
            .unwrap();
        let expected = NaiveDate::from_ymd_opt(2016, 7, 8)
            .unwrap()
            .and_hms_nano_opt(9, 10, 12, 0)
            .unwrap();
        let actual = get_next_second(now);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_next_second_leap_second() {
        // うるう秒はnano部分が1_000_000_000を越えるデータとして表現される
        let now = NaiveDate::from_ymd_opt(2015, 7, 1)
            .unwrap()
            .and_hms_nano_opt(8, 59, 59, 1_234_567_890)
            .unwrap();
        let expected = NaiveDate::from_ymd_opt(2015, 7, 1)
            .unwrap()
            .and_hms_nano_opt(9, 0, 0, 0)
            .unwrap();
        let actual = get_next_second(now);
        assert_eq!(actual, expected);
    }
}
