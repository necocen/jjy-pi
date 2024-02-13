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
