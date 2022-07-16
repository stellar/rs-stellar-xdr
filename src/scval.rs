use crate::{Hash, PublicKey, ScBigInt, ScHash, ScMap, ScObject, ScStatic, ScStatus, ScVal, ScVec};

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

// TODO: Reverse conditions for ScVal/etc => ScStatic.

impl From<ScObject> for ScVal {
    fn from(v: ScObject) -> Self {
        ScVal::Object(Some(v))
    }
}

// TODO: Reverse conditions for ScVal/etc => ScObject.

impl From<ScStatus> for ScVal {
    fn from(v: ScStatus) -> Self {
        ScVal::Status(v)
    }
}

// TODO: Reverse conditions for ScVal/etc => ScStatus.

impl From<i32> for ScVal {
    fn from(v: i32) -> ScVal {
        ScVal::I32(v)
    }
}

// TODO: Reverse conditions for ScVal/etc => i32.

impl From<u32> for ScVal {
    fn from(v: u32) -> ScVal {
        ScVal::U32(v)
    }
}

// TODO: Reverse conditions for ScVal/etc => u32.

impl From<i64> for ScVal {
    fn from(v: i64) -> ScVal {
        if v < 0 {
            ScObject::I64(v).into()
        } else {
            ScVal::U63(v)
        }
    }
}

// TODO: Reverse conditions for ScVal/etc => i64.

impl From<u64> for ScVal {
    fn from(v: u64) -> ScVal {
        ScObject::U64(v).into()
    }
}

// TODO: Reverse conditions for ScVal/etc => u64.

impl From<()> for ScVal {
    fn from(_: ()) -> Self {
        ScStatic::Void.into()
    }
}

// TODO: Reverse conditions for ScVal/etc => ().

impl From<bool> for ScVal {
    fn from(v: bool) -> Self {
        if v { ScStatic::True } else { ScStatic::False }.into()
    }
}

// TODO: Reverse conditions for ScVal/etc => bool.

impl From<i64> for ScObject {
    fn from(v: i64) -> ScObject {
        ScObject::I64(v)
    }
}

// TODO: Reverse conditions for ScVal/etc => i64.

impl From<u64> for ScObject {
    fn from(v: u64) -> ScObject {
        ScObject::U64(v)
    }
}

// TODO: Reverse conditions for ScVal/etc => u64.

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

// TODO: Reverse conditions for ScVal/etc => ScHash/Hash.

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

// TODO: Reverse conditions for ScVal/etc => PublicKey.

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

// TODO: Reverse conditions for ScVal/etc => String/&str/etc.

#[cfg(feature = "alloc")]
impl TryFrom<Vec<u8>> for ScObject {
    type Error = ();
    fn try_from(v: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(ScObject::Binary(v.try_into().map_err(|_| ())?))
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<Vec<u8>> for ScVal {
    type Error = ();
    fn try_from(v: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(<_ as TryInto<ScObject>>::try_into(v)?.into())
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

// TODO: Reverse conditions for ScVal/etc => Binary/etc.

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

// TODO: Reverse conditions for ScVal/etc => Vec/etc.

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

// TODO: Is there a way to provide this conversion without recursion?
// impl<T: TryFrom<ScVal>> TryFrom<ScVal> for Option<T> {
//     type Error = T::Error;
//     fn try_from(v: ScVal) -> Result<Self, Self::Error> {
//         match v {
//             ScVal::Static(ScStatic::Void) => Ok(None),
//             _ => <_ as TryFrom<ScVal>>::try_from(v),
//         }
//     }
// }
