
use core::{fmt::Display, num::ParseIntError, str::FromStr};
use ethnum::{I256, U256};

pub fn u256_str_from_pieces(hi_hi: u64, hi_lo: u64, lo_hi: u64, lo_lo: u64) -> impl Display {
    u256_from_pieces(hi_hi, hi_lo, lo_hi, lo_lo)
}

pub fn u256_str_into_pieces(s: &str) -> Result<(u64, u64, u64, u64), ParseIntError> {
    let u256 = U256::from_str(s)?;
    Ok(u256_into_pieces(u256))
}

pub fn i256_str_from_pieces(hi_hi: i64, hi_lo: u64, lo_hi: u64, lo_lo: u64) -> impl Display {
    i256_from_pieces(hi_hi, hi_lo, lo_hi, lo_lo)
}

pub fn i256_str_into_pieces(s: &str) -> Result<(i64, u64, u64, u64), ParseIntError> {
    let i256 = I256::from_str(s)?;
    Ok(i256_into_pieces(i256))
}

// The following functions were copied from:
// https://github.com/stellar/rs-soroban-env/blob/fdf898963d314f2edec4a2b1e609f70c6737638a/soroban-env-common/src/num.rs#L431-L455

fn u256_from_pieces(hi_hi: u64, hi_lo: u64, lo_hi: u64, lo_lo: u64) -> U256 {
    let high = (u128::from(hi_hi) << 64) | u128::from(hi_lo);
    let low = (u128::from(lo_hi) << 64) | u128::from(lo_lo);
    U256::from_words(high, low)
}

fn u256_into_pieces(u: U256) -> (u64, u64, u64, u64) {
    let (high, low) = u.into_words();
    let (hi_hi, hi_lo) = ((high >> 64) as u64, high as u64);
    let (lo_hi, lo_lo) = ((low >> 64) as u64, low as u64);
    (hi_hi, hi_lo, lo_hi, lo_lo)
}

fn i256_from_pieces(hi_hi: i64, hi_lo: u64, lo_hi: u64, lo_lo: u64) -> I256 {
    let high = ((u128::from(hi_hi as u64) << 64) | u128::from(hi_lo)) as i128;
    let low = ((u128::from(lo_hi) << 64) | u128::from(lo_lo)) as i128;
    I256::from_words(high, low)
}

fn i256_into_pieces(i: I256) -> (i64, u64, u64, u64) {
    let (high, low) = i.into_words();
    let (hi_hi, hi_lo) = ((high >> 64) as i64, high as u64);
    let (lo_hi, lo_lo) = ((low >> 64) as u64, low as u64);
    (hi_hi, hi_lo, lo_hi, lo_lo)
}
