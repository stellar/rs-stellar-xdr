use core::{fmt::Display, num::ParseIntError};

pub fn u128_str_from_pieces(hi: u64, lo: u64) -> impl Display {
    (u128::from(hi) << 64) | u128::from(lo)
}

#[allow(clippy::cast_possible_truncation)]
pub fn u128_str_into_pieces(s: &str) -> Result<(u64, u64), ParseIntError> {
    let v = s.parse::<u128>()?;
    let hi = (v >> 64) as u64;
    let lo = v as u64;
    Ok((hi, lo))
}

pub fn i128_str_from_pieces(hi: i64, lo: u64) -> impl Display {
    (i128::from(hi) << 64) | i128::from(lo)
}

#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
pub fn i128_str_into_pieces(s: &str) -> Result<(i64, u64), ParseIntError> {
    let v = s.parse::<i128>()?;
    let hi = (v >> 64) as i64;
    let lo = v as u64;
    Ok((hi, lo))
}
