use super::{
    Int128Parts, ScBytes, ScError, ScMap, ScMapEntry, ScSymbol, ScVal, ScVec, UInt128Parts,
};

#[cfg(all(not(feature = "std"), feature = "alloc"))]
extern crate alloc;
#[cfg(all(not(feature = "std"), feature = "alloc"))]
use alloc::{string::String, vec, vec::Vec};

// TODO: Use the Error type for conversions in this file.

impl From<ScError> for ScVal {
    fn from(v: ScError) -> Self {
        ScVal::Error(v)
    }
}

impl TryFrom<ScVal> for ScError {
    type Error = ();
    fn try_from(v: ScVal) -> Result<Self, Self::Error> {
        if let ScVal::Error(s) = v {
            Ok(s)
        } else {
            Err(())
        }
    }
}

impl From<i32> for ScVal {
    fn from(v: i32) -> ScVal {
        ScVal::I32(v)
    }
}

impl From<&i32> for ScVal {
    fn from(v: &i32) -> ScVal {
        ScVal::I32(*v)
    }
}

impl TryFrom<ScVal> for i32 {
    type Error = ();
    fn try_from(v: ScVal) -> Result<Self, Self::Error> {
        if let ScVal::I32(i) = v {
            Ok(i)
        } else {
            Err(())
        }
    }
}

impl From<u32> for ScVal {
    fn from(v: u32) -> ScVal {
        ScVal::U32(v)
    }
}

impl From<&u32> for ScVal {
    fn from(v: &u32) -> ScVal {
        ScVal::U32(*v)
    }
}

impl TryFrom<ScVal> for u32 {
    type Error = ();
    fn try_from(v: ScVal) -> Result<Self, Self::Error> {
        if let ScVal::U32(i) = v {
            Ok(i)
        } else {
            Err(())
        }
    }
}

impl From<i64> for ScVal {
    fn from(v: i64) -> ScVal {
        ScVal::I64(v)
    }
}

impl From<&i64> for ScVal {
    fn from(v: &i64) -> ScVal {
        <_ as Into<ScVal>>::into(*v)
    }
}

impl TryFrom<ScVal> for i64 {
    type Error = ();
    fn try_from(v: ScVal) -> Result<Self, Self::Error> {
        if let ScVal::I64(i) = v {
            Ok(i)
        } else {
            Err(())
        }
    }
}

impl From<()> for ScVal {
    fn from((): ()) -> Self {
        ScVal::Void
    }
}

impl From<&()> for ScVal {
    fn from((): &()) -> Self {
        ScVal::Void
    }
}

impl TryFrom<ScVal> for () {
    type Error = ();
    fn try_from(v: ScVal) -> Result<Self, Self::Error> {
        if let ScVal::Void = v {
            Ok(())
        } else {
            Err(())
        }
    }
}

impl From<bool> for ScVal {
    fn from(v: bool) -> Self {
        ScVal::Bool(v)
    }
}

impl From<&bool> for ScVal {
    fn from(v: &bool) -> Self {
        <_ as Into<ScVal>>::into(*v)
    }
}

impl TryFrom<ScVal> for bool {
    type Error = ();
    fn try_from(v: ScVal) -> Result<Self, Self::Error> {
        if let ScVal::Bool(b) = v {
            Ok(b)
        } else {
            Err(())
        }
    }
}

impl From<u64> for ScVal {
    fn from(v: u64) -> Self {
        ScVal::U64(v)
    }
}

impl From<&u64> for ScVal {
    fn from(v: &u64) -> Self {
        ScVal::U64(*v)
    }
}

impl TryFrom<ScVal> for u64 {
    type Error = ();
    fn try_from(v: ScVal) -> Result<Self, Self::Error> {
        if let ScVal::U64(i) = v {
            Ok(i)
        } else {
            Err(())
        }
    }
}

pub mod int128_helpers {
    #[must_use]
    #[inline(always)]
    #[allow(clippy::inline_always, clippy::cast_possible_truncation)]
    pub fn u128_hi(u: u128) -> u64 {
        (u >> 64) as u64
    }

    #[must_use]
    #[inline(always)]
    #[allow(clippy::inline_always, clippy::cast_possible_truncation)]
    pub fn u128_lo(u: u128) -> u64 {
        u as u64
    }

