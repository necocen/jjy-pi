// nのBCD表現のi桁目の値を返す
pub fn bcd(n: u32, i: u8) -> u8 {
    let k = i >> 2;
    let l = 10u32.pow(k as u32);
    if ((n / l % 10) as u8) & (1 << (i & 3)) == 0 {
        0
    } else {
        1
    }
}
