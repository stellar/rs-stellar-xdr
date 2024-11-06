use core::str::FromStr;

use super::{Int128Parts, Int256Parts, UInt128Parts, UInt256Parts};

impl From<(u128, u128)> for UInt256Parts {
    fn from((hi, lo): (u128, u128)) -> Self {
        let UInt128Parts {
            hi: hi_hi,
            lo: hi_lo,
        } = hi.into();
        let UInt128Parts {
            hi: lo_hi,
            lo: lo_lo,
        } = lo.into();
        UInt256Parts {
            hi_hi,
            hi_lo,
            lo_hi,
            lo_lo,
        }
    }
}

impl From<UInt256Parts> for (u128, u128) {
    fn from(parts: UInt256Parts) -> Self {
        let hi = (u128::from(parts.hi_hi) << 64) | u128::from(parts.hi_lo);
        let lo = (u128::from(parts.lo_hi) << 64) | u128::from(parts.lo_lo);
        (hi, lo)
    }
}

impl From<(i128, i128)> for Int256Parts {
    fn from((hi, lo): (i128, i128)) -> Self {
        let Int128Parts {
            hi: hi_hi,
            lo: hi_lo,
        } = hi.into();
        let UInt128Parts {
            hi: lo_hi,
            lo: lo_lo,
        } = (lo as u128).into();
        Int256Parts {
            hi_hi,
            hi_lo,
            lo_hi,
            lo_lo,
        }
    }
}

impl From<Int256Parts> for (i128, i128) {
    fn from(parts: Int256Parts) -> Self {
        let hi = (i128::from(parts.hi_hi) << 64) | i128::from(parts.hi_lo);
        let lo = (i128::from(parts.lo_hi) << 64) | i128::from(parts.lo_lo);
        (hi, lo)
    }
}

impl From<u128> for UInt128Parts {
    fn from(val: u128) -> Self {
        let hi = (val >> 64) as u64;
        let lo = val as u64;
        let hi: [u8; 8] = hi.to_be_bytes();
        let lo: [u8; 8] = lo.to_be_bytes();
        Self {
            hi: u64::from_be_bytes(hi),
            lo: u64::from_be_bytes(lo),
        }
    }
}

impl From<UInt128Parts> for u128 {
    fn from(parts: UInt128Parts) -> Self {
        let hi: [u8; 8] = parts.hi.to_be_bytes();
        let lo: [u8; 8] = parts.lo.to_be_bytes();
        let hi = u64::from_be_bytes(hi);
        let lo = u64::from_be_bytes(lo);
        (hi as u128) << 64 | lo as u128
    }
}

impl From<i128> for Int128Parts {
    fn from(val: i128) -> Self {
        let hi = (val >> 64) as u64;
        let lo = val as i64;
        let hi: [u8; 8] = hi.to_be_bytes();
        let lo: [u8; 8] = lo.to_be_bytes();
        Int128Parts {
            hi: i64::from_be_bytes(hi),
            lo: u64::from_be_bytes(lo),
        }
    }
}

impl From<Int128Parts> for i128 {
    fn from(parts: Int128Parts) -> Self {
        let hi: [u8; 8] = parts.hi.to_be_bytes();
        let lo: [u8; 8] = parts.lo.to_be_bytes();
        let hi = i64::from_be_bytes(hi);
        let lo = u64::from_be_bytes(lo);
        (hi as i128) << 64 | lo as i128
    }
}

impl FromStr for UInt128Parts {
    type Err = super::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(u128::from_str(s).map_err(|_| Self::Err::Invalid)?.into())
    }
}

impl FromStr for Int128Parts {
    type Err = super::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(i128::from_str(s).map_err(|_| Self::Err::Invalid)?.into())
    }
}