    #[must_use]
    #[inline(always)]
    #[allow(clippy::inline_always)]
    pub fn u128_from_pieces(hi: u64, lo: u64) -> u128 {
        (u128::from(hi) << 64) | u128::from(lo)
    }

    #[must_use]
    #[inline(always)]
    #[allow(clippy::inline_always, clippy::cast_possible_truncation)]
    pub fn i128_hi(i: i128) -> i64 {
        (i >> 64) as i64
    }

    #[must_use]
    #[inline(always)]
    #[allow(
        clippy::inline_always,
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss
    )]
    pub fn i128_lo(i: i128) -> u64 {
        i as u64
    }

    #[must_use]
    #[inline(always)]
    #[allow(
        clippy::inline_always,
        clippy::cast_sign_loss,
        clippy::cast_possible_wrap
    )]
    pub fn i128_from_pieces(hi: i64, lo: u64) -> i128 {
        (u128::from(hi as u64) << 64 | u128::from(lo)) as i128
    }
}

#[allow(clippy::wildcard_imports)]
use int128_helpers::*;

impl From<u128> for ScVal {
    fn from(v: u128) -> Self {
        ScVal::U128(UInt128Parts {
            hi: u128_hi(v),
            lo: u128_lo(v),
        })
    }
}

impl From<&u128> for ScVal {
    fn from(v: &u128) -> Self {
        <ScVal as From<u128>>::from(*v)
    }
}

impl From<&UInt128Parts> for u128 {
    fn from(v: &UInt128Parts) -> Self {
        u128_from_pieces(v.hi, v.lo)
    }
}

impl TryFrom<ScVal> for u128 {
    type Error = ();
    fn try_from(v: ScVal) -> Result<Self, Self::Error> {
        if let ScVal::U128(i) = v {
            Ok((&i).into())
        } else {
            Err(())
        }
    }
}

impl From<i128> for ScVal {
    fn from(v: i128) -> Self {
        ScVal::I128(Int128Parts {
            hi: i128_hi(v),
            lo: i128_lo(v),
        })
    }
}

impl From<&i128> for ScVal {
    fn from(v: &i128) -> Self {
        <ScVal as From<i128>>::from(*v)
    }
}

impl From<&Int128Parts> for i128 {
    fn from(v: &Int128Parts) -> Self {
        i128_from_pieces(v.hi, v.lo)
    }
}

impl TryFrom<ScVal> for i128 {
    type Error = ();
    fn try_from(v: ScVal) -> Result<Self, Self::Error> {
        if let ScVal::I128(i) = v {
            Ok((&i).into())
        } else {
            Err(())
        }
    }
}

impl From<ScSymbol> for ScVal {
    fn from(v: ScSymbol) -> Self {
        ScVal::Symbol(v)
    }
}

