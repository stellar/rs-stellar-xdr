use crate::{
    Hash, PublicKey, ScBigInt, ScHash, ScMap, ScObject, ScStatic, ScStatus, ScSymbol, ScVal, ScVec,
};

#[cfg(all(not(feature = "std"), feature = "alloc"))]
extern crate alloc;
#[cfg(all(not(feature = "std"), feature = "alloc"))]
use alloc::{string::String, vec::Vec};

#[cfg(feature = "num-bigint")]
use num_bigint::{BigInt, Sign};

impl From<ScStatic> for ScVal {
    fn from(v: ScStatic) -> Self {
        Self::Static(v)
    }
}

impl From<&ScStatic> for ScVal {
    fn from(v: &ScStatic) -> Self {
        Self::Static(*v)
    }
}

impl TryFrom<ScVal> for ScStatic {
    type Error = ();
    fn try_from(v: ScVal) -> Result<Self, Self::Error> {
        if let ScVal::Static(s) = v {
            Ok(s)
        } else {
            Err(())
        }
    }
}

impl From<ScObject> for ScVal {
    fn from(v: ScObject) -> Self {
        ScVal::Object(Some(v))
    }
}

impl TryFrom<ScVal> for ScObject {
    type Error = ();
    fn try_from(v: ScVal) -> Result<Self, Self::Error> {
        if let ScVal::Object(Some(o)) = v {
            Ok(o)
        } else {
            Err(())
        }
    }
}

impl From<ScStatus> for ScVal {
    fn from(v: ScStatus) -> Self {
        ScVal::Status(v)
    }
}

