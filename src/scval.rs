use crate::{Hash, PublicKey, ScHash, ScMap, ScObject, ScStatic, ScStatus, ScVal, ScVec};

#[cfg(all(not(feature = "std"), feature = "alloc"))]
extern crate alloc;
#[cfg(all(not(feature = "std"), feature = "alloc"))]
use alloc::{string::String, vec::Vec};

impl From<ScStatic> for ScVal {
    fn from(v: ScStatic) -> Self {
        Self::Static(v)
    }
}

impl From<ScObject> for ScVal {
    fn from(v: ScObject) -> Self {
        ScVal::Object(Some(v))
    }
}

impl From<ScStatus> for ScVal {
    fn from(v: ScStatus) -> Self {
        ScVal::Status(v)
    }
}

impl From<i32> for ScVal {
    fn from(v: i32) -> ScVal {
        ScVal::I32(v)
    }
}

impl From<u32> for ScVal {
    fn from(v: u32) -> ScVal {
        ScVal::U32(v)
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

impl From<u64> for ScVal {
    fn from(v: u64) -> ScVal {
        ScObject::U64(v).into()
    }
}

impl From<()> for ScVal {
    fn from(_: ()) -> Self {
        ScStatic::Void.into()
    }
}

impl From<bool> for ScVal {
    fn from(v: bool) -> Self {
        if v { ScStatic::True } else { ScStatic::False }.into()
    }
}

impl From<i64> for ScObject {
    fn from(v: i64) -> ScObject {
        ScObject::I64(v)
    }
}

impl From<u64> for ScObject {
    fn from(v: u64) -> ScObject {
        ScObject::U64(v)
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

// TODO: Map(ScMap),
// TODO: BigInt(ScBigInt),

// TODO: Reverse conversions for all types above.

impl<T: Into<ScVal>> From<Option<T>> for ScVal {
    fn from(v: Option<T>) -> Self {
        match v {
            Some(v) => v.into(),
            None => ().into(),
        }
    }
}