impl TryFrom<ScVal> for ScSymbol {
    type Error = ();
    fn try_from(v: ScVal) -> Result<Self, Self::Error> {
        if let ScVal::Symbol(s) = v {
            Ok(s)
        } else {
            Err(())
        }
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<String> for ScVal {
    type Error = ();
    fn try_from(v: String) -> Result<Self, ()> {
        Ok(ScVal::Symbol(v.try_into()?))
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<&String> for ScVal {
    type Error = ();
    fn try_from(v: &String) -> Result<Self, ()> {
        Ok(ScVal::Symbol(v.try_into()?))
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<String> for ScSymbol {
    type Error = ();
    fn try_from(v: String) -> Result<Self, Self::Error> {
        Ok(ScSymbol(v.try_into().map_err(|_| ())?))
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<&String> for ScSymbol {
    type Error = ();
    fn try_from(v: &String) -> Result<Self, Self::Error> {
        Ok(ScSymbol(v.try_into().map_err(|_| ())?))
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<&str> for ScVal {
    type Error = ();
    fn try_from(v: &str) -> Result<Self, ()> {
        Ok(ScVal::Symbol(v.try_into()?))
    }
}

#[cfg(not(feature = "alloc"))]
impl TryFrom<&'static str> for ScVal {
    type Error = ();
    fn try_from(v: &'static str) -> Result<Self, ()> {
        Ok(ScVal::Symbol(v.try_into()?))
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<&str> for ScSymbol {
    type Error = ();
    fn try_from(v: &str) -> Result<Self, Self::Error> {
        Ok(ScSymbol(v.try_into().map_err(|_| ())?))
    }
}

#[cfg(not(feature = "alloc"))]
impl TryFrom<&'static str> for ScSymbol {
    type Error = ();
    fn try_from(v: &'static str) -> Result<Self, Self::Error> {
        Ok(ScSymbol(v.try_into().map_err(|_| ())?))
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<ScVal> for String {
    type Error = ();
    fn try_from(v: ScVal) -> Result<Self, Self::Error> {
        if let ScVal::Symbol(s) = v {
            // TODO: It might be worth distinguishing the error case where this
            // is an invalid symbol with invalid characters.
            Ok(s.0.into_utf8_string().map_err(|_| ())?)
        } else {
            Err(())
        }
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<Vec<u8>> for ScVal {
    type Error = ();
    fn try_from(v: Vec<u8>) -> Result<Self, ()> {
        Ok(ScVal::Bytes(ScBytes(v.try_into().map_err(|_| ())?)))
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<&Vec<u8>> for ScVal {
    type Error = ();
    fn try_from(v: &Vec<u8>) -> Result<Self, ()> {
        Ok(ScVal::Bytes(ScBytes(v.try_into().map_err(|_| ())?)))
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<&[u8]> for ScVal {
    type Error = ();
    fn try_from(v: &[u8]) -> Result<Self, ()> {
        Ok(ScVal::Bytes(ScBytes(v.try_into().map_err(|_| ())?)))
    }
}

#[cfg(feature = "alloc")]
impl<const N: usize> TryFrom<[u8; N]> for ScVal {
    type Error = ();
    fn try_from(v: [u8; N]) -> Result<Self, ()> {
        Ok(ScVal::Bytes(ScBytes(v.try_into().map_err(|_| ())?)))
    }
}

#[cfg(feature = "alloc")]
impl<const N: usize> TryFrom<&[u8; N]> for ScVal {
    type Error = ();
    fn try_from(v: &[u8; N]) -> Result<Self, ()> {
        Ok(ScVal::Bytes(ScBytes(v.try_into().map_err(|_| ())?)))
    }
}

#[cfg(not(feature = "alloc"))]
impl TryFrom<&'static [u8]> for ScVal {
    type Error = ();
    fn try_from(v: &'static [u8]) -> Result<Self, ()> {
        Ok(ScVal::Bytes(ScBytes(v.try_into().map_err(|_| ())?)))
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<ScVal> for Vec<u8> {
    type Error = ();
    fn try_from(v: ScVal) -> Result<Self, Self::Error> {
        if let ScVal::Bytes(ScBytes(b)) = v {
            Ok(b.into())
        } else {
            Err(())
        }
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<&ScVal> for Vec<u8> {
    type Error = ();
    fn try_from(v: &ScVal) -> Result<Self, Self::Error> {
        if let ScVal::Bytes(ScBytes(b)) = v {
            Ok(b.into())
        } else {
            Err(())
        }
    }
}

impl From<ScVec> for ScVal {
    fn from(v: ScVec) -> Self {
        ScVal::Vec(Some(v))
    }
}

#[cfg(feature = "alloc")]
impl<T: TryInto<ScVal>> TryFrom<Vec<T>> for ScVal {
    type Error = ();
    fn try_from(v: Vec<T>) -> Result<Self, ()> {
        Ok(ScVal::Vec(Some(
            v.into_iter()
                .map(|t| <_ as TryInto<ScVal>>::try_into(t))
                .collect::<Result<Vec<_>, _>>() // TODO: Impl conversion from Iterator to VecM in xdrgen generated code.
                .map_err(|_| ())?
                .try_into()
                .map_err(|_| ())?,
        )))
    }
}

#[cfg(feature = "alloc")]
impl<T: TryInto<ScVal> + Clone> TryFrom<&Vec<T>> for ScVal {
    type Error = ();
    fn try_from(v: &Vec<T>) -> Result<Self, ()> {
        Ok(ScVal::Vec(Some(
            v.iter()
                .map(|t| <_ as TryInto<ScVal>>::try_into(t.clone()))
                .collect::<Result<Vec<_>, _>>() // TODO: Impl conversion from Iterator to VecM in xdrgen generated code.
                .map_err(|_| ())?
                .try_into()
                .map_err(|_| ())?,
        )))
    }
}

#[cfg(feature = "alloc")]
impl<T: TryInto<ScVal> + Clone> TryFrom<&[T]> for ScVal {
    type Error = ();
    fn try_from(v: &[T]) -> Result<Self, ()> {
        Ok(ScVal::Vec(Some(
            v.iter()
                .map(|t| <_ as TryInto<ScVal>>::try_into(t.clone()))
                .collect::<Result<Vec<_>, _>>() // TODO: Impl conversion from Iterator to VecM in xdrgen generated code.
                .map_err(|_| ())?
                .try_into()
                .map_err(|_| ())?,
        )))
    }
}

#[cfg(feature = "alloc")]
impl<T: TryFrom<ScVal> + Clone> TryFrom<ScVal> for Vec<T> {
    type Error = ();
    fn try_from(v: ScVal) -> Result<Self, Self::Error> {
        if let ScVal::Vec(Some(v)) = v {
            v.iter()
                .map(|t| T::try_from(t.clone()).map_err(|_| ()))
                .collect::<Result<Vec<T>, _>>()
        } else {
            Err(())
        }
    }
}

impl From<ScMap> for ScVal {
    fn from(v: ScMap) -> Self {
        ScVal::Map(Some(v))
    }
}

impl TryFrom<ScVal> for ScMap {
    type Error = ();
    fn try_from(v: ScVal) -> Result<Self, Self::Error> {
        if let ScVal::Map(Some(m)) = v {
            Ok(m)
        } else {
            Err(())
        }
    }
}

impl<K, V> TryFrom<(K, V)> for ScMapEntry
where
    K: TryInto<ScVal>,
    V: TryInto<ScVal>,
{
    type Error = ();

    fn try_from(v: (K, V)) -> Result<Self, Self::Error> {
        Ok(ScMapEntry {
            key: v.0.try_into().map_err(|_| ())?,
            val: v.1.try_into().map_err(|_| ())?,
        })
    }
}

// TODO: Add conversions from std::collections::HashMap, im_rcOrdMap, and other
// popular map types to ScMap.

impl<T: Into<ScVal>> From<Option<T>> for ScVal {
    fn from(v: Option<T>) -> Self {
        match v {
            Some(v) => v.into(),
            None => ().into(),
        }
    }
}

impl<T> From<&Option<T>> for ScVal
where
    T: Into<ScVal> + Clone,
{
    fn from(v: &Option<T>) -> Self {
        match v {
            Some(v) => v.clone().into(),
            None => ().into(),
        }
    }
}

macro_rules! impl_for_tuple {
    ( $count:literal $($typ:ident $idx:tt)+ ) => {
        #[cfg(feature = "alloc")]
        impl<$($typ),*> TryFrom<($($typ,)*)> for ScVec
        where
            $($typ: TryInto<ScVal>),*
        {
            type Error = ();
            fn try_from(v: ($($typ,)*)) -> Result<Self, Self::Error> {
                let vec: Vec<ScVal> = vec![$(v.$idx.try_into().map_err(|_| ())?),+];
                Ok(ScVec(vec.try_into()?))
            }
        }

        #[cfg(feature = "alloc")]
        impl<$($typ),*> TryFrom<&($($typ,)*)> for ScVec
        where
            $($typ: TryInto<ScVal> + Clone),*
        {
            type Error = ();
            fn try_from(v: &($($typ,)*)) -> Result<Self, Self::Error> {
                let vec: Vec<ScVal> = vec![$(v.$idx.clone().try_into().map_err(|_| ())?),+];
                Ok(ScVec(vec.try_into()?))
            }
        }

        #[cfg(feature = "alloc")]
        impl<$($typ),*> TryFrom<($($typ,)*)> for ScVal
        where
            $($typ: TryInto<ScVal>),*
        {
            type Error = ();
            fn try_from(v: ($($typ,)*)) -> Result<Self, ()> {
                Ok(ScVal::Vec(Some(<_ as TryInto<ScVec>>::try_into(v)?)))
            }
        }

        #[cfg(feature = "alloc")]
        impl<$($typ),*> TryFrom<&($($typ,)*)> for ScVal
        where
            $($typ: TryInto<ScVal> + Clone),*
        {
            type Error = ();
            fn try_from(v: &($($typ,)*)) -> Result<Self, ()> {
                Ok(ScVal::Vec(Some(<_ as TryInto<ScVec>>::try_into(v)?)))
            }
        }

        impl<$($typ),*> TryFrom<ScVec> for ($($typ,)*)
        where
            // TODO: Consider removing the Clone constraint by changing the
            // try_from to use a reference.
            $($typ: TryFrom<ScVal> + Clone),*
        {
            type Error = ();

            fn try_from(vec: ScVec) -> Result<Self, Self::Error> {
                if vec.len() != $count {
                    return Err(());
                }
                Ok((
                    $({
                        let idx: usize = $idx;
                        let val = vec[idx].clone();
                        $typ::try_from(val).map_err(|_| ())?
                    },)*
                ))
            }
        }

        impl<$($typ),*> TryFrom<ScVal> for ($($typ,)*)
        where
            $($typ: TryFrom<ScVal> + Clone),*
        {
            type Error = ();

            fn try_from(obj: ScVal) -> Result<Self, Self::Error> {
                if let ScVal::Vec(Some(vec)) = obj {
                    <_ as TryFrom<ScVec>>::try_from(vec)
                } else {
                    Err(())
                }
            }
        }
    };
}

impl_for_tuple! {  1 T0 0 }
impl_for_tuple! {  2 T0 0 T1 1 }
impl_for_tuple! {  3 T0 0 T1 1 T2 2 }
impl_for_tuple! {  4 T0 0 T1 1 T2 2 T3 3 }
impl_for_tuple! {  5 T0 0 T1 1 T2 2 T3 3 T4 4 }
impl_for_tuple! {  6 T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 }
impl_for_tuple! {  7 T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 T6 6 }
impl_for_tuple! {  8 T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 T6 6 T7 7 }
impl_for_tuple! {  9 T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 T6 6 T7 7 T8 8 }
impl_for_tuple! { 10 T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 T6 6 T7 7 T8 8 T9 9 }
impl_for_tuple! { 11 T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 T6 6 T7 7 T8 8 T9 9 T10 10 }
impl_for_tuple! { 12 T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 T6 6 T7 7 T8 8 T9 9 T10 10 T11 11 }
impl_for_tuple! { 13 T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 T6 6 T7 7 T8 8 T9 9 T10 10 T11 11 T12 12 }

#[cfg(test)]
mod test {
    use super::{int128_helpers, Int128Parts, ScVal, ScVec, UInt128Parts};

    #[test]
    fn i32_pos() {
        let v = 5;
        let val: ScVal = v.into();
        assert_eq!(val, ScVal::I32(5));
        let roundtrip: i32 = val.try_into().unwrap();
        assert_eq!(v, roundtrip);
    }

    #[test]
    fn i32_neg() {
        let v = -5;
        let val: ScVal = v.into();
        assert_eq!(val, ScVal::I32(-5));
        let roundtrip: i32 = val.try_into().unwrap();
        assert_eq!(v, roundtrip);
    }

    #[test]
    fn u32() {
        use super::ScVal;

        let v = 5u32;
        let val: ScVal = v.into();
        assert_eq!(val, ScVal::U32(5));
        let roundtrip: u32 = val.try_into().unwrap();
        assert_eq!(v, roundtrip);
    }

    #[test]
    fn i64_pos() {
        let v = 5i64;
        let val: ScVal = v.into();
        assert_eq!(val, ScVal::I64(5));
        let roundtrip: i64 = val.try_into().unwrap();
        assert_eq!(v, roundtrip);
    }

    #[test]
    fn i64_neg() {
        let v = -5i64;
        let val: ScVal = v.into();
        assert_eq!(val, ScVal::I64(-5));
        let roundtrip: i64 = val.try_into().unwrap();
        assert_eq!(v, roundtrip);
    }

    #[test]
    fn u64() {
        let v = 5u64;
        let val: ScVal = v.into();
        assert_eq!(val, ScVal::U64(5));
        let roundtrip: u64 = val.try_into().unwrap();
        assert_eq!(v, roundtrip);
    }

    #[test]
    fn u128() {
        let hi = 0x1234_5678_9abc_def0u64;
        let lo = 0xfedc_ba98_7654_3210u64;
        let u = int128_helpers::u128_from_pieces(hi, lo);
        assert_eq!(u, 0x1234_5678_9abc_def0_fedc_ba98_7654_3210);
        assert_eq!(int128_helpers::u128_hi(u), hi);
        assert_eq!(int128_helpers::u128_lo(u), lo);

        let val: ScVal = u.into();
        assert_eq!(val, ScVal::U128(UInt128Parts { hi, lo }));
        let roundtrip: u128 = val.try_into().unwrap();
        assert_eq!(u, roundtrip);
    }

    #[test]
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
    fn i128() {
        let part1 = 0x00ab_cdef_9876_5432u64; // some positive int64
        let part2 = 0xfedc_ba98_7654_3210u64; // some negative int64
        let roundtrip = |hi: i64, lo: u64, ref_i128: i128| {
            let i = int128_helpers::i128_from_pieces(hi, lo);
            assert_eq!(i, ref_i128);
            assert_eq!(int128_helpers::i128_hi(i), hi);
            assert_eq!(int128_helpers::i128_lo(i), lo);

            let val: ScVal = i.into();
            assert_eq!(val, ScVal::I128(Int128Parts { hi, lo }));
            let roundtrip: i128 = val.try_into().unwrap();
            assert_eq!(i, roundtrip);
        };
        roundtrip(
            part1 as i64,
            part1,
            0x00ab_cdef_9876_5432_00ab_cdef_9876_5432,
        );
        roundtrip(
            part2 as i64,
            part2,
            0xfedc_ba98_7654_3210_fedc_ba98_7654_3210u128 as i128,
        );
        roundtrip(
            part1 as i64,
            part2,
            0x00ab_cdef_9876_5432_fedc_ba98_7654_3210,
        );
        roundtrip(
            part2 as i64,
            part1,
            0xfedc_ba98_7654_3210_00ab_cdef_9876_5432u128 as i128,
        );
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn binary() {
        extern crate alloc;
        use alloc::vec;

        let v = [1, 2, 3, 4, 5];
        let val: ScVal = v.try_into().unwrap();
        assert_eq!(val, ScVal::Bytes(vec![1, 2, 3, 4, 5].try_into().unwrap()));

        let v = &[1, 2, 3, 4, 5];
        let val: ScVal = v.try_into().unwrap();
        assert_eq!(val, ScVal::Bytes(vec![1, 2, 3, 4, 5].try_into().unwrap()));
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn vec() {
        extern crate alloc;
        use alloc::vec;
        use alloc::vec::Vec;

        let v = vec![1, 2, 3, 4, 5];
        let val: ScVal = v.clone().try_into().unwrap();
        let roundtrip: Vec<i32> = val.try_into().unwrap();
        assert_eq!(v, roundtrip);

        let v = vec![vec![true], vec![false]];
        let val: ScVal = v.clone().try_into().unwrap();
        let roundtrip: Vec<Vec<bool>> = val.try_into().unwrap();
        assert_eq!(v, roundtrip);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn tuple() {
        extern crate alloc;
        use alloc::vec;
        use alloc::vec::Vec;
        let v = (1i32, 2i64, vec![true, false]);
        let val: ScVal = v.clone().try_into().unwrap();
        let roundtrip: (i32, i64, Vec<bool>) = val.try_into().unwrap();
        assert_eq!(v, roundtrip);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn tuple_refs() {
        extern crate alloc;
        use alloc::vec;
        use alloc::vec::Vec;
        let v = &(1i32, 2i64, vec![true, false]);
        let val: ScVal = v.try_into().unwrap();
        let roundtrip: (i32, i64, Vec<bool>) = val.try_into().unwrap();
        assert_eq!(v, &roundtrip);
    }

    #[test]
    fn option() {
        let v: Option<i64> = Some(1);
        let val: ScVal = v.into();
        assert_eq!(val, ScVal::I64(1));

        let v: Option<i64> = None;
        let val: ScVal = v.into();
        assert_eq!(val, ScVal::Void);
    }

    #[test]
    fn scval_from() {
        let _ = ScVal::from(ScVec::default());
    }
}
