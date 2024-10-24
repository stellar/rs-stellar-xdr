use super::{Int128Parts, Int256Parts, UInt128Parts, UInt256Parts};

impl TryFrom<(u128, u128)> for UInt256Parts {
    type Error = super::Error;
    fn try_from((hi, lo): (u128, u128)) -> Result<Self, Self::Error> {
        let hi_bytes = hi.to_be_bytes();
        let (hi_hi, hi_lo) = hi_bytes.split_at(8);
        let lo_bytes = lo.to_be_bytes();
        let (lo_hi, lo_lo) = lo_bytes.split_at(8);
        Ok(UInt256Parts {
            hi_hi: u64::from_be_bytes(hi_hi.try_into()?),
            hi_lo: u64::from_be_bytes(hi_lo.try_into()?),
            lo_hi: u64::from_be_bytes(lo_hi.try_into()?),
            lo_lo: u64::from_be_bytes(lo_lo.try_into()?),
        })
    }
}

impl TryFrom<(i128, i128)> for Int256Parts {
    type Error = super::Error;
    fn try_from((hi, lo): (i128, i128)) -> Result<Self, Self::Error> {
        let hi_bytes = hi.to_be_bytes();
        let (hi_hi, hi_lo) = hi_bytes.split_at(8);
        let lo_bytes = lo.to_be_bytes();
        let (lo_hi, lo_lo) = lo_bytes.split_at(8);
        Ok(Int256Parts {
            hi_hi: i64::from_be_bytes(hi_hi.try_into()?),
            hi_lo: u64::from_be_bytes(hi_lo.try_into()?),
            lo_hi: u64::from_be_bytes(lo_hi.try_into()?),
            lo_lo: u64::from_be_bytes(lo_lo.try_into()?),
        })
    }
}

/*
/ Number parsing
        (ScType::U128, Value::String(s)) => {
            let val: u128 = u128::from_str(s)
                .map(Into::into)
                .map_err(|_| Error::InvalidValue(Some(t.clone())))?;
            let bytes = val.to_be_bytes();
            let (hi, lo) = bytes.split_at(8);
            ScVal::U128(UInt128Parts {
                hi: u64::from_be_bytes(hi.try_into()?),
                lo: u64::from_be_bytes(lo.try_into()?),
            })
        }

        (ScType::I128, Value::String(s)) => {
            let val: i128 = i128::from_str(s)
                .map(Into::into)
                .map_err(|_| Error::InvalidValue(Some(t.clone())))?;
            let bytes = val.to_be_bytes();
            let (hi, lo) = bytes.split_at(8);
            ScVal::I128(Int128Parts {
                hi: i64::from_be_bytes(hi.try_into()?),
                lo: u64::from_be_bytes(lo.try_into()?),
            })
        }
 */

 impl TryFrom<u128> for UInt128Parts {
    type Error = super::Error;
    fn try_from(val: u128) -> Result<Self, Self::Error> {
        let bytes = val.to_be_bytes();
        let (hi, lo) = bytes.split_at(8);
        Ok(UInt128Parts {
            hi: u64::from_be_bytes(hi.try_into()?),
            lo: u64::from_be_bytes(lo.try_into()?),
        })
    }
}
impl TryFrom<i128> for Int128Parts {
    type Error = super::Error;
    fn try_from(val: i128) -> Result<Self, Self::Error> {
        let bytes = val.to_be_bytes();
        let (hi, lo) = bytes.split_at(8);
        Ok(Int128Parts {
            hi: i64::from_be_bytes(hi.try_into()?),
            lo: u64::from_be_bytes(lo.try_into()?),
        })
    }
}