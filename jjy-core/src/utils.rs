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

/// パリティ計算をする（nは99まで）
pub fn parity(n: u32) -> u8 {
    if n >= 100 {
        panic!("n must be less than 100");
    }
    (0..8).map(|i| bcd(n, i)).sum::<u8>() % 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bcd() {
        assert_eq!(bcd(0, 0), 0);
        assert_eq!(bcd(0, 1), 0);
        assert_eq!(bcd(0, 2), 0);
        assert_eq!(bcd(0, 3), 0);

        assert_eq!(bcd(3, 0), 1);
        assert_eq!(bcd(3, 1), 1);
        assert_eq!(bcd(3, 2), 0);
        assert_eq!(bcd(3, 3), 0);

        assert_eq!(bcd(9, 0), 1);
        assert_eq!(bcd(9, 1), 0);
        assert_eq!(bcd(9, 2), 0);
        assert_eq!(bcd(9, 3), 1);

        assert_eq!(bcd(10, 0), 0);
        assert_eq!(bcd(10, 1), 0);
        assert_eq!(bcd(10, 2), 0);
        assert_eq!(bcd(10, 3), 0);
        assert_eq!(bcd(10, 4), 1); // BCDなので桁が上がる

        assert_eq!(bcd(30, 0), 0);
        assert_eq!(bcd(30, 1), 0);
        assert_eq!(bcd(30, 2), 0);
        assert_eq!(bcd(30, 3), 0);
        assert_eq!(bcd(30, 4), 1);
        assert_eq!(bcd(30, 5), 1);

        assert_eq!(bcd(99, 0), 1);
        assert_eq!(bcd(99, 3), 1);
        assert_eq!(bcd(99, 4), 1);
        assert_eq!(bcd(99, 7), 1);

        assert_eq!(bcd(100, 0), 0);
        assert_eq!(bcd(100, 1), 0);
        assert_eq!(bcd(100, 2), 0);
        assert_eq!(bcd(100, 3), 0);
        assert_eq!(bcd(100, 4), 0);
        assert_eq!(bcd(100, 5), 0);
        assert_eq!(bcd(100, 6), 0);
        assert_eq!(bcd(100, 7), 0);
        assert_eq!(bcd(100, 8), 1);
    }

    #[test]
    fn test_parity() {
        assert_eq!(parity(5), 0); // 4 + 1
        assert_eq!(parity(7), 1); // 4 + 2 + 1
        assert_eq!(parity(11), 0); // 10 + 1
        assert_eq!(parity(32), 1); // 20 + 10 + 2
    }
}