impl TryFrom<ScVal> for ScStatus {
    type Error = ();
    fn try_from(v: ScVal) -> Result<Self, Self::Error> {
        if let ScVal::Status(s) = v {
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
        if v < 0 {
            ScObject::I64(v).into()
        } else {
            ScVal::U63(v)
        }
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
        if let ScVal::U63(i) | ScVal::Object(Some(ScObject::I64(i))) = v {
            Ok(i)
        } else {
            Err(())
        }
    }
}

impl From<()> for ScVal {
    fn from(_: ()) -> Self {
        ScStatic::Void.into()
    }
}

impl From<&()> for ScVal {
    fn from(_: &()) -> Self {
        ScStatic::Void.into()
    }
}

impl TryFrom<ScVal> for () {
    type Error = ();
    fn try_from(v: ScVal) -> Result<Self, Self::Error> {
        if let ScVal::Static(ScStatic::Void) = v {
            Ok(())
        } else {
            Err(())
        }
    }
}

impl From<bool> for ScVal {
    fn from(v: bool) -> Self {
        if v { ScStatic::True } else { ScStatic::False }.into()
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
        match v {
            ScVal::Static(ScStatic::False) => Ok(false),
            ScVal::Static(ScStatic::True) => Ok(true),
            _ => Err(()),
        }
    }
}

impl From<i64> for ScObject {
    fn from(v: i64) -> Self {
        ScObject::I64(v)
    }
}

impl From<&i64> for ScObject {
    fn from(v: &i64) -> Self {
        ScObject::I64(*v)
    }
}

impl TryFrom<ScObject> for i64 {
    type Error = ();
    fn try_from(v: ScObject) -> Result<Self, Self::Error> {
        if let ScObject::I64(i) = v {
            Ok(i)
        } else {
            Err(())
        }
    }
}

impl From<u64> for ScObject {
    fn from(v: u64) -> Self {
        ScObject::U64(v)
    }
}

impl From<&u64> for ScObject {
    fn from(v: &u64) -> Self {
        ScObject::U64(*v)
    }
}

impl From<u64> for ScVal {
    fn from(v: u64) -> Self {
        <_ as Into<ScObject>>::into(v).into()
    }
}

impl TryFrom<ScObject> for u64 {
    type Error = ();
    fn try_from(v: ScObject) -> Result<Self, Self::Error> {
        if let ScObject::U64(i) = v {
            Ok(i)
        } else {
            Err(())
        }
    }
}

impl TryFrom<ScVal> for u64 {
    type Error = ();
    fn try_from(v: ScVal) -> Result<Self, Self::Error> {
        if let ScVal::Object(Some(ScObject::U64(i))) = v {
            Ok(i)
        } else {
            Err(())
        }
    }
}

impl From<ScHash> for ScObject {
    fn from(v: ScHash) -> Self {
        ScObject::Hash(v)
    }
}

impl From<ScHash> for ScVal {
    fn from(v: ScHash) -> Self {
        <_ as Into<ScObject>>::into(v).into()
    }
}

impl From<Hash> for ScHash {
    fn from(v: Hash) -> Self {
        ScHash::SchashSha256(v)
    }
}

impl From<Hash> for ScObject {
    fn from(v: Hash) -> Self {
        <_ as Into<ScHash>>::into(v).into()
    }
}

impl From<Hash> for ScVal {
    fn from(v: Hash) -> Self {
        <_ as Into<ScObject>>::into(v).into()
    }
}

impl TryFrom<ScObject> for ScHash {
    type Error = ();
    fn try_from(v: ScObject) -> Result<Self, Self::Error> {
        if let ScObject::Hash(h) = v {
            Ok(h)
        } else {
            Err(())
        }
    }
}

impl TryFrom<ScVal> for ScHash {
    type Error = ();
    fn try_from(v: ScVal) -> Result<Self, Self::Error> {
        if let ScVal::Object(Some(ScObject::Hash(h))) = v {
            Ok(h)
        } else {
            Err(())
        }
    }
}

impl TryFrom<ScHash> for Hash {
    type Error = ();
    fn try_from(v: ScHash) -> Result<Self, Self::Error> {
        let ScHash::SchashSha256(h) = v;
        Ok(h)
    }
}

impl TryFrom<ScObject> for Hash {
    type Error = ();
    fn try_from(v: ScObject) -> Result<Self, Self::Error> {
        if let ScObject::Hash(ScHash::SchashSha256(h)) = v {
            Ok(h)
        } else {
            Err(())
        }
    }
}

impl TryFrom<ScVal> for Hash {
    type Error = ();
    fn try_from(v: ScVal) -> Result<Self, Self::Error> {
        if let ScVal::Object(Some(ScObject::Hash(ScHash::SchashSha256(h)))) = v {
            Ok(h)
        } else {
            Err(())
        }
    }
}

impl From<PublicKey> for ScObject {
    fn from(v: PublicKey) -> Self {
        ScObject::PublicKey(v)
    }
}

impl From<PublicKey> for ScVal {
    fn from(v: PublicKey) -> Self {
        <_ as Into<ScObject>>::into(v).into()
    }
}

impl TryFrom<ScObject> for PublicKey {
    type Error = ();
    fn try_from(v: ScObject) -> Result<Self, Self::Error> {
        if let ScObject::PublicKey(k) = v {
            Ok(k)
        } else {
            Err(())
        }
    }
}

impl TryFrom<ScVal> for PublicKey {
    type Error = ();
    fn try_from(v: ScVal) -> Result<Self, Self::Error> {
        if let ScVal::Object(Some(ScObject::PublicKey(k))) = v {
            Ok(k)
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
    fn try_from(v: String) -> Result<Self, Self::Error> {
        Ok(ScVal::Symbol(v.try_into().map_err(|_| ())?))
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<&String> for ScVal {
    type Error = ();
    fn try_from(v: &String) -> Result<Self, Self::Error> {
        Ok(ScVal::Symbol(v.try_into().map_err(|_| ())?))
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<&str> for ScVal {
    type Error = ();
    fn try_from(v: &str) -> Result<Self, Self::Error> {
        Ok(ScVal::Symbol(v.try_into().map_err(|_| ())?))
    }
}

#[cfg(not(feature = "alloc"))]
impl TryFrom<&'static str> for ScVal {
    type Error = ();
    fn try_from(v: &'static str) -> Result<Self, Self::Error> {
        Ok(ScVal::Symbol(v.try_into().map_err(|_| ())?))
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<ScVal> for String {
    type Error = ();
    fn try_from(v: ScVal) -> Result<Self, Self::Error> {
        if let ScVal::Symbol(s) = v {
            // TODO: It might be worth distinguishing the error case where this
            // is an invalid symbol with invalid characters.
            Ok(s.into_string().map_err(|_| ())?)
        } else {
            Err(())
        }
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<Vec<u8>> for ScObject {
    type Error = ();
    fn try_from(v: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(ScObject::Binary(v.try_into().map_err(|_| ())?))
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<&Vec<u8>> for ScObject {
    type Error = ();
    fn try_from(v: &Vec<u8>) -> Result<Self, Self::Error> {
        Ok(ScObject::Binary(v.try_into().map_err(|_| ())?))
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<Vec<u8>> for ScVal {
    type Error = ();
    fn try_from(v: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(<_ as TryInto<ScObject>>::try_into(&v)?.into())
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<&Vec<u8>> for ScVal {
    type Error = ();
    fn try_from(v: &Vec<u8>) -> Result<Self, Self::Error> {
        Ok(<_ as TryInto<ScObject>>::try_into(v)?.into())
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<&[u8]> for ScObject {
    type Error = ();
    fn try_from(v: &[u8]) -> Result<Self, Self::Error> {
        Ok(ScObject::Binary(v.try_into().map_err(|_| ())?))
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<&[u8]> for ScVal {
    type Error = ();
    fn try_from(v: &[u8]) -> Result<Self, Self::Error> {
        Ok(<_ as TryInto<ScObject>>::try_into(v)?.into())
    }
}

#[cfg(not(feature = "alloc"))]
impl TryFrom<&'static [u8]> for ScObject {
    type Error = ();
    fn try_from(v: &'static [u8]) -> Result<Self, Self::Error> {
        Ok(ScObject::Binary(v.try_into().map_err(|_| ())?))
    }
}

#[cfg(not(feature = "alloc"))]
impl TryFrom<&'static [u8]> for ScVal {
    type Error = ();
    fn try_from(v: &'static [u8]) -> Result<Self, Self::Error> {
        Ok(<_ as TryInto<ScObject>>::try_into(v)?.into())
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<ScObject> for Vec<u8> {
    type Error = ();
    fn try_from(v: ScObject) -> Result<Self, Self::Error> {
        if let ScObject::Binary(b) = v {
            Ok(b.into())
        } else {
            Err(())
        }
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<&ScObject> for Vec<u8> {
    type Error = ();
    fn try_from(v: &ScObject) -> Result<Self, Self::Error> {
        if let ScObject::Binary(b) = v {
            Ok(b.into())
        } else {
            Err(())
        }
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<ScVal> for Vec<u8> {
    type Error = ();
    fn try_from(v: ScVal) -> Result<Self, Self::Error> {
        if let ScVal::Object(Some(ScObject::Binary(b))) = v {
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
        if let ScVal::Object(Some(ScObject::Binary(b))) = v {
            Ok(b.into())
        } else {
            Err(())
        }
    }
}

impl From<ScVec> for ScObject {
    fn from(v: ScVec) -> Self {
        ScObject::Vec(v)
    }
}

impl From<ScVec> for ScVal {
    fn from(v: ScVec) -> Self {
        <_ as Into<ScObject>>::into(v).into()
    }
}

#[cfg(feature = "alloc")]
impl<T: Into<ScVal>> TryFrom<Vec<T>> for ScObject {
    type Error = ();
    fn try_from(v: Vec<T>) -> Result<Self, Self::Error> {
        Ok(ScObject::Vec(
            v.into_iter()
                .map(|t| <_ as Into<ScVal>>::into(t))
                .collect::<Vec<_>>() // TODO: Impl conversion from Iterator to VecM in xdrgen generated code.
                .try_into()
                .map_err(|_| ())?,
        ))
    }
}

#[cfg(feature = "alloc")]
impl<T: Into<ScVal>> TryFrom<Vec<T>> for ScVal {
    type Error = ();
    fn try_from(v: Vec<T>) -> Result<Self, Self::Error> {
        Ok(<_ as TryInto<ScObject>>::try_into(v)?.into())
    }
}

#[cfg(feature = "alloc")]
impl<T: Into<ScVal> + Clone> TryFrom<&Vec<T>> for ScObject {
    type Error = ();
    fn try_from(v: &Vec<T>) -> Result<Self, Self::Error> {
        Ok(ScObject::Vec(
            v.iter()
                .map(|t| <_ as Into<ScVal>>::into(t.clone()))
                .collect::<Vec<_>>() // TODO: Impl conversion from Iterator to VecM in xdrgen generated code.
                .try_into()
                .map_err(|_| ())?,
        ))
    }
}

#[cfg(feature = "alloc")]
impl<'a, T: Into<ScVal>> TryFrom<&'a Vec<T>> for ScVal
where
    ScObject: TryFrom<&'a Vec<T>>,
{
    type Error = ();
    fn try_from(v: &'a Vec<T>) -> Result<Self, Self::Error> {
        Ok(<_ as TryInto<ScObject>>::try_into(v)
            .map_err(|_| ())?
            .into())
    }
}

#[cfg(feature = "alloc")]
impl<T: Into<ScVal> + Clone> TryFrom<&[T]> for ScObject {
    type Error = ();
    fn try_from(v: &[T]) -> Result<Self, Self::Error> {
        Ok(ScObject::Vec(
            v.iter()
                .map(|t| <_ as Into<ScVal>>::into(t.clone()))
                .collect::<Vec<_>>() // TODO: Impl conversion from Iterator to VecM in xdrgen generated code.
                .try_into()
                .map_err(|_| ())?,
        ))
    }
}

#[cfg(feature = "alloc")]
impl<'a, T: Into<ScVal>> TryFrom<&'a [T]> for ScVal
where
    ScObject: TryFrom<&'a [T]>,
{
    type Error = ();
    fn try_from(v: &'a [T]) -> Result<Self, Self::Error> {
        Ok(<_ as TryInto<ScObject>>::try_into(v)
            .map_err(|_| ())?
            .into())
    }
}

#[cfg(feature = "alloc")]
impl<T: TryFrom<ScVal> + Clone> TryFrom<ScObject> for Vec<T> {
    type Error = ();
    fn try_from(v: ScObject) -> Result<Self, Self::Error> {
        if let ScObject::Vec(v) = v {
            v.iter()
                .map(|t| T::try_from(t.clone()).map_err(|_| ()))
                .collect::<Result<Vec<T>, _>>()
        } else {
            Err(())
        }
    }
}

#[cfg(feature = "alloc")]
impl<T: TryFrom<ScVal> + Clone> TryFrom<ScVal> for Vec<T> {
    type Error = ();
    fn try_from(v: ScVal) -> Result<Self, Self::Error> {
        if let ScVal::Object(Some(o)) = v {
            <_ as TryInto<Self>>::try_into(o)
        } else {
            Err(())
        }
    }
}

impl From<ScMap> for ScObject {
    fn from(v: ScMap) -> Self {
        ScObject::Map(v)
    }
}

impl From<ScMap> for ScVal {
    fn from(v: ScMap) -> Self {
        <_ as Into<ScObject>>::into(v).into()
    }
}

impl TryFrom<ScObject> for ScMap {
    type Error = ();
    fn try_from(v: ScObject) -> Result<Self, Self::Error> {
        if let ScObject::Map(m) = v {
            Ok(m)
        } else {
            Err(())
        }
    }
}

impl TryFrom<ScVal> for ScMap {
    type Error = ();
    fn try_from(v: ScVal) -> Result<Self, Self::Error> {
        if let ScVal::Object(Some(o)) = v {
            <_ as TryInto<ScMap>>::try_into(o)
        } else {
            Err(())
        }
    }
}

// TODO: Add conversions from std::collections::HashMap, im_rcOrdMap, and other
// popular map types to ScMap.

impl From<ScBigInt> for ScObject {
    fn from(v: ScBigInt) -> Self {
        ScObject::BigInt(v)
    }
}

impl From<ScBigInt> for ScVal {
    fn from(v: ScBigInt) -> Self {
        <_ as Into<ScObject>>::into(v).into()
    }
}

#[cfg(feature = "num-bigint")]
impl TryFrom<BigInt> for ScBigInt {
    type Error = ();
    fn try_from(v: BigInt) -> Result<Self, Self::Error> {
        Ok(match v.to_bytes_be() {
            (Sign::NoSign, _) => ScBigInt::Zero,
            (Sign::Plus, bytes) => ScBigInt::Positive(bytes.try_into().map_err(|_| ())?),
            (Sign::Minus, bytes) => ScBigInt::Negative(bytes.try_into().map_err(|_| ())?),
        })
    }
}

#[cfg(feature = "num-bigint")]
impl TryFrom<BigInt> for ScObject {
    type Error = ();
    fn try_from(v: BigInt) -> Result<Self, Self::Error> {
        Ok(<_ as TryInto<ScBigInt>>::try_into(v)?.into())
    }
}

#[cfg(feature = "num-bigint")]
impl TryFrom<BigInt> for ScVal {
    type Error = ();
    fn try_from(v: BigInt) -> Result<Self, Self::Error> {
        Ok(<_ as TryInto<ScObject>>::try_into(v)?.into())
    }
}

#[cfg(feature = "num-bigint")]
impl From<ScBigInt> for BigInt {
    fn from(v: ScBigInt) -> Self {
        match v {
            ScBigInt::Zero => 0u32.into(),
            ScBigInt::Positive(bytes) => BigInt::from_bytes_be(Sign::Plus, &bytes),
            ScBigInt::Negative(bytes) => BigInt::from_bytes_be(Sign::Minus, &bytes),
        }
    }
}

#[cfg(feature = "num-bigint")]
impl TryFrom<ScObject> for BigInt {
    type Error = ();
    fn try_from(v: ScObject) -> Result<Self, Self::Error> {
        if let ScObject::BigInt(b) = v {
            Ok(<_ as TryInto<BigInt>>::try_into(b).map_err(|_| ())?)
        } else {
            Err(())
        }
    }
}

#[cfg(feature = "num-bigint")]
impl TryFrom<ScVal> for BigInt {
    type Error = ();
    fn try_from(v: ScVal) -> Result<Self, Self::Error> {
        if let ScVal::Object(Some(o)) = v {
            Ok(<_ as TryInto<BigInt>>::try_into(o).map_err(|_| ())?)
        } else {
            Err(())
        }
    }
}

impl<T: Into<ScVal>> From<Option<T>> for ScVal {
    fn from(v: Option<T>) -> Self {
        match v {
            Some(v) => v.into(),
            None => ().into(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{ScObject, ScVal};

    #[test]
    fn i32_pos() {
        let v = 5;
        let val: ScVal = v.try_into().unwrap();
        assert_eq!(val, ScVal::I32(5));
        let roundtrip: i32 = val.try_into().unwrap();
        assert_eq!(v, roundtrip);
    }

    #[test]
    fn i32_neg() {
        let v = -5;
        let val: ScVal = v.try_into().unwrap();
        assert_eq!(val, ScVal::I32(-5));
        let roundtrip: i32 = val.try_into().unwrap();
        assert_eq!(v, roundtrip);
    }

    #[test]
    fn u32() {
        use crate::ScVal;

        let v = 5u32;
        let val: ScVal = v.try_into().unwrap();
        assert_eq!(val, ScVal::U32(5));
        let roundtrip: u32 = val.try_into().unwrap();
        assert_eq!(v, roundtrip);
    }

    #[test]
    fn i64_pos() {
        let v = 5i64;
        let val: ScVal = v.try_into().unwrap();
        assert_eq!(val, ScVal::U63(5));
        let roundtrip: i64 = val.try_into().unwrap();
        assert_eq!(v, roundtrip);
    }

    #[test]
    fn i64_neg() {
        let v = -5i64;
        let val: ScVal = v.try_into().unwrap();
        assert_eq!(val, ScVal::Object(Some(ScObject::I64(-5))));
        let roundtrip: i64 = val.try_into().unwrap();
        assert_eq!(v, roundtrip);
    }

    #[test]
    fn u64() {
        let v = 5u64;
        let val: ScVal = v.try_into().unwrap();
        assert_eq!(val, ScVal::Object(Some(ScObject::U64(5))));
        let roundtrip: u64 = val.try_into().unwrap();
        assert_eq!(v, roundtrip);
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
    }

    #[cfg(feature = "num-bigint")]
    #[test]
    fn bigint() {
        use core::str::FromStr;
        use num_bigint::BigInt;

        let v = BigInt::from_str("18446744073709551616").unwrap();
        let val: ScVal = v.clone().try_into().unwrap();
        let roundtrip: BigInt = val.try_into().unwrap();
        assert_eq!(v, roundtrip);
    }
}
