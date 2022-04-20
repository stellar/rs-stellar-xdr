// Module  is generated from:
//  xdr/Stellar-SCP.x
//  xdr/Stellar-ledger-entries.x
//  xdr/Stellar-ledger.x
//  xdr/Stellar-overlay.x
//  xdr/Stellar-transaction.x
//  xdr/Stellar-types.x

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::{
    fmt::Debug,
    io,
    io::{Cursor, Read, Write},
    mem::{transmute_copy, MaybeUninit},
    num::TryFromIntError,
};

#[derive(Debug)]
pub enum Error {
    Invalid,
    IO(io::Error),
    OutOfRange(TryFromIntError),
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IO(e)
    }
}

impl From<TryFromIntError> for Error {
    fn from(e: TryFromIntError) -> Self {
        Error::OutOfRange(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub trait ReadXDR
where
    Self: Sized,
{
    fn read_xdr(r: &mut impl Read) -> Result<Self>;

    fn read_xdr_into(&mut self, r: &mut impl Read) -> Result<()> {
        *self = Self::read_xdr(r)?;
        Ok(())
    }

    fn from_xdr_base64(b64: String) -> Result<Self> {
        let mut b64_reader = Cursor::new(b64);
        let mut dec = base64::read::DecoderReader::new(&mut b64_reader, base64::STANDARD);
        let t = Self::read_xdr(&mut dec)?;
        Ok(t)
    }
}

pub trait WriteXDR {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()>;

    fn to_xdr_base64(&self) -> Result<String> {
        let mut enc = base64::write::EncoderStringWriter::new(base64::STANDARD);
        self.write_xdr(&mut enc)?;
        let b64 = enc.into_inner();
        Ok(b64)
    }
}

impl ReadXDR for i32 {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let i = r.read_i32::<BigEndian>()?;
        Ok(i)
    }
}

impl WriteXDR for i32 {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        w.write_i32::<BigEndian>(*self).map_err(Error::IO)?;
        Ok(())
    }
}

impl ReadXDR for u32 {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let i = r.read_u32::<BigEndian>()?;
        Ok(i)
    }
}

impl WriteXDR for u32 {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        w.write_u32::<BigEndian>(*self)?;
        Ok(())
    }
}

impl ReadXDR for i64 {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let i = r.read_i64::<BigEndian>()?;
        Ok(i)
    }
}

impl WriteXDR for i64 {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        w.write_i64::<BigEndian>(*self)?;
        Ok(())
    }
}

impl ReadXDR for u64 {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let i = r.read_u64::<BigEndian>()?;
        Ok(i)
    }
}

impl WriteXDR for u64 {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        w.write_u64::<BigEndian>(*self)?;
        Ok(())
    }
}

impl ReadXDR for bool {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let i = r.read_u32::<BigEndian>()?;
        let b = i == 1;
        Ok(b)
    }
}

impl WriteXDR for bool {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i = if *self { 1 } else { 0 };
        w.write_u32::<BigEndian>(i)?;
        Ok(())
    }
}

impl<T: ReadXDR> ReadXDR for Option<T> {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let i: u32 = r.read_u32::<BigEndian>()?;
        match i {
            0 => Ok(None),
            1 => {
                let t = T::read_xdr(r)?;
                Ok(Some(t))
            }
            _ => Err(Error::Invalid),
        }
    }
}

impl<T: WriteXDR> WriteXDR for Option<T> {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        if let Some(t) = self {
            w.write_u32::<BigEndian>(1)?;
            t.write_xdr(w)?;
        } else {
            w.write_u32::<BigEndian>(0)?;
        }
        Ok(())
    }
}

impl<T: ReadXDR> ReadXDR for Box<T> {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let t = T::read_xdr(r)?;
        Ok(Box::new(t))
    }
}

impl<T: WriteXDR> WriteXDR for Box<T> {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        T::write_xdr(self, w)?;
        Ok(())
    }
}

impl ReadXDR for () {
    fn read_xdr(_r: &mut impl Read) -> Result<Self> {
        Ok(())
    }
}

impl WriteXDR for () {
    fn write_xdr(&self, _w: &mut impl Write) -> Result<()> {
        Ok(())
    }
}

impl ReadXDR for Vec<u8> {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let len: u32 = u32::read_xdr(r)?;
        // TODO: Error on length greater than max length.

        let mut vec = vec![0u8; len as usize];
        r.read_exact(&mut vec)?;

        let pad_len = (4 - (len % 4)) % 4;
        let mut pad = vec![0u8; pad_len as usize];
        r.read_exact(&mut pad)?;

        Ok(vec)
    }
}

impl WriteXDR for Vec<u8> {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let len: u32 = self.len().try_into()?;
        // TODO: Error on length greater than max length.
        w.write_u32::<BigEndian>(len)?;

        w.write_all(self)?;

        let pad_len = (4 - (len % 4)) % 4;
        let mut pad = vec![0u8; pad_len as usize];
        w.write_all(&mut pad)?;

        Ok(())
    }
}

impl<T: ReadXDR> ReadXDR for Vec<T> {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let len = u32::read_xdr(r)?;
        // TODO: Error on length greater than max length.

        let mut vec = Vec::with_capacity(len.try_into()?);
        for _ in 0..len {
            let t = T::read_xdr(r)?;
            vec.push(t);
        }

        Ok(vec)
    }
}

impl<T: WriteXDR> WriteXDR for Vec<T> {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let len: u32 = self.len().try_into()?;
        // TODO: Error on length greater than max length.
        w.write_u32::<BigEndian>(len)?;

        for t in self.iter() {
            t.write_xdr(w)?;
        }

        Ok(())
    }
}

impl<const N: usize> ReadXDR for [u8; N] {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let mut arr = [0u8; N];
        r.read_exact(&mut arr)?;
        Ok(arr)
    }
}

impl<const N: usize> WriteXDR for [u8; N] {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        w.write_all(self)?;
        Ok(())
    }
}

impl<T: ReadXDR + Clone, const N: usize> ReadXDR for [T; N] {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        // TODO: Make this not horrible. Ideally we could use [(); N].try_map
        // here, but it is still an unstable feature. Or maybe using fixed size
        // arrays for fixed sized data sets is more trouble than it is worth.
        let arr = {
            let mut arr: [MaybeUninit<T>; N] = unsafe { MaybeUninit::uninit().assume_init() };

            for e in arr.iter_mut() {
                let t = T::read_xdr(r)?;
                *e = MaybeUninit::new(t);
            }

            unsafe { transmute_copy::<_, [T; N]>(&arr) }
        };
        Ok(arr)
    }
}

impl<T: WriteXDR + Clone, const N: usize> WriteXDR for [T; N] {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        for t in self {
            t.write_xdr(w)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    // TODO: Write tests.
}

// Value is an XDR Typedef defines as:
//
//   typedef opaque Value<>;
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Value(pub Vec<u8>);

impl From<Value> for Vec<u8> {
    fn from(x: Value) -> Self {
        x.0
    }
}

impl From<Vec<u8>> for Value {
    fn from(x: Vec<u8>) -> Self {
        Value(x)
    }
}

impl ReadXDR for Value {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let i = <Vec<u8> as ReadXDR>::read_xdr(r)?;
        let v = Value(i);
        Ok(v)
    }
}

impl WriteXDR for Value {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.0.write_xdr(w)
    }
}

// ScpBallot is an XDR Struct defines as:
//
//   struct SCPBallot
//    {
//        uint32 counter; // n
//        Value value;    // x
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScpBallot {
    pub counter: Uint32,
    pub value: Value,
}

impl ReadXDR for ScpBallot {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            counter: <Uint32 as ReadXDR>::read_xdr(r)?,
            value: <Value as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for ScpBallot {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.counter.write_xdr(w)?;
        self.value.write_xdr(w)?;
        Ok(())
    }
}

// ScpStatementType is an XDR Enum defines as:
//
//   enum SCPStatementType
//    {
//        SCP_ST_PREPARE = 0,
//        SCP_ST_CONFIRM = 1,
//        SCP_ST_EXTERNALIZE = 2,
//        SCP_ST_NOMINATE = 3
//    };
//
// enum
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum ScpStatementType {
    ScpStPrepare = 0,
    ScpStConfirm = 1,
    ScpStExternalize = 2,
    ScpStNominate = 3,
}

impl TryFrom<i32> for ScpStatementType {
    type Error = Error;

    fn try_from(i: i32) -> std::result::Result<Self, Self::Error> {
        let e = match i {
            0 => ScpStatementType::ScpStPrepare,
            1 => ScpStatementType::ScpStConfirm,
            2 => ScpStatementType::ScpStExternalize,
            3 => ScpStatementType::ScpStNominate,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<ScpStatementType> for i32 {
    fn from(e: ScpStatementType) -> Self {
        e as Self
    }
}

impl ReadXDR for ScpStatementType {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXDR for ScpStatementType {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// ScpNomination is an XDR Struct defines as:
//
//   struct SCPNomination
//    {
//        Hash quorumSetHash; // D
//        Value votes<>;      // X
//        Value accepted<>;   // Y
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScpNomination {
    pub quorum_set_hash: Hash,
    pub votes: Vec<Value>,
    pub accepted: Vec<Value>,
}

impl ReadXDR for ScpNomination {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            quorum_set_hash: <Hash as ReadXDR>::read_xdr(r)?,
            votes: <Vec<Value> as ReadXDR>::read_xdr(r)?,
            accepted: <Vec<Value> as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for ScpNomination {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.quorum_set_hash.write_xdr(w)?;
        self.votes.write_xdr(w)?;
        self.accepted.write_xdr(w)?;
        Ok(())
    }
}

// ScpStatementPrepare is an XDR NestedStruct defines as:
//
//   struct
//            {
//                Hash quorumSetHash;       // D
//                SCPBallot ballot;         // b
//                SCPBallot* prepared;      // p
//                SCPBallot* preparedPrime; // p'
//                uint32 nC;                // c.n
//                uint32 nH;                // h.n
//            }
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScpStatementPrepare {
    pub quorum_set_hash: Hash,
    pub ballot: ScpBallot,
    pub prepared: Option<ScpBallot>,
    pub prepared_prime: Option<ScpBallot>,
    pub n_c: Uint32,
    pub n_h: Uint32,
}

impl ReadXDR for ScpStatementPrepare {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            quorum_set_hash: <Hash as ReadXDR>::read_xdr(r)?,
            ballot: <ScpBallot as ReadXDR>::read_xdr(r)?,
            prepared: <Option<ScpBallot> as ReadXDR>::read_xdr(r)?,
            prepared_prime: <Option<ScpBallot> as ReadXDR>::read_xdr(r)?,
            n_c: <Uint32 as ReadXDR>::read_xdr(r)?,
            n_h: <Uint32 as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for ScpStatementPrepare {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.quorum_set_hash.write_xdr(w)?;
        self.ballot.write_xdr(w)?;
        self.prepared.write_xdr(w)?;
        self.prepared_prime.write_xdr(w)?;
        self.n_c.write_xdr(w)?;
        self.n_h.write_xdr(w)?;
        Ok(())
    }
}

// ScpStatementConfirm is an XDR NestedStruct defines as:
//
//   struct
//            {
//                SCPBallot ballot;   // b
//                uint32 nPrepared;   // p.n
//                uint32 nCommit;     // c.n
//                uint32 nH;          // h.n
//                Hash quorumSetHash; // D
//            }
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScpStatementConfirm {
    pub ballot: ScpBallot,
    pub n_prepared: Uint32,
    pub n_commit: Uint32,
    pub n_h: Uint32,
    pub quorum_set_hash: Hash,
}

impl ReadXDR for ScpStatementConfirm {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            ballot: <ScpBallot as ReadXDR>::read_xdr(r)?,
            n_prepared: <Uint32 as ReadXDR>::read_xdr(r)?,
            n_commit: <Uint32 as ReadXDR>::read_xdr(r)?,
            n_h: <Uint32 as ReadXDR>::read_xdr(r)?,
            quorum_set_hash: <Hash as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for ScpStatementConfirm {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.ballot.write_xdr(w)?;
        self.n_prepared.write_xdr(w)?;
        self.n_commit.write_xdr(w)?;
        self.n_h.write_xdr(w)?;
        self.quorum_set_hash.write_xdr(w)?;
        Ok(())
    }
}

// ScpStatementExternalize is an XDR NestedStruct defines as:
//
//   struct
//            {
//                SCPBallot commit;         // c
//                uint32 nH;                // h.n
//                Hash commitQuorumSetHash; // D used before EXTERNALIZE
//            }
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScpStatementExternalize {
    pub commit: ScpBallot,
    pub n_h: Uint32,
    pub commit_quorum_set_hash: Hash,
}

impl ReadXDR for ScpStatementExternalize {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            commit: <ScpBallot as ReadXDR>::read_xdr(r)?,
            n_h: <Uint32 as ReadXDR>::read_xdr(r)?,
            commit_quorum_set_hash: <Hash as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for ScpStatementExternalize {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.commit.write_xdr(w)?;
        self.n_h.write_xdr(w)?;
        self.commit_quorum_set_hash.write_xdr(w)?;
        Ok(())
    }
}

// ScpStatementPledges is an XDR NestedUnion defines as:
//
//   union switch (SCPStatementType type)
//        {
//        case SCP_ST_PREPARE:
//            struct
//            {
//                Hash quorumSetHash;       // D
//                SCPBallot ballot;         // b
//                SCPBallot* prepared;      // p
//                SCPBallot* preparedPrime; // p'
//                uint32 nC;                // c.n
//                uint32 nH;                // h.n
//            } prepare;
//        case SCP_ST_CONFIRM:
//            struct
//            {
//                SCPBallot ballot;   // b
//                uint32 nPrepared;   // p.n
//                uint32 nCommit;     // c.n
//                uint32 nH;          // h.n
//                Hash quorumSetHash; // D
//            } confirm;
//        case SCP_ST_EXTERNALIZE:
//            struct
//            {
//                SCPBallot commit;         // c
//                uint32 nH;                // h.n
//                Hash commitQuorumSetHash; // D used before EXTERNALIZE
//            } externalize;
//        case SCP_ST_NOMINATE:
//            SCPNomination nominate;
//        }
//
// union with discriminant ScpStatementType
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ScpStatementPledges {
    ScpStPrepare(ScpStatementPrepare),
    ScpStConfirm(ScpStatementConfirm),
    ScpStExternalize(ScpStatementExternalize),
    ScpStNominate(ScpNomination),
}

impl ScpStatementPledges {
    fn discriminant(&self) -> ScpStatementType {
        match self {
            Self::ScpStPrepare(_) => ScpStatementType::ScpStPrepare,
            Self::ScpStConfirm(_) => ScpStatementType::ScpStConfirm,
            Self::ScpStExternalize(_) => ScpStatementType::ScpStExternalize,
            Self::ScpStNominate(_) => ScpStatementType::ScpStNominate,
        }
    }
}

impl ReadXDR for ScpStatementPledges {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: ScpStatementType = <ScpStatementType as ReadXDR>::read_xdr(r)?;
        let v = match dv {
            ScpStatementType::ScpStPrepare => {
                Self::ScpStPrepare(<ScpStatementPrepare as ReadXDR>::read_xdr(r)?)
            }
            ScpStatementType::ScpStConfirm => {
                Self::ScpStConfirm(<ScpStatementConfirm as ReadXDR>::read_xdr(r)?)
            }
            ScpStatementType::ScpStExternalize => {
                Self::ScpStExternalize(<ScpStatementExternalize as ReadXDR>::read_xdr(r)?)
            }
            ScpStatementType::ScpStNominate => {
                Self::ScpStNominate(<ScpNomination as ReadXDR>::read_xdr(r)?)
            }
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for ScpStatementPledges {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::ScpStPrepare(v) => v.write_xdr(w)?,
            Self::ScpStConfirm(v) => v.write_xdr(w)?,
            Self::ScpStExternalize(v) => v.write_xdr(w)?,
            Self::ScpStNominate(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// ScpStatement is an XDR Struct defines as:
//
//   struct SCPStatement
//    {
//        NodeID nodeID;    // v
//        uint64 slotIndex; // i
//
//        union switch (SCPStatementType type)
//        {
//        case SCP_ST_PREPARE:
//            struct
//            {
//                Hash quorumSetHash;       // D
//                SCPBallot ballot;         // b
//                SCPBallot* prepared;      // p
//                SCPBallot* preparedPrime; // p'
//                uint32 nC;                // c.n
//                uint32 nH;                // h.n
//            } prepare;
//        case SCP_ST_CONFIRM:
//            struct
//            {
//                SCPBallot ballot;   // b
//                uint32 nPrepared;   // p.n
//                uint32 nCommit;     // c.n
//                uint32 nH;          // h.n
//                Hash quorumSetHash; // D
//            } confirm;
//        case SCP_ST_EXTERNALIZE:
//            struct
//            {
//                SCPBallot commit;         // c
//                uint32 nH;                // h.n
//                Hash commitQuorumSetHash; // D used before EXTERNALIZE
//            } externalize;
//        case SCP_ST_NOMINATE:
//            SCPNomination nominate;
//        }
//        pledges;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScpStatement {
    pub node_id: NodeId,
    pub slot_index: Uint64,
    pub pledges: ScpStatementPledges,
}

impl ReadXDR for ScpStatement {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            node_id: <NodeId as ReadXDR>::read_xdr(r)?,
            slot_index: <Uint64 as ReadXDR>::read_xdr(r)?,
            pledges: <ScpStatementPledges as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for ScpStatement {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.node_id.write_xdr(w)?;
        self.slot_index.write_xdr(w)?;
        self.pledges.write_xdr(w)?;
        Ok(())
    }
}

// ScpEnvelope is an XDR Struct defines as:
//
//   struct SCPEnvelope
//    {
//        SCPStatement statement;
//        Signature signature;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScpEnvelope {
    pub statement: ScpStatement,
    pub signature: Signature,
}

impl ReadXDR for ScpEnvelope {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            statement: <ScpStatement as ReadXDR>::read_xdr(r)?,
            signature: <Signature as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for ScpEnvelope {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.statement.write_xdr(w)?;
        self.signature.write_xdr(w)?;
        Ok(())
    }
}

// ScpQuorumSet is an XDR Struct defines as:
//
//   struct SCPQuorumSet
//    {
//        uint32 threshold;
//        NodeID validators<>;
//        SCPQuorumSet innerSets<>;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScpQuorumSet {
    pub threshold: Uint32,
    pub validators: Vec<NodeId>,
    pub inner_sets: Vec<ScpQuorumSet>,
}

impl ReadXDR for ScpQuorumSet {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            threshold: <Uint32 as ReadXDR>::read_xdr(r)?,
            validators: <Vec<NodeId> as ReadXDR>::read_xdr(r)?,
            inner_sets: <Vec<ScpQuorumSet> as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for ScpQuorumSet {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.threshold.write_xdr(w)?;
        self.validators.write_xdr(w)?;
        self.inner_sets.write_xdr(w)?;
        Ok(())
    }
}

// AccountId is an XDR Typedef defines as:
//
//   typedef PublicKey AccountID;
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AccountId(pub PublicKey);

impl From<AccountId> for PublicKey {
    fn from(x: AccountId) -> Self {
        x.0
    }
}

impl From<PublicKey> for AccountId {
    fn from(x: PublicKey) -> Self {
        AccountId(x)
    }
}

impl ReadXDR for AccountId {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let i = <PublicKey as ReadXDR>::read_xdr(r)?;
        let v = AccountId(i);
        Ok(v)
    }
}

impl WriteXDR for AccountId {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.0.write_xdr(w)
    }
}

// Thresholds is an XDR Typedef defines as:
//
//   typedef opaque Thresholds[4];
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Thresholds(pub [u8; 4]);

impl From<Thresholds> for [u8; 4] {
    fn from(x: Thresholds) -> Self {
        x.0
    }
}

impl From<[u8; 4]> for Thresholds {
    fn from(x: [u8; 4]) -> Self {
        Thresholds(x)
    }
}

impl ReadXDR for Thresholds {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let i = <[u8; 4] as ReadXDR>::read_xdr(r)?;
        let v = Thresholds(i);
        Ok(v)
    }
}

impl WriteXDR for Thresholds {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.0.write_xdr(w)
    }
}

// String32 is an XDR Typedef defines as:
//
//   typedef string string32<32>;
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct String32(pub Vec<u8>);

impl From<String32> for Vec<u8> {
    fn from(x: String32) -> Self {
        x.0
    }
}

impl From<Vec<u8>> for String32 {
    fn from(x: Vec<u8>) -> Self {
        String32(x)
    }
}

impl ReadXDR for String32 {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let i = <Vec<u8> as ReadXDR>::read_xdr(r)?;
        let v = String32(i);
        Ok(v)
    }
}

impl WriteXDR for String32 {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.0.write_xdr(w)
    }
}

// String64 is an XDR Typedef defines as:
//
//   typedef string string64<64>;
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct String64(pub Vec<u8>);

impl From<String64> for Vec<u8> {
    fn from(x: String64) -> Self {
        x.0
    }
}

impl From<Vec<u8>> for String64 {
    fn from(x: Vec<u8>) -> Self {
        String64(x)
    }
}

impl ReadXDR for String64 {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let i = <Vec<u8> as ReadXDR>::read_xdr(r)?;
        let v = String64(i);
        Ok(v)
    }
}

impl WriteXDR for String64 {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.0.write_xdr(w)
    }
}

// SequenceNumber is an XDR Typedef defines as:
//
//   typedef int64 SequenceNumber;
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SequenceNumber(pub Int64);

impl From<SequenceNumber> for Int64 {
    fn from(x: SequenceNumber) -> Self {
        x.0
    }
}

impl From<Int64> for SequenceNumber {
    fn from(x: Int64) -> Self {
        SequenceNumber(x)
    }
}

impl ReadXDR for SequenceNumber {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let i = <Int64 as ReadXDR>::read_xdr(r)?;
        let v = SequenceNumber(i);
        Ok(v)
    }
}

impl WriteXDR for SequenceNumber {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.0.write_xdr(w)
    }
}

// TimePoint is an XDR Typedef defines as:
//
//   typedef uint64 TimePoint;
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TimePoint(pub Uint64);

impl From<TimePoint> for Uint64 {
    fn from(x: TimePoint) -> Self {
        x.0
    }
}

impl From<Uint64> for TimePoint {
    fn from(x: Uint64) -> Self {
        TimePoint(x)
    }
}

impl ReadXDR for TimePoint {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let i = <Uint64 as ReadXDR>::read_xdr(r)?;
        let v = TimePoint(i);
        Ok(v)
    }
}

impl WriteXDR for TimePoint {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.0.write_xdr(w)
    }
}

// Duration is an XDR Typedef defines as:
//
//   typedef uint64 Duration;
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Duration(pub Uint64);

impl From<Duration> for Uint64 {
    fn from(x: Duration) -> Self {
        x.0
    }
}

impl From<Uint64> for Duration {
    fn from(x: Uint64) -> Self {
        Duration(x)
    }
}

impl ReadXDR for Duration {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let i = <Uint64 as ReadXDR>::read_xdr(r)?;
        let v = Duration(i);
        Ok(v)
    }
}

impl WriteXDR for Duration {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.0.write_xdr(w)
    }
}

// DataValue is an XDR Typedef defines as:
//
//   typedef opaque DataValue<64>;
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DataValue(pub Vec<u8>);

impl From<DataValue> for Vec<u8> {
    fn from(x: DataValue) -> Self {
        x.0
    }
}

impl From<Vec<u8>> for DataValue {
    fn from(x: Vec<u8>) -> Self {
        DataValue(x)
    }
}

impl ReadXDR for DataValue {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let i = <Vec<u8> as ReadXDR>::read_xdr(r)?;
        let v = DataValue(i);
        Ok(v)
    }
}

impl WriteXDR for DataValue {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.0.write_xdr(w)
    }
}

// PoolId is an XDR Typedef defines as:
//
//   typedef Hash PoolID;
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PoolId(pub Hash);

impl From<PoolId> for Hash {
    fn from(x: PoolId) -> Self {
        x.0
    }
}

impl From<Hash> for PoolId {
    fn from(x: Hash) -> Self {
        PoolId(x)
    }
}

impl ReadXDR for PoolId {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let i = <Hash as ReadXDR>::read_xdr(r)?;
        let v = PoolId(i);
        Ok(v)
    }
}

impl WriteXDR for PoolId {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.0.write_xdr(w)
    }
}

// AssetCode4 is an XDR Typedef defines as:
//
//   typedef opaque AssetCode4[4];
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AssetCode4(pub [u8; 4]);

impl From<AssetCode4> for [u8; 4] {
    fn from(x: AssetCode4) -> Self {
        x.0
    }
}

impl From<[u8; 4]> for AssetCode4 {
    fn from(x: [u8; 4]) -> Self {
        AssetCode4(x)
    }
}

impl ReadXDR for AssetCode4 {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let i = <[u8; 4] as ReadXDR>::read_xdr(r)?;
        let v = AssetCode4(i);
        Ok(v)
    }
}

impl WriteXDR for AssetCode4 {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.0.write_xdr(w)
    }
}

// AssetCode12 is an XDR Typedef defines as:
//
//   typedef opaque AssetCode12[12];
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AssetCode12(pub [u8; 12]);

impl From<AssetCode12> for [u8; 12] {
    fn from(x: AssetCode12) -> Self {
        x.0
    }
}

impl From<[u8; 12]> for AssetCode12 {
    fn from(x: [u8; 12]) -> Self {
        AssetCode12(x)
    }
}

impl ReadXDR for AssetCode12 {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let i = <[u8; 12] as ReadXDR>::read_xdr(r)?;
        let v = AssetCode12(i);
        Ok(v)
    }
}

impl WriteXDR for AssetCode12 {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.0.write_xdr(w)
    }
}

// AssetType is an XDR Enum defines as:
//
//   enum AssetType
//    {
//        ASSET_TYPE_NATIVE = 0,
//        ASSET_TYPE_CREDIT_ALPHANUM4 = 1,
//        ASSET_TYPE_CREDIT_ALPHANUM12 = 2,
//        ASSET_TYPE_POOL_SHARE = 3
//    };
//
// enum
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum AssetType {
    AssetTypeNative = 0,
    AssetTypeCreditAlphanum4 = 1,
    AssetTypeCreditAlphanum12 = 2,
    AssetTypePoolShare = 3,
}

impl TryFrom<i32> for AssetType {
    type Error = Error;

    fn try_from(i: i32) -> std::result::Result<Self, Self::Error> {
        let e = match i {
            0 => AssetType::AssetTypeNative,
            1 => AssetType::AssetTypeCreditAlphanum4,
            2 => AssetType::AssetTypeCreditAlphanum12,
            3 => AssetType::AssetTypePoolShare,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<AssetType> for i32 {
    fn from(e: AssetType) -> Self {
        e as Self
    }
}

impl ReadXDR for AssetType {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXDR for AssetType {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// AssetCode is an XDR Union defines as:
//
//   union AssetCode switch (AssetType type)
//    {
//    case ASSET_TYPE_CREDIT_ALPHANUM4:
//        AssetCode4 assetCode4;
//
//    case ASSET_TYPE_CREDIT_ALPHANUM12:
//        AssetCode12 assetCode12;
//
//        // add other asset types here in the future
//    };
//
// union with discriminant AssetType
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AssetCode {
    AssetTypeCreditAlphanum4(AssetCode4),
    AssetTypeCreditAlphanum12(AssetCode12),
}

impl AssetCode {
    fn discriminant(&self) -> AssetType {
        match self {
            Self::AssetTypeCreditAlphanum4(_) => AssetType::AssetTypeCreditAlphanum4,
            Self::AssetTypeCreditAlphanum12(_) => AssetType::AssetTypeCreditAlphanum12,
        }
    }
}

impl ReadXDR for AssetCode {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: AssetType = <AssetType as ReadXDR>::read_xdr(r)?;
        let v = match dv {
            AssetType::AssetTypeCreditAlphanum4 => {
                Self::AssetTypeCreditAlphanum4(<AssetCode4 as ReadXDR>::read_xdr(r)?)
            }
            AssetType::AssetTypeCreditAlphanum12 => {
                Self::AssetTypeCreditAlphanum12(<AssetCode12 as ReadXDR>::read_xdr(r)?)
            }
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for AssetCode {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::AssetTypeCreditAlphanum4(v) => v.write_xdr(w)?,
            Self::AssetTypeCreditAlphanum12(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// AlphaNum4 is an XDR Struct defines as:
//
//   struct AlphaNum4
//    {
//        AssetCode4 assetCode;
//        AccountID issuer;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AlphaNum4 {
    pub asset_code: AssetCode4,
    pub issuer: AccountId,
}

impl ReadXDR for AlphaNum4 {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            asset_code: <AssetCode4 as ReadXDR>::read_xdr(r)?,
            issuer: <AccountId as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for AlphaNum4 {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.asset_code.write_xdr(w)?;
        self.issuer.write_xdr(w)?;
        Ok(())
    }
}

// AlphaNum12 is an XDR Struct defines as:
//
//   struct AlphaNum12
//    {
//        AssetCode12 assetCode;
//        AccountID issuer;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AlphaNum12 {
    pub asset_code: AssetCode12,
    pub issuer: AccountId,
}

impl ReadXDR for AlphaNum12 {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            asset_code: <AssetCode12 as ReadXDR>::read_xdr(r)?,
            issuer: <AccountId as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for AlphaNum12 {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.asset_code.write_xdr(w)?;
        self.issuer.write_xdr(w)?;
        Ok(())
    }
}

// Asset is an XDR Union defines as:
//
//   union Asset switch (AssetType type)
//    {
//    case ASSET_TYPE_NATIVE: // Not credit
//        void;
//
//    case ASSET_TYPE_CREDIT_ALPHANUM4:
//        AlphaNum4 alphaNum4;
//
//    case ASSET_TYPE_CREDIT_ALPHANUM12:
//        AlphaNum12 alphaNum12;
//
//        // add other asset types here in the future
//    };
//
// union with discriminant AssetType
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Asset {
    AssetTypeNative,
    AssetTypeCreditAlphanum4(AlphaNum4),
    AssetTypeCreditAlphanum12(AlphaNum12),
}

impl Asset {
    fn discriminant(&self) -> AssetType {
        match self {
            Self::AssetTypeNative => AssetType::AssetTypeNative,
            Self::AssetTypeCreditAlphanum4(_) => AssetType::AssetTypeCreditAlphanum4,
            Self::AssetTypeCreditAlphanum12(_) => AssetType::AssetTypeCreditAlphanum12,
        }
    }
}

impl ReadXDR for Asset {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: AssetType = <AssetType as ReadXDR>::read_xdr(r)?;
        let v = match dv {
            AssetType::AssetTypeNative => Self::AssetTypeNative,
            AssetType::AssetTypeCreditAlphanum4 => {
                Self::AssetTypeCreditAlphanum4(<AlphaNum4 as ReadXDR>::read_xdr(r)?)
            }
            AssetType::AssetTypeCreditAlphanum12 => {
                Self::AssetTypeCreditAlphanum12(<AlphaNum12 as ReadXDR>::read_xdr(r)?)
            }
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for Asset {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::AssetTypeNative => ().write_xdr(w)?,
            Self::AssetTypeCreditAlphanum4(v) => v.write_xdr(w)?,
            Self::AssetTypeCreditAlphanum12(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// Price is an XDR Struct defines as:
//
//   struct Price
//    {
//        int32 n; // numerator
//        int32 d; // denominator
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Price {
    pub n: Int32,
    pub d: Int32,
}

impl ReadXDR for Price {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            n: <Int32 as ReadXDR>::read_xdr(r)?,
            d: <Int32 as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for Price {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.n.write_xdr(w)?;
        self.d.write_xdr(w)?;
        Ok(())
    }
}

// Liabilities is an XDR Struct defines as:
//
//   struct Liabilities
//    {
//        int64 buying;
//        int64 selling;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Liabilities {
    pub buying: Int64,
    pub selling: Int64,
}

impl ReadXDR for Liabilities {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            buying: <Int64 as ReadXDR>::read_xdr(r)?,
            selling: <Int64 as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for Liabilities {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.buying.write_xdr(w)?;
        self.selling.write_xdr(w)?;
        Ok(())
    }
}

// ThresholdIndexes is an XDR Enum defines as:
//
//   enum ThresholdIndexes
//    {
//        THRESHOLD_MASTER_WEIGHT = 0,
//        THRESHOLD_LOW = 1,
//        THRESHOLD_MED = 2,
//        THRESHOLD_HIGH = 3
//    };
//
// enum
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum ThresholdIndexes {
    ThresholdMasterWeight = 0,
    ThresholdLow = 1,
    ThresholdMed = 2,
    ThresholdHigh = 3,
}

impl TryFrom<i32> for ThresholdIndexes {
    type Error = Error;

    fn try_from(i: i32) -> std::result::Result<Self, Self::Error> {
        let e = match i {
            0 => ThresholdIndexes::ThresholdMasterWeight,
            1 => ThresholdIndexes::ThresholdLow,
            2 => ThresholdIndexes::ThresholdMed,
            3 => ThresholdIndexes::ThresholdHigh,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<ThresholdIndexes> for i32 {
    fn from(e: ThresholdIndexes) -> Self {
        e as Self
    }
}

impl ReadXDR for ThresholdIndexes {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXDR for ThresholdIndexes {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// LedgerEntryType is an XDR Enum defines as:
//
//   enum LedgerEntryType
//    {
//        ACCOUNT = 0,
//        TRUSTLINE = 1,
//        OFFER = 2,
//        DATA = 3,
//        CLAIMABLE_BALANCE = 4,
//        LIQUIDITY_POOL = 5
//    };
//
// enum
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum LedgerEntryType {
    Account = 0,
    Trustline = 1,
    Offer = 2,
    Data = 3,
    ClaimableBalance = 4,
    LiquidityPool = 5,
}

impl TryFrom<i32> for LedgerEntryType {
    type Error = Error;

    fn try_from(i: i32) -> std::result::Result<Self, Self::Error> {
        let e = match i {
            0 => LedgerEntryType::Account,
            1 => LedgerEntryType::Trustline,
            2 => LedgerEntryType::Offer,
            3 => LedgerEntryType::Data,
            4 => LedgerEntryType::ClaimableBalance,
            5 => LedgerEntryType::LiquidityPool,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<LedgerEntryType> for i32 {
    fn from(e: LedgerEntryType) -> Self {
        e as Self
    }
}

impl ReadXDR for LedgerEntryType {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXDR for LedgerEntryType {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// Signer is an XDR Struct defines as:
//
//   struct Signer
//    {
//        SignerKey key;
//        uint32 weight; // really only need 1 byte
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Signer {
    pub key: SignerKey,
    pub weight: Uint32,
}

impl ReadXDR for Signer {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            key: <SignerKey as ReadXDR>::read_xdr(r)?,
            weight: <Uint32 as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for Signer {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.key.write_xdr(w)?;
        self.weight.write_xdr(w)?;
        Ok(())
    }
}

// AccountFlags is an XDR Enum defines as:
//
//   enum AccountFlags
//    { // masks for each flag
//
//        // Flags set on issuer accounts
//        // TrustLines are created with authorized set to "false" requiring
//        // the issuer to set it for each TrustLine
//        AUTH_REQUIRED_FLAG = 0x1,
//        // If set, the authorized flag in TrustLines can be cleared
//        // otherwise, authorization cannot be revoked
//        AUTH_REVOCABLE_FLAG = 0x2,
//        // Once set, causes all AUTH_* flags to be read-only
//        AUTH_IMMUTABLE_FLAG = 0x4,
//        // Trustlines are created with clawback enabled set to "true",
//        // and claimable balances created from those trustlines are created
//        // with clawback enabled set to "true"
//        AUTH_CLAWBACK_ENABLED_FLAG = 0x8
//    };
//
// enum
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum AccountFlags {
    AuthRequiredFlag = 1,
    AuthRevocableFlag = 2,
    AuthImmutableFlag = 4,
    AuthClawbackEnabledFlag = 8,
}

impl TryFrom<i32> for AccountFlags {
    type Error = Error;

    fn try_from(i: i32) -> std::result::Result<Self, Self::Error> {
        let e = match i {
            1 => AccountFlags::AuthRequiredFlag,
            2 => AccountFlags::AuthRevocableFlag,
            4 => AccountFlags::AuthImmutableFlag,
            8 => AccountFlags::AuthClawbackEnabledFlag,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<AccountFlags> for i32 {
    fn from(e: AccountFlags) -> Self {
        e as Self
    }
}

impl ReadXDR for AccountFlags {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXDR for AccountFlags {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// MaskAccountFlags is an XDR Const defines as:
//
//   const MASK_ACCOUNT_FLAGS = 0x7;
//
pub const MASK_ACCOUNT_FLAGS: u64 = 0x7;

// MaskAccountFlagsV17 is an XDR Const defines as:
//
//   const MASK_ACCOUNT_FLAGS_V17 = 0xF;
//
pub const MASK_ACCOUNT_FLAGS_V17: u64 = 0xF;

// MaxSigners is an XDR Const defines as:
//
//   const MAX_SIGNERS = 20;
//
pub const MAX_SIGNERS: u64 = 20;

// SponsorshipDescriptor is an XDR Typedef defines as:
//
//   typedef AccountID* SponsorshipDescriptor;
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SponsorshipDescriptor(pub Option<AccountId>);

impl From<SponsorshipDescriptor> for Option<AccountId> {
    fn from(x: SponsorshipDescriptor) -> Self {
        x.0
    }
}

impl From<Option<AccountId>> for SponsorshipDescriptor {
    fn from(x: Option<AccountId>) -> Self {
        SponsorshipDescriptor(x)
    }
}

impl ReadXDR for SponsorshipDescriptor {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let i = <Option<AccountId> as ReadXDR>::read_xdr(r)?;
        let v = SponsorshipDescriptor(i);
        Ok(v)
    }
}

impl WriteXDR for SponsorshipDescriptor {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.0.write_xdr(w)
    }
}

// AccountEntryExtensionV3 is an XDR Struct defines as:
//
//   struct AccountEntryExtensionV3
//    {
//        // We can use this to add more fields, or because it is first, to
//        // change AccountEntryExtensionV3 into a union.
//        ExtensionPoint ext;
//
//        // Ledger number at which `seqNum` took on its present value.
//        uint32 seqLedger;
//
//        // Time at which `seqNum` took on its present value.
//        TimePoint seqTime;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AccountEntryExtensionV3 {
    pub ext: ExtensionPoint,
    pub seq_ledger: Uint32,
    pub seq_time: TimePoint,
}

impl ReadXDR for AccountEntryExtensionV3 {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            ext: <ExtensionPoint as ReadXDR>::read_xdr(r)?,
            seq_ledger: <Uint32 as ReadXDR>::read_xdr(r)?,
            seq_time: <TimePoint as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for AccountEntryExtensionV3 {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.ext.write_xdr(w)?;
        self.seq_ledger.write_xdr(w)?;
        self.seq_time.write_xdr(w)?;
        Ok(())
    }
}

// AccountEntryExtensionV2Ext is an XDR NestedUnion defines as:
//
//   union switch (int v)
//        {
//        case 0:
//            void;
//        case 3:
//            AccountEntryExtensionV3 v3;
//        }
//
// union with discriminant i32
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AccountEntryExtensionV2Ext {
    V0,
    V3(AccountEntryExtensionV3),
}

impl AccountEntryExtensionV2Ext {
    fn discriminant(&self) -> i32 {
        match self {
            Self::V0 => 0,
            Self::V3(_) => 3,
        }
    }
}

impl ReadXDR for AccountEntryExtensionV2Ext {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: i32 = <i32 as ReadXDR>::read_xdr(r)?;
        let v = match dv.into() {
            0 => Self::V0,
            3 => Self::V3(<AccountEntryExtensionV3 as ReadXDR>::read_xdr(r)?),
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for AccountEntryExtensionV2Ext {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::V0 => ().write_xdr(w)?,
            Self::V3(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// AccountEntryExtensionV2 is an XDR Struct defines as:
//
//   struct AccountEntryExtensionV2
//    {
//        uint32 numSponsored;
//        uint32 numSponsoring;
//        SponsorshipDescriptor signerSponsoringIDs<MAX_SIGNERS>;
//
//        union switch (int v)
//        {
//        case 0:
//            void;
//        case 3:
//            AccountEntryExtensionV3 v3;
//        }
//        ext;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AccountEntryExtensionV2 {
    pub num_sponsored: Uint32,
    pub num_sponsoring: Uint32,
    pub signer_sponsoring_i_ds: Vec<SponsorshipDescriptor>,
    pub ext: AccountEntryExtensionV2Ext,
}

impl ReadXDR for AccountEntryExtensionV2 {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            num_sponsored: <Uint32 as ReadXDR>::read_xdr(r)?,
            num_sponsoring: <Uint32 as ReadXDR>::read_xdr(r)?,
            signer_sponsoring_i_ds: <Vec<SponsorshipDescriptor> as ReadXDR>::read_xdr(r)?,
            ext: <AccountEntryExtensionV2Ext as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for AccountEntryExtensionV2 {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.num_sponsored.write_xdr(w)?;
        self.num_sponsoring.write_xdr(w)?;
        self.signer_sponsoring_i_ds.write_xdr(w)?;
        self.ext.write_xdr(w)?;
        Ok(())
    }
}

// AccountEntryExtensionV1Ext is an XDR NestedUnion defines as:
//
//   union switch (int v)
//        {
//        case 0:
//            void;
//        case 2:
//            AccountEntryExtensionV2 v2;
//        }
//
// union with discriminant i32
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AccountEntryExtensionV1Ext {
    V0,
    V2(AccountEntryExtensionV2),
}

impl AccountEntryExtensionV1Ext {
    fn discriminant(&self) -> i32 {
        match self {
            Self::V0 => 0,
            Self::V2(_) => 2,
        }
    }
}

impl ReadXDR for AccountEntryExtensionV1Ext {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: i32 = <i32 as ReadXDR>::read_xdr(r)?;
        let v = match dv.into() {
            0 => Self::V0,
            2 => Self::V2(<AccountEntryExtensionV2 as ReadXDR>::read_xdr(r)?),
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for AccountEntryExtensionV1Ext {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::V0 => ().write_xdr(w)?,
            Self::V2(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// AccountEntryExtensionV1 is an XDR Struct defines as:
//
//   struct AccountEntryExtensionV1
//    {
//        Liabilities liabilities;
//
//        union switch (int v)
//        {
//        case 0:
//            void;
//        case 2:
//            AccountEntryExtensionV2 v2;
//        }
//        ext;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AccountEntryExtensionV1 {
    pub liabilities: Liabilities,
    pub ext: AccountEntryExtensionV1Ext,
}

impl ReadXDR for AccountEntryExtensionV1 {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            liabilities: <Liabilities as ReadXDR>::read_xdr(r)?,
            ext: <AccountEntryExtensionV1Ext as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for AccountEntryExtensionV1 {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.liabilities.write_xdr(w)?;
        self.ext.write_xdr(w)?;
        Ok(())
    }
}

// AccountEntryExt is an XDR NestedUnion defines as:
//
//   union switch (int v)
//        {
//        case 0:
//            void;
//        case 1:
//            AccountEntryExtensionV1 v1;
//        }
//
// union with discriminant i32
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AccountEntryExt {
    V0,
    V1(AccountEntryExtensionV1),
}

impl AccountEntryExt {
    fn discriminant(&self) -> i32 {
        match self {
            Self::V0 => 0,
            Self::V1(_) => 1,
        }
    }
}

impl ReadXDR for AccountEntryExt {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: i32 = <i32 as ReadXDR>::read_xdr(r)?;
        let v = match dv.into() {
            0 => Self::V0,
            1 => Self::V1(<AccountEntryExtensionV1 as ReadXDR>::read_xdr(r)?),
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for AccountEntryExt {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::V0 => ().write_xdr(w)?,
            Self::V1(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// AccountEntry is an XDR Struct defines as:
//
//   struct AccountEntry
//    {
//        AccountID accountID;      // master public key for this account
//        int64 balance;            // in stroops
//        SequenceNumber seqNum;    // last sequence number used for this account
//        uint32 numSubEntries;     // number of sub-entries this account has
//                                  // drives the reserve
//        AccountID* inflationDest; // Account to vote for during inflation
//        uint32 flags;             // see AccountFlags
//
//        string32 homeDomain; // can be used for reverse federation and memo lookup
//
//        // fields used for signatures
//        // thresholds stores unsigned bytes: [weight of master|low|medium|high]
//        Thresholds thresholds;
//
//        Signer signers<MAX_SIGNERS>; // possible signers for this account
//
//        // reserved for future use
//        union switch (int v)
//        {
//        case 0:
//            void;
//        case 1:
//            AccountEntryExtensionV1 v1;
//        }
//        ext;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AccountEntry {
    pub account_id: AccountId,
    pub balance: Int64,
    pub seq_num: SequenceNumber,
    pub num_sub_entries: Uint32,
    pub inflation_dest: Option<AccountId>,
    pub flags: Uint32,
    pub home_domain: String32,
    pub thresholds: Thresholds,
    pub signers: Vec<Signer>,
    pub ext: AccountEntryExt,
}

impl ReadXDR for AccountEntry {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            account_id: <AccountId as ReadXDR>::read_xdr(r)?,
            balance: <Int64 as ReadXDR>::read_xdr(r)?,
            seq_num: <SequenceNumber as ReadXDR>::read_xdr(r)?,
            num_sub_entries: <Uint32 as ReadXDR>::read_xdr(r)?,
            inflation_dest: <Option<AccountId> as ReadXDR>::read_xdr(r)?,
            flags: <Uint32 as ReadXDR>::read_xdr(r)?,
            home_domain: <String32 as ReadXDR>::read_xdr(r)?,
            thresholds: <Thresholds as ReadXDR>::read_xdr(r)?,
            signers: <Vec<Signer> as ReadXDR>::read_xdr(r)?,
            ext: <AccountEntryExt as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for AccountEntry {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.account_id.write_xdr(w)?;
        self.balance.write_xdr(w)?;
        self.seq_num.write_xdr(w)?;
        self.num_sub_entries.write_xdr(w)?;
        self.inflation_dest.write_xdr(w)?;
        self.flags.write_xdr(w)?;
        self.home_domain.write_xdr(w)?;
        self.thresholds.write_xdr(w)?;
        self.signers.write_xdr(w)?;
        self.ext.write_xdr(w)?;
        Ok(())
    }
}

// TrustLineFlags is an XDR Enum defines as:
//
//   enum TrustLineFlags
//    {
//        // issuer has authorized account to perform transactions with its credit
//        AUTHORIZED_FLAG = 1,
//        // issuer has authorized account to maintain and reduce liabilities for its
//        // credit
//        AUTHORIZED_TO_MAINTAIN_LIABILITIES_FLAG = 2,
//        // issuer has specified that it may clawback its credit, and that claimable
//        // balances created with its credit may also be clawed back
//        TRUSTLINE_CLAWBACK_ENABLED_FLAG = 4
//    };
//
// enum
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum TrustLineFlags {
    AuthorizedFlag = 1,
    AuthorizedToMaintainLiabilitiesFlag = 2,
    TrustlineClawbackEnabledFlag = 4,
}

impl TryFrom<i32> for TrustLineFlags {
    type Error = Error;

    fn try_from(i: i32) -> std::result::Result<Self, Self::Error> {
        let e = match i {
            1 => TrustLineFlags::AuthorizedFlag,
            2 => TrustLineFlags::AuthorizedToMaintainLiabilitiesFlag,
            4 => TrustLineFlags::TrustlineClawbackEnabledFlag,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<TrustLineFlags> for i32 {
    fn from(e: TrustLineFlags) -> Self {
        e as Self
    }
}

impl ReadXDR for TrustLineFlags {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXDR for TrustLineFlags {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// MaskTrustlineFlags is an XDR Const defines as:
//
//   const MASK_TRUSTLINE_FLAGS = 1;
//
pub const MASK_TRUSTLINE_FLAGS: u64 = 1;

// MaskTrustlineFlagsV13 is an XDR Const defines as:
//
//   const MASK_TRUSTLINE_FLAGS_V13 = 3;
//
pub const MASK_TRUSTLINE_FLAGS_V13: u64 = 3;

// MaskTrustlineFlagsV17 is an XDR Const defines as:
//
//   const MASK_TRUSTLINE_FLAGS_V17 = 7;
//
pub const MASK_TRUSTLINE_FLAGS_V17: u64 = 7;

// LiquidityPoolType is an XDR Enum defines as:
//
//   enum LiquidityPoolType
//    {
//        LIQUIDITY_POOL_CONSTANT_PRODUCT = 0
//    };
//
// enum
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum LiquidityPoolType {
    LiquidityPoolConstantProduct = 0,
}

impl TryFrom<i32> for LiquidityPoolType {
    type Error = Error;

    fn try_from(i: i32) -> std::result::Result<Self, Self::Error> {
        let e = match i {
            0 => LiquidityPoolType::LiquidityPoolConstantProduct,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<LiquidityPoolType> for i32 {
    fn from(e: LiquidityPoolType) -> Self {
        e as Self
    }
}

impl ReadXDR for LiquidityPoolType {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXDR for LiquidityPoolType {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// TrustLineAsset is an XDR Union defines as:
//
//   union TrustLineAsset switch (AssetType type)
//    {
//    case ASSET_TYPE_NATIVE: // Not credit
//        void;
//
//    case ASSET_TYPE_CREDIT_ALPHANUM4:
//        AlphaNum4 alphaNum4;
//
//    case ASSET_TYPE_CREDIT_ALPHANUM12:
//        AlphaNum12 alphaNum12;
//
//    case ASSET_TYPE_POOL_SHARE:
//        PoolID liquidityPoolID;
//
//        // add other asset types here in the future
//    };
//
// union with discriminant AssetType
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TrustLineAsset {
    AssetTypeNative,
    AssetTypeCreditAlphanum4(AlphaNum4),
    AssetTypeCreditAlphanum12(AlphaNum12),
    AssetTypePoolShare(PoolId),
}

impl TrustLineAsset {
    fn discriminant(&self) -> AssetType {
        match self {
            Self::AssetTypeNative => AssetType::AssetTypeNative,
            Self::AssetTypeCreditAlphanum4(_) => AssetType::AssetTypeCreditAlphanum4,
            Self::AssetTypeCreditAlphanum12(_) => AssetType::AssetTypeCreditAlphanum12,
            Self::AssetTypePoolShare(_) => AssetType::AssetTypePoolShare,
        }
    }
}

impl ReadXDR for TrustLineAsset {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: AssetType = <AssetType as ReadXDR>::read_xdr(r)?;
        let v = match dv {
            AssetType::AssetTypeNative => Self::AssetTypeNative,
            AssetType::AssetTypeCreditAlphanum4 => {
                Self::AssetTypeCreditAlphanum4(<AlphaNum4 as ReadXDR>::read_xdr(r)?)
            }
            AssetType::AssetTypeCreditAlphanum12 => {
                Self::AssetTypeCreditAlphanum12(<AlphaNum12 as ReadXDR>::read_xdr(r)?)
            }
            AssetType::AssetTypePoolShare => {
                Self::AssetTypePoolShare(<PoolId as ReadXDR>::read_xdr(r)?)
            }
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for TrustLineAsset {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::AssetTypeNative => ().write_xdr(w)?,
            Self::AssetTypeCreditAlphanum4(v) => v.write_xdr(w)?,
            Self::AssetTypeCreditAlphanum12(v) => v.write_xdr(w)?,
            Self::AssetTypePoolShare(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// TrustLineEntryExtensionV2Ext is an XDR NestedUnion defines as:
//
//   union switch (int v)
//        {
//        case 0:
//            void;
//        }
//
// union with discriminant i32
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TrustLineEntryExtensionV2Ext {
    V0,
}

impl TrustLineEntryExtensionV2Ext {
    fn discriminant(&self) -> i32 {
        match self {
            Self::V0 => 0,
        }
    }
}

impl ReadXDR for TrustLineEntryExtensionV2Ext {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: i32 = <i32 as ReadXDR>::read_xdr(r)?;
        let v = match dv.into() {
            0 => Self::V0,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for TrustLineEntryExtensionV2Ext {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::V0 => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// TrustLineEntryExtensionV2 is an XDR Struct defines as:
//
//   struct TrustLineEntryExtensionV2
//    {
//        int32 liquidityPoolUseCount;
//
//        union switch (int v)
//        {
//        case 0:
//            void;
//        }
//        ext;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrustLineEntryExtensionV2 {
    pub liquidity_pool_use_count: Int32,
    pub ext: TrustLineEntryExtensionV2Ext,
}

impl ReadXDR for TrustLineEntryExtensionV2 {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            liquidity_pool_use_count: <Int32 as ReadXDR>::read_xdr(r)?,
            ext: <TrustLineEntryExtensionV2Ext as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for TrustLineEntryExtensionV2 {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.liquidity_pool_use_count.write_xdr(w)?;
        self.ext.write_xdr(w)?;
        Ok(())
    }
}

// TrustLineEntryV1Ext is an XDR NestedUnion defines as:
//
//   union switch (int v)
//                {
//                case 0:
//                    void;
//                case 2:
//                    TrustLineEntryExtensionV2 v2;
//                }
//
// union with discriminant i32
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TrustLineEntryV1Ext {
    V0,
    V2(TrustLineEntryExtensionV2),
}

impl TrustLineEntryV1Ext {
    fn discriminant(&self) -> i32 {
        match self {
            Self::V0 => 0,
            Self::V2(_) => 2,
        }
    }
}

impl ReadXDR for TrustLineEntryV1Ext {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: i32 = <i32 as ReadXDR>::read_xdr(r)?;
        let v = match dv.into() {
            0 => Self::V0,
            2 => Self::V2(<TrustLineEntryExtensionV2 as ReadXDR>::read_xdr(r)?),
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for TrustLineEntryV1Ext {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::V0 => ().write_xdr(w)?,
            Self::V2(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// TrustLineEntryV1 is an XDR NestedStruct defines as:
//
//   struct
//            {
//                Liabilities liabilities;
//
//                union switch (int v)
//                {
//                case 0:
//                    void;
//                case 2:
//                    TrustLineEntryExtensionV2 v2;
//                }
//                ext;
//            }
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrustLineEntryV1 {
    pub liabilities: Liabilities,
    pub ext: TrustLineEntryV1Ext,
}

impl ReadXDR for TrustLineEntryV1 {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            liabilities: <Liabilities as ReadXDR>::read_xdr(r)?,
            ext: <TrustLineEntryV1Ext as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for TrustLineEntryV1 {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.liabilities.write_xdr(w)?;
        self.ext.write_xdr(w)?;
        Ok(())
    }
}

// TrustLineEntryExt is an XDR NestedUnion defines as:
//
//   union switch (int v)
//        {
//        case 0:
//            void;
//        case 1:
//            struct
//            {
//                Liabilities liabilities;
//
//                union switch (int v)
//                {
//                case 0:
//                    void;
//                case 2:
//                    TrustLineEntryExtensionV2 v2;
//                }
//                ext;
//            } v1;
//        }
//
// union with discriminant i32
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TrustLineEntryExt {
    V0,
    V1(TrustLineEntryV1),
}

impl TrustLineEntryExt {
    fn discriminant(&self) -> i32 {
        match self {
            Self::V0 => 0,
            Self::V1(_) => 1,
        }
    }
}

impl ReadXDR for TrustLineEntryExt {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: i32 = <i32 as ReadXDR>::read_xdr(r)?;
        let v = match dv.into() {
            0 => Self::V0,
            1 => Self::V1(<TrustLineEntryV1 as ReadXDR>::read_xdr(r)?),
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for TrustLineEntryExt {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::V0 => ().write_xdr(w)?,
            Self::V1(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// TrustLineEntry is an XDR Struct defines as:
//
//   struct TrustLineEntry
//    {
//        AccountID accountID;  // account this trustline belongs to
//        TrustLineAsset asset; // type of asset (with issuer)
//        int64 balance;        // how much of this asset the user has.
//                              // Asset defines the unit for this;
//
//        int64 limit;  // balance cannot be above this
//        uint32 flags; // see TrustLineFlags
//
//        // reserved for future use
//        union switch (int v)
//        {
//        case 0:
//            void;
//        case 1:
//            struct
//            {
//                Liabilities liabilities;
//
//                union switch (int v)
//                {
//                case 0:
//                    void;
//                case 2:
//                    TrustLineEntryExtensionV2 v2;
//                }
//                ext;
//            } v1;
//        }
//        ext;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrustLineEntry {
    pub account_id: AccountId,
    pub asset: TrustLineAsset,
    pub balance: Int64,
    pub limit: Int64,
    pub flags: Uint32,
    pub ext: TrustLineEntryExt,
}

impl ReadXDR for TrustLineEntry {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            account_id: <AccountId as ReadXDR>::read_xdr(r)?,
            asset: <TrustLineAsset as ReadXDR>::read_xdr(r)?,
            balance: <Int64 as ReadXDR>::read_xdr(r)?,
            limit: <Int64 as ReadXDR>::read_xdr(r)?,
            flags: <Uint32 as ReadXDR>::read_xdr(r)?,
            ext: <TrustLineEntryExt as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for TrustLineEntry {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.account_id.write_xdr(w)?;
        self.asset.write_xdr(w)?;
        self.balance.write_xdr(w)?;
        self.limit.write_xdr(w)?;
        self.flags.write_xdr(w)?;
        self.ext.write_xdr(w)?;
        Ok(())
    }
}

// OfferEntryFlags is an XDR Enum defines as:
//
//   enum OfferEntryFlags
//    {
//        // an offer with this flag will not act on and take a reverse offer of equal
//        // price
//        PASSIVE_FLAG = 1
//    };
//
// enum
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum OfferEntryFlags {
    PassiveFlag = 1,
}

impl TryFrom<i32> for OfferEntryFlags {
    type Error = Error;

    fn try_from(i: i32) -> std::result::Result<Self, Self::Error> {
        let e = match i {
            1 => OfferEntryFlags::PassiveFlag,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<OfferEntryFlags> for i32 {
    fn from(e: OfferEntryFlags) -> Self {
        e as Self
    }
}

impl ReadXDR for OfferEntryFlags {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXDR for OfferEntryFlags {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// MaskOfferentryFlags is an XDR Const defines as:
//
//   const MASK_OFFERENTRY_FLAGS = 1;
//
pub const MASK_OFFERENTRY_FLAGS: u64 = 1;

// OfferEntryExt is an XDR NestedUnion defines as:
//
//   union switch (int v)
//        {
//        case 0:
//            void;
//        }
//
// union with discriminant i32
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OfferEntryExt {
    V0,
}

impl OfferEntryExt {
    fn discriminant(&self) -> i32 {
        match self {
            Self::V0 => 0,
        }
    }
}

impl ReadXDR for OfferEntryExt {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: i32 = <i32 as ReadXDR>::read_xdr(r)?;
        let v = match dv.into() {
            0 => Self::V0,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for OfferEntryExt {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::V0 => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// OfferEntry is an XDR Struct defines as:
//
//   struct OfferEntry
//    {
//        AccountID sellerID;
//        int64 offerID;
//        Asset selling; // A
//        Asset buying;  // B
//        int64 amount;  // amount of A
//
//        /* price for this offer:
//            price of A in terms of B
//            price=AmountB/AmountA=priceNumerator/priceDenominator
//            price is after fees
//        */
//        Price price;
//        uint32 flags; // see OfferEntryFlags
//
//        // reserved for future use
//        union switch (int v)
//        {
//        case 0:
//            void;
//        }
//        ext;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OfferEntry {
    pub seller_id: AccountId,
    pub offer_id: Int64,
    pub selling: Asset,
    pub buying: Asset,
    pub amount: Int64,
    pub price: Price,
    pub flags: Uint32,
    pub ext: OfferEntryExt,
}

impl ReadXDR for OfferEntry {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            seller_id: <AccountId as ReadXDR>::read_xdr(r)?,
            offer_id: <Int64 as ReadXDR>::read_xdr(r)?,
            selling: <Asset as ReadXDR>::read_xdr(r)?,
            buying: <Asset as ReadXDR>::read_xdr(r)?,
            amount: <Int64 as ReadXDR>::read_xdr(r)?,
            price: <Price as ReadXDR>::read_xdr(r)?,
            flags: <Uint32 as ReadXDR>::read_xdr(r)?,
            ext: <OfferEntryExt as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for OfferEntry {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.seller_id.write_xdr(w)?;
        self.offer_id.write_xdr(w)?;
        self.selling.write_xdr(w)?;
        self.buying.write_xdr(w)?;
        self.amount.write_xdr(w)?;
        self.price.write_xdr(w)?;
        self.flags.write_xdr(w)?;
        self.ext.write_xdr(w)?;
        Ok(())
    }
}

// DataEntryExt is an XDR NestedUnion defines as:
//
//   union switch (int v)
//        {
//        case 0:
//            void;
//        }
//
// union with discriminant i32
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DataEntryExt {
    V0,
}

impl DataEntryExt {
    fn discriminant(&self) -> i32 {
        match self {
            Self::V0 => 0,
        }
    }
}

impl ReadXDR for DataEntryExt {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: i32 = <i32 as ReadXDR>::read_xdr(r)?;
        let v = match dv.into() {
            0 => Self::V0,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for DataEntryExt {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::V0 => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// DataEntry is an XDR Struct defines as:
//
//   struct DataEntry
//    {
//        AccountID accountID; // account this data belongs to
//        string64 dataName;
//        DataValue dataValue;
//
//        // reserved for future use
//        union switch (int v)
//        {
//        case 0:
//            void;
//        }
//        ext;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DataEntry {
    pub account_id: AccountId,
    pub data_name: String64,
    pub data_value: DataValue,
    pub ext: DataEntryExt,
}

impl ReadXDR for DataEntry {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            account_id: <AccountId as ReadXDR>::read_xdr(r)?,
            data_name: <String64 as ReadXDR>::read_xdr(r)?,
            data_value: <DataValue as ReadXDR>::read_xdr(r)?,
            ext: <DataEntryExt as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for DataEntry {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.account_id.write_xdr(w)?;
        self.data_name.write_xdr(w)?;
        self.data_value.write_xdr(w)?;
        self.ext.write_xdr(w)?;
        Ok(())
    }
}

// ClaimPredicateType is an XDR Enum defines as:
//
//   enum ClaimPredicateType
//    {
//        CLAIM_PREDICATE_UNCONDITIONAL = 0,
//        CLAIM_PREDICATE_AND = 1,
//        CLAIM_PREDICATE_OR = 2,
//        CLAIM_PREDICATE_NOT = 3,
//        CLAIM_PREDICATE_BEFORE_ABSOLUTE_TIME = 4,
//        CLAIM_PREDICATE_BEFORE_RELATIVE_TIME = 5
//    };
//
// enum
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum ClaimPredicateType {
    ClaimPredicateUnconditional = 0,
    ClaimPredicateAnd = 1,
    ClaimPredicateOr = 2,
    ClaimPredicateNot = 3,
    ClaimPredicateBeforeAbsoluteTime = 4,
    ClaimPredicateBeforeRelativeTime = 5,
}

impl TryFrom<i32> for ClaimPredicateType {
    type Error = Error;

    fn try_from(i: i32) -> std::result::Result<Self, Self::Error> {
        let e = match i {
            0 => ClaimPredicateType::ClaimPredicateUnconditional,
            1 => ClaimPredicateType::ClaimPredicateAnd,
            2 => ClaimPredicateType::ClaimPredicateOr,
            3 => ClaimPredicateType::ClaimPredicateNot,
            4 => ClaimPredicateType::ClaimPredicateBeforeAbsoluteTime,
            5 => ClaimPredicateType::ClaimPredicateBeforeRelativeTime,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<ClaimPredicateType> for i32 {
    fn from(e: ClaimPredicateType) -> Self {
        e as Self
    }
}

impl ReadXDR for ClaimPredicateType {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXDR for ClaimPredicateType {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// ClaimPredicate is an XDR Union defines as:
//
//   union ClaimPredicate switch (ClaimPredicateType type)
//    {
//    case CLAIM_PREDICATE_UNCONDITIONAL:
//        void;
//    case CLAIM_PREDICATE_AND:
//        ClaimPredicate andPredicates<2>;
//    case CLAIM_PREDICATE_OR:
//        ClaimPredicate orPredicates<2>;
//    case CLAIM_PREDICATE_NOT:
//        ClaimPredicate* notPredicate;
//    case CLAIM_PREDICATE_BEFORE_ABSOLUTE_TIME:
//        int64 absBefore; // Predicate will be true if closeTime < absBefore
//    case CLAIM_PREDICATE_BEFORE_RELATIVE_TIME:
//        int64 relBefore; // Seconds since closeTime of the ledger in which the
//                         // ClaimableBalanceEntry was created
//    };
//
// union with discriminant ClaimPredicateType
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ClaimPredicate {
    ClaimPredicateUnconditional,
    ClaimPredicateAnd(Vec<ClaimPredicate>),
    ClaimPredicateOr(Vec<ClaimPredicate>),
    ClaimPredicateNot(Option<Box<ClaimPredicate>>),
    ClaimPredicateBeforeAbsoluteTime(Int64),
    ClaimPredicateBeforeRelativeTime(Int64),
}

impl ClaimPredicate {
    fn discriminant(&self) -> ClaimPredicateType {
        match self {
            Self::ClaimPredicateUnconditional => ClaimPredicateType::ClaimPredicateUnconditional,
            Self::ClaimPredicateAnd(_) => ClaimPredicateType::ClaimPredicateAnd,
            Self::ClaimPredicateOr(_) => ClaimPredicateType::ClaimPredicateOr,
            Self::ClaimPredicateNot(_) => ClaimPredicateType::ClaimPredicateNot,
            Self::ClaimPredicateBeforeAbsoluteTime(_) => {
                ClaimPredicateType::ClaimPredicateBeforeAbsoluteTime
            }
            Self::ClaimPredicateBeforeRelativeTime(_) => {
                ClaimPredicateType::ClaimPredicateBeforeRelativeTime
            }
        }
    }
}

impl ReadXDR for ClaimPredicate {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: ClaimPredicateType = <ClaimPredicateType as ReadXDR>::read_xdr(r)?;
        let v = match dv {
            ClaimPredicateType::ClaimPredicateUnconditional => Self::ClaimPredicateUnconditional,
            ClaimPredicateType::ClaimPredicateAnd => {
                Self::ClaimPredicateAnd(<Vec<ClaimPredicate> as ReadXDR>::read_xdr(r)?)
            }
            ClaimPredicateType::ClaimPredicateOr => {
                Self::ClaimPredicateOr(<Vec<ClaimPredicate> as ReadXDR>::read_xdr(r)?)
            }
            ClaimPredicateType::ClaimPredicateNot => {
                Self::ClaimPredicateNot(<Option<Box<ClaimPredicate>> as ReadXDR>::read_xdr(r)?)
            }
            ClaimPredicateType::ClaimPredicateBeforeAbsoluteTime => {
                Self::ClaimPredicateBeforeAbsoluteTime(<Int64 as ReadXDR>::read_xdr(r)?)
            }
            ClaimPredicateType::ClaimPredicateBeforeRelativeTime => {
                Self::ClaimPredicateBeforeRelativeTime(<Int64 as ReadXDR>::read_xdr(r)?)
            }
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for ClaimPredicate {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::ClaimPredicateUnconditional => ().write_xdr(w)?,
            Self::ClaimPredicateAnd(v) => v.write_xdr(w)?,
            Self::ClaimPredicateOr(v) => v.write_xdr(w)?,
            Self::ClaimPredicateNot(v) => v.write_xdr(w)?,
            Self::ClaimPredicateBeforeAbsoluteTime(v) => v.write_xdr(w)?,
            Self::ClaimPredicateBeforeRelativeTime(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// ClaimantType is an XDR Enum defines as:
//
//   enum ClaimantType
//    {
//        CLAIMANT_TYPE_V0 = 0
//    };
//
// enum
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum ClaimantType {
    ClaimantTypeV0 = 0,
}

impl TryFrom<i32> for ClaimantType {
    type Error = Error;

    fn try_from(i: i32) -> std::result::Result<Self, Self::Error> {
        let e = match i {
            0 => ClaimantType::ClaimantTypeV0,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<ClaimantType> for i32 {
    fn from(e: ClaimantType) -> Self {
        e as Self
    }
}

impl ReadXDR for ClaimantType {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXDR for ClaimantType {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// ClaimantV0 is an XDR NestedStruct defines as:
//
//   struct
//        {
//            AccountID destination;    // The account that can use this condition
//            ClaimPredicate predicate; // Claimable if predicate is true
//        }
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClaimantV0 {
    pub destination: AccountId,
    pub predicate: ClaimPredicate,
}

impl ReadXDR for ClaimantV0 {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            destination: <AccountId as ReadXDR>::read_xdr(r)?,
            predicate: <ClaimPredicate as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for ClaimantV0 {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.destination.write_xdr(w)?;
        self.predicate.write_xdr(w)?;
        Ok(())
    }
}

// Claimant is an XDR Union defines as:
//
//   union Claimant switch (ClaimantType type)
//    {
//    case CLAIMANT_TYPE_V0:
//        struct
//        {
//            AccountID destination;    // The account that can use this condition
//            ClaimPredicate predicate; // Claimable if predicate is true
//        } v0;
//    };
//
// union with discriminant ClaimantType
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Claimant {
    ClaimantTypeV0(ClaimantV0),
}

impl Claimant {
    fn discriminant(&self) -> ClaimantType {
        match self {
            Self::ClaimantTypeV0(_) => ClaimantType::ClaimantTypeV0,
        }
    }
}

impl ReadXDR for Claimant {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: ClaimantType = <ClaimantType as ReadXDR>::read_xdr(r)?;
        let v = match dv {
            ClaimantType::ClaimantTypeV0 => {
                Self::ClaimantTypeV0(<ClaimantV0 as ReadXDR>::read_xdr(r)?)
            }
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for Claimant {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::ClaimantTypeV0(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// ClaimableBalanceIdType is an XDR Enum defines as:
//
//   enum ClaimableBalanceIDType
//    {
//        CLAIMABLE_BALANCE_ID_TYPE_V0 = 0
//    };
//
// enum
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum ClaimableBalanceIdType {
    ClaimableBalanceIdTypeV0 = 0,
}

impl TryFrom<i32> for ClaimableBalanceIdType {
    type Error = Error;

    fn try_from(i: i32) -> std::result::Result<Self, Self::Error> {
        let e = match i {
            0 => ClaimableBalanceIdType::ClaimableBalanceIdTypeV0,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<ClaimableBalanceIdType> for i32 {
    fn from(e: ClaimableBalanceIdType) -> Self {
        e as Self
    }
}

impl ReadXDR for ClaimableBalanceIdType {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXDR for ClaimableBalanceIdType {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// ClaimableBalanceId is an XDR Union defines as:
//
//   union ClaimableBalanceID switch (ClaimableBalanceIDType type)
//    {
//    case CLAIMABLE_BALANCE_ID_TYPE_V0:
//        Hash v0;
//    };
//
// union with discriminant ClaimableBalanceIdType
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ClaimableBalanceId {
    ClaimableBalanceIdTypeV0(Hash),
}

impl ClaimableBalanceId {
    fn discriminant(&self) -> ClaimableBalanceIdType {
        match self {
            Self::ClaimableBalanceIdTypeV0(_) => ClaimableBalanceIdType::ClaimableBalanceIdTypeV0,
        }
    }
}

impl ReadXDR for ClaimableBalanceId {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: ClaimableBalanceIdType = <ClaimableBalanceIdType as ReadXDR>::read_xdr(r)?;
        let v = match dv {
            ClaimableBalanceIdType::ClaimableBalanceIdTypeV0 => {
                Self::ClaimableBalanceIdTypeV0(<Hash as ReadXDR>::read_xdr(r)?)
            }
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for ClaimableBalanceId {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::ClaimableBalanceIdTypeV0(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// ClaimableBalanceFlags is an XDR Enum defines as:
//
//   enum ClaimableBalanceFlags
//    {
//        // If set, the issuer account of the asset held by the claimable balance may
//        // clawback the claimable balance
//        CLAIMABLE_BALANCE_CLAWBACK_ENABLED_FLAG = 0x1
//    };
//
// enum
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum ClaimableBalanceFlags {
    ClaimableBalanceClawbackEnabledFlag = 1,
}

impl TryFrom<i32> for ClaimableBalanceFlags {
    type Error = Error;

    fn try_from(i: i32) -> std::result::Result<Self, Self::Error> {
        let e = match i {
            1 => ClaimableBalanceFlags::ClaimableBalanceClawbackEnabledFlag,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<ClaimableBalanceFlags> for i32 {
    fn from(e: ClaimableBalanceFlags) -> Self {
        e as Self
    }
}

impl ReadXDR for ClaimableBalanceFlags {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXDR for ClaimableBalanceFlags {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// MaskClaimableBalanceFlags is an XDR Const defines as:
//
//   const MASK_CLAIMABLE_BALANCE_FLAGS = 0x1;
//
pub const MASK_CLAIMABLE_BALANCE_FLAGS: u64 = 0x1;

// ClaimableBalanceEntryExtensionV1Ext is an XDR NestedUnion defines as:
//
//   union switch (int v)
//        {
//        case 0:
//            void;
//        }
//
// union with discriminant i32
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ClaimableBalanceEntryExtensionV1Ext {
    V0,
}

impl ClaimableBalanceEntryExtensionV1Ext {
    fn discriminant(&self) -> i32 {
        match self {
            Self::V0 => 0,
        }
    }
}

impl ReadXDR for ClaimableBalanceEntryExtensionV1Ext {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: i32 = <i32 as ReadXDR>::read_xdr(r)?;
        let v = match dv.into() {
            0 => Self::V0,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for ClaimableBalanceEntryExtensionV1Ext {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::V0 => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// ClaimableBalanceEntryExtensionV1 is an XDR Struct defines as:
//
//   struct ClaimableBalanceEntryExtensionV1
//    {
//        union switch (int v)
//        {
//        case 0:
//            void;
//        }
//        ext;
//
//        uint32 flags; // see ClaimableBalanceFlags
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClaimableBalanceEntryExtensionV1 {
    pub ext: ClaimableBalanceEntryExtensionV1Ext,
    pub flags: Uint32,
}

impl ReadXDR for ClaimableBalanceEntryExtensionV1 {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            ext: <ClaimableBalanceEntryExtensionV1Ext as ReadXDR>::read_xdr(r)?,
            flags: <Uint32 as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for ClaimableBalanceEntryExtensionV1 {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.ext.write_xdr(w)?;
        self.flags.write_xdr(w)?;
        Ok(())
    }
}

// ClaimableBalanceEntryExt is an XDR NestedUnion defines as:
//
//   union switch (int v)
//        {
//        case 0:
//            void;
//        case 1:
//            ClaimableBalanceEntryExtensionV1 v1;
//        }
//
// union with discriminant i32
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ClaimableBalanceEntryExt {
    V0,
    V1(ClaimableBalanceEntryExtensionV1),
}

impl ClaimableBalanceEntryExt {
    fn discriminant(&self) -> i32 {
        match self {
            Self::V0 => 0,
            Self::V1(_) => 1,
        }
    }
}

impl ReadXDR for ClaimableBalanceEntryExt {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: i32 = <i32 as ReadXDR>::read_xdr(r)?;
        let v = match dv.into() {
            0 => Self::V0,
            1 => Self::V1(<ClaimableBalanceEntryExtensionV1 as ReadXDR>::read_xdr(r)?),
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for ClaimableBalanceEntryExt {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::V0 => ().write_xdr(w)?,
            Self::V1(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// ClaimableBalanceEntry is an XDR Struct defines as:
//
//   struct ClaimableBalanceEntry
//    {
//        // Unique identifier for this ClaimableBalanceEntry
//        ClaimableBalanceID balanceID;
//
//        // List of claimants with associated predicate
//        Claimant claimants<10>;
//
//        // Any asset including native
//        Asset asset;
//
//        // Amount of asset
//        int64 amount;
//
//        // reserved for future use
//        union switch (int v)
//        {
//        case 0:
//            void;
//        case 1:
//            ClaimableBalanceEntryExtensionV1 v1;
//        }
//        ext;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClaimableBalanceEntry {
    pub balance_id: ClaimableBalanceId,
    pub claimants: Vec<Claimant>,
    pub asset: Asset,
    pub amount: Int64,
    pub ext: ClaimableBalanceEntryExt,
}

impl ReadXDR for ClaimableBalanceEntry {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            balance_id: <ClaimableBalanceId as ReadXDR>::read_xdr(r)?,
            claimants: <Vec<Claimant> as ReadXDR>::read_xdr(r)?,
            asset: <Asset as ReadXDR>::read_xdr(r)?,
            amount: <Int64 as ReadXDR>::read_xdr(r)?,
            ext: <ClaimableBalanceEntryExt as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for ClaimableBalanceEntry {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.balance_id.write_xdr(w)?;
        self.claimants.write_xdr(w)?;
        self.asset.write_xdr(w)?;
        self.amount.write_xdr(w)?;
        self.ext.write_xdr(w)?;
        Ok(())
    }
}

// LiquidityPoolConstantProductParameters is an XDR Struct defines as:
//
//   struct LiquidityPoolConstantProductParameters
//    {
//        Asset assetA; // assetA < assetB
//        Asset assetB;
//        int32 fee; // Fee is in basis points, so the actual rate is (fee/100)%
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LiquidityPoolConstantProductParameters {
    pub asset_a: Asset,
    pub asset_b: Asset,
    pub fee: Int32,
}

impl ReadXDR for LiquidityPoolConstantProductParameters {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            asset_a: <Asset as ReadXDR>::read_xdr(r)?,
            asset_b: <Asset as ReadXDR>::read_xdr(r)?,
            fee: <Int32 as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for LiquidityPoolConstantProductParameters {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.asset_a.write_xdr(w)?;
        self.asset_b.write_xdr(w)?;
        self.fee.write_xdr(w)?;
        Ok(())
    }
}

// LiquidityPoolEntryConstantProduct is an XDR NestedStruct defines as:
//
//   struct
//            {
//                LiquidityPoolConstantProductParameters params;
//
//                int64 reserveA;        // amount of A in the pool
//                int64 reserveB;        // amount of B in the pool
//                int64 totalPoolShares; // total number of pool shares issued
//                int64 poolSharesTrustLineCount; // number of trust lines for the
//                                                // associated pool shares
//            }
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LiquidityPoolEntryConstantProduct {
    pub params: LiquidityPoolConstantProductParameters,
    pub reserve_a: Int64,
    pub reserve_b: Int64,
    pub total_pool_shares: Int64,
    pub pool_shares_trust_line_count: Int64,
}

impl ReadXDR for LiquidityPoolEntryConstantProduct {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            params: <LiquidityPoolConstantProductParameters as ReadXDR>::read_xdr(r)?,
            reserve_a: <Int64 as ReadXDR>::read_xdr(r)?,
            reserve_b: <Int64 as ReadXDR>::read_xdr(r)?,
            total_pool_shares: <Int64 as ReadXDR>::read_xdr(r)?,
            pool_shares_trust_line_count: <Int64 as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for LiquidityPoolEntryConstantProduct {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.params.write_xdr(w)?;
        self.reserve_a.write_xdr(w)?;
        self.reserve_b.write_xdr(w)?;
        self.total_pool_shares.write_xdr(w)?;
        self.pool_shares_trust_line_count.write_xdr(w)?;
        Ok(())
    }
}

// LiquidityPoolEntryBody is an XDR NestedUnion defines as:
//
//   union switch (LiquidityPoolType type)
//        {
//        case LIQUIDITY_POOL_CONSTANT_PRODUCT:
//            struct
//            {
//                LiquidityPoolConstantProductParameters params;
//
//                int64 reserveA;        // amount of A in the pool
//                int64 reserveB;        // amount of B in the pool
//                int64 totalPoolShares; // total number of pool shares issued
//                int64 poolSharesTrustLineCount; // number of trust lines for the
//                                                // associated pool shares
//            } constantProduct;
//        }
//
// union with discriminant LiquidityPoolType
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LiquidityPoolEntryBody {
    LiquidityPoolConstantProduct(LiquidityPoolEntryConstantProduct),
}

impl LiquidityPoolEntryBody {
    fn discriminant(&self) -> LiquidityPoolType {
        match self {
            Self::LiquidityPoolConstantProduct(_) => {
                LiquidityPoolType::LiquidityPoolConstantProduct
            }
        }
    }
}

impl ReadXDR for LiquidityPoolEntryBody {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: LiquidityPoolType = <LiquidityPoolType as ReadXDR>::read_xdr(r)?;
        let v = match dv {
            LiquidityPoolType::LiquidityPoolConstantProduct => Self::LiquidityPoolConstantProduct(
                <LiquidityPoolEntryConstantProduct as ReadXDR>::read_xdr(r)?,
            ),
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for LiquidityPoolEntryBody {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::LiquidityPoolConstantProduct(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// LiquidityPoolEntry is an XDR Struct defines as:
//
//   struct LiquidityPoolEntry
//    {
//        PoolID liquidityPoolID;
//
//        union switch (LiquidityPoolType type)
//        {
//        case LIQUIDITY_POOL_CONSTANT_PRODUCT:
//            struct
//            {
//                LiquidityPoolConstantProductParameters params;
//
//                int64 reserveA;        // amount of A in the pool
//                int64 reserveB;        // amount of B in the pool
//                int64 totalPoolShares; // total number of pool shares issued
//                int64 poolSharesTrustLineCount; // number of trust lines for the
//                                                // associated pool shares
//            } constantProduct;
//        }
//        body;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LiquidityPoolEntry {
    pub liquidity_pool_id: PoolId,
    pub body: LiquidityPoolEntryBody,
}

impl ReadXDR for LiquidityPoolEntry {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            liquidity_pool_id: <PoolId as ReadXDR>::read_xdr(r)?,
            body: <LiquidityPoolEntryBody as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for LiquidityPoolEntry {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.liquidity_pool_id.write_xdr(w)?;
        self.body.write_xdr(w)?;
        Ok(())
    }
}

// LedgerEntryExtensionV1Ext is an XDR NestedUnion defines as:
//
//   union switch (int v)
//        {
//        case 0:
//            void;
//        }
//
// union with discriminant i32
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LedgerEntryExtensionV1Ext {
    V0,
}

impl LedgerEntryExtensionV1Ext {
    fn discriminant(&self) -> i32 {
        match self {
            Self::V0 => 0,
        }
    }
}

impl ReadXDR for LedgerEntryExtensionV1Ext {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: i32 = <i32 as ReadXDR>::read_xdr(r)?;
        let v = match dv.into() {
            0 => Self::V0,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for LedgerEntryExtensionV1Ext {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::V0 => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// LedgerEntryExtensionV1 is an XDR Struct defines as:
//
//   struct LedgerEntryExtensionV1
//    {
//        SponsorshipDescriptor sponsoringID;
//
//        union switch (int v)
//        {
//        case 0:
//            void;
//        }
//        ext;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LedgerEntryExtensionV1 {
    pub sponsoring_id: SponsorshipDescriptor,
    pub ext: LedgerEntryExtensionV1Ext,
}

impl ReadXDR for LedgerEntryExtensionV1 {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            sponsoring_id: <SponsorshipDescriptor as ReadXDR>::read_xdr(r)?,
            ext: <LedgerEntryExtensionV1Ext as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for LedgerEntryExtensionV1 {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.sponsoring_id.write_xdr(w)?;
        self.ext.write_xdr(w)?;
        Ok(())
    }
}

// LedgerEntryData is an XDR NestedUnion defines as:
//
//   union switch (LedgerEntryType type)
//        {
//        case ACCOUNT:
//            AccountEntry account;
//        case TRUSTLINE:
//            TrustLineEntry trustLine;
//        case OFFER:
//            OfferEntry offer;
//        case DATA:
//            DataEntry data;
//        case CLAIMABLE_BALANCE:
//            ClaimableBalanceEntry claimableBalance;
//        case LIQUIDITY_POOL:
//            LiquidityPoolEntry liquidityPool;
//        }
//
// union with discriminant LedgerEntryType
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LedgerEntryData {
    Account(AccountEntry),
    Trustline(TrustLineEntry),
    Offer(OfferEntry),
    Data(DataEntry),
    ClaimableBalance(ClaimableBalanceEntry),
    LiquidityPool(LiquidityPoolEntry),
}

impl LedgerEntryData {
    fn discriminant(&self) -> LedgerEntryType {
        match self {
            Self::Account(_) => LedgerEntryType::Account,
            Self::Trustline(_) => LedgerEntryType::Trustline,
            Self::Offer(_) => LedgerEntryType::Offer,
            Self::Data(_) => LedgerEntryType::Data,
            Self::ClaimableBalance(_) => LedgerEntryType::ClaimableBalance,
            Self::LiquidityPool(_) => LedgerEntryType::LiquidityPool,
        }
    }
}

impl ReadXDR for LedgerEntryData {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: LedgerEntryType = <LedgerEntryType as ReadXDR>::read_xdr(r)?;
        let v = match dv {
            LedgerEntryType::Account => Self::Account(<AccountEntry as ReadXDR>::read_xdr(r)?),
            LedgerEntryType::Trustline => {
                Self::Trustline(<TrustLineEntry as ReadXDR>::read_xdr(r)?)
            }
            LedgerEntryType::Offer => Self::Offer(<OfferEntry as ReadXDR>::read_xdr(r)?),
            LedgerEntryType::Data => Self::Data(<DataEntry as ReadXDR>::read_xdr(r)?),
            LedgerEntryType::ClaimableBalance => {
                Self::ClaimableBalance(<ClaimableBalanceEntry as ReadXDR>::read_xdr(r)?)
            }
            LedgerEntryType::LiquidityPool => {
                Self::LiquidityPool(<LiquidityPoolEntry as ReadXDR>::read_xdr(r)?)
            }
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for LedgerEntryData {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::Account(v) => v.write_xdr(w)?,
            Self::Trustline(v) => v.write_xdr(w)?,
            Self::Offer(v) => v.write_xdr(w)?,
            Self::Data(v) => v.write_xdr(w)?,
            Self::ClaimableBalance(v) => v.write_xdr(w)?,
            Self::LiquidityPool(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// LedgerEntryExt is an XDR NestedUnion defines as:
//
//   union switch (int v)
//        {
//        case 0:
//            void;
//        case 1:
//            LedgerEntryExtensionV1 v1;
//        }
//
// union with discriminant i32
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LedgerEntryExt {
    V0,
    V1(LedgerEntryExtensionV1),
}

impl LedgerEntryExt {
    fn discriminant(&self) -> i32 {
        match self {
            Self::V0 => 0,
            Self::V1(_) => 1,
        }
    }
}

impl ReadXDR for LedgerEntryExt {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: i32 = <i32 as ReadXDR>::read_xdr(r)?;
        let v = match dv.into() {
            0 => Self::V0,
            1 => Self::V1(<LedgerEntryExtensionV1 as ReadXDR>::read_xdr(r)?),
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for LedgerEntryExt {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::V0 => ().write_xdr(w)?,
            Self::V1(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// LedgerEntry is an XDR Struct defines as:
//
//   struct LedgerEntry
//    {
//        uint32 lastModifiedLedgerSeq; // ledger the LedgerEntry was last changed
//
//        union switch (LedgerEntryType type)
//        {
//        case ACCOUNT:
//            AccountEntry account;
//        case TRUSTLINE:
//            TrustLineEntry trustLine;
//        case OFFER:
//            OfferEntry offer;
//        case DATA:
//            DataEntry data;
//        case CLAIMABLE_BALANCE:
//            ClaimableBalanceEntry claimableBalance;
//        case LIQUIDITY_POOL:
//            LiquidityPoolEntry liquidityPool;
//        }
//        data;
//
//        // reserved for future use
//        union switch (int v)
//        {
//        case 0:
//            void;
//        case 1:
//            LedgerEntryExtensionV1 v1;
//        }
//        ext;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LedgerEntry {
    pub last_modified_ledger_seq: Uint32,
    pub data: LedgerEntryData,
    pub ext: LedgerEntryExt,
}

impl ReadXDR for LedgerEntry {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            last_modified_ledger_seq: <Uint32 as ReadXDR>::read_xdr(r)?,
            data: <LedgerEntryData as ReadXDR>::read_xdr(r)?,
            ext: <LedgerEntryExt as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for LedgerEntry {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.last_modified_ledger_seq.write_xdr(w)?;
        self.data.write_xdr(w)?;
        self.ext.write_xdr(w)?;
        Ok(())
    }
}

// LedgerKeyAccount is an XDR NestedStruct defines as:
//
//   struct
//        {
//            AccountID accountID;
//        }
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LedgerKeyAccount {
    pub account_id: AccountId,
}

impl ReadXDR for LedgerKeyAccount {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            account_id: <AccountId as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for LedgerKeyAccount {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.account_id.write_xdr(w)?;
        Ok(())
    }
}

// LedgerKeyTrustLine is an XDR NestedStruct defines as:
//
//   struct
//        {
//            AccountID accountID;
//            TrustLineAsset asset;
//        }
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LedgerKeyTrustLine {
    pub account_id: AccountId,
    pub asset: TrustLineAsset,
}

impl ReadXDR for LedgerKeyTrustLine {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            account_id: <AccountId as ReadXDR>::read_xdr(r)?,
            asset: <TrustLineAsset as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for LedgerKeyTrustLine {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.account_id.write_xdr(w)?;
        self.asset.write_xdr(w)?;
        Ok(())
    }
}

// LedgerKeyOffer is an XDR NestedStruct defines as:
//
//   struct
//        {
//            AccountID sellerID;
//            int64 offerID;
//        }
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LedgerKeyOffer {
    pub seller_id: AccountId,
    pub offer_id: Int64,
}

impl ReadXDR for LedgerKeyOffer {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            seller_id: <AccountId as ReadXDR>::read_xdr(r)?,
            offer_id: <Int64 as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for LedgerKeyOffer {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.seller_id.write_xdr(w)?;
        self.offer_id.write_xdr(w)?;
        Ok(())
    }
}

// LedgerKeyData is an XDR NestedStruct defines as:
//
//   struct
//        {
//            AccountID accountID;
//            string64 dataName;
//        }
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LedgerKeyData {
    pub account_id: AccountId,
    pub data_name: String64,
}

impl ReadXDR for LedgerKeyData {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            account_id: <AccountId as ReadXDR>::read_xdr(r)?,
            data_name: <String64 as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for LedgerKeyData {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.account_id.write_xdr(w)?;
        self.data_name.write_xdr(w)?;
        Ok(())
    }
}

// LedgerKeyClaimableBalance is an XDR NestedStruct defines as:
//
//   struct
//        {
//            ClaimableBalanceID balanceID;
//        }
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LedgerKeyClaimableBalance {
    pub balance_id: ClaimableBalanceId,
}

impl ReadXDR for LedgerKeyClaimableBalance {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            balance_id: <ClaimableBalanceId as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for LedgerKeyClaimableBalance {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.balance_id.write_xdr(w)?;
        Ok(())
    }
}

// LedgerKeyLiquidityPool is an XDR NestedStruct defines as:
//
//   struct
//        {
//            PoolID liquidityPoolID;
//        }
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LedgerKeyLiquidityPool {
    pub liquidity_pool_id: PoolId,
}

impl ReadXDR for LedgerKeyLiquidityPool {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            liquidity_pool_id: <PoolId as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for LedgerKeyLiquidityPool {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.liquidity_pool_id.write_xdr(w)?;
        Ok(())
    }
}

// LedgerKey is an XDR Union defines as:
//
//   union LedgerKey switch (LedgerEntryType type)
//    {
//    case ACCOUNT:
//        struct
//        {
//            AccountID accountID;
//        } account;
//
//    case TRUSTLINE:
//        struct
//        {
//            AccountID accountID;
//            TrustLineAsset asset;
//        } trustLine;
//
//    case OFFER:
//        struct
//        {
//            AccountID sellerID;
//            int64 offerID;
//        } offer;
//
//    case DATA:
//        struct
//        {
//            AccountID accountID;
//            string64 dataName;
//        } data;
//
//    case CLAIMABLE_BALANCE:
//        struct
//        {
//            ClaimableBalanceID balanceID;
//        } claimableBalance;
//
//    case LIQUIDITY_POOL:
//        struct
//        {
//            PoolID liquidityPoolID;
//        } liquidityPool;
//    };
//
// union with discriminant LedgerEntryType
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LedgerKey {
    Account(LedgerKeyAccount),
    Trustline(LedgerKeyTrustLine),
    Offer(LedgerKeyOffer),
    Data(LedgerKeyData),
    ClaimableBalance(LedgerKeyClaimableBalance),
    LiquidityPool(LedgerKeyLiquidityPool),
}

impl LedgerKey {
    fn discriminant(&self) -> LedgerEntryType {
        match self {
            Self::Account(_) => LedgerEntryType::Account,
            Self::Trustline(_) => LedgerEntryType::Trustline,
            Self::Offer(_) => LedgerEntryType::Offer,
            Self::Data(_) => LedgerEntryType::Data,
            Self::ClaimableBalance(_) => LedgerEntryType::ClaimableBalance,
            Self::LiquidityPool(_) => LedgerEntryType::LiquidityPool,
        }
    }
}

impl ReadXDR for LedgerKey {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: LedgerEntryType = <LedgerEntryType as ReadXDR>::read_xdr(r)?;
        let v = match dv {
            LedgerEntryType::Account => Self::Account(<LedgerKeyAccount as ReadXDR>::read_xdr(r)?),
            LedgerEntryType::Trustline => {
                Self::Trustline(<LedgerKeyTrustLine as ReadXDR>::read_xdr(r)?)
            }
            LedgerEntryType::Offer => Self::Offer(<LedgerKeyOffer as ReadXDR>::read_xdr(r)?),
            LedgerEntryType::Data => Self::Data(<LedgerKeyData as ReadXDR>::read_xdr(r)?),
            LedgerEntryType::ClaimableBalance => {
                Self::ClaimableBalance(<LedgerKeyClaimableBalance as ReadXDR>::read_xdr(r)?)
            }
            LedgerEntryType::LiquidityPool => {
                Self::LiquidityPool(<LedgerKeyLiquidityPool as ReadXDR>::read_xdr(r)?)
            }
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for LedgerKey {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::Account(v) => v.write_xdr(w)?,
            Self::Trustline(v) => v.write_xdr(w)?,
            Self::Offer(v) => v.write_xdr(w)?,
            Self::Data(v) => v.write_xdr(w)?,
            Self::ClaimableBalance(v) => v.write_xdr(w)?,
            Self::LiquidityPool(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// EnvelopeType is an XDR Enum defines as:
//
//   enum EnvelopeType
//    {
//        ENVELOPE_TYPE_TX_V0 = 0,
//        ENVELOPE_TYPE_SCP = 1,
//        ENVELOPE_TYPE_TX = 2,
//        ENVELOPE_TYPE_AUTH = 3,
//        ENVELOPE_TYPE_SCPVALUE = 4,
//        ENVELOPE_TYPE_TX_FEE_BUMP = 5,
//        ENVELOPE_TYPE_OP_ID = 6,
//        ENVELOPE_TYPE_POOL_REVOKE_OP_ID = 7
//    };
//
// enum
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum EnvelopeType {
    EnvelopeTypeTxV0 = 0,
    EnvelopeTypeScp = 1,
    EnvelopeTypeTx = 2,
    EnvelopeTypeAuth = 3,
    EnvelopeTypeScpvalue = 4,
    EnvelopeTypeTxFeeBump = 5,
    EnvelopeTypeOpId = 6,
    EnvelopeTypePoolRevokeOpId = 7,
}

impl TryFrom<i32> for EnvelopeType {
    type Error = Error;

    fn try_from(i: i32) -> std::result::Result<Self, Self::Error> {
        let e = match i {
            0 => EnvelopeType::EnvelopeTypeTxV0,
            1 => EnvelopeType::EnvelopeTypeScp,
            2 => EnvelopeType::EnvelopeTypeTx,
            3 => EnvelopeType::EnvelopeTypeAuth,
            4 => EnvelopeType::EnvelopeTypeScpvalue,
            5 => EnvelopeType::EnvelopeTypeTxFeeBump,
            6 => EnvelopeType::EnvelopeTypeOpId,
            7 => EnvelopeType::EnvelopeTypePoolRevokeOpId,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<EnvelopeType> for i32 {
    fn from(e: EnvelopeType) -> Self {
        e as Self
    }
}

impl ReadXDR for EnvelopeType {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXDR for EnvelopeType {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// UpgradeType is an XDR Typedef defines as:
//
//   typedef opaque UpgradeType<128>;
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UpgradeType(pub Vec<u8>);

impl From<UpgradeType> for Vec<u8> {
    fn from(x: UpgradeType) -> Self {
        x.0
    }
}

impl From<Vec<u8>> for UpgradeType {
    fn from(x: Vec<u8>) -> Self {
        UpgradeType(x)
    }
}

impl ReadXDR for UpgradeType {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let i = <Vec<u8> as ReadXDR>::read_xdr(r)?;
        let v = UpgradeType(i);
        Ok(v)
    }
}

impl WriteXDR for UpgradeType {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.0.write_xdr(w)
    }
}

// StellarValueType is an XDR Enum defines as:
//
//   enum StellarValueType
//    {
//        STELLAR_VALUE_BASIC = 0,
//        STELLAR_VALUE_SIGNED = 1
//    };
//
// enum
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum StellarValueType {
    StellarValueBasic = 0,
    StellarValueSigned = 1,
}

impl TryFrom<i32> for StellarValueType {
    type Error = Error;

    fn try_from(i: i32) -> std::result::Result<Self, Self::Error> {
        let e = match i {
            0 => StellarValueType::StellarValueBasic,
            1 => StellarValueType::StellarValueSigned,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<StellarValueType> for i32 {
    fn from(e: StellarValueType) -> Self {
        e as Self
    }
}

impl ReadXDR for StellarValueType {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXDR for StellarValueType {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// LedgerCloseValueSignature is an XDR Struct defines as:
//
//   struct LedgerCloseValueSignature
//    {
//        NodeID nodeID;       // which node introduced the value
//        Signature signature; // nodeID's signature
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LedgerCloseValueSignature {
    pub node_id: NodeId,
    pub signature: Signature,
}

impl ReadXDR for LedgerCloseValueSignature {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            node_id: <NodeId as ReadXDR>::read_xdr(r)?,
            signature: <Signature as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for LedgerCloseValueSignature {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.node_id.write_xdr(w)?;
        self.signature.write_xdr(w)?;
        Ok(())
    }
}

// StellarValueExt is an XDR NestedUnion defines as:
//
//   union switch (StellarValueType v)
//        {
//        case STELLAR_VALUE_BASIC:
//            void;
//        case STELLAR_VALUE_SIGNED:
//            LedgerCloseValueSignature lcValueSignature;
//        }
//
// union with discriminant StellarValueType
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StellarValueExt {
    StellarValueBasic,
    StellarValueSigned(LedgerCloseValueSignature),
}

impl StellarValueExt {
    fn discriminant(&self) -> StellarValueType {
        match self {
            Self::StellarValueBasic => StellarValueType::StellarValueBasic,
            Self::StellarValueSigned(_) => StellarValueType::StellarValueSigned,
        }
    }
}

impl ReadXDR for StellarValueExt {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: StellarValueType = <StellarValueType as ReadXDR>::read_xdr(r)?;
        let v = match dv {
            StellarValueType::StellarValueBasic => Self::StellarValueBasic,
            StellarValueType::StellarValueSigned => {
                Self::StellarValueSigned(<LedgerCloseValueSignature as ReadXDR>::read_xdr(r)?)
            }
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for StellarValueExt {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::StellarValueBasic => ().write_xdr(w)?,
            Self::StellarValueSigned(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// StellarValue is an XDR Struct defines as:
//
//   struct StellarValue
//    {
//        Hash txSetHash;      // transaction set to apply to previous ledger
//        TimePoint closeTime; // network close time
//
//        // upgrades to apply to the previous ledger (usually empty)
//        // this is a vector of encoded 'LedgerUpgrade' so that nodes can drop
//        // unknown steps during consensus if needed.
//        // see notes below on 'LedgerUpgrade' for more detail
//        // max size is dictated by number of upgrade types (+ room for future)
//        UpgradeType upgrades<6>;
//
//        // reserved for future use
//        union switch (StellarValueType v)
//        {
//        case STELLAR_VALUE_BASIC:
//            void;
//        case STELLAR_VALUE_SIGNED:
//            LedgerCloseValueSignature lcValueSignature;
//        }
//        ext;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StellarValue {
    pub tx_set_hash: Hash,
    pub close_time: TimePoint,
    pub upgrades: Vec<UpgradeType>,
    pub ext: StellarValueExt,
}

impl ReadXDR for StellarValue {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            tx_set_hash: <Hash as ReadXDR>::read_xdr(r)?,
            close_time: <TimePoint as ReadXDR>::read_xdr(r)?,
            upgrades: <Vec<UpgradeType> as ReadXDR>::read_xdr(r)?,
            ext: <StellarValueExt as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for StellarValue {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.tx_set_hash.write_xdr(w)?;
        self.close_time.write_xdr(w)?;
        self.upgrades.write_xdr(w)?;
        self.ext.write_xdr(w)?;
        Ok(())
    }
}

// MaskLedgerHeaderFlags is an XDR Const defines as:
//
//   const MASK_LEDGER_HEADER_FLAGS = 0x7;
//
pub const MASK_LEDGER_HEADER_FLAGS: u64 = 0x7;

// LedgerHeaderFlags is an XDR Enum defines as:
//
//   enum LedgerHeaderFlags
//    {
//        DISABLE_LIQUIDITY_POOL_TRADING_FLAG = 0x1,
//        DISABLE_LIQUIDITY_POOL_DEPOSIT_FLAG = 0x2,
//        DISABLE_LIQUIDITY_POOL_WITHDRAWAL_FLAG = 0x4
//    };
//
// enum
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum LedgerHeaderFlags {
    DisableLiquidityPoolTradingFlag = 1,
    DisableLiquidityPoolDepositFlag = 2,
    DisableLiquidityPoolWithdrawalFlag = 4,
}

impl TryFrom<i32> for LedgerHeaderFlags {
    type Error = Error;

    fn try_from(i: i32) -> std::result::Result<Self, Self::Error> {
        let e = match i {
            1 => LedgerHeaderFlags::DisableLiquidityPoolTradingFlag,
            2 => LedgerHeaderFlags::DisableLiquidityPoolDepositFlag,
            4 => LedgerHeaderFlags::DisableLiquidityPoolWithdrawalFlag,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<LedgerHeaderFlags> for i32 {
    fn from(e: LedgerHeaderFlags) -> Self {
        e as Self
    }
}

impl ReadXDR for LedgerHeaderFlags {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXDR for LedgerHeaderFlags {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// LedgerHeaderExtensionV1Ext is an XDR NestedUnion defines as:
//
//   union switch (int v)
//        {
//        case 0:
//            void;
//        }
//
// union with discriminant i32
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LedgerHeaderExtensionV1Ext {
    V0,
}

impl LedgerHeaderExtensionV1Ext {
    fn discriminant(&self) -> i32 {
        match self {
            Self::V0 => 0,
        }
    }
}

impl ReadXDR for LedgerHeaderExtensionV1Ext {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: i32 = <i32 as ReadXDR>::read_xdr(r)?;
        let v = match dv.into() {
            0 => Self::V0,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for LedgerHeaderExtensionV1Ext {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::V0 => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// LedgerHeaderExtensionV1 is an XDR Struct defines as:
//
//   struct LedgerHeaderExtensionV1
//    {
//        uint32 flags; // LedgerHeaderFlags
//
//        union switch (int v)
//        {
//        case 0:
//            void;
//        }
//        ext;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LedgerHeaderExtensionV1 {
    pub flags: Uint32,
    pub ext: LedgerHeaderExtensionV1Ext,
}

impl ReadXDR for LedgerHeaderExtensionV1 {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            flags: <Uint32 as ReadXDR>::read_xdr(r)?,
            ext: <LedgerHeaderExtensionV1Ext as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for LedgerHeaderExtensionV1 {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.flags.write_xdr(w)?;
        self.ext.write_xdr(w)?;
        Ok(())
    }
}

// LedgerHeaderExt is an XDR NestedUnion defines as:
//
//   union switch (int v)
//        {
//        case 0:
//            void;
//        case 1:
//            LedgerHeaderExtensionV1 v1;
//        }
//
// union with discriminant i32
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LedgerHeaderExt {
    V0,
    V1(LedgerHeaderExtensionV1),
}

impl LedgerHeaderExt {
    fn discriminant(&self) -> i32 {
        match self {
            Self::V0 => 0,
            Self::V1(_) => 1,
        }
    }
}

impl ReadXDR for LedgerHeaderExt {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: i32 = <i32 as ReadXDR>::read_xdr(r)?;
        let v = match dv.into() {
            0 => Self::V0,
            1 => Self::V1(<LedgerHeaderExtensionV1 as ReadXDR>::read_xdr(r)?),
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for LedgerHeaderExt {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::V0 => ().write_xdr(w)?,
            Self::V1(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// LedgerHeader is an XDR Struct defines as:
//
//   struct LedgerHeader
//    {
//        uint32 ledgerVersion;    // the protocol version of the ledger
//        Hash previousLedgerHash; // hash of the previous ledger header
//        StellarValue scpValue;   // what consensus agreed to
//        Hash txSetResultHash;    // the TransactionResultSet that led to this ledger
//        Hash bucketListHash;     // hash of the ledger state
//
//        uint32 ledgerSeq; // sequence number of this ledger
//
//        int64 totalCoins; // total number of stroops in existence.
//                          // 10,000,000 stroops in 1 XLM
//
//        int64 feePool;       // fees burned since last inflation run
//        uint32 inflationSeq; // inflation sequence number
//
//        uint64 idPool; // last used global ID, used for generating objects
//
//        uint32 baseFee;     // base fee per operation in stroops
//        uint32 baseReserve; // account base reserve in stroops
//
//        uint32 maxTxSetSize; // maximum size a transaction set can be
//
//        Hash skipList[4]; // hashes of ledgers in the past. allows you to jump back
//                          // in time without walking the chain back ledger by ledger
//                          // each slot contains the oldest ledger that is mod of
//                          // either 50  5000  50000 or 500000 depending on index
//                          // skipList[0] mod(50), skipList[1] mod(5000), etc
//
//        // reserved for future use
//        union switch (int v)
//        {
//        case 0:
//            void;
//        case 1:
//            LedgerHeaderExtensionV1 v1;
//        }
//        ext;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LedgerHeader {
    pub ledger_version: Uint32,
    pub previous_ledger_hash: Hash,
    pub scp_value: StellarValue,
    pub tx_set_result_hash: Hash,
    pub bucket_list_hash: Hash,
    pub ledger_seq: Uint32,
    pub total_coins: Int64,
    pub fee_pool: Int64,
    pub inflation_seq: Uint32,
    pub id_pool: Uint64,
    pub base_fee: Uint32,
    pub base_reserve: Uint32,
    pub max_tx_set_size: Uint32,
    pub skip_list: [Hash; 4],
    pub ext: LedgerHeaderExt,
}

impl ReadXDR for LedgerHeader {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            ledger_version: <Uint32 as ReadXDR>::read_xdr(r)?,
            previous_ledger_hash: <Hash as ReadXDR>::read_xdr(r)?,
            scp_value: <StellarValue as ReadXDR>::read_xdr(r)?,
            tx_set_result_hash: <Hash as ReadXDR>::read_xdr(r)?,
            bucket_list_hash: <Hash as ReadXDR>::read_xdr(r)?,
            ledger_seq: <Uint32 as ReadXDR>::read_xdr(r)?,
            total_coins: <Int64 as ReadXDR>::read_xdr(r)?,
            fee_pool: <Int64 as ReadXDR>::read_xdr(r)?,
            inflation_seq: <Uint32 as ReadXDR>::read_xdr(r)?,
            id_pool: <Uint64 as ReadXDR>::read_xdr(r)?,
            base_fee: <Uint32 as ReadXDR>::read_xdr(r)?,
            base_reserve: <Uint32 as ReadXDR>::read_xdr(r)?,
            max_tx_set_size: <Uint32 as ReadXDR>::read_xdr(r)?,
            skip_list: <[Hash; 4] as ReadXDR>::read_xdr(r)?,
            ext: <LedgerHeaderExt as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for LedgerHeader {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.ledger_version.write_xdr(w)?;
        self.previous_ledger_hash.write_xdr(w)?;
        self.scp_value.write_xdr(w)?;
        self.tx_set_result_hash.write_xdr(w)?;
        self.bucket_list_hash.write_xdr(w)?;
        self.ledger_seq.write_xdr(w)?;
        self.total_coins.write_xdr(w)?;
        self.fee_pool.write_xdr(w)?;
        self.inflation_seq.write_xdr(w)?;
        self.id_pool.write_xdr(w)?;
        self.base_fee.write_xdr(w)?;
        self.base_reserve.write_xdr(w)?;
        self.max_tx_set_size.write_xdr(w)?;
        self.skip_list.write_xdr(w)?;
        self.ext.write_xdr(w)?;
        Ok(())
    }
}

// LedgerUpgradeType is an XDR Enum defines as:
//
//   enum LedgerUpgradeType
//    {
//        LEDGER_UPGRADE_VERSION = 1,
//        LEDGER_UPGRADE_BASE_FEE = 2,
//        LEDGER_UPGRADE_MAX_TX_SET_SIZE = 3,
//        LEDGER_UPGRADE_BASE_RESERVE = 4,
//        LEDGER_UPGRADE_FLAGS = 5
//    };
//
// enum
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum LedgerUpgradeType {
    LedgerUpgradeVersion = 1,
    LedgerUpgradeBaseFee = 2,
    LedgerUpgradeMaxTxSetSize = 3,
    LedgerUpgradeBaseReserve = 4,
    LedgerUpgradeFlags = 5,
}

impl TryFrom<i32> for LedgerUpgradeType {
    type Error = Error;

    fn try_from(i: i32) -> std::result::Result<Self, Self::Error> {
        let e = match i {
            1 => LedgerUpgradeType::LedgerUpgradeVersion,
            2 => LedgerUpgradeType::LedgerUpgradeBaseFee,
            3 => LedgerUpgradeType::LedgerUpgradeMaxTxSetSize,
            4 => LedgerUpgradeType::LedgerUpgradeBaseReserve,
            5 => LedgerUpgradeType::LedgerUpgradeFlags,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<LedgerUpgradeType> for i32 {
    fn from(e: LedgerUpgradeType) -> Self {
        e as Self
    }
}

impl ReadXDR for LedgerUpgradeType {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXDR for LedgerUpgradeType {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// LedgerUpgrade is an XDR Union defines as:
//
//   union LedgerUpgrade switch (LedgerUpgradeType type)
//    {
//    case LEDGER_UPGRADE_VERSION:
//        uint32 newLedgerVersion; // update ledgerVersion
//    case LEDGER_UPGRADE_BASE_FEE:
//        uint32 newBaseFee; // update baseFee
//    case LEDGER_UPGRADE_MAX_TX_SET_SIZE:
//        uint32 newMaxTxSetSize; // update maxTxSetSize
//    case LEDGER_UPGRADE_BASE_RESERVE:
//        uint32 newBaseReserve; // update baseReserve
//    case LEDGER_UPGRADE_FLAGS:
//        uint32 newFlags; // update flags
//    };
//
// union with discriminant LedgerUpgradeType
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LedgerUpgrade {
    LedgerUpgradeVersion(Uint32),
    LedgerUpgradeBaseFee(Uint32),
    LedgerUpgradeMaxTxSetSize(Uint32),
    LedgerUpgradeBaseReserve(Uint32),
    LedgerUpgradeFlags(Uint32),
}

impl LedgerUpgrade {
    fn discriminant(&self) -> LedgerUpgradeType {
        match self {
            Self::LedgerUpgradeVersion(_) => LedgerUpgradeType::LedgerUpgradeVersion,
            Self::LedgerUpgradeBaseFee(_) => LedgerUpgradeType::LedgerUpgradeBaseFee,
            Self::LedgerUpgradeMaxTxSetSize(_) => LedgerUpgradeType::LedgerUpgradeMaxTxSetSize,
            Self::LedgerUpgradeBaseReserve(_) => LedgerUpgradeType::LedgerUpgradeBaseReserve,
            Self::LedgerUpgradeFlags(_) => LedgerUpgradeType::LedgerUpgradeFlags,
        }
    }
}

impl ReadXDR for LedgerUpgrade {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: LedgerUpgradeType = <LedgerUpgradeType as ReadXDR>::read_xdr(r)?;
        let v = match dv {
            LedgerUpgradeType::LedgerUpgradeVersion => {
                Self::LedgerUpgradeVersion(<Uint32 as ReadXDR>::read_xdr(r)?)
            }
            LedgerUpgradeType::LedgerUpgradeBaseFee => {
                Self::LedgerUpgradeBaseFee(<Uint32 as ReadXDR>::read_xdr(r)?)
            }
            LedgerUpgradeType::LedgerUpgradeMaxTxSetSize => {
                Self::LedgerUpgradeMaxTxSetSize(<Uint32 as ReadXDR>::read_xdr(r)?)
            }
            LedgerUpgradeType::LedgerUpgradeBaseReserve => {
                Self::LedgerUpgradeBaseReserve(<Uint32 as ReadXDR>::read_xdr(r)?)
            }
            LedgerUpgradeType::LedgerUpgradeFlags => {
                Self::LedgerUpgradeFlags(<Uint32 as ReadXDR>::read_xdr(r)?)
            }
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for LedgerUpgrade {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::LedgerUpgradeVersion(v) => v.write_xdr(w)?,
            Self::LedgerUpgradeBaseFee(v) => v.write_xdr(w)?,
            Self::LedgerUpgradeMaxTxSetSize(v) => v.write_xdr(w)?,
            Self::LedgerUpgradeBaseReserve(v) => v.write_xdr(w)?,
            Self::LedgerUpgradeFlags(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// BucketEntryType is an XDR Enum defines as:
//
//   enum BucketEntryType
//    {
//        METAENTRY =
//            -1, // At-and-after protocol 11: bucket metadata, should come first.
//        LIVEENTRY = 0, // Before protocol 11: created-or-updated;
//                       // At-and-after protocol 11: only updated.
//        DEADENTRY = 1,
//        INITENTRY = 2 // At-and-after protocol 11: only created.
//    };
//
// enum
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum BucketEntryType {
    Metaentry = -1,
    Liveentry = 0,
    Deadentry = 1,
    Initentry = 2,
}

impl TryFrom<i32> for BucketEntryType {
    type Error = Error;

    fn try_from(i: i32) -> std::result::Result<Self, Self::Error> {
        let e = match i {
            -1 => BucketEntryType::Metaentry,
            0 => BucketEntryType::Liveentry,
            1 => BucketEntryType::Deadentry,
            2 => BucketEntryType::Initentry,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<BucketEntryType> for i32 {
    fn from(e: BucketEntryType) -> Self {
        e as Self
    }
}

impl ReadXDR for BucketEntryType {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXDR for BucketEntryType {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// BucketMetadataExt is an XDR NestedUnion defines as:
//
//   union switch (int v)
//        {
//        case 0:
//            void;
//        }
//
// union with discriminant i32
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BucketMetadataExt {
    V0,
}

impl BucketMetadataExt {
    fn discriminant(&self) -> i32 {
        match self {
            Self::V0 => 0,
        }
    }
}

impl ReadXDR for BucketMetadataExt {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: i32 = <i32 as ReadXDR>::read_xdr(r)?;
        let v = match dv.into() {
            0 => Self::V0,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for BucketMetadataExt {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::V0 => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// BucketMetadata is an XDR Struct defines as:
//
//   struct BucketMetadata
//    {
//        // Indicates the protocol version used to create / merge this bucket.
//        uint32 ledgerVersion;
//
//        // reserved for future use
//        union switch (int v)
//        {
//        case 0:
//            void;
//        }
//        ext;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BucketMetadata {
    pub ledger_version: Uint32,
    pub ext: BucketMetadataExt,
}

impl ReadXDR for BucketMetadata {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            ledger_version: <Uint32 as ReadXDR>::read_xdr(r)?,
            ext: <BucketMetadataExt as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for BucketMetadata {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.ledger_version.write_xdr(w)?;
        self.ext.write_xdr(w)?;
        Ok(())
    }
}

// BucketEntry is an XDR Union defines as:
//
//   union BucketEntry switch (BucketEntryType type)
//    {
//    case LIVEENTRY:
//    case INITENTRY:
//        LedgerEntry liveEntry;
//
//    case DEADENTRY:
//        LedgerKey deadEntry;
//    case METAENTRY:
//        BucketMetadata metaEntry;
//    };
//
// union with discriminant BucketEntryType
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BucketEntry {
    Liveentry(LedgerEntry),
    Initentry(LedgerEntry),
    Deadentry(LedgerKey),
    Metaentry(BucketMetadata),
}

impl BucketEntry {
    fn discriminant(&self) -> BucketEntryType {
        match self {
            Self::Liveentry(_) => BucketEntryType::Liveentry,
            Self::Initentry(_) => BucketEntryType::Initentry,
            Self::Deadentry(_) => BucketEntryType::Deadentry,
            Self::Metaentry(_) => BucketEntryType::Metaentry,
        }
    }
}

impl ReadXDR for BucketEntry {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: BucketEntryType = <BucketEntryType as ReadXDR>::read_xdr(r)?;
        let v = match dv {
            BucketEntryType::Liveentry => Self::Liveentry(<LedgerEntry as ReadXDR>::read_xdr(r)?),
            BucketEntryType::Initentry => Self::Initentry(<LedgerEntry as ReadXDR>::read_xdr(r)?),
            BucketEntryType::Deadentry => Self::Deadentry(<LedgerKey as ReadXDR>::read_xdr(r)?),
            BucketEntryType::Metaentry => {
                Self::Metaentry(<BucketMetadata as ReadXDR>::read_xdr(r)?)
            }
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for BucketEntry {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::Liveentry(v) => v.write_xdr(w)?,
            Self::Initentry(v) => v.write_xdr(w)?,
            Self::Deadentry(v) => v.write_xdr(w)?,
            Self::Metaentry(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// TransactionSet is an XDR Struct defines as:
//
//   struct TransactionSet
//    {
//        Hash previousLedgerHash;
//        TransactionEnvelope txs<>;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransactionSet {
    pub previous_ledger_hash: Hash,
    pub txs: Vec<TransactionEnvelope>,
}

impl ReadXDR for TransactionSet {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            previous_ledger_hash: <Hash as ReadXDR>::read_xdr(r)?,
            txs: <Vec<TransactionEnvelope> as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for TransactionSet {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.previous_ledger_hash.write_xdr(w)?;
        self.txs.write_xdr(w)?;
        Ok(())
    }
}

// TransactionResultPair is an XDR Struct defines as:
//
//   struct TransactionResultPair
//    {
//        Hash transactionHash;
//        TransactionResult result; // result for the transaction
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransactionResultPair {
    pub transaction_hash: Hash,
    pub result: TransactionResult,
}

impl ReadXDR for TransactionResultPair {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            transaction_hash: <Hash as ReadXDR>::read_xdr(r)?,
            result: <TransactionResult as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for TransactionResultPair {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.transaction_hash.write_xdr(w)?;
        self.result.write_xdr(w)?;
        Ok(())
    }
}

// TransactionResultSet is an XDR Struct defines as:
//
//   struct TransactionResultSet
//    {
//        TransactionResultPair results<>;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransactionResultSet {
    pub results: Vec<TransactionResultPair>,
}

impl ReadXDR for TransactionResultSet {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            results: <Vec<TransactionResultPair> as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for TransactionResultSet {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.results.write_xdr(w)?;
        Ok(())
    }
}

// TransactionHistoryEntryExt is an XDR NestedUnion defines as:
//
//   union switch (int v)
//        {
//        case 0:
//            void;
//        }
//
// union with discriminant i32
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TransactionHistoryEntryExt {
    V0,
}

impl TransactionHistoryEntryExt {
    fn discriminant(&self) -> i32 {
        match self {
            Self::V0 => 0,
        }
    }
}

impl ReadXDR for TransactionHistoryEntryExt {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: i32 = <i32 as ReadXDR>::read_xdr(r)?;
        let v = match dv.into() {
            0 => Self::V0,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for TransactionHistoryEntryExt {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::V0 => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// TransactionHistoryEntry is an XDR Struct defines as:
//
//   struct TransactionHistoryEntry
//    {
//        uint32 ledgerSeq;
//        TransactionSet txSet;
//
//        // reserved for future use
//        union switch (int v)
//        {
//        case 0:
//            void;
//        }
//        ext;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransactionHistoryEntry {
    pub ledger_seq: Uint32,
    pub tx_set: TransactionSet,
    pub ext: TransactionHistoryEntryExt,
}

impl ReadXDR for TransactionHistoryEntry {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            ledger_seq: <Uint32 as ReadXDR>::read_xdr(r)?,
            tx_set: <TransactionSet as ReadXDR>::read_xdr(r)?,
            ext: <TransactionHistoryEntryExt as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for TransactionHistoryEntry {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.ledger_seq.write_xdr(w)?;
        self.tx_set.write_xdr(w)?;
        self.ext.write_xdr(w)?;
        Ok(())
    }
}

// TransactionHistoryResultEntryExt is an XDR NestedUnion defines as:
//
//   union switch (int v)
//        {
//        case 0:
//            void;
//        }
//
// union with discriminant i32
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TransactionHistoryResultEntryExt {
    V0,
}

impl TransactionHistoryResultEntryExt {
    fn discriminant(&self) -> i32 {
        match self {
            Self::V0 => 0,
        }
    }
}

impl ReadXDR for TransactionHistoryResultEntryExt {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: i32 = <i32 as ReadXDR>::read_xdr(r)?;
        let v = match dv.into() {
            0 => Self::V0,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for TransactionHistoryResultEntryExt {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::V0 => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// TransactionHistoryResultEntry is an XDR Struct defines as:
//
//   struct TransactionHistoryResultEntry
//    {
//        uint32 ledgerSeq;
//        TransactionResultSet txResultSet;
//
//        // reserved for future use
//        union switch (int v)
//        {
//        case 0:
//            void;
//        }
//        ext;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransactionHistoryResultEntry {
    pub ledger_seq: Uint32,
    pub tx_result_set: TransactionResultSet,
    pub ext: TransactionHistoryResultEntryExt,
}

impl ReadXDR for TransactionHistoryResultEntry {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            ledger_seq: <Uint32 as ReadXDR>::read_xdr(r)?,
            tx_result_set: <TransactionResultSet as ReadXDR>::read_xdr(r)?,
            ext: <TransactionHistoryResultEntryExt as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for TransactionHistoryResultEntry {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.ledger_seq.write_xdr(w)?;
        self.tx_result_set.write_xdr(w)?;
        self.ext.write_xdr(w)?;
        Ok(())
    }
}

// LedgerHeaderHistoryEntryExt is an XDR NestedUnion defines as:
//
//   union switch (int v)
//        {
//        case 0:
//            void;
//        }
//
// union with discriminant i32
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LedgerHeaderHistoryEntryExt {
    V0,
}

impl LedgerHeaderHistoryEntryExt {
    fn discriminant(&self) -> i32 {
        match self {
            Self::V0 => 0,
        }
    }
}

impl ReadXDR for LedgerHeaderHistoryEntryExt {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: i32 = <i32 as ReadXDR>::read_xdr(r)?;
        let v = match dv.into() {
            0 => Self::V0,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for LedgerHeaderHistoryEntryExt {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::V0 => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// LedgerHeaderHistoryEntry is an XDR Struct defines as:
//
//   struct LedgerHeaderHistoryEntry
//    {
//        Hash hash;
//        LedgerHeader header;
//
//        // reserved for future use
//        union switch (int v)
//        {
//        case 0:
//            void;
//        }
//        ext;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LedgerHeaderHistoryEntry {
    pub hash: Hash,
    pub header: LedgerHeader,
    pub ext: LedgerHeaderHistoryEntryExt,
}

impl ReadXDR for LedgerHeaderHistoryEntry {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            hash: <Hash as ReadXDR>::read_xdr(r)?,
            header: <LedgerHeader as ReadXDR>::read_xdr(r)?,
            ext: <LedgerHeaderHistoryEntryExt as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for LedgerHeaderHistoryEntry {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.hash.write_xdr(w)?;
        self.header.write_xdr(w)?;
        self.ext.write_xdr(w)?;
        Ok(())
    }
}

// LedgerScpMessages is an XDR Struct defines as:
//
//   struct LedgerSCPMessages
//    {
//        uint32 ledgerSeq;
//        SCPEnvelope messages<>;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LedgerScpMessages {
    pub ledger_seq: Uint32,
    pub messages: Vec<ScpEnvelope>,
}

impl ReadXDR for LedgerScpMessages {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            ledger_seq: <Uint32 as ReadXDR>::read_xdr(r)?,
            messages: <Vec<ScpEnvelope> as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for LedgerScpMessages {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.ledger_seq.write_xdr(w)?;
        self.messages.write_xdr(w)?;
        Ok(())
    }
}

// ScpHistoryEntryV0 is an XDR Struct defines as:
//
//   struct SCPHistoryEntryV0
//    {
//        SCPQuorumSet quorumSets<>; // additional quorum sets used by ledgerMessages
//        LedgerSCPMessages ledgerMessages;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScpHistoryEntryV0 {
    pub quorum_sets: Vec<ScpQuorumSet>,
    pub ledger_messages: LedgerScpMessages,
}

impl ReadXDR for ScpHistoryEntryV0 {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            quorum_sets: <Vec<ScpQuorumSet> as ReadXDR>::read_xdr(r)?,
            ledger_messages: <LedgerScpMessages as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for ScpHistoryEntryV0 {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.quorum_sets.write_xdr(w)?;
        self.ledger_messages.write_xdr(w)?;
        Ok(())
    }
}

// ScpHistoryEntry is an XDR Union defines as:
//
//   union SCPHistoryEntry switch (int v)
//    {
//    case 0:
//        SCPHistoryEntryV0 v0;
//    };
//
// union with discriminant i32
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ScpHistoryEntry {
    V0(ScpHistoryEntryV0),
}

impl ScpHistoryEntry {
    fn discriminant(&self) -> i32 {
        match self {
            Self::V0(_) => 0,
        }
    }
}

impl ReadXDR for ScpHistoryEntry {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: i32 = <i32 as ReadXDR>::read_xdr(r)?;
        let v = match dv.into() {
            0 => Self::V0(<ScpHistoryEntryV0 as ReadXDR>::read_xdr(r)?),
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for ScpHistoryEntry {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::V0(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// LedgerEntryChangeType is an XDR Enum defines as:
//
//   enum LedgerEntryChangeType
//    {
//        LEDGER_ENTRY_CREATED = 0, // entry was added to the ledger
//        LEDGER_ENTRY_UPDATED = 1, // entry was modified in the ledger
//        LEDGER_ENTRY_REMOVED = 2, // entry was removed from the ledger
//        LEDGER_ENTRY_STATE = 3    // value of the entry
//    };
//
// enum
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum LedgerEntryChangeType {
    LedgerEntryCreated = 0,
    LedgerEntryUpdated = 1,
    LedgerEntryRemoved = 2,
    LedgerEntryState = 3,
}

impl TryFrom<i32> for LedgerEntryChangeType {
    type Error = Error;

    fn try_from(i: i32) -> std::result::Result<Self, Self::Error> {
        let e = match i {
            0 => LedgerEntryChangeType::LedgerEntryCreated,
            1 => LedgerEntryChangeType::LedgerEntryUpdated,
            2 => LedgerEntryChangeType::LedgerEntryRemoved,
            3 => LedgerEntryChangeType::LedgerEntryState,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<LedgerEntryChangeType> for i32 {
    fn from(e: LedgerEntryChangeType) -> Self {
        e as Self
    }
}

impl ReadXDR for LedgerEntryChangeType {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXDR for LedgerEntryChangeType {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// LedgerEntryChange is an XDR Union defines as:
//
//   union LedgerEntryChange switch (LedgerEntryChangeType type)
//    {
//    case LEDGER_ENTRY_CREATED:
//        LedgerEntry created;
//    case LEDGER_ENTRY_UPDATED:
//        LedgerEntry updated;
//    case LEDGER_ENTRY_REMOVED:
//        LedgerKey removed;
//    case LEDGER_ENTRY_STATE:
//        LedgerEntry state;
//    };
//
// union with discriminant LedgerEntryChangeType
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LedgerEntryChange {
    LedgerEntryCreated(LedgerEntry),
    LedgerEntryUpdated(LedgerEntry),
    LedgerEntryRemoved(LedgerKey),
    LedgerEntryState(LedgerEntry),
}

impl LedgerEntryChange {
    fn discriminant(&self) -> LedgerEntryChangeType {
        match self {
            Self::LedgerEntryCreated(_) => LedgerEntryChangeType::LedgerEntryCreated,
            Self::LedgerEntryUpdated(_) => LedgerEntryChangeType::LedgerEntryUpdated,
            Self::LedgerEntryRemoved(_) => LedgerEntryChangeType::LedgerEntryRemoved,
            Self::LedgerEntryState(_) => LedgerEntryChangeType::LedgerEntryState,
        }
    }
}

impl ReadXDR for LedgerEntryChange {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: LedgerEntryChangeType = <LedgerEntryChangeType as ReadXDR>::read_xdr(r)?;
        let v = match dv {
            LedgerEntryChangeType::LedgerEntryCreated => {
                Self::LedgerEntryCreated(<LedgerEntry as ReadXDR>::read_xdr(r)?)
            }
            LedgerEntryChangeType::LedgerEntryUpdated => {
                Self::LedgerEntryUpdated(<LedgerEntry as ReadXDR>::read_xdr(r)?)
            }
            LedgerEntryChangeType::LedgerEntryRemoved => {
                Self::LedgerEntryRemoved(<LedgerKey as ReadXDR>::read_xdr(r)?)
            }
            LedgerEntryChangeType::LedgerEntryState => {
                Self::LedgerEntryState(<LedgerEntry as ReadXDR>::read_xdr(r)?)
            }
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for LedgerEntryChange {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::LedgerEntryCreated(v) => v.write_xdr(w)?,
            Self::LedgerEntryUpdated(v) => v.write_xdr(w)?,
            Self::LedgerEntryRemoved(v) => v.write_xdr(w)?,
            Self::LedgerEntryState(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// LedgerEntryChanges is an XDR Typedef defines as:
//
//   typedef LedgerEntryChange LedgerEntryChanges<>;
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LedgerEntryChanges(pub Vec<LedgerEntryChange>);

impl From<LedgerEntryChanges> for Vec<LedgerEntryChange> {
    fn from(x: LedgerEntryChanges) -> Self {
        x.0
    }
}

impl From<Vec<LedgerEntryChange>> for LedgerEntryChanges {
    fn from(x: Vec<LedgerEntryChange>) -> Self {
        LedgerEntryChanges(x)
    }
}

impl ReadXDR for LedgerEntryChanges {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let i = <Vec<LedgerEntryChange> as ReadXDR>::read_xdr(r)?;
        let v = LedgerEntryChanges(i);
        Ok(v)
    }
}

impl WriteXDR for LedgerEntryChanges {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.0.write_xdr(w)
    }
}

// OperationMeta is an XDR Struct defines as:
//
//   struct OperationMeta
//    {
//        LedgerEntryChanges changes;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OperationMeta {
    pub changes: LedgerEntryChanges,
}

impl ReadXDR for OperationMeta {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            changes: <LedgerEntryChanges as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for OperationMeta {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.changes.write_xdr(w)?;
        Ok(())
    }
}

// TransactionMetaV1 is an XDR Struct defines as:
//
//   struct TransactionMetaV1
//    {
//        LedgerEntryChanges txChanges; // tx level changes if any
//        OperationMeta operations<>;   // meta for each operation
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransactionMetaV1 {
    pub tx_changes: LedgerEntryChanges,
    pub operations: Vec<OperationMeta>,
}

impl ReadXDR for TransactionMetaV1 {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            tx_changes: <LedgerEntryChanges as ReadXDR>::read_xdr(r)?,
            operations: <Vec<OperationMeta> as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for TransactionMetaV1 {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.tx_changes.write_xdr(w)?;
        self.operations.write_xdr(w)?;
        Ok(())
    }
}

// TransactionMetaV2 is an XDR Struct defines as:
//
//   struct TransactionMetaV2
//    {
//        LedgerEntryChanges txChangesBefore; // tx level changes before operations
//                                            // are applied if any
//        OperationMeta operations<>;         // meta for each operation
//        LedgerEntryChanges txChangesAfter;  // tx level changes after operations are
//                                            // applied if any
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransactionMetaV2 {
    pub tx_changes_before: LedgerEntryChanges,
    pub operations: Vec<OperationMeta>,
    pub tx_changes_after: LedgerEntryChanges,
}

impl ReadXDR for TransactionMetaV2 {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            tx_changes_before: <LedgerEntryChanges as ReadXDR>::read_xdr(r)?,
            operations: <Vec<OperationMeta> as ReadXDR>::read_xdr(r)?,
            tx_changes_after: <LedgerEntryChanges as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for TransactionMetaV2 {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.tx_changes_before.write_xdr(w)?;
        self.operations.write_xdr(w)?;
        self.tx_changes_after.write_xdr(w)?;
        Ok(())
    }
}

// TransactionMeta is an XDR Union defines as:
//
//   union TransactionMeta switch (int v)
//    {
//    case 0:
//        OperationMeta operations<>;
//    case 1:
//        TransactionMetaV1 v1;
//    case 2:
//        TransactionMetaV2 v2;
//    };
//
// union with discriminant i32
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TransactionMeta {
    V0(Vec<OperationMeta>),
    V1(TransactionMetaV1),
    V2(TransactionMetaV2),
}

impl TransactionMeta {
    fn discriminant(&self) -> i32 {
        match self {
            Self::V0(_) => 0,
            Self::V1(_) => 1,
            Self::V2(_) => 2,
        }
    }
}

impl ReadXDR for TransactionMeta {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: i32 = <i32 as ReadXDR>::read_xdr(r)?;
        let v = match dv.into() {
            0 => Self::V0(<Vec<OperationMeta> as ReadXDR>::read_xdr(r)?),
            1 => Self::V1(<TransactionMetaV1 as ReadXDR>::read_xdr(r)?),
            2 => Self::V2(<TransactionMetaV2 as ReadXDR>::read_xdr(r)?),
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for TransactionMeta {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::V0(v) => v.write_xdr(w)?,
            Self::V1(v) => v.write_xdr(w)?,
            Self::V2(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// TransactionResultMeta is an XDR Struct defines as:
//
//   struct TransactionResultMeta
//    {
//        TransactionResultPair result;
//        LedgerEntryChanges feeProcessing;
//        TransactionMeta txApplyProcessing;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransactionResultMeta {
    pub result: TransactionResultPair,
    pub fee_processing: LedgerEntryChanges,
    pub tx_apply_processing: TransactionMeta,
}

impl ReadXDR for TransactionResultMeta {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            result: <TransactionResultPair as ReadXDR>::read_xdr(r)?,
            fee_processing: <LedgerEntryChanges as ReadXDR>::read_xdr(r)?,
            tx_apply_processing: <TransactionMeta as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for TransactionResultMeta {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.result.write_xdr(w)?;
        self.fee_processing.write_xdr(w)?;
        self.tx_apply_processing.write_xdr(w)?;
        Ok(())
    }
}

// UpgradeEntryMeta is an XDR Struct defines as:
//
//   struct UpgradeEntryMeta
//    {
//        LedgerUpgrade upgrade;
//        LedgerEntryChanges changes;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UpgradeEntryMeta {
    pub upgrade: LedgerUpgrade,
    pub changes: LedgerEntryChanges,
}

impl ReadXDR for UpgradeEntryMeta {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            upgrade: <LedgerUpgrade as ReadXDR>::read_xdr(r)?,
            changes: <LedgerEntryChanges as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for UpgradeEntryMeta {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.upgrade.write_xdr(w)?;
        self.changes.write_xdr(w)?;
        Ok(())
    }
}

// LedgerCloseMetaV0 is an XDR Struct defines as:
//
//   struct LedgerCloseMetaV0
//    {
//        LedgerHeaderHistoryEntry ledgerHeader;
//        // NB: txSet is sorted in "Hash order"
//        TransactionSet txSet;
//
//        // NB: transactions are sorted in apply order here
//        // fees for all transactions are processed first
//        // followed by applying transactions
//        TransactionResultMeta txProcessing<>;
//
//        // upgrades are applied last
//        UpgradeEntryMeta upgradesProcessing<>;
//
//        // other misc information attached to the ledger close
//        SCPHistoryEntry scpInfo<>;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LedgerCloseMetaV0 {
    pub ledger_header: LedgerHeaderHistoryEntry,
    pub tx_set: TransactionSet,
    pub tx_processing: Vec<TransactionResultMeta>,
    pub upgrades_processing: Vec<UpgradeEntryMeta>,
    pub scp_info: Vec<ScpHistoryEntry>,
}

impl ReadXDR for LedgerCloseMetaV0 {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            ledger_header: <LedgerHeaderHistoryEntry as ReadXDR>::read_xdr(r)?,
            tx_set: <TransactionSet as ReadXDR>::read_xdr(r)?,
            tx_processing: <Vec<TransactionResultMeta> as ReadXDR>::read_xdr(r)?,
            upgrades_processing: <Vec<UpgradeEntryMeta> as ReadXDR>::read_xdr(r)?,
            scp_info: <Vec<ScpHistoryEntry> as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for LedgerCloseMetaV0 {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.ledger_header.write_xdr(w)?;
        self.tx_set.write_xdr(w)?;
        self.tx_processing.write_xdr(w)?;
        self.upgrades_processing.write_xdr(w)?;
        self.scp_info.write_xdr(w)?;
        Ok(())
    }
}

// LedgerCloseMeta is an XDR Union defines as:
//
//   union LedgerCloseMeta switch (int v)
//    {
//    case 0:
//        LedgerCloseMetaV0 v0;
//    };
//
// union with discriminant i32
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LedgerCloseMeta {
    V0(LedgerCloseMetaV0),
}

impl LedgerCloseMeta {
    fn discriminant(&self) -> i32 {
        match self {
            Self::V0(_) => 0,
        }
    }
}

impl ReadXDR for LedgerCloseMeta {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: i32 = <i32 as ReadXDR>::read_xdr(r)?;
        let v = match dv.into() {
            0 => Self::V0(<LedgerCloseMetaV0 as ReadXDR>::read_xdr(r)?),
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for LedgerCloseMeta {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::V0(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// ErrorCode is an XDR Enum defines as:
//
//   enum ErrorCode
//    {
//        ERR_MISC = 0, // Unspecific error
//        ERR_DATA = 1, // Malformed data
//        ERR_CONF = 2, // Misconfiguration error
//        ERR_AUTH = 3, // Authentication failure
//        ERR_LOAD = 4  // System overloaded
//    };
//
// enum
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum ErrorCode {
    ErrMisc = 0,
    ErrData = 1,
    ErrConf = 2,
    ErrAuth = 3,
    ErrLoad = 4,
}

impl TryFrom<i32> for ErrorCode {
    type Error = Error;

    fn try_from(i: i32) -> std::result::Result<Self, Self::Error> {
        let e = match i {
            0 => ErrorCode::ErrMisc,
            1 => ErrorCode::ErrData,
            2 => ErrorCode::ErrConf,
            3 => ErrorCode::ErrAuth,
            4 => ErrorCode::ErrLoad,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<ErrorCode> for i32 {
    fn from(e: ErrorCode) -> Self {
        e as Self
    }
}

impl ReadXDR for ErrorCode {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXDR for ErrorCode {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// SError is an XDR Struct defines as:
//
//   struct Error
//    {
//        ErrorCode code;
//        string msg<100>;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SError {
    pub code: ErrorCode,
    pub msg: Vec<u8>,
}

impl ReadXDR for SError {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            code: <ErrorCode as ReadXDR>::read_xdr(r)?,
            msg: <Vec<u8> as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for SError {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.code.write_xdr(w)?;
        self.msg.write_xdr(w)?;
        Ok(())
    }
}

// SendMore is an XDR Struct defines as:
//
//   struct SendMore
//    {
//        uint32 numMessages;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SendMore {
    pub num_messages: Uint32,
}

impl ReadXDR for SendMore {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            num_messages: <Uint32 as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for SendMore {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.num_messages.write_xdr(w)?;
        Ok(())
    }
}

// AuthCert is an XDR Struct defines as:
//
//   struct AuthCert
//    {
//        Curve25519Public pubkey;
//        uint64 expiration;
//        Signature sig;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AuthCert {
    pub pubkey: Curve25519Public,
    pub expiration: Uint64,
    pub sig: Signature,
}

impl ReadXDR for AuthCert {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            pubkey: <Curve25519Public as ReadXDR>::read_xdr(r)?,
            expiration: <Uint64 as ReadXDR>::read_xdr(r)?,
            sig: <Signature as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for AuthCert {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.pubkey.write_xdr(w)?;
        self.expiration.write_xdr(w)?;
        self.sig.write_xdr(w)?;
        Ok(())
    }
}

// Hello is an XDR Struct defines as:
//
//   struct Hello
//    {
//        uint32 ledgerVersion;
//        uint32 overlayVersion;
//        uint32 overlayMinVersion;
//        Hash networkID;
//        string versionStr<100>;
//        int listeningPort;
//        NodeID peerID;
//        AuthCert cert;
//        uint256 nonce;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Hello {
    pub ledger_version: Uint32,
    pub overlay_version: Uint32,
    pub overlay_min_version: Uint32,
    pub network_id: Hash,
    pub version_str: Vec<u8>,
    pub listening_port: i32,
    pub peer_id: NodeId,
    pub cert: AuthCert,
    pub nonce: Uint256,
}

impl ReadXDR for Hello {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            ledger_version: <Uint32 as ReadXDR>::read_xdr(r)?,
            overlay_version: <Uint32 as ReadXDR>::read_xdr(r)?,
            overlay_min_version: <Uint32 as ReadXDR>::read_xdr(r)?,
            network_id: <Hash as ReadXDR>::read_xdr(r)?,
            version_str: <Vec<u8> as ReadXDR>::read_xdr(r)?,
            listening_port: <i32 as ReadXDR>::read_xdr(r)?,
            peer_id: <NodeId as ReadXDR>::read_xdr(r)?,
            cert: <AuthCert as ReadXDR>::read_xdr(r)?,
            nonce: <Uint256 as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for Hello {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.ledger_version.write_xdr(w)?;
        self.overlay_version.write_xdr(w)?;
        self.overlay_min_version.write_xdr(w)?;
        self.network_id.write_xdr(w)?;
        self.version_str.write_xdr(w)?;
        self.listening_port.write_xdr(w)?;
        self.peer_id.write_xdr(w)?;
        self.cert.write_xdr(w)?;
        self.nonce.write_xdr(w)?;
        Ok(())
    }
}

// Auth is an XDR Struct defines as:
//
//   struct Auth
//    {
//        // Empty message, just to confirm
//        // establishment of MAC keys.
//        int unused;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Auth {
    pub unused: i32,
}

impl ReadXDR for Auth {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            unused: <i32 as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for Auth {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.unused.write_xdr(w)?;
        Ok(())
    }
}

// IpAddrType is an XDR Enum defines as:
//
//   enum IPAddrType
//    {
//        IPv4 = 0,
//        IPv6 = 1
//    };
//
// enum
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum IpAddrType {
    IPv4 = 0,
    IPv6 = 1,
}

impl TryFrom<i32> for IpAddrType {
    type Error = Error;

    fn try_from(i: i32) -> std::result::Result<Self, Self::Error> {
        let e = match i {
            0 => IpAddrType::IPv4,
            1 => IpAddrType::IPv6,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<IpAddrType> for i32 {
    fn from(e: IpAddrType) -> Self {
        e as Self
    }
}

impl ReadXDR for IpAddrType {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXDR for IpAddrType {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// PeerAddressIp is an XDR NestedUnion defines as:
//
//   union switch (IPAddrType type)
//        {
//        case IPv4:
//            opaque ipv4[4];
//        case IPv6:
//            opaque ipv6[16];
//        }
//
// union with discriminant IpAddrType
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PeerAddressIp {
    IPv4([u8; 4]),
    IPv6([u8; 16]),
}

impl PeerAddressIp {
    fn discriminant(&self) -> IpAddrType {
        match self {
            Self::IPv4(_) => IpAddrType::IPv4,
            Self::IPv6(_) => IpAddrType::IPv6,
        }
    }
}

impl ReadXDR for PeerAddressIp {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: IpAddrType = <IpAddrType as ReadXDR>::read_xdr(r)?;
        let v = match dv {
            IpAddrType::IPv4 => Self::IPv4(<[u8; 4] as ReadXDR>::read_xdr(r)?),
            IpAddrType::IPv6 => Self::IPv6(<[u8; 16] as ReadXDR>::read_xdr(r)?),
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for PeerAddressIp {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::IPv4(v) => v.write_xdr(w)?,
            Self::IPv6(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// PeerAddress is an XDR Struct defines as:
//
//   struct PeerAddress
//    {
//        union switch (IPAddrType type)
//        {
//        case IPv4:
//            opaque ipv4[4];
//        case IPv6:
//            opaque ipv6[16];
//        }
//        ip;
//        uint32 port;
//        uint32 numFailures;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PeerAddress {
    pub ip: PeerAddressIp,
    pub port: Uint32,
    pub num_failures: Uint32,
}

impl ReadXDR for PeerAddress {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            ip: <PeerAddressIp as ReadXDR>::read_xdr(r)?,
            port: <Uint32 as ReadXDR>::read_xdr(r)?,
            num_failures: <Uint32 as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for PeerAddress {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.ip.write_xdr(w)?;
        self.port.write_xdr(w)?;
        self.num_failures.write_xdr(w)?;
        Ok(())
    }
}

// MessageType is an XDR Enum defines as:
//
//   enum MessageType
//    {
//        ERROR_MSG = 0,
//        AUTH = 2,
//        DONT_HAVE = 3,
//
//        GET_PEERS = 4, // gets a list of peers this guy knows about
//        PEERS = 5,
//
//        GET_TX_SET = 6, // gets a particular txset by hash
//        TX_SET = 7,
//
//        TRANSACTION = 8, // pass on a tx you have heard about
//
//        // SCP
//        GET_SCP_QUORUMSET = 9,
//        SCP_QUORUMSET = 10,
//        SCP_MESSAGE = 11,
//        GET_SCP_STATE = 12,
//
//        // new messages
//        HELLO = 13,
//
//        SURVEY_REQUEST = 14,
//        SURVEY_RESPONSE = 15,
//
//        SEND_MORE = 16
//    };
//
// enum
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum MessageType {
    ErrorMsg = 0,
    Auth = 2,
    DontHave = 3,
    GetPeers = 4,
    Peers = 5,
    GetTxSet = 6,
    TxSet = 7,
    Transaction = 8,
    GetScpQuorumset = 9,
    ScpQuorumset = 10,
    ScpMessage = 11,
    GetScpState = 12,
    Hello = 13,
    SurveyRequest = 14,
    SurveyResponse = 15,
    SendMore = 16,
}

impl TryFrom<i32> for MessageType {
    type Error = Error;

    fn try_from(i: i32) -> std::result::Result<Self, Self::Error> {
        let e = match i {
            0 => MessageType::ErrorMsg,
            2 => MessageType::Auth,
            3 => MessageType::DontHave,
            4 => MessageType::GetPeers,
            5 => MessageType::Peers,
            6 => MessageType::GetTxSet,
            7 => MessageType::TxSet,
            8 => MessageType::Transaction,
            9 => MessageType::GetScpQuorumset,
            10 => MessageType::ScpQuorumset,
            11 => MessageType::ScpMessage,
            12 => MessageType::GetScpState,
            13 => MessageType::Hello,
            14 => MessageType::SurveyRequest,
            15 => MessageType::SurveyResponse,
            16 => MessageType::SendMore,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<MessageType> for i32 {
    fn from(e: MessageType) -> Self {
        e as Self
    }
}

impl ReadXDR for MessageType {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXDR for MessageType {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// DontHave is an XDR Struct defines as:
//
//   struct DontHave
//    {
//        MessageType type;
//        uint256 reqHash;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DontHave {
    pub type_: MessageType,
    pub req_hash: Uint256,
}

impl ReadXDR for DontHave {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            type_: <MessageType as ReadXDR>::read_xdr(r)?,
            req_hash: <Uint256 as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for DontHave {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.type_.write_xdr(w)?;
        self.req_hash.write_xdr(w)?;
        Ok(())
    }
}

// SurveyMessageCommandType is an XDR Enum defines as:
//
//   enum SurveyMessageCommandType
//    {
//        SURVEY_TOPOLOGY = 0
//    };
//
// enum
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum SurveyMessageCommandType {
    SurveyTopology = 0,
}

impl TryFrom<i32> for SurveyMessageCommandType {
    type Error = Error;

    fn try_from(i: i32) -> std::result::Result<Self, Self::Error> {
        let e = match i {
            0 => SurveyMessageCommandType::SurveyTopology,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<SurveyMessageCommandType> for i32 {
    fn from(e: SurveyMessageCommandType) -> Self {
        e as Self
    }
}

impl ReadXDR for SurveyMessageCommandType {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXDR for SurveyMessageCommandType {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// SurveyRequestMessage is an XDR Struct defines as:
//
//   struct SurveyRequestMessage
//    {
//        NodeID surveyorPeerID;
//        NodeID surveyedPeerID;
//        uint32 ledgerNum;
//        Curve25519Public encryptionKey;
//        SurveyMessageCommandType commandType;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SurveyRequestMessage {
    pub surveyor_peer_id: NodeId,
    pub surveyed_peer_id: NodeId,
    pub ledger_num: Uint32,
    pub encryption_key: Curve25519Public,
    pub command_type: SurveyMessageCommandType,
}

impl ReadXDR for SurveyRequestMessage {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            surveyor_peer_id: <NodeId as ReadXDR>::read_xdr(r)?,
            surveyed_peer_id: <NodeId as ReadXDR>::read_xdr(r)?,
            ledger_num: <Uint32 as ReadXDR>::read_xdr(r)?,
            encryption_key: <Curve25519Public as ReadXDR>::read_xdr(r)?,
            command_type: <SurveyMessageCommandType as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for SurveyRequestMessage {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.surveyor_peer_id.write_xdr(w)?;
        self.surveyed_peer_id.write_xdr(w)?;
        self.ledger_num.write_xdr(w)?;
        self.encryption_key.write_xdr(w)?;
        self.command_type.write_xdr(w)?;
        Ok(())
    }
}

// SignedSurveyRequestMessage is an XDR Struct defines as:
//
//   struct SignedSurveyRequestMessage
//    {
//        Signature requestSignature;
//        SurveyRequestMessage request;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SignedSurveyRequestMessage {
    pub request_signature: Signature,
    pub request: SurveyRequestMessage,
}

impl ReadXDR for SignedSurveyRequestMessage {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            request_signature: <Signature as ReadXDR>::read_xdr(r)?,
            request: <SurveyRequestMessage as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for SignedSurveyRequestMessage {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.request_signature.write_xdr(w)?;
        self.request.write_xdr(w)?;
        Ok(())
    }
}

// EncryptedBody is an XDR Typedef defines as:
//
//   typedef opaque EncryptedBody<64000>;
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EncryptedBody(pub Vec<u8>);

impl From<EncryptedBody> for Vec<u8> {
    fn from(x: EncryptedBody) -> Self {
        x.0
    }
}

impl From<Vec<u8>> for EncryptedBody {
    fn from(x: Vec<u8>) -> Self {
        EncryptedBody(x)
    }
}

impl ReadXDR for EncryptedBody {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let i = <Vec<u8> as ReadXDR>::read_xdr(r)?;
        let v = EncryptedBody(i);
        Ok(v)
    }
}

impl WriteXDR for EncryptedBody {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.0.write_xdr(w)
    }
}

// SurveyResponseMessage is an XDR Struct defines as:
//
//   struct SurveyResponseMessage
//    {
//        NodeID surveyorPeerID;
//        NodeID surveyedPeerID;
//        uint32 ledgerNum;
//        SurveyMessageCommandType commandType;
//        EncryptedBody encryptedBody;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SurveyResponseMessage {
    pub surveyor_peer_id: NodeId,
    pub surveyed_peer_id: NodeId,
    pub ledger_num: Uint32,
    pub command_type: SurveyMessageCommandType,
    pub encrypted_body: EncryptedBody,
}

impl ReadXDR for SurveyResponseMessage {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            surveyor_peer_id: <NodeId as ReadXDR>::read_xdr(r)?,
            surveyed_peer_id: <NodeId as ReadXDR>::read_xdr(r)?,
            ledger_num: <Uint32 as ReadXDR>::read_xdr(r)?,
            command_type: <SurveyMessageCommandType as ReadXDR>::read_xdr(r)?,
            encrypted_body: <EncryptedBody as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for SurveyResponseMessage {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.surveyor_peer_id.write_xdr(w)?;
        self.surveyed_peer_id.write_xdr(w)?;
        self.ledger_num.write_xdr(w)?;
        self.command_type.write_xdr(w)?;
        self.encrypted_body.write_xdr(w)?;
        Ok(())
    }
}

// SignedSurveyResponseMessage is an XDR Struct defines as:
//
//   struct SignedSurveyResponseMessage
//    {
//        Signature responseSignature;
//        SurveyResponseMessage response;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SignedSurveyResponseMessage {
    pub response_signature: Signature,
    pub response: SurveyResponseMessage,
}

impl ReadXDR for SignedSurveyResponseMessage {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            response_signature: <Signature as ReadXDR>::read_xdr(r)?,
            response: <SurveyResponseMessage as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for SignedSurveyResponseMessage {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.response_signature.write_xdr(w)?;
        self.response.write_xdr(w)?;
        Ok(())
    }
}

// PeerStats is an XDR Struct defines as:
//
//   struct PeerStats
//    {
//        NodeID id;
//        string versionStr<100>;
//        uint64 messagesRead;
//        uint64 messagesWritten;
//        uint64 bytesRead;
//        uint64 bytesWritten;
//        uint64 secondsConnected;
//
//        uint64 uniqueFloodBytesRecv;
//        uint64 duplicateFloodBytesRecv;
//        uint64 uniqueFetchBytesRecv;
//        uint64 duplicateFetchBytesRecv;
//
//        uint64 uniqueFloodMessageRecv;
//        uint64 duplicateFloodMessageRecv;
//        uint64 uniqueFetchMessageRecv;
//        uint64 duplicateFetchMessageRecv;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PeerStats {
    pub id: NodeId,
    pub version_str: Vec<u8>,
    pub messages_read: Uint64,
    pub messages_written: Uint64,
    pub bytes_read: Uint64,
    pub bytes_written: Uint64,
    pub seconds_connected: Uint64,
    pub unique_flood_bytes_recv: Uint64,
    pub duplicate_flood_bytes_recv: Uint64,
    pub unique_fetch_bytes_recv: Uint64,
    pub duplicate_fetch_bytes_recv: Uint64,
    pub unique_flood_message_recv: Uint64,
    pub duplicate_flood_message_recv: Uint64,
    pub unique_fetch_message_recv: Uint64,
    pub duplicate_fetch_message_recv: Uint64,
}

impl ReadXDR for PeerStats {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            id: <NodeId as ReadXDR>::read_xdr(r)?,
            version_str: <Vec<u8> as ReadXDR>::read_xdr(r)?,
            messages_read: <Uint64 as ReadXDR>::read_xdr(r)?,
            messages_written: <Uint64 as ReadXDR>::read_xdr(r)?,
            bytes_read: <Uint64 as ReadXDR>::read_xdr(r)?,
            bytes_written: <Uint64 as ReadXDR>::read_xdr(r)?,
            seconds_connected: <Uint64 as ReadXDR>::read_xdr(r)?,
            unique_flood_bytes_recv: <Uint64 as ReadXDR>::read_xdr(r)?,
            duplicate_flood_bytes_recv: <Uint64 as ReadXDR>::read_xdr(r)?,
            unique_fetch_bytes_recv: <Uint64 as ReadXDR>::read_xdr(r)?,
            duplicate_fetch_bytes_recv: <Uint64 as ReadXDR>::read_xdr(r)?,
            unique_flood_message_recv: <Uint64 as ReadXDR>::read_xdr(r)?,
            duplicate_flood_message_recv: <Uint64 as ReadXDR>::read_xdr(r)?,
            unique_fetch_message_recv: <Uint64 as ReadXDR>::read_xdr(r)?,
            duplicate_fetch_message_recv: <Uint64 as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for PeerStats {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.id.write_xdr(w)?;
        self.version_str.write_xdr(w)?;
        self.messages_read.write_xdr(w)?;
        self.messages_written.write_xdr(w)?;
        self.bytes_read.write_xdr(w)?;
        self.bytes_written.write_xdr(w)?;
        self.seconds_connected.write_xdr(w)?;
        self.unique_flood_bytes_recv.write_xdr(w)?;
        self.duplicate_flood_bytes_recv.write_xdr(w)?;
        self.unique_fetch_bytes_recv.write_xdr(w)?;
        self.duplicate_fetch_bytes_recv.write_xdr(w)?;
        self.unique_flood_message_recv.write_xdr(w)?;
        self.duplicate_flood_message_recv.write_xdr(w)?;
        self.unique_fetch_message_recv.write_xdr(w)?;
        self.duplicate_fetch_message_recv.write_xdr(w)?;
        Ok(())
    }
}

// PeerStatList is an XDR Typedef defines as:
//
//   typedef PeerStats PeerStatList<25>;
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PeerStatList(pub Vec<PeerStats>);

impl From<PeerStatList> for Vec<PeerStats> {
    fn from(x: PeerStatList) -> Self {
        x.0
    }
}

impl From<Vec<PeerStats>> for PeerStatList {
    fn from(x: Vec<PeerStats>) -> Self {
        PeerStatList(x)
    }
}

impl ReadXDR for PeerStatList {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let i = <Vec<PeerStats> as ReadXDR>::read_xdr(r)?;
        let v = PeerStatList(i);
        Ok(v)
    }
}

impl WriteXDR for PeerStatList {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.0.write_xdr(w)
    }
}

// TopologyResponseBody is an XDR Struct defines as:
//
//   struct TopologyResponseBody
//    {
//        PeerStatList inboundPeers;
//        PeerStatList outboundPeers;
//
//        uint32 totalInboundPeerCount;
//        uint32 totalOutboundPeerCount;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TopologyResponseBody {
    pub inbound_peers: PeerStatList,
    pub outbound_peers: PeerStatList,
    pub total_inbound_peer_count: Uint32,
    pub total_outbound_peer_count: Uint32,
}

impl ReadXDR for TopologyResponseBody {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            inbound_peers: <PeerStatList as ReadXDR>::read_xdr(r)?,
            outbound_peers: <PeerStatList as ReadXDR>::read_xdr(r)?,
            total_inbound_peer_count: <Uint32 as ReadXDR>::read_xdr(r)?,
            total_outbound_peer_count: <Uint32 as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for TopologyResponseBody {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.inbound_peers.write_xdr(w)?;
        self.outbound_peers.write_xdr(w)?;
        self.total_inbound_peer_count.write_xdr(w)?;
        self.total_outbound_peer_count.write_xdr(w)?;
        Ok(())
    }
}

// SurveyResponseBody is an XDR Union defines as:
//
//   union SurveyResponseBody switch (SurveyMessageCommandType type)
//    {
//    case SURVEY_TOPOLOGY:
//        TopologyResponseBody topologyResponseBody;
//    };
//
// union with discriminant SurveyMessageCommandType
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SurveyResponseBody {
    SurveyTopology(TopologyResponseBody),
}

impl SurveyResponseBody {
    fn discriminant(&self) -> SurveyMessageCommandType {
        match self {
            Self::SurveyTopology(_) => SurveyMessageCommandType::SurveyTopology,
        }
    }
}

impl ReadXDR for SurveyResponseBody {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: SurveyMessageCommandType = <SurveyMessageCommandType as ReadXDR>::read_xdr(r)?;
        let v = match dv {
            SurveyMessageCommandType::SurveyTopology => {
                Self::SurveyTopology(<TopologyResponseBody as ReadXDR>::read_xdr(r)?)
            }
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for SurveyResponseBody {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::SurveyTopology(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// StellarMessage is an XDR Union defines as:
//
//   union StellarMessage switch (MessageType type)
//    {
//    case ERROR_MSG:
//        Error error;
//    case HELLO:
//        Hello hello;
//    case AUTH:
//        Auth auth;
//    case DONT_HAVE:
//        DontHave dontHave;
//    case GET_PEERS:
//        void;
//    case PEERS:
//        PeerAddress peers<100>;
//
//    case GET_TX_SET:
//        uint256 txSetHash;
//    case TX_SET:
//        TransactionSet txSet;
//
//    case TRANSACTION:
//        TransactionEnvelope transaction;
//
//    case SURVEY_REQUEST:
//        SignedSurveyRequestMessage signedSurveyRequestMessage;
//
//    case SURVEY_RESPONSE:
//        SignedSurveyResponseMessage signedSurveyResponseMessage;
//
//    // SCP
//    case GET_SCP_QUORUMSET:
//        uint256 qSetHash;
//    case SCP_QUORUMSET:
//        SCPQuorumSet qSet;
//    case SCP_MESSAGE:
//        SCPEnvelope envelope;
//    case GET_SCP_STATE:
//        uint32 getSCPLedgerSeq; // ledger seq requested ; if 0, requests the latest
//    case SEND_MORE:
//        SendMore sendMoreMessage;
//    };
//
// union with discriminant MessageType
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StellarMessage {
    ErrorMsg(SError),
    Hello(Hello),
    Auth(Auth),
    DontHave(DontHave),
    GetPeers,
    Peers(Vec<PeerAddress>),
    GetTxSet(Uint256),
    TxSet(TransactionSet),
    Transaction(TransactionEnvelope),
    SurveyRequest(SignedSurveyRequestMessage),
    SurveyResponse(SignedSurveyResponseMessage),
    GetScpQuorumset(Uint256),
    ScpQuorumset(ScpQuorumSet),
    ScpMessage(ScpEnvelope),
    GetScpState(Uint32),
    SendMore(SendMore),
}

impl StellarMessage {
    fn discriminant(&self) -> MessageType {
        match self {
            Self::ErrorMsg(_) => MessageType::ErrorMsg,
            Self::Hello(_) => MessageType::Hello,
            Self::Auth(_) => MessageType::Auth,
            Self::DontHave(_) => MessageType::DontHave,
            Self::GetPeers => MessageType::GetPeers,
            Self::Peers(_) => MessageType::Peers,
            Self::GetTxSet(_) => MessageType::GetTxSet,
            Self::TxSet(_) => MessageType::TxSet,
            Self::Transaction(_) => MessageType::Transaction,
            Self::SurveyRequest(_) => MessageType::SurveyRequest,
            Self::SurveyResponse(_) => MessageType::SurveyResponse,
            Self::GetScpQuorumset(_) => MessageType::GetScpQuorumset,
            Self::ScpQuorumset(_) => MessageType::ScpQuorumset,
            Self::ScpMessage(_) => MessageType::ScpMessage,
            Self::GetScpState(_) => MessageType::GetScpState,
            Self::SendMore(_) => MessageType::SendMore,
        }
    }
}

impl ReadXDR for StellarMessage {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: MessageType = <MessageType as ReadXDR>::read_xdr(r)?;
        let v = match dv {
            MessageType::ErrorMsg => Self::ErrorMsg(<SError as ReadXDR>::read_xdr(r)?),
            MessageType::Hello => Self::Hello(<Hello as ReadXDR>::read_xdr(r)?),
            MessageType::Auth => Self::Auth(<Auth as ReadXDR>::read_xdr(r)?),
            MessageType::DontHave => Self::DontHave(<DontHave as ReadXDR>::read_xdr(r)?),
            MessageType::GetPeers => Self::GetPeers,
            MessageType::Peers => Self::Peers(<Vec<PeerAddress> as ReadXDR>::read_xdr(r)?),
            MessageType::GetTxSet => Self::GetTxSet(<Uint256 as ReadXDR>::read_xdr(r)?),
            MessageType::TxSet => Self::TxSet(<TransactionSet as ReadXDR>::read_xdr(r)?),
            MessageType::Transaction => {
                Self::Transaction(<TransactionEnvelope as ReadXDR>::read_xdr(r)?)
            }
            MessageType::SurveyRequest => {
                Self::SurveyRequest(<SignedSurveyRequestMessage as ReadXDR>::read_xdr(r)?)
            }
            MessageType::SurveyResponse => {
                Self::SurveyResponse(<SignedSurveyResponseMessage as ReadXDR>::read_xdr(r)?)
            }
            MessageType::GetScpQuorumset => {
                Self::GetScpQuorumset(<Uint256 as ReadXDR>::read_xdr(r)?)
            }
            MessageType::ScpQuorumset => {
                Self::ScpQuorumset(<ScpQuorumSet as ReadXDR>::read_xdr(r)?)
            }
            MessageType::ScpMessage => Self::ScpMessage(<ScpEnvelope as ReadXDR>::read_xdr(r)?),
            MessageType::GetScpState => Self::GetScpState(<Uint32 as ReadXDR>::read_xdr(r)?),
            MessageType::SendMore => Self::SendMore(<SendMore as ReadXDR>::read_xdr(r)?),
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for StellarMessage {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::ErrorMsg(v) => v.write_xdr(w)?,
            Self::Hello(v) => v.write_xdr(w)?,
            Self::Auth(v) => v.write_xdr(w)?,
            Self::DontHave(v) => v.write_xdr(w)?,
            Self::GetPeers => ().write_xdr(w)?,
            Self::Peers(v) => v.write_xdr(w)?,
            Self::GetTxSet(v) => v.write_xdr(w)?,
            Self::TxSet(v) => v.write_xdr(w)?,
            Self::Transaction(v) => v.write_xdr(w)?,
            Self::SurveyRequest(v) => v.write_xdr(w)?,
            Self::SurveyResponse(v) => v.write_xdr(w)?,
            Self::GetScpQuorumset(v) => v.write_xdr(w)?,
            Self::ScpQuorumset(v) => v.write_xdr(w)?,
            Self::ScpMessage(v) => v.write_xdr(w)?,
            Self::GetScpState(v) => v.write_xdr(w)?,
            Self::SendMore(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// AuthenticatedMessageV0 is an XDR NestedStruct defines as:
//
//   struct
//        {
//            uint64 sequence;
//            StellarMessage message;
//            HmacSha256Mac mac;
//        }
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AuthenticatedMessageV0 {
    pub sequence: Uint64,
    pub message: StellarMessage,
    pub mac: HmacSha256Mac,
}

impl ReadXDR for AuthenticatedMessageV0 {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            sequence: <Uint64 as ReadXDR>::read_xdr(r)?,
            message: <StellarMessage as ReadXDR>::read_xdr(r)?,
            mac: <HmacSha256Mac as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for AuthenticatedMessageV0 {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.sequence.write_xdr(w)?;
        self.message.write_xdr(w)?;
        self.mac.write_xdr(w)?;
        Ok(())
    }
}

// AuthenticatedMessage is an XDR Union defines as:
//
//   union AuthenticatedMessage switch (uint32 v)
//    {
//    case 0:
//        struct
//        {
//            uint64 sequence;
//            StellarMessage message;
//            HmacSha256Mac mac;
//        } v0;
//    };
//
// union with discriminant Uint32
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AuthenticatedMessage {
    V0(AuthenticatedMessageV0),
}

impl AuthenticatedMessage {
    fn discriminant(&self) -> Uint32 {
        match self {
            Self::V0(_) => Uint32(0),
        }
    }
}

impl ReadXDR for AuthenticatedMessage {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: Uint32 = <Uint32 as ReadXDR>::read_xdr(r)?;
        let v = match dv.into() {
            0 => Self::V0(<AuthenticatedMessageV0 as ReadXDR>::read_xdr(r)?),
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for AuthenticatedMessage {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::V0(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// LiquidityPoolParameters is an XDR Union defines as:
//
//   union LiquidityPoolParameters switch (LiquidityPoolType type)
//    {
//    case LIQUIDITY_POOL_CONSTANT_PRODUCT:
//        LiquidityPoolConstantProductParameters constantProduct;
//    };
//
// union with discriminant LiquidityPoolType
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LiquidityPoolParameters {
    LiquidityPoolConstantProduct(LiquidityPoolConstantProductParameters),
}

impl LiquidityPoolParameters {
    fn discriminant(&self) -> LiquidityPoolType {
        match self {
            Self::LiquidityPoolConstantProduct(_) => {
                LiquidityPoolType::LiquidityPoolConstantProduct
            }
        }
    }
}

impl ReadXDR for LiquidityPoolParameters {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: LiquidityPoolType = <LiquidityPoolType as ReadXDR>::read_xdr(r)?;
        let v = match dv {
            LiquidityPoolType::LiquidityPoolConstantProduct => Self::LiquidityPoolConstantProduct(
                <LiquidityPoolConstantProductParameters as ReadXDR>::read_xdr(r)?,
            ),
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for LiquidityPoolParameters {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::LiquidityPoolConstantProduct(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// MuxedAccountMed25519 is an XDR NestedStruct defines as:
//
//   struct
//        {
//            uint64 id;
//            uint256 ed25519;
//        }
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MuxedAccountMed25519 {
    pub id: Uint64,
    pub ed25519: Uint256,
}

impl ReadXDR for MuxedAccountMed25519 {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            id: <Uint64 as ReadXDR>::read_xdr(r)?,
            ed25519: <Uint256 as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for MuxedAccountMed25519 {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.id.write_xdr(w)?;
        self.ed25519.write_xdr(w)?;
        Ok(())
    }
}

// MuxedAccount is an XDR Union defines as:
//
//   union MuxedAccount switch (CryptoKeyType type)
//    {
//    case KEY_TYPE_ED25519:
//        uint256 ed25519;
//    case KEY_TYPE_MUXED_ED25519:
//        struct
//        {
//            uint64 id;
//            uint256 ed25519;
//        } med25519;
//    };
//
// union with discriminant CryptoKeyType
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MuxedAccount {
    KeyTypeEd25519(Uint256),
    KeyTypeMuxedEd25519(MuxedAccountMed25519),
}

impl MuxedAccount {
    fn discriminant(&self) -> CryptoKeyType {
        match self {
            Self::KeyTypeEd25519(_) => CryptoKeyType::KeyTypeEd25519,
            Self::KeyTypeMuxedEd25519(_) => CryptoKeyType::KeyTypeMuxedEd25519,
        }
    }
}

impl ReadXDR for MuxedAccount {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: CryptoKeyType = <CryptoKeyType as ReadXDR>::read_xdr(r)?;
        let v = match dv {
            CryptoKeyType::KeyTypeEd25519 => {
                Self::KeyTypeEd25519(<Uint256 as ReadXDR>::read_xdr(r)?)
            }
            CryptoKeyType::KeyTypeMuxedEd25519 => {
                Self::KeyTypeMuxedEd25519(<MuxedAccountMed25519 as ReadXDR>::read_xdr(r)?)
            }
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for MuxedAccount {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::KeyTypeEd25519(v) => v.write_xdr(w)?,
            Self::KeyTypeMuxedEd25519(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// DecoratedSignature is an XDR Struct defines as:
//
//   struct DecoratedSignature
//    {
//        SignatureHint hint;  // last 4 bytes of the public key, used as a hint
//        Signature signature; // actual signature
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DecoratedSignature {
    pub hint: SignatureHint,
    pub signature: Signature,
}

impl ReadXDR for DecoratedSignature {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            hint: <SignatureHint as ReadXDR>::read_xdr(r)?,
            signature: <Signature as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for DecoratedSignature {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.hint.write_xdr(w)?;
        self.signature.write_xdr(w)?;
        Ok(())
    }
}

// OperationType is an XDR Enum defines as:
//
//   enum OperationType
//    {
//        CREATE_ACCOUNT = 0,
//        PAYMENT = 1,
//        PATH_PAYMENT_STRICT_RECEIVE = 2,
//        MANAGE_SELL_OFFER = 3,
//        CREATE_PASSIVE_SELL_OFFER = 4,
//        SET_OPTIONS = 5,
//        CHANGE_TRUST = 6,
//        ALLOW_TRUST = 7,
//        ACCOUNT_MERGE = 8,
//        INFLATION = 9,
//        MANAGE_DATA = 10,
//        BUMP_SEQUENCE = 11,
//        MANAGE_BUY_OFFER = 12,
//        PATH_PAYMENT_STRICT_SEND = 13,
//        CREATE_CLAIMABLE_BALANCE = 14,
//        CLAIM_CLAIMABLE_BALANCE = 15,
//        BEGIN_SPONSORING_FUTURE_RESERVES = 16,
//        END_SPONSORING_FUTURE_RESERVES = 17,
//        REVOKE_SPONSORSHIP = 18,
//        CLAWBACK = 19,
//        CLAWBACK_CLAIMABLE_BALANCE = 20,
//        SET_TRUST_LINE_FLAGS = 21,
//        LIQUIDITY_POOL_DEPOSIT = 22,
//        LIQUIDITY_POOL_WITHDRAW = 23
//    };
//
// enum
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum OperationType {
    CreateAccount = 0,
    Payment = 1,
    PathPaymentStrictReceive = 2,
    ManageSellOffer = 3,
    CreatePassiveSellOffer = 4,
    SetOptions = 5,
    ChangeTrust = 6,
    AllowTrust = 7,
    AccountMerge = 8,
    Inflation = 9,
    ManageData = 10,
    BumpSequence = 11,
    ManageBuyOffer = 12,
    PathPaymentStrictSend = 13,
    CreateClaimableBalance = 14,
    ClaimClaimableBalance = 15,
    BeginSponsoringFutureReserves = 16,
    EndSponsoringFutureReserves = 17,
    RevokeSponsorship = 18,
    Clawback = 19,
    ClawbackClaimableBalance = 20,
    SetTrustLineFlags = 21,
    LiquidityPoolDeposit = 22,
    LiquidityPoolWithdraw = 23,
}

impl TryFrom<i32> for OperationType {
    type Error = Error;

    fn try_from(i: i32) -> std::result::Result<Self, Self::Error> {
        let e = match i {
            0 => OperationType::CreateAccount,
            1 => OperationType::Payment,
            2 => OperationType::PathPaymentStrictReceive,
            3 => OperationType::ManageSellOffer,
            4 => OperationType::CreatePassiveSellOffer,
            5 => OperationType::SetOptions,
            6 => OperationType::ChangeTrust,
            7 => OperationType::AllowTrust,
            8 => OperationType::AccountMerge,
            9 => OperationType::Inflation,
            10 => OperationType::ManageData,
            11 => OperationType::BumpSequence,
            12 => OperationType::ManageBuyOffer,
            13 => OperationType::PathPaymentStrictSend,
            14 => OperationType::CreateClaimableBalance,
            15 => OperationType::ClaimClaimableBalance,
            16 => OperationType::BeginSponsoringFutureReserves,
            17 => OperationType::EndSponsoringFutureReserves,
            18 => OperationType::RevokeSponsorship,
            19 => OperationType::Clawback,
            20 => OperationType::ClawbackClaimableBalance,
            21 => OperationType::SetTrustLineFlags,
            22 => OperationType::LiquidityPoolDeposit,
            23 => OperationType::LiquidityPoolWithdraw,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<OperationType> for i32 {
    fn from(e: OperationType) -> Self {
        e as Self
    }
}

impl ReadXDR for OperationType {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXDR for OperationType {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// CreateAccountOp is an XDR Struct defines as:
//
//   struct CreateAccountOp
//    {
//        AccountID destination; // account to create
//        int64 startingBalance; // amount they end up with
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CreateAccountOp {
    pub destination: AccountId,
    pub starting_balance: Int64,
}

impl ReadXDR for CreateAccountOp {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            destination: <AccountId as ReadXDR>::read_xdr(r)?,
            starting_balance: <Int64 as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for CreateAccountOp {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.destination.write_xdr(w)?;
        self.starting_balance.write_xdr(w)?;
        Ok(())
    }
}

// PaymentOp is an XDR Struct defines as:
//
//   struct PaymentOp
//    {
//        MuxedAccount destination; // recipient of the payment
//        Asset asset;              // what they end up with
//        int64 amount;             // amount they end up with
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PaymentOp {
    pub destination: MuxedAccount,
    pub asset: Asset,
    pub amount: Int64,
}

impl ReadXDR for PaymentOp {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            destination: <MuxedAccount as ReadXDR>::read_xdr(r)?,
            asset: <Asset as ReadXDR>::read_xdr(r)?,
            amount: <Int64 as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for PaymentOp {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.destination.write_xdr(w)?;
        self.asset.write_xdr(w)?;
        self.amount.write_xdr(w)?;
        Ok(())
    }
}

// PathPaymentStrictReceiveOp is an XDR Struct defines as:
//
//   struct PathPaymentStrictReceiveOp
//    {
//        Asset sendAsset; // asset we pay with
//        int64 sendMax;   // the maximum amount of sendAsset to
//                         // send (excluding fees).
//                         // The operation will fail if can't be met
//
//        MuxedAccount destination; // recipient of the payment
//        Asset destAsset;          // what they end up with
//        int64 destAmount;         // amount they end up with
//
//        Asset path<5>; // additional hops it must go through to get there
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PathPaymentStrictReceiveOp {
    pub send_asset: Asset,
    pub send_max: Int64,
    pub destination: MuxedAccount,
    pub dest_asset: Asset,
    pub dest_amount: Int64,
    pub path: Vec<Asset>,
}

impl ReadXDR for PathPaymentStrictReceiveOp {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            send_asset: <Asset as ReadXDR>::read_xdr(r)?,
            send_max: <Int64 as ReadXDR>::read_xdr(r)?,
            destination: <MuxedAccount as ReadXDR>::read_xdr(r)?,
            dest_asset: <Asset as ReadXDR>::read_xdr(r)?,
            dest_amount: <Int64 as ReadXDR>::read_xdr(r)?,
            path: <Vec<Asset> as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for PathPaymentStrictReceiveOp {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.send_asset.write_xdr(w)?;
        self.send_max.write_xdr(w)?;
        self.destination.write_xdr(w)?;
        self.dest_asset.write_xdr(w)?;
        self.dest_amount.write_xdr(w)?;
        self.path.write_xdr(w)?;
        Ok(())
    }
}

// PathPaymentStrictSendOp is an XDR Struct defines as:
//
//   struct PathPaymentStrictSendOp
//    {
//        Asset sendAsset;  // asset we pay with
//        int64 sendAmount; // amount of sendAsset to send (excluding fees)
//
//        MuxedAccount destination; // recipient of the payment
//        Asset destAsset;          // what they end up with
//        int64 destMin;            // the minimum amount of dest asset to
//                                  // be received
//                                  // The operation will fail if it can't be met
//
//        Asset path<5>; // additional hops it must go through to get there
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PathPaymentStrictSendOp {
    pub send_asset: Asset,
    pub send_amount: Int64,
    pub destination: MuxedAccount,
    pub dest_asset: Asset,
    pub dest_min: Int64,
    pub path: Vec<Asset>,
}

impl ReadXDR for PathPaymentStrictSendOp {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            send_asset: <Asset as ReadXDR>::read_xdr(r)?,
            send_amount: <Int64 as ReadXDR>::read_xdr(r)?,
            destination: <MuxedAccount as ReadXDR>::read_xdr(r)?,
            dest_asset: <Asset as ReadXDR>::read_xdr(r)?,
            dest_min: <Int64 as ReadXDR>::read_xdr(r)?,
            path: <Vec<Asset> as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for PathPaymentStrictSendOp {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.send_asset.write_xdr(w)?;
        self.send_amount.write_xdr(w)?;
        self.destination.write_xdr(w)?;
        self.dest_asset.write_xdr(w)?;
        self.dest_min.write_xdr(w)?;
        self.path.write_xdr(w)?;
        Ok(())
    }
}

// ManageSellOfferOp is an XDR Struct defines as:
//
//   struct ManageSellOfferOp
//    {
//        Asset selling;
//        Asset buying;
//        int64 amount; // amount being sold. if set to 0, delete the offer
//        Price price;  // price of thing being sold in terms of what you are buying
//
//        // 0=create a new offer, otherwise edit an existing offer
//        int64 offerID;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ManageSellOfferOp {
    pub selling: Asset,
    pub buying: Asset,
    pub amount: Int64,
    pub price: Price,
    pub offer_id: Int64,
}

impl ReadXDR for ManageSellOfferOp {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            selling: <Asset as ReadXDR>::read_xdr(r)?,
            buying: <Asset as ReadXDR>::read_xdr(r)?,
            amount: <Int64 as ReadXDR>::read_xdr(r)?,
            price: <Price as ReadXDR>::read_xdr(r)?,
            offer_id: <Int64 as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for ManageSellOfferOp {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.selling.write_xdr(w)?;
        self.buying.write_xdr(w)?;
        self.amount.write_xdr(w)?;
        self.price.write_xdr(w)?;
        self.offer_id.write_xdr(w)?;
        Ok(())
    }
}

// ManageBuyOfferOp is an XDR Struct defines as:
//
//   struct ManageBuyOfferOp
//    {
//        Asset selling;
//        Asset buying;
//        int64 buyAmount; // amount being bought. if set to 0, delete the offer
//        Price price;     // price of thing being bought in terms of what you are
//                         // selling
//
//        // 0=create a new offer, otherwise edit an existing offer
//        int64 offerID;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ManageBuyOfferOp {
    pub selling: Asset,
    pub buying: Asset,
    pub buy_amount: Int64,
    pub price: Price,
    pub offer_id: Int64,
}

impl ReadXDR for ManageBuyOfferOp {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            selling: <Asset as ReadXDR>::read_xdr(r)?,
            buying: <Asset as ReadXDR>::read_xdr(r)?,
            buy_amount: <Int64 as ReadXDR>::read_xdr(r)?,
            price: <Price as ReadXDR>::read_xdr(r)?,
            offer_id: <Int64 as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for ManageBuyOfferOp {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.selling.write_xdr(w)?;
        self.buying.write_xdr(w)?;
        self.buy_amount.write_xdr(w)?;
        self.price.write_xdr(w)?;
        self.offer_id.write_xdr(w)?;
        Ok(())
    }
}

// CreatePassiveSellOfferOp is an XDR Struct defines as:
//
//   struct CreatePassiveSellOfferOp
//    {
//        Asset selling; // A
//        Asset buying;  // B
//        int64 amount;  // amount taker gets
//        Price price;   // cost of A in terms of B
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CreatePassiveSellOfferOp {
    pub selling: Asset,
    pub buying: Asset,
    pub amount: Int64,
    pub price: Price,
}

impl ReadXDR for CreatePassiveSellOfferOp {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            selling: <Asset as ReadXDR>::read_xdr(r)?,
            buying: <Asset as ReadXDR>::read_xdr(r)?,
            amount: <Int64 as ReadXDR>::read_xdr(r)?,
            price: <Price as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for CreatePassiveSellOfferOp {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.selling.write_xdr(w)?;
        self.buying.write_xdr(w)?;
        self.amount.write_xdr(w)?;
        self.price.write_xdr(w)?;
        Ok(())
    }
}

// SetOptionsOp is an XDR Struct defines as:
//
//   struct SetOptionsOp
//    {
//        AccountID* inflationDest; // sets the inflation destination
//
//        uint32* clearFlags; // which flags to clear
//        uint32* setFlags;   // which flags to set
//
//        // account threshold manipulation
//        uint32* masterWeight; // weight of the master account
//        uint32* lowThreshold;
//        uint32* medThreshold;
//        uint32* highThreshold;
//
//        string32* homeDomain; // sets the home domain
//
//        // Add, update or remove a signer for the account
//        // signer is deleted if the weight is 0
//        Signer* signer;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SetOptionsOp {
    pub inflation_dest: Option<AccountId>,
    pub clear_flags: Option<Uint32>,
    pub set_flags: Option<Uint32>,
    pub master_weight: Option<Uint32>,
    pub low_threshold: Option<Uint32>,
    pub med_threshold: Option<Uint32>,
    pub high_threshold: Option<Uint32>,
    pub home_domain: Option<String32>,
    pub signer: Option<Signer>,
}

impl ReadXDR for SetOptionsOp {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            inflation_dest: <Option<AccountId> as ReadXDR>::read_xdr(r)?,
            clear_flags: <Option<Uint32> as ReadXDR>::read_xdr(r)?,
            set_flags: <Option<Uint32> as ReadXDR>::read_xdr(r)?,
            master_weight: <Option<Uint32> as ReadXDR>::read_xdr(r)?,
            low_threshold: <Option<Uint32> as ReadXDR>::read_xdr(r)?,
            med_threshold: <Option<Uint32> as ReadXDR>::read_xdr(r)?,
            high_threshold: <Option<Uint32> as ReadXDR>::read_xdr(r)?,
            home_domain: <Option<String32> as ReadXDR>::read_xdr(r)?,
            signer: <Option<Signer> as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for SetOptionsOp {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.inflation_dest.write_xdr(w)?;
        self.clear_flags.write_xdr(w)?;
        self.set_flags.write_xdr(w)?;
        self.master_weight.write_xdr(w)?;
        self.low_threshold.write_xdr(w)?;
        self.med_threshold.write_xdr(w)?;
        self.high_threshold.write_xdr(w)?;
        self.home_domain.write_xdr(w)?;
        self.signer.write_xdr(w)?;
        Ok(())
    }
}

// ChangeTrustAsset is an XDR Union defines as:
//
//   union ChangeTrustAsset switch (AssetType type)
//    {
//    case ASSET_TYPE_NATIVE: // Not credit
//        void;
//
//    case ASSET_TYPE_CREDIT_ALPHANUM4:
//        AlphaNum4 alphaNum4;
//
//    case ASSET_TYPE_CREDIT_ALPHANUM12:
//        AlphaNum12 alphaNum12;
//
//    case ASSET_TYPE_POOL_SHARE:
//        LiquidityPoolParameters liquidityPool;
//
//        // add other asset types here in the future
//    };
//
// union with discriminant AssetType
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ChangeTrustAsset {
    AssetTypeNative,
    AssetTypeCreditAlphanum4(AlphaNum4),
    AssetTypeCreditAlphanum12(AlphaNum12),
    AssetTypePoolShare(LiquidityPoolParameters),
}

impl ChangeTrustAsset {
    fn discriminant(&self) -> AssetType {
        match self {
            Self::AssetTypeNative => AssetType::AssetTypeNative,
            Self::AssetTypeCreditAlphanum4(_) => AssetType::AssetTypeCreditAlphanum4,
            Self::AssetTypeCreditAlphanum12(_) => AssetType::AssetTypeCreditAlphanum12,
            Self::AssetTypePoolShare(_) => AssetType::AssetTypePoolShare,
        }
    }
}

impl ReadXDR for ChangeTrustAsset {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: AssetType = <AssetType as ReadXDR>::read_xdr(r)?;
        let v = match dv {
            AssetType::AssetTypeNative => Self::AssetTypeNative,
            AssetType::AssetTypeCreditAlphanum4 => {
                Self::AssetTypeCreditAlphanum4(<AlphaNum4 as ReadXDR>::read_xdr(r)?)
            }
            AssetType::AssetTypeCreditAlphanum12 => {
                Self::AssetTypeCreditAlphanum12(<AlphaNum12 as ReadXDR>::read_xdr(r)?)
            }
            AssetType::AssetTypePoolShare => {
                Self::AssetTypePoolShare(<LiquidityPoolParameters as ReadXDR>::read_xdr(r)?)
            }
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for ChangeTrustAsset {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::AssetTypeNative => ().write_xdr(w)?,
            Self::AssetTypeCreditAlphanum4(v) => v.write_xdr(w)?,
            Self::AssetTypeCreditAlphanum12(v) => v.write_xdr(w)?,
            Self::AssetTypePoolShare(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// ChangeTrustOp is an XDR Struct defines as:
//
//   struct ChangeTrustOp
//    {
//        ChangeTrustAsset line;
//
//        // if limit is set to 0, deletes the trust line
//        int64 limit;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChangeTrustOp {
    pub line: ChangeTrustAsset,
    pub limit: Int64,
}

impl ReadXDR for ChangeTrustOp {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            line: <ChangeTrustAsset as ReadXDR>::read_xdr(r)?,
            limit: <Int64 as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for ChangeTrustOp {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.line.write_xdr(w)?;
        self.limit.write_xdr(w)?;
        Ok(())
    }
}

// AllowTrustOp is an XDR Struct defines as:
//
//   struct AllowTrustOp
//    {
//        AccountID trustor;
//        AssetCode asset;
//
//        // One of 0, AUTHORIZED_FLAG, or AUTHORIZED_TO_MAINTAIN_LIABILITIES_FLAG
//        uint32 authorize;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AllowTrustOp {
    pub trustor: AccountId,
    pub asset: AssetCode,
    pub authorize: Uint32,
}

impl ReadXDR for AllowTrustOp {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            trustor: <AccountId as ReadXDR>::read_xdr(r)?,
            asset: <AssetCode as ReadXDR>::read_xdr(r)?,
            authorize: <Uint32 as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for AllowTrustOp {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.trustor.write_xdr(w)?;
        self.asset.write_xdr(w)?;
        self.authorize.write_xdr(w)?;
        Ok(())
    }
}

// ManageDataOp is an XDR Struct defines as:
//
//   struct ManageDataOp
//    {
//        string64 dataName;
//        DataValue* dataValue; // set to null to clear
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ManageDataOp {
    pub data_name: String64,
    pub data_value: Option<DataValue>,
}

impl ReadXDR for ManageDataOp {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            data_name: <String64 as ReadXDR>::read_xdr(r)?,
            data_value: <Option<DataValue> as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for ManageDataOp {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.data_name.write_xdr(w)?;
        self.data_value.write_xdr(w)?;
        Ok(())
    }
}

// BumpSequenceOp is an XDR Struct defines as:
//
//   struct BumpSequenceOp
//    {
//        SequenceNumber bumpTo;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BumpSequenceOp {
    pub bump_to: SequenceNumber,
}

impl ReadXDR for BumpSequenceOp {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            bump_to: <SequenceNumber as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for BumpSequenceOp {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.bump_to.write_xdr(w)?;
        Ok(())
    }
}

// CreateClaimableBalanceOp is an XDR Struct defines as:
//
//   struct CreateClaimableBalanceOp
//    {
//        Asset asset;
//        int64 amount;
//        Claimant claimants<10>;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CreateClaimableBalanceOp {
    pub asset: Asset,
    pub amount: Int64,
    pub claimants: Vec<Claimant>,
}

impl ReadXDR for CreateClaimableBalanceOp {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            asset: <Asset as ReadXDR>::read_xdr(r)?,
            amount: <Int64 as ReadXDR>::read_xdr(r)?,
            claimants: <Vec<Claimant> as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for CreateClaimableBalanceOp {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.asset.write_xdr(w)?;
        self.amount.write_xdr(w)?;
        self.claimants.write_xdr(w)?;
        Ok(())
    }
}

// ClaimClaimableBalanceOp is an XDR Struct defines as:
//
//   struct ClaimClaimableBalanceOp
//    {
//        ClaimableBalanceID balanceID;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClaimClaimableBalanceOp {
    pub balance_id: ClaimableBalanceId,
}

impl ReadXDR for ClaimClaimableBalanceOp {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            balance_id: <ClaimableBalanceId as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for ClaimClaimableBalanceOp {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.balance_id.write_xdr(w)?;
        Ok(())
    }
}

// BeginSponsoringFutureReservesOp is an XDR Struct defines as:
//
//   struct BeginSponsoringFutureReservesOp
//    {
//        AccountID sponsoredID;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BeginSponsoringFutureReservesOp {
    pub sponsored_id: AccountId,
}

impl ReadXDR for BeginSponsoringFutureReservesOp {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            sponsored_id: <AccountId as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for BeginSponsoringFutureReservesOp {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.sponsored_id.write_xdr(w)?;
        Ok(())
    }
}

// RevokeSponsorshipType is an XDR Enum defines as:
//
//   enum RevokeSponsorshipType
//    {
//        REVOKE_SPONSORSHIP_LEDGER_ENTRY = 0,
//        REVOKE_SPONSORSHIP_SIGNER = 1
//    };
//
// enum
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum RevokeSponsorshipType {
    RevokeSponsorshipLedgerEntry = 0,
    RevokeSponsorshipSigner = 1,
}

impl TryFrom<i32> for RevokeSponsorshipType {
    type Error = Error;

    fn try_from(i: i32) -> std::result::Result<Self, Self::Error> {
        let e = match i {
            0 => RevokeSponsorshipType::RevokeSponsorshipLedgerEntry,
            1 => RevokeSponsorshipType::RevokeSponsorshipSigner,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<RevokeSponsorshipType> for i32 {
    fn from(e: RevokeSponsorshipType) -> Self {
        e as Self
    }
}

impl ReadXDR for RevokeSponsorshipType {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXDR for RevokeSponsorshipType {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// RevokeSponsorshipOpSigner is an XDR NestedStruct defines as:
//
//   struct
//        {
//            AccountID accountID;
//            SignerKey signerKey;
//        }
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RevokeSponsorshipOpSigner {
    pub account_id: AccountId,
    pub signer_key: SignerKey,
}

impl ReadXDR for RevokeSponsorshipOpSigner {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            account_id: <AccountId as ReadXDR>::read_xdr(r)?,
            signer_key: <SignerKey as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for RevokeSponsorshipOpSigner {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.account_id.write_xdr(w)?;
        self.signer_key.write_xdr(w)?;
        Ok(())
    }
}

// RevokeSponsorshipOp is an XDR Union defines as:
//
//   union RevokeSponsorshipOp switch (RevokeSponsorshipType type)
//    {
//    case REVOKE_SPONSORSHIP_LEDGER_ENTRY:
//        LedgerKey ledgerKey;
//    case REVOKE_SPONSORSHIP_SIGNER:
//        struct
//        {
//            AccountID accountID;
//            SignerKey signerKey;
//        } signer;
//    };
//
// union with discriminant RevokeSponsorshipType
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RevokeSponsorshipOp {
    RevokeSponsorshipLedgerEntry(LedgerKey),
    RevokeSponsorshipSigner(RevokeSponsorshipOpSigner),
}

impl RevokeSponsorshipOp {
    fn discriminant(&self) -> RevokeSponsorshipType {
        match self {
            Self::RevokeSponsorshipLedgerEntry(_) => {
                RevokeSponsorshipType::RevokeSponsorshipLedgerEntry
            }
            Self::RevokeSponsorshipSigner(_) => RevokeSponsorshipType::RevokeSponsorshipSigner,
        }
    }
}

impl ReadXDR for RevokeSponsorshipOp {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: RevokeSponsorshipType = <RevokeSponsorshipType as ReadXDR>::read_xdr(r)?;
        let v = match dv {
            RevokeSponsorshipType::RevokeSponsorshipLedgerEntry => {
                Self::RevokeSponsorshipLedgerEntry(<LedgerKey as ReadXDR>::read_xdr(r)?)
            }
            RevokeSponsorshipType::RevokeSponsorshipSigner => {
                Self::RevokeSponsorshipSigner(<RevokeSponsorshipOpSigner as ReadXDR>::read_xdr(r)?)
            }
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for RevokeSponsorshipOp {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::RevokeSponsorshipLedgerEntry(v) => v.write_xdr(w)?,
            Self::RevokeSponsorshipSigner(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// ClawbackOp is an XDR Struct defines as:
//
//   struct ClawbackOp
//    {
//        Asset asset;
//        MuxedAccount from;
//        int64 amount;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClawbackOp {
    pub asset: Asset,
    pub from: MuxedAccount,
    pub amount: Int64,
}

impl ReadXDR for ClawbackOp {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            asset: <Asset as ReadXDR>::read_xdr(r)?,
            from: <MuxedAccount as ReadXDR>::read_xdr(r)?,
            amount: <Int64 as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for ClawbackOp {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.asset.write_xdr(w)?;
        self.from.write_xdr(w)?;
        self.amount.write_xdr(w)?;
        Ok(())
    }
}

// ClawbackClaimableBalanceOp is an XDR Struct defines as:
//
//   struct ClawbackClaimableBalanceOp
//    {
//        ClaimableBalanceID balanceID;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClawbackClaimableBalanceOp {
    pub balance_id: ClaimableBalanceId,
}

impl ReadXDR for ClawbackClaimableBalanceOp {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            balance_id: <ClaimableBalanceId as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for ClawbackClaimableBalanceOp {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.balance_id.write_xdr(w)?;
        Ok(())
    }
}

// SetTrustLineFlagsOp is an XDR Struct defines as:
//
//   struct SetTrustLineFlagsOp
//    {
//        AccountID trustor;
//        Asset asset;
//
//        uint32 clearFlags; // which flags to clear
//        uint32 setFlags;   // which flags to set
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SetTrustLineFlagsOp {
    pub trustor: AccountId,
    pub asset: Asset,
    pub clear_flags: Uint32,
    pub set_flags: Uint32,
}

impl ReadXDR for SetTrustLineFlagsOp {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            trustor: <AccountId as ReadXDR>::read_xdr(r)?,
            asset: <Asset as ReadXDR>::read_xdr(r)?,
            clear_flags: <Uint32 as ReadXDR>::read_xdr(r)?,
            set_flags: <Uint32 as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for SetTrustLineFlagsOp {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.trustor.write_xdr(w)?;
        self.asset.write_xdr(w)?;
        self.clear_flags.write_xdr(w)?;
        self.set_flags.write_xdr(w)?;
        Ok(())
    }
}

// LiquidityPoolFeeV18 is an XDR Const defines as:
//
//   const LIQUIDITY_POOL_FEE_V18 = 30;
//
pub const LIQUIDITY_POOL_FEE_V18: u64 = 30;

// LiquidityPoolDepositOp is an XDR Struct defines as:
//
//   struct LiquidityPoolDepositOp
//    {
//        PoolID liquidityPoolID;
//        int64 maxAmountA; // maximum amount of first asset to deposit
//        int64 maxAmountB; // maximum amount of second asset to deposit
//        Price minPrice;   // minimum depositA/depositB
//        Price maxPrice;   // maximum depositA/depositB
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LiquidityPoolDepositOp {
    pub liquidity_pool_id: PoolId,
    pub max_amount_a: Int64,
    pub max_amount_b: Int64,
    pub min_price: Price,
    pub max_price: Price,
}

impl ReadXDR for LiquidityPoolDepositOp {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            liquidity_pool_id: <PoolId as ReadXDR>::read_xdr(r)?,
            max_amount_a: <Int64 as ReadXDR>::read_xdr(r)?,
            max_amount_b: <Int64 as ReadXDR>::read_xdr(r)?,
            min_price: <Price as ReadXDR>::read_xdr(r)?,
            max_price: <Price as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for LiquidityPoolDepositOp {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.liquidity_pool_id.write_xdr(w)?;
        self.max_amount_a.write_xdr(w)?;
        self.max_amount_b.write_xdr(w)?;
        self.min_price.write_xdr(w)?;
        self.max_price.write_xdr(w)?;
        Ok(())
    }
}

// LiquidityPoolWithdrawOp is an XDR Struct defines as:
//
//   struct LiquidityPoolWithdrawOp
//    {
//        PoolID liquidityPoolID;
//        int64 amount;     // amount of pool shares to withdraw
//        int64 minAmountA; // minimum amount of first asset to withdraw
//        int64 minAmountB; // minimum amount of second asset to withdraw
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LiquidityPoolWithdrawOp {
    pub liquidity_pool_id: PoolId,
    pub amount: Int64,
    pub min_amount_a: Int64,
    pub min_amount_b: Int64,
}

impl ReadXDR for LiquidityPoolWithdrawOp {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            liquidity_pool_id: <PoolId as ReadXDR>::read_xdr(r)?,
            amount: <Int64 as ReadXDR>::read_xdr(r)?,
            min_amount_a: <Int64 as ReadXDR>::read_xdr(r)?,
            min_amount_b: <Int64 as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for LiquidityPoolWithdrawOp {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.liquidity_pool_id.write_xdr(w)?;
        self.amount.write_xdr(w)?;
        self.min_amount_a.write_xdr(w)?;
        self.min_amount_b.write_xdr(w)?;
        Ok(())
    }
}

// OperationBody is an XDR NestedUnion defines as:
//
//   union switch (OperationType type)
//        {
//        case CREATE_ACCOUNT:
//            CreateAccountOp createAccountOp;
//        case PAYMENT:
//            PaymentOp paymentOp;
//        case PATH_PAYMENT_STRICT_RECEIVE:
//            PathPaymentStrictReceiveOp pathPaymentStrictReceiveOp;
//        case MANAGE_SELL_OFFER:
//            ManageSellOfferOp manageSellOfferOp;
//        case CREATE_PASSIVE_SELL_OFFER:
//            CreatePassiveSellOfferOp createPassiveSellOfferOp;
//        case SET_OPTIONS:
//            SetOptionsOp setOptionsOp;
//        case CHANGE_TRUST:
//            ChangeTrustOp changeTrustOp;
//        case ALLOW_TRUST:
//            AllowTrustOp allowTrustOp;
//        case ACCOUNT_MERGE:
//            MuxedAccount destination;
//        case INFLATION:
//            void;
//        case MANAGE_DATA:
//            ManageDataOp manageDataOp;
//        case BUMP_SEQUENCE:
//            BumpSequenceOp bumpSequenceOp;
//        case MANAGE_BUY_OFFER:
//            ManageBuyOfferOp manageBuyOfferOp;
//        case PATH_PAYMENT_STRICT_SEND:
//            PathPaymentStrictSendOp pathPaymentStrictSendOp;
//        case CREATE_CLAIMABLE_BALANCE:
//            CreateClaimableBalanceOp createClaimableBalanceOp;
//        case CLAIM_CLAIMABLE_BALANCE:
//            ClaimClaimableBalanceOp claimClaimableBalanceOp;
//        case BEGIN_SPONSORING_FUTURE_RESERVES:
//            BeginSponsoringFutureReservesOp beginSponsoringFutureReservesOp;
//        case END_SPONSORING_FUTURE_RESERVES:
//            void;
//        case REVOKE_SPONSORSHIP:
//            RevokeSponsorshipOp revokeSponsorshipOp;
//        case CLAWBACK:
//            ClawbackOp clawbackOp;
//        case CLAWBACK_CLAIMABLE_BALANCE:
//            ClawbackClaimableBalanceOp clawbackClaimableBalanceOp;
//        case SET_TRUST_LINE_FLAGS:
//            SetTrustLineFlagsOp setTrustLineFlagsOp;
//        case LIQUIDITY_POOL_DEPOSIT:
//            LiquidityPoolDepositOp liquidityPoolDepositOp;
//        case LIQUIDITY_POOL_WITHDRAW:
//            LiquidityPoolWithdrawOp liquidityPoolWithdrawOp;
//        }
//
// union with discriminant OperationType
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OperationBody {
    CreateAccount(CreateAccountOp),
    Payment(PaymentOp),
    PathPaymentStrictReceive(PathPaymentStrictReceiveOp),
    ManageSellOffer(ManageSellOfferOp),
    CreatePassiveSellOffer(CreatePassiveSellOfferOp),
    SetOptions(SetOptionsOp),
    ChangeTrust(ChangeTrustOp),
    AllowTrust(AllowTrustOp),
    AccountMerge(MuxedAccount),
    Inflation,
    ManageData(ManageDataOp),
    BumpSequence(BumpSequenceOp),
    ManageBuyOffer(ManageBuyOfferOp),
    PathPaymentStrictSend(PathPaymentStrictSendOp),
    CreateClaimableBalance(CreateClaimableBalanceOp),
    ClaimClaimableBalance(ClaimClaimableBalanceOp),
    BeginSponsoringFutureReserves(BeginSponsoringFutureReservesOp),
    EndSponsoringFutureReserves,
    RevokeSponsorship(RevokeSponsorshipOp),
    Clawback(ClawbackOp),
    ClawbackClaimableBalance(ClawbackClaimableBalanceOp),
    SetTrustLineFlags(SetTrustLineFlagsOp),
    LiquidityPoolDeposit(LiquidityPoolDepositOp),
    LiquidityPoolWithdraw(LiquidityPoolWithdrawOp),
}

impl OperationBody {
    fn discriminant(&self) -> OperationType {
        match self {
            Self::CreateAccount(_) => OperationType::CreateAccount,
            Self::Payment(_) => OperationType::Payment,
            Self::PathPaymentStrictReceive(_) => OperationType::PathPaymentStrictReceive,
            Self::ManageSellOffer(_) => OperationType::ManageSellOffer,
            Self::CreatePassiveSellOffer(_) => OperationType::CreatePassiveSellOffer,
            Self::SetOptions(_) => OperationType::SetOptions,
            Self::ChangeTrust(_) => OperationType::ChangeTrust,
            Self::AllowTrust(_) => OperationType::AllowTrust,
            Self::AccountMerge(_) => OperationType::AccountMerge,
            Self::Inflation => OperationType::Inflation,
            Self::ManageData(_) => OperationType::ManageData,
            Self::BumpSequence(_) => OperationType::BumpSequence,
            Self::ManageBuyOffer(_) => OperationType::ManageBuyOffer,
            Self::PathPaymentStrictSend(_) => OperationType::PathPaymentStrictSend,
            Self::CreateClaimableBalance(_) => OperationType::CreateClaimableBalance,
            Self::ClaimClaimableBalance(_) => OperationType::ClaimClaimableBalance,
            Self::BeginSponsoringFutureReserves(_) => OperationType::BeginSponsoringFutureReserves,
            Self::EndSponsoringFutureReserves => OperationType::EndSponsoringFutureReserves,
            Self::RevokeSponsorship(_) => OperationType::RevokeSponsorship,
            Self::Clawback(_) => OperationType::Clawback,
            Self::ClawbackClaimableBalance(_) => OperationType::ClawbackClaimableBalance,
            Self::SetTrustLineFlags(_) => OperationType::SetTrustLineFlags,
            Self::LiquidityPoolDeposit(_) => OperationType::LiquidityPoolDeposit,
            Self::LiquidityPoolWithdraw(_) => OperationType::LiquidityPoolWithdraw,
        }
    }
}

impl ReadXDR for OperationBody {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: OperationType = <OperationType as ReadXDR>::read_xdr(r)?;
        let v = match dv {
            OperationType::CreateAccount => {
                Self::CreateAccount(<CreateAccountOp as ReadXDR>::read_xdr(r)?)
            }
            OperationType::Payment => Self::Payment(<PaymentOp as ReadXDR>::read_xdr(r)?),
            OperationType::PathPaymentStrictReceive => Self::PathPaymentStrictReceive(
                <PathPaymentStrictReceiveOp as ReadXDR>::read_xdr(r)?,
            ),
            OperationType::ManageSellOffer => {
                Self::ManageSellOffer(<ManageSellOfferOp as ReadXDR>::read_xdr(r)?)
            }
            OperationType::CreatePassiveSellOffer => {
                Self::CreatePassiveSellOffer(<CreatePassiveSellOfferOp as ReadXDR>::read_xdr(r)?)
            }
            OperationType::SetOptions => Self::SetOptions(<SetOptionsOp as ReadXDR>::read_xdr(r)?),
            OperationType::ChangeTrust => {
                Self::ChangeTrust(<ChangeTrustOp as ReadXDR>::read_xdr(r)?)
            }
            OperationType::AllowTrust => Self::AllowTrust(<AllowTrustOp as ReadXDR>::read_xdr(r)?),
            OperationType::AccountMerge => {
                Self::AccountMerge(<MuxedAccount as ReadXDR>::read_xdr(r)?)
            }
            OperationType::Inflation => Self::Inflation,
            OperationType::ManageData => Self::ManageData(<ManageDataOp as ReadXDR>::read_xdr(r)?),
            OperationType::BumpSequence => {
                Self::BumpSequence(<BumpSequenceOp as ReadXDR>::read_xdr(r)?)
            }
            OperationType::ManageBuyOffer => {
                Self::ManageBuyOffer(<ManageBuyOfferOp as ReadXDR>::read_xdr(r)?)
            }
            OperationType::PathPaymentStrictSend => {
                Self::PathPaymentStrictSend(<PathPaymentStrictSendOp as ReadXDR>::read_xdr(r)?)
            }
            OperationType::CreateClaimableBalance => {
                Self::CreateClaimableBalance(<CreateClaimableBalanceOp as ReadXDR>::read_xdr(r)?)
            }
            OperationType::ClaimClaimableBalance => {
                Self::ClaimClaimableBalance(<ClaimClaimableBalanceOp as ReadXDR>::read_xdr(r)?)
            }
            OperationType::BeginSponsoringFutureReserves => Self::BeginSponsoringFutureReserves(
                <BeginSponsoringFutureReservesOp as ReadXDR>::read_xdr(r)?,
            ),
            OperationType::EndSponsoringFutureReserves => Self::EndSponsoringFutureReserves,
            OperationType::RevokeSponsorship => {
                Self::RevokeSponsorship(<RevokeSponsorshipOp as ReadXDR>::read_xdr(r)?)
            }
            OperationType::Clawback => Self::Clawback(<ClawbackOp as ReadXDR>::read_xdr(r)?),
            OperationType::ClawbackClaimableBalance => Self::ClawbackClaimableBalance(
                <ClawbackClaimableBalanceOp as ReadXDR>::read_xdr(r)?,
            ),
            OperationType::SetTrustLineFlags => {
                Self::SetTrustLineFlags(<SetTrustLineFlagsOp as ReadXDR>::read_xdr(r)?)
            }
            OperationType::LiquidityPoolDeposit => {
                Self::LiquidityPoolDeposit(<LiquidityPoolDepositOp as ReadXDR>::read_xdr(r)?)
            }
            OperationType::LiquidityPoolWithdraw => {
                Self::LiquidityPoolWithdraw(<LiquidityPoolWithdrawOp as ReadXDR>::read_xdr(r)?)
            }
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for OperationBody {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::CreateAccount(v) => v.write_xdr(w)?,
            Self::Payment(v) => v.write_xdr(w)?,
            Self::PathPaymentStrictReceive(v) => v.write_xdr(w)?,
            Self::ManageSellOffer(v) => v.write_xdr(w)?,
            Self::CreatePassiveSellOffer(v) => v.write_xdr(w)?,
            Self::SetOptions(v) => v.write_xdr(w)?,
            Self::ChangeTrust(v) => v.write_xdr(w)?,
            Self::AllowTrust(v) => v.write_xdr(w)?,
            Self::AccountMerge(v) => v.write_xdr(w)?,
            Self::Inflation => ().write_xdr(w)?,
            Self::ManageData(v) => v.write_xdr(w)?,
            Self::BumpSequence(v) => v.write_xdr(w)?,
            Self::ManageBuyOffer(v) => v.write_xdr(w)?,
            Self::PathPaymentStrictSend(v) => v.write_xdr(w)?,
            Self::CreateClaimableBalance(v) => v.write_xdr(w)?,
            Self::ClaimClaimableBalance(v) => v.write_xdr(w)?,
            Self::BeginSponsoringFutureReserves(v) => v.write_xdr(w)?,
            Self::EndSponsoringFutureReserves => ().write_xdr(w)?,
            Self::RevokeSponsorship(v) => v.write_xdr(w)?,
            Self::Clawback(v) => v.write_xdr(w)?,
            Self::ClawbackClaimableBalance(v) => v.write_xdr(w)?,
            Self::SetTrustLineFlags(v) => v.write_xdr(w)?,
            Self::LiquidityPoolDeposit(v) => v.write_xdr(w)?,
            Self::LiquidityPoolWithdraw(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// Operation is an XDR Struct defines as:
//
//   struct Operation
//    {
//        // sourceAccount is the account used to run the operation
//        // if not set, the runtime defaults to "sourceAccount" specified at
//        // the transaction level
//        MuxedAccount* sourceAccount;
//
//        union switch (OperationType type)
//        {
//        case CREATE_ACCOUNT:
//            CreateAccountOp createAccountOp;
//        case PAYMENT:
//            PaymentOp paymentOp;
//        case PATH_PAYMENT_STRICT_RECEIVE:
//            PathPaymentStrictReceiveOp pathPaymentStrictReceiveOp;
//        case MANAGE_SELL_OFFER:
//            ManageSellOfferOp manageSellOfferOp;
//        case CREATE_PASSIVE_SELL_OFFER:
//            CreatePassiveSellOfferOp createPassiveSellOfferOp;
//        case SET_OPTIONS:
//            SetOptionsOp setOptionsOp;
//        case CHANGE_TRUST:
//            ChangeTrustOp changeTrustOp;
//        case ALLOW_TRUST:
//            AllowTrustOp allowTrustOp;
//        case ACCOUNT_MERGE:
//            MuxedAccount destination;
//        case INFLATION:
//            void;
//        case MANAGE_DATA:
//            ManageDataOp manageDataOp;
//        case BUMP_SEQUENCE:
//            BumpSequenceOp bumpSequenceOp;
//        case MANAGE_BUY_OFFER:
//            ManageBuyOfferOp manageBuyOfferOp;
//        case PATH_PAYMENT_STRICT_SEND:
//            PathPaymentStrictSendOp pathPaymentStrictSendOp;
//        case CREATE_CLAIMABLE_BALANCE:
//            CreateClaimableBalanceOp createClaimableBalanceOp;
//        case CLAIM_CLAIMABLE_BALANCE:
//            ClaimClaimableBalanceOp claimClaimableBalanceOp;
//        case BEGIN_SPONSORING_FUTURE_RESERVES:
//            BeginSponsoringFutureReservesOp beginSponsoringFutureReservesOp;
//        case END_SPONSORING_FUTURE_RESERVES:
//            void;
//        case REVOKE_SPONSORSHIP:
//            RevokeSponsorshipOp revokeSponsorshipOp;
//        case CLAWBACK:
//            ClawbackOp clawbackOp;
//        case CLAWBACK_CLAIMABLE_BALANCE:
//            ClawbackClaimableBalanceOp clawbackClaimableBalanceOp;
//        case SET_TRUST_LINE_FLAGS:
//            SetTrustLineFlagsOp setTrustLineFlagsOp;
//        case LIQUIDITY_POOL_DEPOSIT:
//            LiquidityPoolDepositOp liquidityPoolDepositOp;
//        case LIQUIDITY_POOL_WITHDRAW:
//            LiquidityPoolWithdrawOp liquidityPoolWithdrawOp;
//        }
//        body;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Operation {
    pub source_account: Option<MuxedAccount>,
    pub body: OperationBody,
}

impl ReadXDR for Operation {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            source_account: <Option<MuxedAccount> as ReadXDR>::read_xdr(r)?,
            body: <OperationBody as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for Operation {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.source_account.write_xdr(w)?;
        self.body.write_xdr(w)?;
        Ok(())
    }
}

// HashIdPreimageOperationId is an XDR NestedStruct defines as:
//
//   struct
//        {
//            AccountID sourceAccount;
//            SequenceNumber seqNum;
//            uint32 opNum;
//        }
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HashIdPreimageOperationId {
    pub source_account: AccountId,
    pub seq_num: SequenceNumber,
    pub op_num: Uint32,
}

impl ReadXDR for HashIdPreimageOperationId {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            source_account: <AccountId as ReadXDR>::read_xdr(r)?,
            seq_num: <SequenceNumber as ReadXDR>::read_xdr(r)?,
            op_num: <Uint32 as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for HashIdPreimageOperationId {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.source_account.write_xdr(w)?;
        self.seq_num.write_xdr(w)?;
        self.op_num.write_xdr(w)?;
        Ok(())
    }
}

// HashIdPreimageRevokeId is an XDR NestedStruct defines as:
//
//   struct
//        {
//            AccountID sourceAccount;
//            SequenceNumber seqNum;
//            uint32 opNum;
//            PoolID liquidityPoolID;
//            Asset asset;
//        }
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HashIdPreimageRevokeId {
    pub source_account: AccountId,
    pub seq_num: SequenceNumber,
    pub op_num: Uint32,
    pub liquidity_pool_id: PoolId,
    pub asset: Asset,
}

impl ReadXDR for HashIdPreimageRevokeId {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            source_account: <AccountId as ReadXDR>::read_xdr(r)?,
            seq_num: <SequenceNumber as ReadXDR>::read_xdr(r)?,
            op_num: <Uint32 as ReadXDR>::read_xdr(r)?,
            liquidity_pool_id: <PoolId as ReadXDR>::read_xdr(r)?,
            asset: <Asset as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for HashIdPreimageRevokeId {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.source_account.write_xdr(w)?;
        self.seq_num.write_xdr(w)?;
        self.op_num.write_xdr(w)?;
        self.liquidity_pool_id.write_xdr(w)?;
        self.asset.write_xdr(w)?;
        Ok(())
    }
}

// HashIdPreimage is an XDR Union defines as:
//
//   union HashIDPreimage switch (EnvelopeType type)
//    {
//    case ENVELOPE_TYPE_OP_ID:
//        struct
//        {
//            AccountID sourceAccount;
//            SequenceNumber seqNum;
//            uint32 opNum;
//        } operationID;
//    case ENVELOPE_TYPE_POOL_REVOKE_OP_ID:
//        struct
//        {
//            AccountID sourceAccount;
//            SequenceNumber seqNum;
//            uint32 opNum;
//            PoolID liquidityPoolID;
//            Asset asset;
//        } revokeID;
//    };
//
// union with discriminant EnvelopeType
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HashIdPreimage {
    EnvelopeTypeOpId(HashIdPreimageOperationId),
    EnvelopeTypePoolRevokeOpId(HashIdPreimageRevokeId),
}

impl HashIdPreimage {
    fn discriminant(&self) -> EnvelopeType {
        match self {
            Self::EnvelopeTypeOpId(_) => EnvelopeType::EnvelopeTypeOpId,
            Self::EnvelopeTypePoolRevokeOpId(_) => EnvelopeType::EnvelopeTypePoolRevokeOpId,
        }
    }
}

impl ReadXDR for HashIdPreimage {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: EnvelopeType = <EnvelopeType as ReadXDR>::read_xdr(r)?;
        let v = match dv {
            EnvelopeType::EnvelopeTypeOpId => {
                Self::EnvelopeTypeOpId(<HashIdPreimageOperationId as ReadXDR>::read_xdr(r)?)
            }
            EnvelopeType::EnvelopeTypePoolRevokeOpId => {
                Self::EnvelopeTypePoolRevokeOpId(<HashIdPreimageRevokeId as ReadXDR>::read_xdr(r)?)
            }
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for HashIdPreimage {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::EnvelopeTypeOpId(v) => v.write_xdr(w)?,
            Self::EnvelopeTypePoolRevokeOpId(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// MemoType is an XDR Enum defines as:
//
//   enum MemoType
//    {
//        MEMO_NONE = 0,
//        MEMO_TEXT = 1,
//        MEMO_ID = 2,
//        MEMO_HASH = 3,
//        MEMO_RETURN = 4
//    };
//
// enum
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum MemoType {
    MemoNone = 0,
    MemoText = 1,
    MemoId = 2,
    MemoHash = 3,
    MemoReturn = 4,
}

impl TryFrom<i32> for MemoType {
    type Error = Error;

    fn try_from(i: i32) -> std::result::Result<Self, Self::Error> {
        let e = match i {
            0 => MemoType::MemoNone,
            1 => MemoType::MemoText,
            2 => MemoType::MemoId,
            3 => MemoType::MemoHash,
            4 => MemoType::MemoReturn,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<MemoType> for i32 {
    fn from(e: MemoType) -> Self {
        e as Self
    }
}

impl ReadXDR for MemoType {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXDR for MemoType {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// Memo is an XDR Union defines as:
//
//   union Memo switch (MemoType type)
//    {
//    case MEMO_NONE:
//        void;
//    case MEMO_TEXT:
//        string text<28>;
//    case MEMO_ID:
//        uint64 id;
//    case MEMO_HASH:
//        Hash hash; // the hash of what to pull from the content server
//    case MEMO_RETURN:
//        Hash retHash; // the hash of the tx you are rejecting
//    };
//
// union with discriminant MemoType
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Memo {
    MemoNone,
    MemoText(Vec<u8>),
    MemoId(Uint64),
    MemoHash(Hash),
    MemoReturn(Hash),
}

impl Memo {
    fn discriminant(&self) -> MemoType {
        match self {
            Self::MemoNone => MemoType::MemoNone,
            Self::MemoText(_) => MemoType::MemoText,
            Self::MemoId(_) => MemoType::MemoId,
            Self::MemoHash(_) => MemoType::MemoHash,
            Self::MemoReturn(_) => MemoType::MemoReturn,
        }
    }
}

impl ReadXDR for Memo {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: MemoType = <MemoType as ReadXDR>::read_xdr(r)?;
        let v = match dv {
            MemoType::MemoNone => Self::MemoNone,
            MemoType::MemoText => Self::MemoText(<Vec<u8> as ReadXDR>::read_xdr(r)?),
            MemoType::MemoId => Self::MemoId(<Uint64 as ReadXDR>::read_xdr(r)?),
            MemoType::MemoHash => Self::MemoHash(<Hash as ReadXDR>::read_xdr(r)?),
            MemoType::MemoReturn => Self::MemoReturn(<Hash as ReadXDR>::read_xdr(r)?),
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for Memo {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::MemoNone => ().write_xdr(w)?,
            Self::MemoText(v) => v.write_xdr(w)?,
            Self::MemoId(v) => v.write_xdr(w)?,
            Self::MemoHash(v) => v.write_xdr(w)?,
            Self::MemoReturn(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// TimeBounds is an XDR Struct defines as:
//
//   struct TimeBounds
//    {
//        TimePoint minTime;
//        TimePoint maxTime; // 0 here means no maxTime
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TimeBounds {
    pub min_time: TimePoint,
    pub max_time: TimePoint,
}

impl ReadXDR for TimeBounds {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            min_time: <TimePoint as ReadXDR>::read_xdr(r)?,
            max_time: <TimePoint as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for TimeBounds {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.min_time.write_xdr(w)?;
        self.max_time.write_xdr(w)?;
        Ok(())
    }
}

// LedgerBounds is an XDR Struct defines as:
//
//   struct LedgerBounds
//    {
//        uint32 minLedger;
//        uint32 maxLedger; // 0 here means no maxLedger
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LedgerBounds {
    pub min_ledger: Uint32,
    pub max_ledger: Uint32,
}

impl ReadXDR for LedgerBounds {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            min_ledger: <Uint32 as ReadXDR>::read_xdr(r)?,
            max_ledger: <Uint32 as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for LedgerBounds {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.min_ledger.write_xdr(w)?;
        self.max_ledger.write_xdr(w)?;
        Ok(())
    }
}

// PreconditionsV2 is an XDR Struct defines as:
//
//   struct PreconditionsV2
//    {
//        TimeBounds* timeBounds;
//
//        // Transaction only valid for ledger numbers n such that
//        // minLedger <= n < maxLedger (if maxLedger == 0, then
//        // only minLedger is checked)
//        LedgerBounds* ledgerBounds;
//
//        // If NULL, only valid when sourceAccount's sequence number
//        // is seqNum - 1.  Otherwise, valid when sourceAccount's
//        // sequence number n satisfies minSeqNum <= n < tx.seqNum.
//        // Note that after execution the account's sequence number
//        // is always raised to tx.seqNum, and a transaction is not
//        // valid if tx.seqNum is too high to ensure replay protection.
//        SequenceNumber* minSeqNum;
//
//        // For the transaction to be valid, the current ledger time must
//        // be at least minSeqAge greater than sourceAccount's seqTime.
//        Duration minSeqAge;
//
//        // For the transaction to be valid, the current ledger number
//        // must be at least minSeqLedgerGap greater than sourceAccount's
//        // seqLedger.
//        uint32 minSeqLedgerGap;
//
//        // For the transaction to be valid, there must be a signature
//        // corresponding to every Signer in this array, even if the
//        // signature is not otherwise required by the sourceAccount or
//        // operations.
//        SignerKey extraSigners<2>;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PreconditionsV2 {
    pub time_bounds: Option<TimeBounds>,
    pub ledger_bounds: Option<LedgerBounds>,
    pub min_seq_num: Option<SequenceNumber>,
    pub min_seq_age: Duration,
    pub min_seq_ledger_gap: Uint32,
    pub extra_signers: Vec<SignerKey>,
}

impl ReadXDR for PreconditionsV2 {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            time_bounds: <Option<TimeBounds> as ReadXDR>::read_xdr(r)?,
            ledger_bounds: <Option<LedgerBounds> as ReadXDR>::read_xdr(r)?,
            min_seq_num: <Option<SequenceNumber> as ReadXDR>::read_xdr(r)?,
            min_seq_age: <Duration as ReadXDR>::read_xdr(r)?,
            min_seq_ledger_gap: <Uint32 as ReadXDR>::read_xdr(r)?,
            extra_signers: <Vec<SignerKey> as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for PreconditionsV2 {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.time_bounds.write_xdr(w)?;
        self.ledger_bounds.write_xdr(w)?;
        self.min_seq_num.write_xdr(w)?;
        self.min_seq_age.write_xdr(w)?;
        self.min_seq_ledger_gap.write_xdr(w)?;
        self.extra_signers.write_xdr(w)?;
        Ok(())
    }
}

// PreconditionType is an XDR Enum defines as:
//
//   enum PreconditionType
//    {
//        PRECOND_NONE = 0,
//        PRECOND_TIME = 1,
//        PRECOND_V2 = 2
//    };
//
// enum
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum PreconditionType {
    PrecondNone = 0,
    PrecondTime = 1,
    PrecondV2 = 2,
}

impl TryFrom<i32> for PreconditionType {
    type Error = Error;

    fn try_from(i: i32) -> std::result::Result<Self, Self::Error> {
        let e = match i {
            0 => PreconditionType::PrecondNone,
            1 => PreconditionType::PrecondTime,
            2 => PreconditionType::PrecondV2,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<PreconditionType> for i32 {
    fn from(e: PreconditionType) -> Self {
        e as Self
    }
}

impl ReadXDR for PreconditionType {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXDR for PreconditionType {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// Preconditions is an XDR Union defines as:
//
//   union Preconditions switch (PreconditionType type)
//    {
//    case PRECOND_NONE:
//        void;
//    case PRECOND_TIME:
//        TimeBounds timeBounds;
//    case PRECOND_V2:
//        PreconditionsV2 v2;
//    };
//
// union with discriminant PreconditionType
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Preconditions {
    PrecondNone,
    PrecondTime(TimeBounds),
    PrecondV2(PreconditionsV2),
}

impl Preconditions {
    fn discriminant(&self) -> PreconditionType {
        match self {
            Self::PrecondNone => PreconditionType::PrecondNone,
            Self::PrecondTime(_) => PreconditionType::PrecondTime,
            Self::PrecondV2(_) => PreconditionType::PrecondV2,
        }
    }
}

impl ReadXDR for Preconditions {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: PreconditionType = <PreconditionType as ReadXDR>::read_xdr(r)?;
        let v = match dv {
            PreconditionType::PrecondNone => Self::PrecondNone,
            PreconditionType::PrecondTime => {
                Self::PrecondTime(<TimeBounds as ReadXDR>::read_xdr(r)?)
            }
            PreconditionType::PrecondV2 => {
                Self::PrecondV2(<PreconditionsV2 as ReadXDR>::read_xdr(r)?)
            }
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for Preconditions {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::PrecondNone => ().write_xdr(w)?,
            Self::PrecondTime(v) => v.write_xdr(w)?,
            Self::PrecondV2(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// MaxOpsPerTx is an XDR Const defines as:
//
//   const MAX_OPS_PER_TX = 100;
//
pub const MAX_OPS_PER_TX: u64 = 100;

// TransactionV0Ext is an XDR NestedUnion defines as:
//
//   union switch (int v)
//        {
//        case 0:
//            void;
//        }
//
// union with discriminant i32
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TransactionV0Ext {
    V0,
}

impl TransactionV0Ext {
    fn discriminant(&self) -> i32 {
        match self {
            Self::V0 => 0,
        }
    }
}

impl ReadXDR for TransactionV0Ext {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: i32 = <i32 as ReadXDR>::read_xdr(r)?;
        let v = match dv.into() {
            0 => Self::V0,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for TransactionV0Ext {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::V0 => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// TransactionV0 is an XDR Struct defines as:
//
//   struct TransactionV0
//    {
//        uint256 sourceAccountEd25519;
//        uint32 fee;
//        SequenceNumber seqNum;
//        TimeBounds* timeBounds;
//        Memo memo;
//        Operation operations<MAX_OPS_PER_TX>;
//        union switch (int v)
//        {
//        case 0:
//            void;
//        }
//        ext;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransactionV0 {
    pub source_account_ed25519: Uint256,
    pub fee: Uint32,
    pub seq_num: SequenceNumber,
    pub time_bounds: Option<TimeBounds>,
    pub memo: Memo,
    pub operations: Vec<Operation>,
    pub ext: TransactionV0Ext,
}

impl ReadXDR for TransactionV0 {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            source_account_ed25519: <Uint256 as ReadXDR>::read_xdr(r)?,
            fee: <Uint32 as ReadXDR>::read_xdr(r)?,
            seq_num: <SequenceNumber as ReadXDR>::read_xdr(r)?,
            time_bounds: <Option<TimeBounds> as ReadXDR>::read_xdr(r)?,
            memo: <Memo as ReadXDR>::read_xdr(r)?,
            operations: <Vec<Operation> as ReadXDR>::read_xdr(r)?,
            ext: <TransactionV0Ext as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for TransactionV0 {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.source_account_ed25519.write_xdr(w)?;
        self.fee.write_xdr(w)?;
        self.seq_num.write_xdr(w)?;
        self.time_bounds.write_xdr(w)?;
        self.memo.write_xdr(w)?;
        self.operations.write_xdr(w)?;
        self.ext.write_xdr(w)?;
        Ok(())
    }
}

// TransactionV0Envelope is an XDR Struct defines as:
//
//   struct TransactionV0Envelope
//    {
//        TransactionV0 tx;
//        /* Each decorated signature is a signature over the SHA256 hash of
//         * a TransactionSignaturePayload */
//        DecoratedSignature signatures<20>;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransactionV0Envelope {
    pub tx: TransactionV0,
    pub signatures: Vec<DecoratedSignature>,
}

impl ReadXDR for TransactionV0Envelope {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            tx: <TransactionV0 as ReadXDR>::read_xdr(r)?,
            signatures: <Vec<DecoratedSignature> as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for TransactionV0Envelope {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.tx.write_xdr(w)?;
        self.signatures.write_xdr(w)?;
        Ok(())
    }
}

// TransactionExt is an XDR NestedUnion defines as:
//
//   union switch (int v)
//        {
//        case 0:
//            void;
//        }
//
// union with discriminant i32
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TransactionExt {
    V0,
}

impl TransactionExt {
    fn discriminant(&self) -> i32 {
        match self {
            Self::V0 => 0,
        }
    }
}

impl ReadXDR for TransactionExt {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: i32 = <i32 as ReadXDR>::read_xdr(r)?;
        let v = match dv.into() {
            0 => Self::V0,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for TransactionExt {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::V0 => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// Transaction is an XDR Struct defines as:
//
//   struct Transaction
//    {
//        // account used to run the transaction
//        MuxedAccount sourceAccount;
//
//        // the fee the sourceAccount will pay
//        uint32 fee;
//
//        // sequence number to consume in the account
//        SequenceNumber seqNum;
//
//        // validity conditions
//        Preconditions cond;
//
//        Memo memo;
//
//        Operation operations<MAX_OPS_PER_TX>;
//
//        // reserved for future use
//        union switch (int v)
//        {
//        case 0:
//            void;
//        }
//        ext;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Transaction {
    pub source_account: MuxedAccount,
    pub fee: Uint32,
    pub seq_num: SequenceNumber,
    pub cond: Preconditions,
    pub memo: Memo,
    pub operations: Vec<Operation>,
    pub ext: TransactionExt,
}

impl ReadXDR for Transaction {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            source_account: <MuxedAccount as ReadXDR>::read_xdr(r)?,
            fee: <Uint32 as ReadXDR>::read_xdr(r)?,
            seq_num: <SequenceNumber as ReadXDR>::read_xdr(r)?,
            cond: <Preconditions as ReadXDR>::read_xdr(r)?,
            memo: <Memo as ReadXDR>::read_xdr(r)?,
            operations: <Vec<Operation> as ReadXDR>::read_xdr(r)?,
            ext: <TransactionExt as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for Transaction {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.source_account.write_xdr(w)?;
        self.fee.write_xdr(w)?;
        self.seq_num.write_xdr(w)?;
        self.cond.write_xdr(w)?;
        self.memo.write_xdr(w)?;
        self.operations.write_xdr(w)?;
        self.ext.write_xdr(w)?;
        Ok(())
    }
}

// TransactionV1Envelope is an XDR Struct defines as:
//
//   struct TransactionV1Envelope
//    {
//        Transaction tx;
//        /* Each decorated signature is a signature over the SHA256 hash of
//         * a TransactionSignaturePayload */
//        DecoratedSignature signatures<20>;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransactionV1Envelope {
    pub tx: Transaction,
    pub signatures: Vec<DecoratedSignature>,
}

impl ReadXDR for TransactionV1Envelope {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            tx: <Transaction as ReadXDR>::read_xdr(r)?,
            signatures: <Vec<DecoratedSignature> as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for TransactionV1Envelope {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.tx.write_xdr(w)?;
        self.signatures.write_xdr(w)?;
        Ok(())
    }
}

// FeeBumpTransactionInnerTx is an XDR NestedUnion defines as:
//
//   union switch (EnvelopeType type)
//        {
//        case ENVELOPE_TYPE_TX:
//            TransactionV1Envelope v1;
//        }
//
// union with discriminant EnvelopeType
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FeeBumpTransactionInnerTx {
    EnvelopeTypeTx(TransactionV1Envelope),
}

impl FeeBumpTransactionInnerTx {
    fn discriminant(&self) -> EnvelopeType {
        match self {
            Self::EnvelopeTypeTx(_) => EnvelopeType::EnvelopeTypeTx,
        }
    }
}

impl ReadXDR for FeeBumpTransactionInnerTx {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: EnvelopeType = <EnvelopeType as ReadXDR>::read_xdr(r)?;
        let v = match dv {
            EnvelopeType::EnvelopeTypeTx => {
                Self::EnvelopeTypeTx(<TransactionV1Envelope as ReadXDR>::read_xdr(r)?)
            }
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for FeeBumpTransactionInnerTx {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::EnvelopeTypeTx(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// FeeBumpTransactionExt is an XDR NestedUnion defines as:
//
//   union switch (int v)
//        {
//        case 0:
//            void;
//        }
//
// union with discriminant i32
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FeeBumpTransactionExt {
    V0,
}

impl FeeBumpTransactionExt {
    fn discriminant(&self) -> i32 {
        match self {
            Self::V0 => 0,
        }
    }
}

impl ReadXDR for FeeBumpTransactionExt {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: i32 = <i32 as ReadXDR>::read_xdr(r)?;
        let v = match dv.into() {
            0 => Self::V0,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for FeeBumpTransactionExt {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::V0 => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// FeeBumpTransaction is an XDR Struct defines as:
//
//   struct FeeBumpTransaction
//    {
//        MuxedAccount feeSource;
//        int64 fee;
//        union switch (EnvelopeType type)
//        {
//        case ENVELOPE_TYPE_TX:
//            TransactionV1Envelope v1;
//        }
//        innerTx;
//        union switch (int v)
//        {
//        case 0:
//            void;
//        }
//        ext;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FeeBumpTransaction {
    pub fee_source: MuxedAccount,
    pub fee: Int64,
    pub inner_tx: FeeBumpTransactionInnerTx,
    pub ext: FeeBumpTransactionExt,
}

impl ReadXDR for FeeBumpTransaction {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            fee_source: <MuxedAccount as ReadXDR>::read_xdr(r)?,
            fee: <Int64 as ReadXDR>::read_xdr(r)?,
            inner_tx: <FeeBumpTransactionInnerTx as ReadXDR>::read_xdr(r)?,
            ext: <FeeBumpTransactionExt as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for FeeBumpTransaction {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.fee_source.write_xdr(w)?;
        self.fee.write_xdr(w)?;
        self.inner_tx.write_xdr(w)?;
        self.ext.write_xdr(w)?;
        Ok(())
    }
}

// FeeBumpTransactionEnvelope is an XDR Struct defines as:
//
//   struct FeeBumpTransactionEnvelope
//    {
//        FeeBumpTransaction tx;
//        /* Each decorated signature is a signature over the SHA256 hash of
//         * a TransactionSignaturePayload */
//        DecoratedSignature signatures<20>;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FeeBumpTransactionEnvelope {
    pub tx: FeeBumpTransaction,
    pub signatures: Vec<DecoratedSignature>,
}

impl ReadXDR for FeeBumpTransactionEnvelope {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            tx: <FeeBumpTransaction as ReadXDR>::read_xdr(r)?,
            signatures: <Vec<DecoratedSignature> as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for FeeBumpTransactionEnvelope {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.tx.write_xdr(w)?;
        self.signatures.write_xdr(w)?;
        Ok(())
    }
}

// TransactionEnvelope is an XDR Union defines as:
//
//   union TransactionEnvelope switch (EnvelopeType type)
//    {
//    case ENVELOPE_TYPE_TX_V0:
//        TransactionV0Envelope v0;
//    case ENVELOPE_TYPE_TX:
//        TransactionV1Envelope v1;
//    case ENVELOPE_TYPE_TX_FEE_BUMP:
//        FeeBumpTransactionEnvelope feeBump;
//    };
//
// union with discriminant EnvelopeType
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TransactionEnvelope {
    EnvelopeTypeTxV0(TransactionV0Envelope),
    EnvelopeTypeTx(TransactionV1Envelope),
    EnvelopeTypeTxFeeBump(FeeBumpTransactionEnvelope),
}

impl TransactionEnvelope {
    fn discriminant(&self) -> EnvelopeType {
        match self {
            Self::EnvelopeTypeTxV0(_) => EnvelopeType::EnvelopeTypeTxV0,
            Self::EnvelopeTypeTx(_) => EnvelopeType::EnvelopeTypeTx,
            Self::EnvelopeTypeTxFeeBump(_) => EnvelopeType::EnvelopeTypeTxFeeBump,
        }
    }
}

impl ReadXDR for TransactionEnvelope {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: EnvelopeType = <EnvelopeType as ReadXDR>::read_xdr(r)?;
        let v = match dv {
            EnvelopeType::EnvelopeTypeTxV0 => {
                Self::EnvelopeTypeTxV0(<TransactionV0Envelope as ReadXDR>::read_xdr(r)?)
            }
            EnvelopeType::EnvelopeTypeTx => {
                Self::EnvelopeTypeTx(<TransactionV1Envelope as ReadXDR>::read_xdr(r)?)
            }
            EnvelopeType::EnvelopeTypeTxFeeBump => {
                Self::EnvelopeTypeTxFeeBump(<FeeBumpTransactionEnvelope as ReadXDR>::read_xdr(r)?)
            }
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for TransactionEnvelope {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::EnvelopeTypeTxV0(v) => v.write_xdr(w)?,
            Self::EnvelopeTypeTx(v) => v.write_xdr(w)?,
            Self::EnvelopeTypeTxFeeBump(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// TransactionSignaturePayloadTaggedTransaction is an XDR NestedUnion defines as:
//
//   union switch (EnvelopeType type)
//        {
//        // Backwards Compatibility: Use ENVELOPE_TYPE_TX to sign ENVELOPE_TYPE_TX_V0
//        case ENVELOPE_TYPE_TX:
//            Transaction tx;
//        case ENVELOPE_TYPE_TX_FEE_BUMP:
//            FeeBumpTransaction feeBump;
//        }
//
// union with discriminant EnvelopeType
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TransactionSignaturePayloadTaggedTransaction {
    EnvelopeTypeTx(Transaction),
    EnvelopeTypeTxFeeBump(FeeBumpTransaction),
}

impl TransactionSignaturePayloadTaggedTransaction {
    fn discriminant(&self) -> EnvelopeType {
        match self {
            Self::EnvelopeTypeTx(_) => EnvelopeType::EnvelopeTypeTx,
            Self::EnvelopeTypeTxFeeBump(_) => EnvelopeType::EnvelopeTypeTxFeeBump,
        }
    }
}

impl ReadXDR for TransactionSignaturePayloadTaggedTransaction {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: EnvelopeType = <EnvelopeType as ReadXDR>::read_xdr(r)?;
        let v = match dv {
            EnvelopeType::EnvelopeTypeTx => {
                Self::EnvelopeTypeTx(<Transaction as ReadXDR>::read_xdr(r)?)
            }
            EnvelopeType::EnvelopeTypeTxFeeBump => {
                Self::EnvelopeTypeTxFeeBump(<FeeBumpTransaction as ReadXDR>::read_xdr(r)?)
            }
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for TransactionSignaturePayloadTaggedTransaction {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::EnvelopeTypeTx(v) => v.write_xdr(w)?,
            Self::EnvelopeTypeTxFeeBump(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// TransactionSignaturePayload is an XDR Struct defines as:
//
//   struct TransactionSignaturePayload
//    {
//        Hash networkId;
//        union switch (EnvelopeType type)
//        {
//        // Backwards Compatibility: Use ENVELOPE_TYPE_TX to sign ENVELOPE_TYPE_TX_V0
//        case ENVELOPE_TYPE_TX:
//            Transaction tx;
//        case ENVELOPE_TYPE_TX_FEE_BUMP:
//            FeeBumpTransaction feeBump;
//        }
//        taggedTransaction;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransactionSignaturePayload {
    pub network_id: Hash,
    pub tagged_transaction: TransactionSignaturePayloadTaggedTransaction,
}

impl ReadXDR for TransactionSignaturePayload {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            network_id: <Hash as ReadXDR>::read_xdr(r)?,
            tagged_transaction:
                <TransactionSignaturePayloadTaggedTransaction as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for TransactionSignaturePayload {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.network_id.write_xdr(w)?;
        self.tagged_transaction.write_xdr(w)?;
        Ok(())
    }
}

// ClaimAtomType is an XDR Enum defines as:
//
//   enum ClaimAtomType
//    {
//        CLAIM_ATOM_TYPE_V0 = 0,
//        CLAIM_ATOM_TYPE_ORDER_BOOK = 1,
//        CLAIM_ATOM_TYPE_LIQUIDITY_POOL = 2
//    };
//
// enum
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum ClaimAtomType {
    ClaimAtomTypeV0 = 0,
    ClaimAtomTypeOrderBook = 1,
    ClaimAtomTypeLiquidityPool = 2,
}

impl TryFrom<i32> for ClaimAtomType {
    type Error = Error;

    fn try_from(i: i32) -> std::result::Result<Self, Self::Error> {
        let e = match i {
            0 => ClaimAtomType::ClaimAtomTypeV0,
            1 => ClaimAtomType::ClaimAtomTypeOrderBook,
            2 => ClaimAtomType::ClaimAtomTypeLiquidityPool,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<ClaimAtomType> for i32 {
    fn from(e: ClaimAtomType) -> Self {
        e as Self
    }
}

impl ReadXDR for ClaimAtomType {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXDR for ClaimAtomType {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// ClaimOfferAtomV0 is an XDR Struct defines as:
//
//   struct ClaimOfferAtomV0
//    {
//        // emitted to identify the offer
//        uint256 sellerEd25519; // Account that owns the offer
//        int64 offerID;
//
//        // amount and asset taken from the owner
//        Asset assetSold;
//        int64 amountSold;
//
//        // amount and asset sent to the owner
//        Asset assetBought;
//        int64 amountBought;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClaimOfferAtomV0 {
    pub seller_ed25519: Uint256,
    pub offer_id: Int64,
    pub asset_sold: Asset,
    pub amount_sold: Int64,
    pub asset_bought: Asset,
    pub amount_bought: Int64,
}

impl ReadXDR for ClaimOfferAtomV0 {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            seller_ed25519: <Uint256 as ReadXDR>::read_xdr(r)?,
            offer_id: <Int64 as ReadXDR>::read_xdr(r)?,
            asset_sold: <Asset as ReadXDR>::read_xdr(r)?,
            amount_sold: <Int64 as ReadXDR>::read_xdr(r)?,
            asset_bought: <Asset as ReadXDR>::read_xdr(r)?,
            amount_bought: <Int64 as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for ClaimOfferAtomV0 {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.seller_ed25519.write_xdr(w)?;
        self.offer_id.write_xdr(w)?;
        self.asset_sold.write_xdr(w)?;
        self.amount_sold.write_xdr(w)?;
        self.asset_bought.write_xdr(w)?;
        self.amount_bought.write_xdr(w)?;
        Ok(())
    }
}

// ClaimOfferAtom is an XDR Struct defines as:
//
//   struct ClaimOfferAtom
//    {
//        // emitted to identify the offer
//        AccountID sellerID; // Account that owns the offer
//        int64 offerID;
//
//        // amount and asset taken from the owner
//        Asset assetSold;
//        int64 amountSold;
//
//        // amount and asset sent to the owner
//        Asset assetBought;
//        int64 amountBought;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClaimOfferAtom {
    pub seller_id: AccountId,
    pub offer_id: Int64,
    pub asset_sold: Asset,
    pub amount_sold: Int64,
    pub asset_bought: Asset,
    pub amount_bought: Int64,
}

impl ReadXDR for ClaimOfferAtom {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            seller_id: <AccountId as ReadXDR>::read_xdr(r)?,
            offer_id: <Int64 as ReadXDR>::read_xdr(r)?,
            asset_sold: <Asset as ReadXDR>::read_xdr(r)?,
            amount_sold: <Int64 as ReadXDR>::read_xdr(r)?,
            asset_bought: <Asset as ReadXDR>::read_xdr(r)?,
            amount_bought: <Int64 as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for ClaimOfferAtom {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.seller_id.write_xdr(w)?;
        self.offer_id.write_xdr(w)?;
        self.asset_sold.write_xdr(w)?;
        self.amount_sold.write_xdr(w)?;
        self.asset_bought.write_xdr(w)?;
        self.amount_bought.write_xdr(w)?;
        Ok(())
    }
}

// ClaimLiquidityAtom is an XDR Struct defines as:
//
//   struct ClaimLiquidityAtom
//    {
//        PoolID liquidityPoolID;
//
//        // amount and asset taken from the pool
//        Asset assetSold;
//        int64 amountSold;
//
//        // amount and asset sent to the pool
//        Asset assetBought;
//        int64 amountBought;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClaimLiquidityAtom {
    pub liquidity_pool_id: PoolId,
    pub asset_sold: Asset,
    pub amount_sold: Int64,
    pub asset_bought: Asset,
    pub amount_bought: Int64,
}

impl ReadXDR for ClaimLiquidityAtom {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            liquidity_pool_id: <PoolId as ReadXDR>::read_xdr(r)?,
            asset_sold: <Asset as ReadXDR>::read_xdr(r)?,
            amount_sold: <Int64 as ReadXDR>::read_xdr(r)?,
            asset_bought: <Asset as ReadXDR>::read_xdr(r)?,
            amount_bought: <Int64 as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for ClaimLiquidityAtom {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.liquidity_pool_id.write_xdr(w)?;
        self.asset_sold.write_xdr(w)?;
        self.amount_sold.write_xdr(w)?;
        self.asset_bought.write_xdr(w)?;
        self.amount_bought.write_xdr(w)?;
        Ok(())
    }
}

// ClaimAtom is an XDR Union defines as:
//
//   union ClaimAtom switch (ClaimAtomType type)
//    {
//    case CLAIM_ATOM_TYPE_V0:
//        ClaimOfferAtomV0 v0;
//    case CLAIM_ATOM_TYPE_ORDER_BOOK:
//        ClaimOfferAtom orderBook;
//    case CLAIM_ATOM_TYPE_LIQUIDITY_POOL:
//        ClaimLiquidityAtom liquidityPool;
//    };
//
// union with discriminant ClaimAtomType
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ClaimAtom {
    ClaimAtomTypeV0(ClaimOfferAtomV0),
    ClaimAtomTypeOrderBook(ClaimOfferAtom),
    ClaimAtomTypeLiquidityPool(ClaimLiquidityAtom),
}

impl ClaimAtom {
    fn discriminant(&self) -> ClaimAtomType {
        match self {
            Self::ClaimAtomTypeV0(_) => ClaimAtomType::ClaimAtomTypeV0,
            Self::ClaimAtomTypeOrderBook(_) => ClaimAtomType::ClaimAtomTypeOrderBook,
            Self::ClaimAtomTypeLiquidityPool(_) => ClaimAtomType::ClaimAtomTypeLiquidityPool,
        }
    }
}

impl ReadXDR for ClaimAtom {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: ClaimAtomType = <ClaimAtomType as ReadXDR>::read_xdr(r)?;
        let v = match dv {
            ClaimAtomType::ClaimAtomTypeV0 => {
                Self::ClaimAtomTypeV0(<ClaimOfferAtomV0 as ReadXDR>::read_xdr(r)?)
            }
            ClaimAtomType::ClaimAtomTypeOrderBook => {
                Self::ClaimAtomTypeOrderBook(<ClaimOfferAtom as ReadXDR>::read_xdr(r)?)
            }
            ClaimAtomType::ClaimAtomTypeLiquidityPool => {
                Self::ClaimAtomTypeLiquidityPool(<ClaimLiquidityAtom as ReadXDR>::read_xdr(r)?)
            }
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for ClaimAtom {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::ClaimAtomTypeV0(v) => v.write_xdr(w)?,
            Self::ClaimAtomTypeOrderBook(v) => v.write_xdr(w)?,
            Self::ClaimAtomTypeLiquidityPool(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// CreateAccountResultCode is an XDR Enum defines as:
//
//   enum CreateAccountResultCode
//    {
//        // codes considered as "success" for the operation
//        CREATE_ACCOUNT_SUCCESS = 0, // account was created
//
//        // codes considered as "failure" for the operation
//        CREATE_ACCOUNT_MALFORMED = -1,   // invalid destination
//        CREATE_ACCOUNT_UNDERFUNDED = -2, // not enough funds in source account
//        CREATE_ACCOUNT_LOW_RESERVE =
//            -3, // would create an account below the min reserve
//        CREATE_ACCOUNT_ALREADY_EXIST = -4 // account already exists
//    };
//
// enum
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum CreateAccountResultCode {
    CreateAccountSuccess = 0,
    CreateAccountMalformed = -1,
    CreateAccountUnderfunded = -2,
    CreateAccountLowReserve = -3,
    CreateAccountAlreadyExist = -4,
}

impl TryFrom<i32> for CreateAccountResultCode {
    type Error = Error;

    fn try_from(i: i32) -> std::result::Result<Self, Self::Error> {
        let e = match i {
            0 => CreateAccountResultCode::CreateAccountSuccess,
            -1 => CreateAccountResultCode::CreateAccountMalformed,
            -2 => CreateAccountResultCode::CreateAccountUnderfunded,
            -3 => CreateAccountResultCode::CreateAccountLowReserve,
            -4 => CreateAccountResultCode::CreateAccountAlreadyExist,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<CreateAccountResultCode> for i32 {
    fn from(e: CreateAccountResultCode) -> Self {
        e as Self
    }
}

impl ReadXDR for CreateAccountResultCode {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXDR for CreateAccountResultCode {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// CreateAccountResult is an XDR Union defines as:
//
//   union CreateAccountResult switch (CreateAccountResultCode code)
//    {
//    case CREATE_ACCOUNT_SUCCESS:
//        void;
//    default:
//        void;
//    };
//
// union with discriminant CreateAccountResultCode
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CreateAccountResult {
    CreateAccountSuccess,
}

impl CreateAccountResult {
    fn discriminant(&self) -> CreateAccountResultCode {
        match self {
            Self::CreateAccountSuccess => CreateAccountResultCode::CreateAccountSuccess,
        }
    }
}

impl ReadXDR for CreateAccountResult {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: CreateAccountResultCode = <CreateAccountResultCode as ReadXDR>::read_xdr(r)?;
        let v = match dv {
            CreateAccountResultCode::CreateAccountSuccess => Self::CreateAccountSuccess,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for CreateAccountResult {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::CreateAccountSuccess => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// PaymentResultCode is an XDR Enum defines as:
//
//   enum PaymentResultCode
//    {
//        // codes considered as "success" for the operation
//        PAYMENT_SUCCESS = 0, // payment successfully completed
//
//        // codes considered as "failure" for the operation
//        PAYMENT_MALFORMED = -1,          // bad input
//        PAYMENT_UNDERFUNDED = -2,        // not enough funds in source account
//        PAYMENT_SRC_NO_TRUST = -3,       // no trust line on source account
//        PAYMENT_SRC_NOT_AUTHORIZED = -4, // source not authorized to transfer
//        PAYMENT_NO_DESTINATION = -5,     // destination account does not exist
//        PAYMENT_NO_TRUST = -6,       // destination missing a trust line for asset
//        PAYMENT_NOT_AUTHORIZED = -7, // destination not authorized to hold asset
//        PAYMENT_LINE_FULL = -8,      // destination would go above their limit
//        PAYMENT_NO_ISSUER = -9       // missing issuer on asset
//    };
//
// enum
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum PaymentResultCode {
    PaymentSuccess = 0,
    PaymentMalformed = -1,
    PaymentUnderfunded = -2,
    PaymentSrcNoTrust = -3,
    PaymentSrcNotAuthorized = -4,
    PaymentNoDestination = -5,
    PaymentNoTrust = -6,
    PaymentNotAuthorized = -7,
    PaymentLineFull = -8,
    PaymentNoIssuer = -9,
}

impl TryFrom<i32> for PaymentResultCode {
    type Error = Error;

    fn try_from(i: i32) -> std::result::Result<Self, Self::Error> {
        let e = match i {
            0 => PaymentResultCode::PaymentSuccess,
            -1 => PaymentResultCode::PaymentMalformed,
            -2 => PaymentResultCode::PaymentUnderfunded,
            -3 => PaymentResultCode::PaymentSrcNoTrust,
            -4 => PaymentResultCode::PaymentSrcNotAuthorized,
            -5 => PaymentResultCode::PaymentNoDestination,
            -6 => PaymentResultCode::PaymentNoTrust,
            -7 => PaymentResultCode::PaymentNotAuthorized,
            -8 => PaymentResultCode::PaymentLineFull,
            -9 => PaymentResultCode::PaymentNoIssuer,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<PaymentResultCode> for i32 {
    fn from(e: PaymentResultCode) -> Self {
        e as Self
    }
}

impl ReadXDR for PaymentResultCode {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXDR for PaymentResultCode {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// PaymentResult is an XDR Union defines as:
//
//   union PaymentResult switch (PaymentResultCode code)
//    {
//    case PAYMENT_SUCCESS:
//        void;
//    default:
//        void;
//    };
//
// union with discriminant PaymentResultCode
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PaymentResult {
    PaymentSuccess,
}

impl PaymentResult {
    fn discriminant(&self) -> PaymentResultCode {
        match self {
            Self::PaymentSuccess => PaymentResultCode::PaymentSuccess,
        }
    }
}

impl ReadXDR for PaymentResult {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: PaymentResultCode = <PaymentResultCode as ReadXDR>::read_xdr(r)?;
        let v = match dv {
            PaymentResultCode::PaymentSuccess => Self::PaymentSuccess,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for PaymentResult {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::PaymentSuccess => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// PathPaymentStrictReceiveResultCode is an XDR Enum defines as:
//
//   enum PathPaymentStrictReceiveResultCode
//    {
//        // codes considered as "success" for the operation
//        PATH_PAYMENT_STRICT_RECEIVE_SUCCESS = 0, // success
//
//        // codes considered as "failure" for the operation
//        PATH_PAYMENT_STRICT_RECEIVE_MALFORMED = -1, // bad input
//        PATH_PAYMENT_STRICT_RECEIVE_UNDERFUNDED =
//            -2, // not enough funds in source account
//        PATH_PAYMENT_STRICT_RECEIVE_SRC_NO_TRUST =
//            -3, // no trust line on source account
//        PATH_PAYMENT_STRICT_RECEIVE_SRC_NOT_AUTHORIZED =
//            -4, // source not authorized to transfer
//        PATH_PAYMENT_STRICT_RECEIVE_NO_DESTINATION =
//            -5, // destination account does not exist
//        PATH_PAYMENT_STRICT_RECEIVE_NO_TRUST =
//            -6, // dest missing a trust line for asset
//        PATH_PAYMENT_STRICT_RECEIVE_NOT_AUTHORIZED =
//            -7, // dest not authorized to hold asset
//        PATH_PAYMENT_STRICT_RECEIVE_LINE_FULL =
//            -8, // dest would go above their limit
//        PATH_PAYMENT_STRICT_RECEIVE_NO_ISSUER = -9, // missing issuer on one asset
//        PATH_PAYMENT_STRICT_RECEIVE_TOO_FEW_OFFERS =
//            -10, // not enough offers to satisfy path
//        PATH_PAYMENT_STRICT_RECEIVE_OFFER_CROSS_SELF =
//            -11, // would cross one of its own offers
//        PATH_PAYMENT_STRICT_RECEIVE_OVER_SENDMAX = -12 // could not satisfy sendmax
//    };
//
// enum
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum PathPaymentStrictReceiveResultCode {
    PathPaymentStrictReceiveSuccess = 0,
    PathPaymentStrictReceiveMalformed = -1,
    PathPaymentStrictReceiveUnderfunded = -2,
    PathPaymentStrictReceiveSrcNoTrust = -3,
    PathPaymentStrictReceiveSrcNotAuthorized = -4,
    PathPaymentStrictReceiveNoDestination = -5,
    PathPaymentStrictReceiveNoTrust = -6,
    PathPaymentStrictReceiveNotAuthorized = -7,
    PathPaymentStrictReceiveLineFull = -8,
    PathPaymentStrictReceiveNoIssuer = -9,
    PathPaymentStrictReceiveTooFewOffers = -10,
    PathPaymentStrictReceiveOfferCrossSelf = -11,
    PathPaymentStrictReceiveOverSendmax = -12,
}

impl TryFrom<i32> for PathPaymentStrictReceiveResultCode {
    type Error = Error;

    fn try_from(i: i32) -> std::result::Result<Self, Self::Error> {
        let e = match i {
            0 => PathPaymentStrictReceiveResultCode::PathPaymentStrictReceiveSuccess,
            -1 => PathPaymentStrictReceiveResultCode::PathPaymentStrictReceiveMalformed,
            -2 => PathPaymentStrictReceiveResultCode::PathPaymentStrictReceiveUnderfunded,
            -3 => PathPaymentStrictReceiveResultCode::PathPaymentStrictReceiveSrcNoTrust,
            -4 => PathPaymentStrictReceiveResultCode::PathPaymentStrictReceiveSrcNotAuthorized,
            -5 => PathPaymentStrictReceiveResultCode::PathPaymentStrictReceiveNoDestination,
            -6 => PathPaymentStrictReceiveResultCode::PathPaymentStrictReceiveNoTrust,
            -7 => PathPaymentStrictReceiveResultCode::PathPaymentStrictReceiveNotAuthorized,
            -8 => PathPaymentStrictReceiveResultCode::PathPaymentStrictReceiveLineFull,
            -9 => PathPaymentStrictReceiveResultCode::PathPaymentStrictReceiveNoIssuer,
            -10 => PathPaymentStrictReceiveResultCode::PathPaymentStrictReceiveTooFewOffers,
            -11 => PathPaymentStrictReceiveResultCode::PathPaymentStrictReceiveOfferCrossSelf,
            -12 => PathPaymentStrictReceiveResultCode::PathPaymentStrictReceiveOverSendmax,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<PathPaymentStrictReceiveResultCode> for i32 {
    fn from(e: PathPaymentStrictReceiveResultCode) -> Self {
        e as Self
    }
}

impl ReadXDR for PathPaymentStrictReceiveResultCode {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXDR for PathPaymentStrictReceiveResultCode {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// SimplePaymentResult is an XDR Struct defines as:
//
//   struct SimplePaymentResult
//    {
//        AccountID destination;
//        Asset asset;
//        int64 amount;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SimplePaymentResult {
    pub destination: AccountId,
    pub asset: Asset,
    pub amount: Int64,
}

impl ReadXDR for SimplePaymentResult {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            destination: <AccountId as ReadXDR>::read_xdr(r)?,
            asset: <Asset as ReadXDR>::read_xdr(r)?,
            amount: <Int64 as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for SimplePaymentResult {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.destination.write_xdr(w)?;
        self.asset.write_xdr(w)?;
        self.amount.write_xdr(w)?;
        Ok(())
    }
}

// PathPaymentStrictReceiveResultSuccess is an XDR NestedStruct defines as:
//
//   struct
//        {
//            ClaimAtom offers<>;
//            SimplePaymentResult last;
//        }
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PathPaymentStrictReceiveResultSuccess {
    pub offers: Vec<ClaimAtom>,
    pub last: SimplePaymentResult,
}

impl ReadXDR for PathPaymentStrictReceiveResultSuccess {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            offers: <Vec<ClaimAtom> as ReadXDR>::read_xdr(r)?,
            last: <SimplePaymentResult as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for PathPaymentStrictReceiveResultSuccess {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.offers.write_xdr(w)?;
        self.last.write_xdr(w)?;
        Ok(())
    }
}

// PathPaymentStrictReceiveResult is an XDR Union defines as:
//
//   union PathPaymentStrictReceiveResult switch (
//        PathPaymentStrictReceiveResultCode code)
//    {
//    case PATH_PAYMENT_STRICT_RECEIVE_SUCCESS:
//        struct
//        {
//            ClaimAtom offers<>;
//            SimplePaymentResult last;
//        } success;
//    case PATH_PAYMENT_STRICT_RECEIVE_NO_ISSUER:
//        Asset noIssuer; // the asset that caused the error
//    default:
//        void;
//    };
//
// union with discriminant PathPaymentStrictReceiveResultCode
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PathPaymentStrictReceiveResult {
    PathPaymentStrictReceiveSuccess(PathPaymentStrictReceiveResultSuccess),
    PathPaymentStrictReceiveNoIssuer(Asset),
}

impl PathPaymentStrictReceiveResult {
    fn discriminant(&self) -> PathPaymentStrictReceiveResultCode {
        match self {
            Self::PathPaymentStrictReceiveSuccess(_) => {
                PathPaymentStrictReceiveResultCode::PathPaymentStrictReceiveSuccess
            }
            Self::PathPaymentStrictReceiveNoIssuer(_) => {
                PathPaymentStrictReceiveResultCode::PathPaymentStrictReceiveNoIssuer
            }
        }
    }
}

impl ReadXDR for PathPaymentStrictReceiveResult {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: PathPaymentStrictReceiveResultCode =
            <PathPaymentStrictReceiveResultCode as ReadXDR>::read_xdr(r)?;
        let v = match dv {
            PathPaymentStrictReceiveResultCode::PathPaymentStrictReceiveSuccess => {
                Self::PathPaymentStrictReceiveSuccess(
                    <PathPaymentStrictReceiveResultSuccess as ReadXDR>::read_xdr(r)?,
                )
            }
            PathPaymentStrictReceiveResultCode::PathPaymentStrictReceiveNoIssuer => {
                Self::PathPaymentStrictReceiveNoIssuer(<Asset as ReadXDR>::read_xdr(r)?)
            }
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for PathPaymentStrictReceiveResult {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::PathPaymentStrictReceiveSuccess(v) => v.write_xdr(w)?,
            Self::PathPaymentStrictReceiveNoIssuer(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// PathPaymentStrictSendResultCode is an XDR Enum defines as:
//
//   enum PathPaymentStrictSendResultCode
//    {
//        // codes considered as "success" for the operation
//        PATH_PAYMENT_STRICT_SEND_SUCCESS = 0, // success
//
//        // codes considered as "failure" for the operation
//        PATH_PAYMENT_STRICT_SEND_MALFORMED = -1, // bad input
//        PATH_PAYMENT_STRICT_SEND_UNDERFUNDED =
//            -2, // not enough funds in source account
//        PATH_PAYMENT_STRICT_SEND_SRC_NO_TRUST =
//            -3, // no trust line on source account
//        PATH_PAYMENT_STRICT_SEND_SRC_NOT_AUTHORIZED =
//            -4, // source not authorized to transfer
//        PATH_PAYMENT_STRICT_SEND_NO_DESTINATION =
//            -5, // destination account does not exist
//        PATH_PAYMENT_STRICT_SEND_NO_TRUST =
//            -6, // dest missing a trust line for asset
//        PATH_PAYMENT_STRICT_SEND_NOT_AUTHORIZED =
//            -7, // dest not authorized to hold asset
//        PATH_PAYMENT_STRICT_SEND_LINE_FULL = -8, // dest would go above their limit
//        PATH_PAYMENT_STRICT_SEND_NO_ISSUER = -9, // missing issuer on one asset
//        PATH_PAYMENT_STRICT_SEND_TOO_FEW_OFFERS =
//            -10, // not enough offers to satisfy path
//        PATH_PAYMENT_STRICT_SEND_OFFER_CROSS_SELF =
//            -11, // would cross one of its own offers
//        PATH_PAYMENT_STRICT_SEND_UNDER_DESTMIN = -12 // could not satisfy destMin
//    };
//
// enum
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum PathPaymentStrictSendResultCode {
    PathPaymentStrictSendSuccess = 0,
    PathPaymentStrictSendMalformed = -1,
    PathPaymentStrictSendUnderfunded = -2,
    PathPaymentStrictSendSrcNoTrust = -3,
    PathPaymentStrictSendSrcNotAuthorized = -4,
    PathPaymentStrictSendNoDestination = -5,
    PathPaymentStrictSendNoTrust = -6,
    PathPaymentStrictSendNotAuthorized = -7,
    PathPaymentStrictSendLineFull = -8,
    PathPaymentStrictSendNoIssuer = -9,
    PathPaymentStrictSendTooFewOffers = -10,
    PathPaymentStrictSendOfferCrossSelf = -11,
    PathPaymentStrictSendUnderDestmin = -12,
}

impl TryFrom<i32> for PathPaymentStrictSendResultCode {
    type Error = Error;

    fn try_from(i: i32) -> std::result::Result<Self, Self::Error> {
        let e = match i {
            0 => PathPaymentStrictSendResultCode::PathPaymentStrictSendSuccess,
            -1 => PathPaymentStrictSendResultCode::PathPaymentStrictSendMalformed,
            -2 => PathPaymentStrictSendResultCode::PathPaymentStrictSendUnderfunded,
            -3 => PathPaymentStrictSendResultCode::PathPaymentStrictSendSrcNoTrust,
            -4 => PathPaymentStrictSendResultCode::PathPaymentStrictSendSrcNotAuthorized,
            -5 => PathPaymentStrictSendResultCode::PathPaymentStrictSendNoDestination,
            -6 => PathPaymentStrictSendResultCode::PathPaymentStrictSendNoTrust,
            -7 => PathPaymentStrictSendResultCode::PathPaymentStrictSendNotAuthorized,
            -8 => PathPaymentStrictSendResultCode::PathPaymentStrictSendLineFull,
            -9 => PathPaymentStrictSendResultCode::PathPaymentStrictSendNoIssuer,
            -10 => PathPaymentStrictSendResultCode::PathPaymentStrictSendTooFewOffers,
            -11 => PathPaymentStrictSendResultCode::PathPaymentStrictSendOfferCrossSelf,
            -12 => PathPaymentStrictSendResultCode::PathPaymentStrictSendUnderDestmin,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<PathPaymentStrictSendResultCode> for i32 {
    fn from(e: PathPaymentStrictSendResultCode) -> Self {
        e as Self
    }
}

impl ReadXDR for PathPaymentStrictSendResultCode {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXDR for PathPaymentStrictSendResultCode {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// PathPaymentStrictSendResultSuccess is an XDR NestedStruct defines as:
//
//   struct
//        {
//            ClaimAtom offers<>;
//            SimplePaymentResult last;
//        }
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PathPaymentStrictSendResultSuccess {
    pub offers: Vec<ClaimAtom>,
    pub last: SimplePaymentResult,
}

impl ReadXDR for PathPaymentStrictSendResultSuccess {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            offers: <Vec<ClaimAtom> as ReadXDR>::read_xdr(r)?,
            last: <SimplePaymentResult as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for PathPaymentStrictSendResultSuccess {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.offers.write_xdr(w)?;
        self.last.write_xdr(w)?;
        Ok(())
    }
}

// PathPaymentStrictSendResult is an XDR Union defines as:
//
//   union PathPaymentStrictSendResult switch (PathPaymentStrictSendResultCode code)
//    {
//    case PATH_PAYMENT_STRICT_SEND_SUCCESS:
//        struct
//        {
//            ClaimAtom offers<>;
//            SimplePaymentResult last;
//        } success;
//    case PATH_PAYMENT_STRICT_SEND_NO_ISSUER:
//        Asset noIssuer; // the asset that caused the error
//    default:
//        void;
//    };
//
// union with discriminant PathPaymentStrictSendResultCode
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PathPaymentStrictSendResult {
    PathPaymentStrictSendSuccess(PathPaymentStrictSendResultSuccess),
    PathPaymentStrictSendNoIssuer(Asset),
}

impl PathPaymentStrictSendResult {
    fn discriminant(&self) -> PathPaymentStrictSendResultCode {
        match self {
            Self::PathPaymentStrictSendSuccess(_) => {
                PathPaymentStrictSendResultCode::PathPaymentStrictSendSuccess
            }
            Self::PathPaymentStrictSendNoIssuer(_) => {
                PathPaymentStrictSendResultCode::PathPaymentStrictSendNoIssuer
            }
        }
    }
}

impl ReadXDR for PathPaymentStrictSendResult {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: PathPaymentStrictSendResultCode =
            <PathPaymentStrictSendResultCode as ReadXDR>::read_xdr(r)?;
        let v = match dv {
            PathPaymentStrictSendResultCode::PathPaymentStrictSendSuccess => {
                Self::PathPaymentStrictSendSuccess(
                    <PathPaymentStrictSendResultSuccess as ReadXDR>::read_xdr(r)?,
                )
            }
            PathPaymentStrictSendResultCode::PathPaymentStrictSendNoIssuer => {
                Self::PathPaymentStrictSendNoIssuer(<Asset as ReadXDR>::read_xdr(r)?)
            }
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for PathPaymentStrictSendResult {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::PathPaymentStrictSendSuccess(v) => v.write_xdr(w)?,
            Self::PathPaymentStrictSendNoIssuer(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// ManageSellOfferResultCode is an XDR Enum defines as:
//
//   enum ManageSellOfferResultCode
//    {
//        // codes considered as "success" for the operation
//        MANAGE_SELL_OFFER_SUCCESS = 0,
//
//        // codes considered as "failure" for the operation
//        MANAGE_SELL_OFFER_MALFORMED = -1, // generated offer would be invalid
//        MANAGE_SELL_OFFER_SELL_NO_TRUST =
//            -2,                              // no trust line for what we're selling
//        MANAGE_SELL_OFFER_BUY_NO_TRUST = -3, // no trust line for what we're buying
//        MANAGE_SELL_OFFER_SELL_NOT_AUTHORIZED = -4, // not authorized to sell
//        MANAGE_SELL_OFFER_BUY_NOT_AUTHORIZED = -5,  // not authorized to buy
//        MANAGE_SELL_OFFER_LINE_FULL = -6, // can't receive more of what it's buying
//        MANAGE_SELL_OFFER_UNDERFUNDED = -7, // doesn't hold what it's trying to sell
//        MANAGE_SELL_OFFER_CROSS_SELF =
//            -8, // would cross an offer from the same user
//        MANAGE_SELL_OFFER_SELL_NO_ISSUER = -9, // no issuer for what we're selling
//        MANAGE_SELL_OFFER_BUY_NO_ISSUER = -10, // no issuer for what we're buying
//
//        // update errors
//        MANAGE_SELL_OFFER_NOT_FOUND =
//            -11, // offerID does not match an existing offer
//
//        MANAGE_SELL_OFFER_LOW_RESERVE =
//            -12 // not enough funds to create a new Offer
//    };
//
// enum
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum ManageSellOfferResultCode {
    ManageSellOfferSuccess = 0,
    ManageSellOfferMalformed = -1,
    ManageSellOfferSellNoTrust = -2,
    ManageSellOfferBuyNoTrust = -3,
    ManageSellOfferSellNotAuthorized = -4,
    ManageSellOfferBuyNotAuthorized = -5,
    ManageSellOfferLineFull = -6,
    ManageSellOfferUnderfunded = -7,
    ManageSellOfferCrossSelf = -8,
    ManageSellOfferSellNoIssuer = -9,
    ManageSellOfferBuyNoIssuer = -10,
    ManageSellOfferNotFound = -11,
    ManageSellOfferLowReserve = -12,
}

impl TryFrom<i32> for ManageSellOfferResultCode {
    type Error = Error;

    fn try_from(i: i32) -> std::result::Result<Self, Self::Error> {
        let e = match i {
            0 => ManageSellOfferResultCode::ManageSellOfferSuccess,
            -1 => ManageSellOfferResultCode::ManageSellOfferMalformed,
            -2 => ManageSellOfferResultCode::ManageSellOfferSellNoTrust,
            -3 => ManageSellOfferResultCode::ManageSellOfferBuyNoTrust,
            -4 => ManageSellOfferResultCode::ManageSellOfferSellNotAuthorized,
            -5 => ManageSellOfferResultCode::ManageSellOfferBuyNotAuthorized,
            -6 => ManageSellOfferResultCode::ManageSellOfferLineFull,
            -7 => ManageSellOfferResultCode::ManageSellOfferUnderfunded,
            -8 => ManageSellOfferResultCode::ManageSellOfferCrossSelf,
            -9 => ManageSellOfferResultCode::ManageSellOfferSellNoIssuer,
            -10 => ManageSellOfferResultCode::ManageSellOfferBuyNoIssuer,
            -11 => ManageSellOfferResultCode::ManageSellOfferNotFound,
            -12 => ManageSellOfferResultCode::ManageSellOfferLowReserve,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<ManageSellOfferResultCode> for i32 {
    fn from(e: ManageSellOfferResultCode) -> Self {
        e as Self
    }
}

impl ReadXDR for ManageSellOfferResultCode {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXDR for ManageSellOfferResultCode {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// ManageOfferEffect is an XDR Enum defines as:
//
//   enum ManageOfferEffect
//    {
//        MANAGE_OFFER_CREATED = 0,
//        MANAGE_OFFER_UPDATED = 1,
//        MANAGE_OFFER_DELETED = 2
//    };
//
// enum
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum ManageOfferEffect {
    ManageOfferCreated = 0,
    ManageOfferUpdated = 1,
    ManageOfferDeleted = 2,
}

impl TryFrom<i32> for ManageOfferEffect {
    type Error = Error;

    fn try_from(i: i32) -> std::result::Result<Self, Self::Error> {
        let e = match i {
            0 => ManageOfferEffect::ManageOfferCreated,
            1 => ManageOfferEffect::ManageOfferUpdated,
            2 => ManageOfferEffect::ManageOfferDeleted,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<ManageOfferEffect> for i32 {
    fn from(e: ManageOfferEffect) -> Self {
        e as Self
    }
}

impl ReadXDR for ManageOfferEffect {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXDR for ManageOfferEffect {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// ManageOfferSuccessResultOffer is an XDR NestedUnion defines as:
//
//   union switch (ManageOfferEffect effect)
//        {
//        case MANAGE_OFFER_CREATED:
//        case MANAGE_OFFER_UPDATED:
//            OfferEntry offer;
//        default:
//            void;
//        }
//
// union with discriminant ManageOfferEffect
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ManageOfferSuccessResultOffer {
    ManageOfferCreated(OfferEntry),
    ManageOfferUpdated(OfferEntry),
}

impl ManageOfferSuccessResultOffer {
    fn discriminant(&self) -> ManageOfferEffect {
        match self {
            Self::ManageOfferCreated(_) => ManageOfferEffect::ManageOfferCreated,
            Self::ManageOfferUpdated(_) => ManageOfferEffect::ManageOfferUpdated,
        }
    }
}

impl ReadXDR for ManageOfferSuccessResultOffer {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: ManageOfferEffect = <ManageOfferEffect as ReadXDR>::read_xdr(r)?;
        let v = match dv {
            ManageOfferEffect::ManageOfferCreated => {
                Self::ManageOfferCreated(<OfferEntry as ReadXDR>::read_xdr(r)?)
            }
            ManageOfferEffect::ManageOfferUpdated => {
                Self::ManageOfferUpdated(<OfferEntry as ReadXDR>::read_xdr(r)?)
            }
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for ManageOfferSuccessResultOffer {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::ManageOfferCreated(v) => v.write_xdr(w)?,
            Self::ManageOfferUpdated(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// ManageOfferSuccessResult is an XDR Struct defines as:
//
//   struct ManageOfferSuccessResult
//    {
//        // offers that got claimed while creating this offer
//        ClaimAtom offersClaimed<>;
//
//        union switch (ManageOfferEffect effect)
//        {
//        case MANAGE_OFFER_CREATED:
//        case MANAGE_OFFER_UPDATED:
//            OfferEntry offer;
//        default:
//            void;
//        }
//        offer;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ManageOfferSuccessResult {
    pub offers_claimed: Vec<ClaimAtom>,
    pub offer: ManageOfferSuccessResultOffer,
}

impl ReadXDR for ManageOfferSuccessResult {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            offers_claimed: <Vec<ClaimAtom> as ReadXDR>::read_xdr(r)?,
            offer: <ManageOfferSuccessResultOffer as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for ManageOfferSuccessResult {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.offers_claimed.write_xdr(w)?;
        self.offer.write_xdr(w)?;
        Ok(())
    }
}

// ManageSellOfferResult is an XDR Union defines as:
//
//   union ManageSellOfferResult switch (ManageSellOfferResultCode code)
//    {
//    case MANAGE_SELL_OFFER_SUCCESS:
//        ManageOfferSuccessResult success;
//    default:
//        void;
//    };
//
// union with discriminant ManageSellOfferResultCode
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ManageSellOfferResult {
    ManageSellOfferSuccess(ManageOfferSuccessResult),
}

impl ManageSellOfferResult {
    fn discriminant(&self) -> ManageSellOfferResultCode {
        match self {
            Self::ManageSellOfferSuccess(_) => ManageSellOfferResultCode::ManageSellOfferSuccess,
        }
    }
}

impl ReadXDR for ManageSellOfferResult {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: ManageSellOfferResultCode = <ManageSellOfferResultCode as ReadXDR>::read_xdr(r)?;
        let v = match dv {
            ManageSellOfferResultCode::ManageSellOfferSuccess => {
                Self::ManageSellOfferSuccess(<ManageOfferSuccessResult as ReadXDR>::read_xdr(r)?)
            }
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for ManageSellOfferResult {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::ManageSellOfferSuccess(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// ManageBuyOfferResultCode is an XDR Enum defines as:
//
//   enum ManageBuyOfferResultCode
//    {
//        // codes considered as "success" for the operation
//        MANAGE_BUY_OFFER_SUCCESS = 0,
//
//        // codes considered as "failure" for the operation
//        MANAGE_BUY_OFFER_MALFORMED = -1,     // generated offer would be invalid
//        MANAGE_BUY_OFFER_SELL_NO_TRUST = -2, // no trust line for what we're selling
//        MANAGE_BUY_OFFER_BUY_NO_TRUST = -3,  // no trust line for what we're buying
//        MANAGE_BUY_OFFER_SELL_NOT_AUTHORIZED = -4, // not authorized to sell
//        MANAGE_BUY_OFFER_BUY_NOT_AUTHORIZED = -5,  // not authorized to buy
//        MANAGE_BUY_OFFER_LINE_FULL = -6,   // can't receive more of what it's buying
//        MANAGE_BUY_OFFER_UNDERFUNDED = -7, // doesn't hold what it's trying to sell
//        MANAGE_BUY_OFFER_CROSS_SELF = -8, // would cross an offer from the same user
//        MANAGE_BUY_OFFER_SELL_NO_ISSUER = -9, // no issuer for what we're selling
//        MANAGE_BUY_OFFER_BUY_NO_ISSUER = -10, // no issuer for what we're buying
//
//        // update errors
//        MANAGE_BUY_OFFER_NOT_FOUND =
//            -11, // offerID does not match an existing offer
//
//        MANAGE_BUY_OFFER_LOW_RESERVE = -12 // not enough funds to create a new Offer
//    };
//
// enum
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum ManageBuyOfferResultCode {
    ManageBuyOfferSuccess = 0,
    ManageBuyOfferMalformed = -1,
    ManageBuyOfferSellNoTrust = -2,
    ManageBuyOfferBuyNoTrust = -3,
    ManageBuyOfferSellNotAuthorized = -4,
    ManageBuyOfferBuyNotAuthorized = -5,
    ManageBuyOfferLineFull = -6,
    ManageBuyOfferUnderfunded = -7,
    ManageBuyOfferCrossSelf = -8,
    ManageBuyOfferSellNoIssuer = -9,
    ManageBuyOfferBuyNoIssuer = -10,
    ManageBuyOfferNotFound = -11,
    ManageBuyOfferLowReserve = -12,
}

impl TryFrom<i32> for ManageBuyOfferResultCode {
    type Error = Error;

    fn try_from(i: i32) -> std::result::Result<Self, Self::Error> {
        let e = match i {
            0 => ManageBuyOfferResultCode::ManageBuyOfferSuccess,
            -1 => ManageBuyOfferResultCode::ManageBuyOfferMalformed,
            -2 => ManageBuyOfferResultCode::ManageBuyOfferSellNoTrust,
            -3 => ManageBuyOfferResultCode::ManageBuyOfferBuyNoTrust,
            -4 => ManageBuyOfferResultCode::ManageBuyOfferSellNotAuthorized,
            -5 => ManageBuyOfferResultCode::ManageBuyOfferBuyNotAuthorized,
            -6 => ManageBuyOfferResultCode::ManageBuyOfferLineFull,
            -7 => ManageBuyOfferResultCode::ManageBuyOfferUnderfunded,
            -8 => ManageBuyOfferResultCode::ManageBuyOfferCrossSelf,
            -9 => ManageBuyOfferResultCode::ManageBuyOfferSellNoIssuer,
            -10 => ManageBuyOfferResultCode::ManageBuyOfferBuyNoIssuer,
            -11 => ManageBuyOfferResultCode::ManageBuyOfferNotFound,
            -12 => ManageBuyOfferResultCode::ManageBuyOfferLowReserve,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<ManageBuyOfferResultCode> for i32 {
    fn from(e: ManageBuyOfferResultCode) -> Self {
        e as Self
    }
}

impl ReadXDR for ManageBuyOfferResultCode {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXDR for ManageBuyOfferResultCode {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// ManageBuyOfferResult is an XDR Union defines as:
//
//   union ManageBuyOfferResult switch (ManageBuyOfferResultCode code)
//    {
//    case MANAGE_BUY_OFFER_SUCCESS:
//        ManageOfferSuccessResult success;
//    default:
//        void;
//    };
//
// union with discriminant ManageBuyOfferResultCode
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ManageBuyOfferResult {
    ManageBuyOfferSuccess(ManageOfferSuccessResult),
}

impl ManageBuyOfferResult {
    fn discriminant(&self) -> ManageBuyOfferResultCode {
        match self {
            Self::ManageBuyOfferSuccess(_) => ManageBuyOfferResultCode::ManageBuyOfferSuccess,
        }
    }
}

impl ReadXDR for ManageBuyOfferResult {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: ManageBuyOfferResultCode = <ManageBuyOfferResultCode as ReadXDR>::read_xdr(r)?;
        let v = match dv {
            ManageBuyOfferResultCode::ManageBuyOfferSuccess => {
                Self::ManageBuyOfferSuccess(<ManageOfferSuccessResult as ReadXDR>::read_xdr(r)?)
            }
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for ManageBuyOfferResult {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::ManageBuyOfferSuccess(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// SetOptionsResultCode is an XDR Enum defines as:
//
//   enum SetOptionsResultCode
//    {
//        // codes considered as "success" for the operation
//        SET_OPTIONS_SUCCESS = 0,
//        // codes considered as "failure" for the operation
//        SET_OPTIONS_LOW_RESERVE = -1,      // not enough funds to add a signer
//        SET_OPTIONS_TOO_MANY_SIGNERS = -2, // max number of signers already reached
//        SET_OPTIONS_BAD_FLAGS = -3,        // invalid combination of clear/set flags
//        SET_OPTIONS_INVALID_INFLATION = -4,      // inflation account does not exist
//        SET_OPTIONS_CANT_CHANGE = -5,            // can no longer change this option
//        SET_OPTIONS_UNKNOWN_FLAG = -6,           // can't set an unknown flag
//        SET_OPTIONS_THRESHOLD_OUT_OF_RANGE = -7, // bad value for weight/threshold
//        SET_OPTIONS_BAD_SIGNER = -8,             // signer cannot be masterkey
//        SET_OPTIONS_INVALID_HOME_DOMAIN = -9,    // malformed home domain
//        SET_OPTIONS_AUTH_REVOCABLE_REQUIRED =
//            -10 // auth revocable is required for clawback
//    };
//
// enum
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum SetOptionsResultCode {
    SetOptionsSuccess = 0,
    SetOptionsLowReserve = -1,
    SetOptionsTooManySigners = -2,
    SetOptionsBadFlags = -3,
    SetOptionsInvalidInflation = -4,
    SetOptionsCantChange = -5,
    SetOptionsUnknownFlag = -6,
    SetOptionsThresholdOutOfRange = -7,
    SetOptionsBadSigner = -8,
    SetOptionsInvalidHomeDomain = -9,
    SetOptionsAuthRevocableRequired = -10,
}

impl TryFrom<i32> for SetOptionsResultCode {
    type Error = Error;

    fn try_from(i: i32) -> std::result::Result<Self, Self::Error> {
        let e = match i {
            0 => SetOptionsResultCode::SetOptionsSuccess,
            -1 => SetOptionsResultCode::SetOptionsLowReserve,
            -2 => SetOptionsResultCode::SetOptionsTooManySigners,
            -3 => SetOptionsResultCode::SetOptionsBadFlags,
            -4 => SetOptionsResultCode::SetOptionsInvalidInflation,
            -5 => SetOptionsResultCode::SetOptionsCantChange,
            -6 => SetOptionsResultCode::SetOptionsUnknownFlag,
            -7 => SetOptionsResultCode::SetOptionsThresholdOutOfRange,
            -8 => SetOptionsResultCode::SetOptionsBadSigner,
            -9 => SetOptionsResultCode::SetOptionsInvalidHomeDomain,
            -10 => SetOptionsResultCode::SetOptionsAuthRevocableRequired,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<SetOptionsResultCode> for i32 {
    fn from(e: SetOptionsResultCode) -> Self {
        e as Self
    }
}

impl ReadXDR for SetOptionsResultCode {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXDR for SetOptionsResultCode {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// SetOptionsResult is an XDR Union defines as:
//
//   union SetOptionsResult switch (SetOptionsResultCode code)
//    {
//    case SET_OPTIONS_SUCCESS:
//        void;
//    default:
//        void;
//    };
//
// union with discriminant SetOptionsResultCode
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SetOptionsResult {
    SetOptionsSuccess,
}

impl SetOptionsResult {
    fn discriminant(&self) -> SetOptionsResultCode {
        match self {
            Self::SetOptionsSuccess => SetOptionsResultCode::SetOptionsSuccess,
        }
    }
}

impl ReadXDR for SetOptionsResult {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: SetOptionsResultCode = <SetOptionsResultCode as ReadXDR>::read_xdr(r)?;
        let v = match dv {
            SetOptionsResultCode::SetOptionsSuccess => Self::SetOptionsSuccess,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for SetOptionsResult {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::SetOptionsSuccess => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// ChangeTrustResultCode is an XDR Enum defines as:
//
//   enum ChangeTrustResultCode
//    {
//        // codes considered as "success" for the operation
//        CHANGE_TRUST_SUCCESS = 0,
//        // codes considered as "failure" for the operation
//        CHANGE_TRUST_MALFORMED = -1,     // bad input
//        CHANGE_TRUST_NO_ISSUER = -2,     // could not find issuer
//        CHANGE_TRUST_INVALID_LIMIT = -3, // cannot drop limit below balance
//                                         // cannot create with a limit of 0
//        CHANGE_TRUST_LOW_RESERVE =
//            -4, // not enough funds to create a new trust line,
//        CHANGE_TRUST_SELF_NOT_ALLOWED = -5,   // trusting self is not allowed
//        CHANGE_TRUST_TRUST_LINE_MISSING = -6, // Asset trustline is missing for pool
//        CHANGE_TRUST_CANNOT_DELETE =
//            -7, // Asset trustline is still referenced in a pool
//        CHANGE_TRUST_NOT_AUTH_MAINTAIN_LIABILITIES =
//            -8 // Asset trustline is deauthorized
//    };
//
// enum
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum ChangeTrustResultCode {
    ChangeTrustSuccess = 0,
    ChangeTrustMalformed = -1,
    ChangeTrustNoIssuer = -2,
    ChangeTrustInvalidLimit = -3,
    ChangeTrustLowReserve = -4,
    ChangeTrustSelfNotAllowed = -5,
    ChangeTrustTrustLineMissing = -6,
    ChangeTrustCannotDelete = -7,
    ChangeTrustNotAuthMaintainLiabilities = -8,
}

impl TryFrom<i32> for ChangeTrustResultCode {
    type Error = Error;

    fn try_from(i: i32) -> std::result::Result<Self, Self::Error> {
        let e = match i {
            0 => ChangeTrustResultCode::ChangeTrustSuccess,
            -1 => ChangeTrustResultCode::ChangeTrustMalformed,
            -2 => ChangeTrustResultCode::ChangeTrustNoIssuer,
            -3 => ChangeTrustResultCode::ChangeTrustInvalidLimit,
            -4 => ChangeTrustResultCode::ChangeTrustLowReserve,
            -5 => ChangeTrustResultCode::ChangeTrustSelfNotAllowed,
            -6 => ChangeTrustResultCode::ChangeTrustTrustLineMissing,
            -7 => ChangeTrustResultCode::ChangeTrustCannotDelete,
            -8 => ChangeTrustResultCode::ChangeTrustNotAuthMaintainLiabilities,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<ChangeTrustResultCode> for i32 {
    fn from(e: ChangeTrustResultCode) -> Self {
        e as Self
    }
}

impl ReadXDR for ChangeTrustResultCode {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXDR for ChangeTrustResultCode {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// ChangeTrustResult is an XDR Union defines as:
//
//   union ChangeTrustResult switch (ChangeTrustResultCode code)
//    {
//    case CHANGE_TRUST_SUCCESS:
//        void;
//    default:
//        void;
//    };
//
// union with discriminant ChangeTrustResultCode
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ChangeTrustResult {
    ChangeTrustSuccess,
}

impl ChangeTrustResult {
    fn discriminant(&self) -> ChangeTrustResultCode {
        match self {
            Self::ChangeTrustSuccess => ChangeTrustResultCode::ChangeTrustSuccess,
        }
    }
}

impl ReadXDR for ChangeTrustResult {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: ChangeTrustResultCode = <ChangeTrustResultCode as ReadXDR>::read_xdr(r)?;
        let v = match dv {
            ChangeTrustResultCode::ChangeTrustSuccess => Self::ChangeTrustSuccess,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for ChangeTrustResult {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::ChangeTrustSuccess => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// AllowTrustResultCode is an XDR Enum defines as:
//
//   enum AllowTrustResultCode
//    {
//        // codes considered as "success" for the operation
//        ALLOW_TRUST_SUCCESS = 0,
//        // codes considered as "failure" for the operation
//        ALLOW_TRUST_MALFORMED = -1,     // asset is not ASSET_TYPE_ALPHANUM
//        ALLOW_TRUST_NO_TRUST_LINE = -2, // trustor does not have a trustline
//                                        // source account does not require trust
//        ALLOW_TRUST_TRUST_NOT_REQUIRED = -3,
//        ALLOW_TRUST_CANT_REVOKE = -4,      // source account can't revoke trust,
//        ALLOW_TRUST_SELF_NOT_ALLOWED = -5, // trusting self is not allowed
//        ALLOW_TRUST_LOW_RESERVE = -6       // claimable balances can't be created
//                                           // on revoke due to low reserves
//    };
//
// enum
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum AllowTrustResultCode {
    AllowTrustSuccess = 0,
    AllowTrustMalformed = -1,
    AllowTrustNoTrustLine = -2,
    AllowTrustTrustNotRequired = -3,
    AllowTrustCantRevoke = -4,
    AllowTrustSelfNotAllowed = -5,
    AllowTrustLowReserve = -6,
}

impl TryFrom<i32> for AllowTrustResultCode {
    type Error = Error;

    fn try_from(i: i32) -> std::result::Result<Self, Self::Error> {
        let e = match i {
            0 => AllowTrustResultCode::AllowTrustSuccess,
            -1 => AllowTrustResultCode::AllowTrustMalformed,
            -2 => AllowTrustResultCode::AllowTrustNoTrustLine,
            -3 => AllowTrustResultCode::AllowTrustTrustNotRequired,
            -4 => AllowTrustResultCode::AllowTrustCantRevoke,
            -5 => AllowTrustResultCode::AllowTrustSelfNotAllowed,
            -6 => AllowTrustResultCode::AllowTrustLowReserve,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<AllowTrustResultCode> for i32 {
    fn from(e: AllowTrustResultCode) -> Self {
        e as Self
    }
}

impl ReadXDR for AllowTrustResultCode {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXDR for AllowTrustResultCode {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// AllowTrustResult is an XDR Union defines as:
//
//   union AllowTrustResult switch (AllowTrustResultCode code)
//    {
//    case ALLOW_TRUST_SUCCESS:
//        void;
//    default:
//        void;
//    };
//
// union with discriminant AllowTrustResultCode
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AllowTrustResult {
    AllowTrustSuccess,
}

impl AllowTrustResult {
    fn discriminant(&self) -> AllowTrustResultCode {
        match self {
            Self::AllowTrustSuccess => AllowTrustResultCode::AllowTrustSuccess,
        }
    }
}

impl ReadXDR for AllowTrustResult {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: AllowTrustResultCode = <AllowTrustResultCode as ReadXDR>::read_xdr(r)?;
        let v = match dv {
            AllowTrustResultCode::AllowTrustSuccess => Self::AllowTrustSuccess,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for AllowTrustResult {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::AllowTrustSuccess => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// AccountMergeResultCode is an XDR Enum defines as:
//
//   enum AccountMergeResultCode
//    {
//        // codes considered as "success" for the operation
//        ACCOUNT_MERGE_SUCCESS = 0,
//        // codes considered as "failure" for the operation
//        ACCOUNT_MERGE_MALFORMED = -1,       // can't merge onto itself
//        ACCOUNT_MERGE_NO_ACCOUNT = -2,      // destination does not exist
//        ACCOUNT_MERGE_IMMUTABLE_SET = -3,   // source account has AUTH_IMMUTABLE set
//        ACCOUNT_MERGE_HAS_SUB_ENTRIES = -4, // account has trust lines/offers
//        ACCOUNT_MERGE_SEQNUM_TOO_FAR = -5,  // sequence number is over max allowed
//        ACCOUNT_MERGE_DEST_FULL = -6,       // can't add source balance to
//                                            // destination balance
//        ACCOUNT_MERGE_IS_SPONSOR = -7       // can't merge account that is a sponsor
//    };
//
// enum
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum AccountMergeResultCode {
    AccountMergeSuccess = 0,
    AccountMergeMalformed = -1,
    AccountMergeNoAccount = -2,
    AccountMergeImmutableSet = -3,
    AccountMergeHasSubEntries = -4,
    AccountMergeSeqnumTooFar = -5,
    AccountMergeDestFull = -6,
    AccountMergeIsSponsor = -7,
}

impl TryFrom<i32> for AccountMergeResultCode {
    type Error = Error;

    fn try_from(i: i32) -> std::result::Result<Self, Self::Error> {
        let e = match i {
            0 => AccountMergeResultCode::AccountMergeSuccess,
            -1 => AccountMergeResultCode::AccountMergeMalformed,
            -2 => AccountMergeResultCode::AccountMergeNoAccount,
            -3 => AccountMergeResultCode::AccountMergeImmutableSet,
            -4 => AccountMergeResultCode::AccountMergeHasSubEntries,
            -5 => AccountMergeResultCode::AccountMergeSeqnumTooFar,
            -6 => AccountMergeResultCode::AccountMergeDestFull,
            -7 => AccountMergeResultCode::AccountMergeIsSponsor,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<AccountMergeResultCode> for i32 {
    fn from(e: AccountMergeResultCode) -> Self {
        e as Self
    }
}

impl ReadXDR for AccountMergeResultCode {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXDR for AccountMergeResultCode {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// AccountMergeResult is an XDR Union defines as:
//
//   union AccountMergeResult switch (AccountMergeResultCode code)
//    {
//    case ACCOUNT_MERGE_SUCCESS:
//        int64 sourceAccountBalance; // how much got transferred from source account
//    default:
//        void;
//    };
//
// union with discriminant AccountMergeResultCode
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AccountMergeResult {
    AccountMergeSuccess(Int64),
}

impl AccountMergeResult {
    fn discriminant(&self) -> AccountMergeResultCode {
        match self {
            Self::AccountMergeSuccess(_) => AccountMergeResultCode::AccountMergeSuccess,
        }
    }
}

impl ReadXDR for AccountMergeResult {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: AccountMergeResultCode = <AccountMergeResultCode as ReadXDR>::read_xdr(r)?;
        let v = match dv {
            AccountMergeResultCode::AccountMergeSuccess => {
                Self::AccountMergeSuccess(<Int64 as ReadXDR>::read_xdr(r)?)
            }
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for AccountMergeResult {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::AccountMergeSuccess(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// InflationResultCode is an XDR Enum defines as:
//
//   enum InflationResultCode
//    {
//        // codes considered as "success" for the operation
//        INFLATION_SUCCESS = 0,
//        // codes considered as "failure" for the operation
//        INFLATION_NOT_TIME = -1
//    };
//
// enum
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum InflationResultCode {
    InflationSuccess = 0,
    InflationNotTime = -1,
}

impl TryFrom<i32> for InflationResultCode {
    type Error = Error;

    fn try_from(i: i32) -> std::result::Result<Self, Self::Error> {
        let e = match i {
            0 => InflationResultCode::InflationSuccess,
            -1 => InflationResultCode::InflationNotTime,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<InflationResultCode> for i32 {
    fn from(e: InflationResultCode) -> Self {
        e as Self
    }
}

impl ReadXDR for InflationResultCode {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXDR for InflationResultCode {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// InflationPayout is an XDR Struct defines as:
//
//   struct InflationPayout // or use PaymentResultAtom to limit types?
//    {
//        AccountID destination;
//        int64 amount;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InflationPayout {
    pub destination: AccountId,
    pub amount: Int64,
}

impl ReadXDR for InflationPayout {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            destination: <AccountId as ReadXDR>::read_xdr(r)?,
            amount: <Int64 as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for InflationPayout {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.destination.write_xdr(w)?;
        self.amount.write_xdr(w)?;
        Ok(())
    }
}

// InflationResult is an XDR Union defines as:
//
//   union InflationResult switch (InflationResultCode code)
//    {
//    case INFLATION_SUCCESS:
//        InflationPayout payouts<>;
//    default:
//        void;
//    };
//
// union with discriminant InflationResultCode
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InflationResult {
    InflationSuccess(Vec<InflationPayout>),
}

impl InflationResult {
    fn discriminant(&self) -> InflationResultCode {
        match self {
            Self::InflationSuccess(_) => InflationResultCode::InflationSuccess,
        }
    }
}

impl ReadXDR for InflationResult {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: InflationResultCode = <InflationResultCode as ReadXDR>::read_xdr(r)?;
        let v = match dv {
            InflationResultCode::InflationSuccess => {
                Self::InflationSuccess(<Vec<InflationPayout> as ReadXDR>::read_xdr(r)?)
            }
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for InflationResult {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::InflationSuccess(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// ManageDataResultCode is an XDR Enum defines as:
//
//   enum ManageDataResultCode
//    {
//        // codes considered as "success" for the operation
//        MANAGE_DATA_SUCCESS = 0,
//        // codes considered as "failure" for the operation
//        MANAGE_DATA_NOT_SUPPORTED_YET =
//            -1, // The network hasn't moved to this protocol change yet
//        MANAGE_DATA_NAME_NOT_FOUND =
//            -2, // Trying to remove a Data Entry that isn't there
//        MANAGE_DATA_LOW_RESERVE = -3, // not enough funds to create a new Data Entry
//        MANAGE_DATA_INVALID_NAME = -4 // Name not a valid string
//    };
//
// enum
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum ManageDataResultCode {
    ManageDataSuccess = 0,
    ManageDataNotSupportedYet = -1,
    ManageDataNameNotFound = -2,
    ManageDataLowReserve = -3,
    ManageDataInvalidName = -4,
}

impl TryFrom<i32> for ManageDataResultCode {
    type Error = Error;

    fn try_from(i: i32) -> std::result::Result<Self, Self::Error> {
        let e = match i {
            0 => ManageDataResultCode::ManageDataSuccess,
            -1 => ManageDataResultCode::ManageDataNotSupportedYet,
            -2 => ManageDataResultCode::ManageDataNameNotFound,
            -3 => ManageDataResultCode::ManageDataLowReserve,
            -4 => ManageDataResultCode::ManageDataInvalidName,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<ManageDataResultCode> for i32 {
    fn from(e: ManageDataResultCode) -> Self {
        e as Self
    }
}

impl ReadXDR for ManageDataResultCode {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXDR for ManageDataResultCode {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// ManageDataResult is an XDR Union defines as:
//
//   union ManageDataResult switch (ManageDataResultCode code)
//    {
//    case MANAGE_DATA_SUCCESS:
//        void;
//    default:
//        void;
//    };
//
// union with discriminant ManageDataResultCode
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ManageDataResult {
    ManageDataSuccess,
}

impl ManageDataResult {
    fn discriminant(&self) -> ManageDataResultCode {
        match self {
            Self::ManageDataSuccess => ManageDataResultCode::ManageDataSuccess,
        }
    }
}

impl ReadXDR for ManageDataResult {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: ManageDataResultCode = <ManageDataResultCode as ReadXDR>::read_xdr(r)?;
        let v = match dv {
            ManageDataResultCode::ManageDataSuccess => Self::ManageDataSuccess,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for ManageDataResult {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::ManageDataSuccess => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// BumpSequenceResultCode is an XDR Enum defines as:
//
//   enum BumpSequenceResultCode
//    {
//        // codes considered as "success" for the operation
//        BUMP_SEQUENCE_SUCCESS = 0,
//        // codes considered as "failure" for the operation
//        BUMP_SEQUENCE_BAD_SEQ = -1 // `bumpTo` is not within bounds
//    };
//
// enum
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum BumpSequenceResultCode {
    BumpSequenceSuccess = 0,
    BumpSequenceBadSeq = -1,
}

impl TryFrom<i32> for BumpSequenceResultCode {
    type Error = Error;

    fn try_from(i: i32) -> std::result::Result<Self, Self::Error> {
        let e = match i {
            0 => BumpSequenceResultCode::BumpSequenceSuccess,
            -1 => BumpSequenceResultCode::BumpSequenceBadSeq,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<BumpSequenceResultCode> for i32 {
    fn from(e: BumpSequenceResultCode) -> Self {
        e as Self
    }
}

impl ReadXDR for BumpSequenceResultCode {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXDR for BumpSequenceResultCode {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// BumpSequenceResult is an XDR Union defines as:
//
//   union BumpSequenceResult switch (BumpSequenceResultCode code)
//    {
//    case BUMP_SEQUENCE_SUCCESS:
//        void;
//    default:
//        void;
//    };
//
// union with discriminant BumpSequenceResultCode
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BumpSequenceResult {
    BumpSequenceSuccess,
}

impl BumpSequenceResult {
    fn discriminant(&self) -> BumpSequenceResultCode {
        match self {
            Self::BumpSequenceSuccess => BumpSequenceResultCode::BumpSequenceSuccess,
        }
    }
}

impl ReadXDR for BumpSequenceResult {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: BumpSequenceResultCode = <BumpSequenceResultCode as ReadXDR>::read_xdr(r)?;
        let v = match dv {
            BumpSequenceResultCode::BumpSequenceSuccess => Self::BumpSequenceSuccess,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for BumpSequenceResult {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::BumpSequenceSuccess => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// CreateClaimableBalanceResultCode is an XDR Enum defines as:
//
//   enum CreateClaimableBalanceResultCode
//    {
//        CREATE_CLAIMABLE_BALANCE_SUCCESS = 0,
//        CREATE_CLAIMABLE_BALANCE_MALFORMED = -1,
//        CREATE_CLAIMABLE_BALANCE_LOW_RESERVE = -2,
//        CREATE_CLAIMABLE_BALANCE_NO_TRUST = -3,
//        CREATE_CLAIMABLE_BALANCE_NOT_AUTHORIZED = -4,
//        CREATE_CLAIMABLE_BALANCE_UNDERFUNDED = -5
//    };
//
// enum
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum CreateClaimableBalanceResultCode {
    CreateClaimableBalanceSuccess = 0,
    CreateClaimableBalanceMalformed = -1,
    CreateClaimableBalanceLowReserve = -2,
    CreateClaimableBalanceNoTrust = -3,
    CreateClaimableBalanceNotAuthorized = -4,
    CreateClaimableBalanceUnderfunded = -5,
}

impl TryFrom<i32> for CreateClaimableBalanceResultCode {
    type Error = Error;

    fn try_from(i: i32) -> std::result::Result<Self, Self::Error> {
        let e = match i {
            0 => CreateClaimableBalanceResultCode::CreateClaimableBalanceSuccess,
            -1 => CreateClaimableBalanceResultCode::CreateClaimableBalanceMalformed,
            -2 => CreateClaimableBalanceResultCode::CreateClaimableBalanceLowReserve,
            -3 => CreateClaimableBalanceResultCode::CreateClaimableBalanceNoTrust,
            -4 => CreateClaimableBalanceResultCode::CreateClaimableBalanceNotAuthorized,
            -5 => CreateClaimableBalanceResultCode::CreateClaimableBalanceUnderfunded,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<CreateClaimableBalanceResultCode> for i32 {
    fn from(e: CreateClaimableBalanceResultCode) -> Self {
        e as Self
    }
}

impl ReadXDR for CreateClaimableBalanceResultCode {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXDR for CreateClaimableBalanceResultCode {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// CreateClaimableBalanceResult is an XDR Union defines as:
//
//   union CreateClaimableBalanceResult switch (
//        CreateClaimableBalanceResultCode code)
//    {
//    case CREATE_CLAIMABLE_BALANCE_SUCCESS:
//        ClaimableBalanceID balanceID;
//    default:
//        void;
//    };
//
// union with discriminant CreateClaimableBalanceResultCode
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CreateClaimableBalanceResult {
    CreateClaimableBalanceSuccess(ClaimableBalanceId),
}

impl CreateClaimableBalanceResult {
    fn discriminant(&self) -> CreateClaimableBalanceResultCode {
        match self {
            Self::CreateClaimableBalanceSuccess(_) => {
                CreateClaimableBalanceResultCode::CreateClaimableBalanceSuccess
            }
        }
    }
}

impl ReadXDR for CreateClaimableBalanceResult {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: CreateClaimableBalanceResultCode =
            <CreateClaimableBalanceResultCode as ReadXDR>::read_xdr(r)?;
        let v = match dv {
            CreateClaimableBalanceResultCode::CreateClaimableBalanceSuccess => {
                Self::CreateClaimableBalanceSuccess(<ClaimableBalanceId as ReadXDR>::read_xdr(r)?)
            }
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for CreateClaimableBalanceResult {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::CreateClaimableBalanceSuccess(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// ClaimClaimableBalanceResultCode is an XDR Enum defines as:
//
//   enum ClaimClaimableBalanceResultCode
//    {
//        CLAIM_CLAIMABLE_BALANCE_SUCCESS = 0,
//        CLAIM_CLAIMABLE_BALANCE_DOES_NOT_EXIST = -1,
//        CLAIM_CLAIMABLE_BALANCE_CANNOT_CLAIM = -2,
//        CLAIM_CLAIMABLE_BALANCE_LINE_FULL = -3,
//        CLAIM_CLAIMABLE_BALANCE_NO_TRUST = -4,
//        CLAIM_CLAIMABLE_BALANCE_NOT_AUTHORIZED = -5
//
//    };
//
// enum
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum ClaimClaimableBalanceResultCode {
    ClaimClaimableBalanceSuccess = 0,
    ClaimClaimableBalanceDoesNotExist = -1,
    ClaimClaimableBalanceCannotClaim = -2,
    ClaimClaimableBalanceLineFull = -3,
    ClaimClaimableBalanceNoTrust = -4,
    ClaimClaimableBalanceNotAuthorized = -5,
}

impl TryFrom<i32> for ClaimClaimableBalanceResultCode {
    type Error = Error;

    fn try_from(i: i32) -> std::result::Result<Self, Self::Error> {
        let e = match i {
            0 => ClaimClaimableBalanceResultCode::ClaimClaimableBalanceSuccess,
            -1 => ClaimClaimableBalanceResultCode::ClaimClaimableBalanceDoesNotExist,
            -2 => ClaimClaimableBalanceResultCode::ClaimClaimableBalanceCannotClaim,
            -3 => ClaimClaimableBalanceResultCode::ClaimClaimableBalanceLineFull,
            -4 => ClaimClaimableBalanceResultCode::ClaimClaimableBalanceNoTrust,
            -5 => ClaimClaimableBalanceResultCode::ClaimClaimableBalanceNotAuthorized,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<ClaimClaimableBalanceResultCode> for i32 {
    fn from(e: ClaimClaimableBalanceResultCode) -> Self {
        e as Self
    }
}

impl ReadXDR for ClaimClaimableBalanceResultCode {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXDR for ClaimClaimableBalanceResultCode {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// ClaimClaimableBalanceResult is an XDR Union defines as:
//
//   union ClaimClaimableBalanceResult switch (ClaimClaimableBalanceResultCode code)
//    {
//    case CLAIM_CLAIMABLE_BALANCE_SUCCESS:
//        void;
//    default:
//        void;
//    };
//
// union with discriminant ClaimClaimableBalanceResultCode
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ClaimClaimableBalanceResult {
    ClaimClaimableBalanceSuccess,
}

impl ClaimClaimableBalanceResult {
    fn discriminant(&self) -> ClaimClaimableBalanceResultCode {
        match self {
            Self::ClaimClaimableBalanceSuccess => {
                ClaimClaimableBalanceResultCode::ClaimClaimableBalanceSuccess
            }
        }
    }
}

impl ReadXDR for ClaimClaimableBalanceResult {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: ClaimClaimableBalanceResultCode =
            <ClaimClaimableBalanceResultCode as ReadXDR>::read_xdr(r)?;
        let v = match dv {
            ClaimClaimableBalanceResultCode::ClaimClaimableBalanceSuccess => {
                Self::ClaimClaimableBalanceSuccess
            }
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for ClaimClaimableBalanceResult {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::ClaimClaimableBalanceSuccess => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// BeginSponsoringFutureReservesResultCode is an XDR Enum defines as:
//
//   enum BeginSponsoringFutureReservesResultCode
//    {
//        // codes considered as "success" for the operation
//        BEGIN_SPONSORING_FUTURE_RESERVES_SUCCESS = 0,
//
//        // codes considered as "failure" for the operation
//        BEGIN_SPONSORING_FUTURE_RESERVES_MALFORMED = -1,
//        BEGIN_SPONSORING_FUTURE_RESERVES_ALREADY_SPONSORED = -2,
//        BEGIN_SPONSORING_FUTURE_RESERVES_RECURSIVE = -3
//    };
//
// enum
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum BeginSponsoringFutureReservesResultCode {
    BeginSponsoringFutureReservesSuccess = 0,
    BeginSponsoringFutureReservesMalformed = -1,
    BeginSponsoringFutureReservesAlreadySponsored = -2,
    BeginSponsoringFutureReservesRecursive = -3,
}

impl TryFrom<i32> for BeginSponsoringFutureReservesResultCode {
    type Error = Error;

    fn try_from(i: i32) -> std::result::Result<Self, Self::Error> {
        let e = match i {
                    0 => BeginSponsoringFutureReservesResultCode::BeginSponsoringFutureReservesSuccess,
-1 => BeginSponsoringFutureReservesResultCode::BeginSponsoringFutureReservesMalformed,
-2 => BeginSponsoringFutureReservesResultCode::BeginSponsoringFutureReservesAlreadySponsored,
-3 => BeginSponsoringFutureReservesResultCode::BeginSponsoringFutureReservesRecursive,
                    #[allow(unreachable_patterns)]
                    _ => return Err(Error::Invalid),
                };
        Ok(e)
    }
}

impl From<BeginSponsoringFutureReservesResultCode> for i32 {
    fn from(e: BeginSponsoringFutureReservesResultCode) -> Self {
        e as Self
    }
}

impl ReadXDR for BeginSponsoringFutureReservesResultCode {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXDR for BeginSponsoringFutureReservesResultCode {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// BeginSponsoringFutureReservesResult is an XDR Union defines as:
//
//   union BeginSponsoringFutureReservesResult switch (
//        BeginSponsoringFutureReservesResultCode code)
//    {
//    case BEGIN_SPONSORING_FUTURE_RESERVES_SUCCESS:
//        void;
//    default:
//        void;
//    };
//
// union with discriminant BeginSponsoringFutureReservesResultCode
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BeginSponsoringFutureReservesResult {
    BeginSponsoringFutureReservesSuccess,
}

impl BeginSponsoringFutureReservesResult {
    fn discriminant(&self) -> BeginSponsoringFutureReservesResultCode {
        match self {
            Self::BeginSponsoringFutureReservesSuccess => {
                BeginSponsoringFutureReservesResultCode::BeginSponsoringFutureReservesSuccess
            }
        }
    }
}

impl ReadXDR for BeginSponsoringFutureReservesResult {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: BeginSponsoringFutureReservesResultCode =
            <BeginSponsoringFutureReservesResultCode as ReadXDR>::read_xdr(r)?;
        let v = match dv {
            BeginSponsoringFutureReservesResultCode::BeginSponsoringFutureReservesSuccess => {
                Self::BeginSponsoringFutureReservesSuccess
            }
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for BeginSponsoringFutureReservesResult {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::BeginSponsoringFutureReservesSuccess => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// EndSponsoringFutureReservesResultCode is an XDR Enum defines as:
//
//   enum EndSponsoringFutureReservesResultCode
//    {
//        // codes considered as "success" for the operation
//        END_SPONSORING_FUTURE_RESERVES_SUCCESS = 0,
//
//        // codes considered as "failure" for the operation
//        END_SPONSORING_FUTURE_RESERVES_NOT_SPONSORED = -1
//    };
//
// enum
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum EndSponsoringFutureReservesResultCode {
    EndSponsoringFutureReservesSuccess = 0,
    EndSponsoringFutureReservesNotSponsored = -1,
}

impl TryFrom<i32> for EndSponsoringFutureReservesResultCode {
    type Error = Error;

    fn try_from(i: i32) -> std::result::Result<Self, Self::Error> {
        let e = match i {
            0 => EndSponsoringFutureReservesResultCode::EndSponsoringFutureReservesSuccess,
            -1 => EndSponsoringFutureReservesResultCode::EndSponsoringFutureReservesNotSponsored,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<EndSponsoringFutureReservesResultCode> for i32 {
    fn from(e: EndSponsoringFutureReservesResultCode) -> Self {
        e as Self
    }
}

impl ReadXDR for EndSponsoringFutureReservesResultCode {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXDR for EndSponsoringFutureReservesResultCode {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// EndSponsoringFutureReservesResult is an XDR Union defines as:
//
//   union EndSponsoringFutureReservesResult switch (
//        EndSponsoringFutureReservesResultCode code)
//    {
//    case END_SPONSORING_FUTURE_RESERVES_SUCCESS:
//        void;
//    default:
//        void;
//    };
//
// union with discriminant EndSponsoringFutureReservesResultCode
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EndSponsoringFutureReservesResult {
    EndSponsoringFutureReservesSuccess,
}

impl EndSponsoringFutureReservesResult {
    fn discriminant(&self) -> EndSponsoringFutureReservesResultCode {
        match self {
            Self::EndSponsoringFutureReservesSuccess => {
                EndSponsoringFutureReservesResultCode::EndSponsoringFutureReservesSuccess
            }
        }
    }
}

impl ReadXDR for EndSponsoringFutureReservesResult {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: EndSponsoringFutureReservesResultCode =
            <EndSponsoringFutureReservesResultCode as ReadXDR>::read_xdr(r)?;
        let v = match dv {
            EndSponsoringFutureReservesResultCode::EndSponsoringFutureReservesSuccess => {
                Self::EndSponsoringFutureReservesSuccess
            }
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for EndSponsoringFutureReservesResult {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::EndSponsoringFutureReservesSuccess => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// RevokeSponsorshipResultCode is an XDR Enum defines as:
//
//   enum RevokeSponsorshipResultCode
//    {
//        // codes considered as "success" for the operation
//        REVOKE_SPONSORSHIP_SUCCESS = 0,
//
//        // codes considered as "failure" for the operation
//        REVOKE_SPONSORSHIP_DOES_NOT_EXIST = -1,
//        REVOKE_SPONSORSHIP_NOT_SPONSOR = -2,
//        REVOKE_SPONSORSHIP_LOW_RESERVE = -3,
//        REVOKE_SPONSORSHIP_ONLY_TRANSFERABLE = -4,
//        REVOKE_SPONSORSHIP_MALFORMED = -5
//    };
//
// enum
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum RevokeSponsorshipResultCode {
    RevokeSponsorshipSuccess = 0,
    RevokeSponsorshipDoesNotExist = -1,
    RevokeSponsorshipNotSponsor = -2,
    RevokeSponsorshipLowReserve = -3,
    RevokeSponsorshipOnlyTransferable = -4,
    RevokeSponsorshipMalformed = -5,
}

impl TryFrom<i32> for RevokeSponsorshipResultCode {
    type Error = Error;

    fn try_from(i: i32) -> std::result::Result<Self, Self::Error> {
        let e = match i {
            0 => RevokeSponsorshipResultCode::RevokeSponsorshipSuccess,
            -1 => RevokeSponsorshipResultCode::RevokeSponsorshipDoesNotExist,
            -2 => RevokeSponsorshipResultCode::RevokeSponsorshipNotSponsor,
            -3 => RevokeSponsorshipResultCode::RevokeSponsorshipLowReserve,
            -4 => RevokeSponsorshipResultCode::RevokeSponsorshipOnlyTransferable,
            -5 => RevokeSponsorshipResultCode::RevokeSponsorshipMalformed,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<RevokeSponsorshipResultCode> for i32 {
    fn from(e: RevokeSponsorshipResultCode) -> Self {
        e as Self
    }
}

impl ReadXDR for RevokeSponsorshipResultCode {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXDR for RevokeSponsorshipResultCode {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// RevokeSponsorshipResult is an XDR Union defines as:
//
//   union RevokeSponsorshipResult switch (RevokeSponsorshipResultCode code)
//    {
//    case REVOKE_SPONSORSHIP_SUCCESS:
//        void;
//    default:
//        void;
//    };
//
// union with discriminant RevokeSponsorshipResultCode
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RevokeSponsorshipResult {
    RevokeSponsorshipSuccess,
}

impl RevokeSponsorshipResult {
    fn discriminant(&self) -> RevokeSponsorshipResultCode {
        match self {
            Self::RevokeSponsorshipSuccess => RevokeSponsorshipResultCode::RevokeSponsorshipSuccess,
        }
    }
}

impl ReadXDR for RevokeSponsorshipResult {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: RevokeSponsorshipResultCode =
            <RevokeSponsorshipResultCode as ReadXDR>::read_xdr(r)?;
        let v = match dv {
            RevokeSponsorshipResultCode::RevokeSponsorshipSuccess => Self::RevokeSponsorshipSuccess,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for RevokeSponsorshipResult {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::RevokeSponsorshipSuccess => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// ClawbackResultCode is an XDR Enum defines as:
//
//   enum ClawbackResultCode
//    {
//        // codes considered as "success" for the operation
//        CLAWBACK_SUCCESS = 0,
//
//        // codes considered as "failure" for the operation
//        CLAWBACK_MALFORMED = -1,
//        CLAWBACK_NOT_CLAWBACK_ENABLED = -2,
//        CLAWBACK_NO_TRUST = -3,
//        CLAWBACK_UNDERFUNDED = -4
//    };
//
// enum
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum ClawbackResultCode {
    ClawbackSuccess = 0,
    ClawbackMalformed = -1,
    ClawbackNotClawbackEnabled = -2,
    ClawbackNoTrust = -3,
    ClawbackUnderfunded = -4,
}

impl TryFrom<i32> for ClawbackResultCode {
    type Error = Error;

    fn try_from(i: i32) -> std::result::Result<Self, Self::Error> {
        let e = match i {
            0 => ClawbackResultCode::ClawbackSuccess,
            -1 => ClawbackResultCode::ClawbackMalformed,
            -2 => ClawbackResultCode::ClawbackNotClawbackEnabled,
            -3 => ClawbackResultCode::ClawbackNoTrust,
            -4 => ClawbackResultCode::ClawbackUnderfunded,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<ClawbackResultCode> for i32 {
    fn from(e: ClawbackResultCode) -> Self {
        e as Self
    }
}

impl ReadXDR for ClawbackResultCode {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXDR for ClawbackResultCode {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// ClawbackResult is an XDR Union defines as:
//
//   union ClawbackResult switch (ClawbackResultCode code)
//    {
//    case CLAWBACK_SUCCESS:
//        void;
//    default:
//        void;
//    };
//
// union with discriminant ClawbackResultCode
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ClawbackResult {
    ClawbackSuccess,
}

impl ClawbackResult {
    fn discriminant(&self) -> ClawbackResultCode {
        match self {
            Self::ClawbackSuccess => ClawbackResultCode::ClawbackSuccess,
        }
    }
}

impl ReadXDR for ClawbackResult {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: ClawbackResultCode = <ClawbackResultCode as ReadXDR>::read_xdr(r)?;
        let v = match dv {
            ClawbackResultCode::ClawbackSuccess => Self::ClawbackSuccess,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for ClawbackResult {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::ClawbackSuccess => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// ClawbackClaimableBalanceResultCode is an XDR Enum defines as:
//
//   enum ClawbackClaimableBalanceResultCode
//    {
//        // codes considered as "success" for the operation
//        CLAWBACK_CLAIMABLE_BALANCE_SUCCESS = 0,
//
//        // codes considered as "failure" for the operation
//        CLAWBACK_CLAIMABLE_BALANCE_DOES_NOT_EXIST = -1,
//        CLAWBACK_CLAIMABLE_BALANCE_NOT_ISSUER = -2,
//        CLAWBACK_CLAIMABLE_BALANCE_NOT_CLAWBACK_ENABLED = -3
//    };
//
// enum
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum ClawbackClaimableBalanceResultCode {
    ClawbackClaimableBalanceSuccess = 0,
    ClawbackClaimableBalanceDoesNotExist = -1,
    ClawbackClaimableBalanceNotIssuer = -2,
    ClawbackClaimableBalanceNotClawbackEnabled = -3,
}

impl TryFrom<i32> for ClawbackClaimableBalanceResultCode {
    type Error = Error;

    fn try_from(i: i32) -> std::result::Result<Self, Self::Error> {
        let e = match i {
            0 => ClawbackClaimableBalanceResultCode::ClawbackClaimableBalanceSuccess,
            -1 => ClawbackClaimableBalanceResultCode::ClawbackClaimableBalanceDoesNotExist,
            -2 => ClawbackClaimableBalanceResultCode::ClawbackClaimableBalanceNotIssuer,
            -3 => ClawbackClaimableBalanceResultCode::ClawbackClaimableBalanceNotClawbackEnabled,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<ClawbackClaimableBalanceResultCode> for i32 {
    fn from(e: ClawbackClaimableBalanceResultCode) -> Self {
        e as Self
    }
}

impl ReadXDR for ClawbackClaimableBalanceResultCode {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXDR for ClawbackClaimableBalanceResultCode {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// ClawbackClaimableBalanceResult is an XDR Union defines as:
//
//   union ClawbackClaimableBalanceResult switch (
//        ClawbackClaimableBalanceResultCode code)
//    {
//    case CLAWBACK_CLAIMABLE_BALANCE_SUCCESS:
//        void;
//    default:
//        void;
//    };
//
// union with discriminant ClawbackClaimableBalanceResultCode
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ClawbackClaimableBalanceResult {
    ClawbackClaimableBalanceSuccess,
}

impl ClawbackClaimableBalanceResult {
    fn discriminant(&self) -> ClawbackClaimableBalanceResultCode {
        match self {
            Self::ClawbackClaimableBalanceSuccess => {
                ClawbackClaimableBalanceResultCode::ClawbackClaimableBalanceSuccess
            }
        }
    }
}

impl ReadXDR for ClawbackClaimableBalanceResult {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: ClawbackClaimableBalanceResultCode =
            <ClawbackClaimableBalanceResultCode as ReadXDR>::read_xdr(r)?;
        let v = match dv {
            ClawbackClaimableBalanceResultCode::ClawbackClaimableBalanceSuccess => {
                Self::ClawbackClaimableBalanceSuccess
            }
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for ClawbackClaimableBalanceResult {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::ClawbackClaimableBalanceSuccess => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// SetTrustLineFlagsResultCode is an XDR Enum defines as:
//
//   enum SetTrustLineFlagsResultCode
//    {
//        // codes considered as "success" for the operation
//        SET_TRUST_LINE_FLAGS_SUCCESS = 0,
//
//        // codes considered as "failure" for the operation
//        SET_TRUST_LINE_FLAGS_MALFORMED = -1,
//        SET_TRUST_LINE_FLAGS_NO_TRUST_LINE = -2,
//        SET_TRUST_LINE_FLAGS_CANT_REVOKE = -3,
//        SET_TRUST_LINE_FLAGS_INVALID_STATE = -4,
//        SET_TRUST_LINE_FLAGS_LOW_RESERVE = -5 // claimable balances can't be created
//                                              // on revoke due to low reserves
//    };
//
// enum
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum SetTrustLineFlagsResultCode {
    SetTrustLineFlagsSuccess = 0,
    SetTrustLineFlagsMalformed = -1,
    SetTrustLineFlagsNoTrustLine = -2,
    SetTrustLineFlagsCantRevoke = -3,
    SetTrustLineFlagsInvalidState = -4,
    SetTrustLineFlagsLowReserve = -5,
}

impl TryFrom<i32> for SetTrustLineFlagsResultCode {
    type Error = Error;

    fn try_from(i: i32) -> std::result::Result<Self, Self::Error> {
        let e = match i {
            0 => SetTrustLineFlagsResultCode::SetTrustLineFlagsSuccess,
            -1 => SetTrustLineFlagsResultCode::SetTrustLineFlagsMalformed,
            -2 => SetTrustLineFlagsResultCode::SetTrustLineFlagsNoTrustLine,
            -3 => SetTrustLineFlagsResultCode::SetTrustLineFlagsCantRevoke,
            -4 => SetTrustLineFlagsResultCode::SetTrustLineFlagsInvalidState,
            -5 => SetTrustLineFlagsResultCode::SetTrustLineFlagsLowReserve,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<SetTrustLineFlagsResultCode> for i32 {
    fn from(e: SetTrustLineFlagsResultCode) -> Self {
        e as Self
    }
}

impl ReadXDR for SetTrustLineFlagsResultCode {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXDR for SetTrustLineFlagsResultCode {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// SetTrustLineFlagsResult is an XDR Union defines as:
//
//   union SetTrustLineFlagsResult switch (SetTrustLineFlagsResultCode code)
//    {
//    case SET_TRUST_LINE_FLAGS_SUCCESS:
//        void;
//    default:
//        void;
//    };
//
// union with discriminant SetTrustLineFlagsResultCode
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SetTrustLineFlagsResult {
    SetTrustLineFlagsSuccess,
}

impl SetTrustLineFlagsResult {
    fn discriminant(&self) -> SetTrustLineFlagsResultCode {
        match self {
            Self::SetTrustLineFlagsSuccess => SetTrustLineFlagsResultCode::SetTrustLineFlagsSuccess,
        }
    }
}

impl ReadXDR for SetTrustLineFlagsResult {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: SetTrustLineFlagsResultCode =
            <SetTrustLineFlagsResultCode as ReadXDR>::read_xdr(r)?;
        let v = match dv {
            SetTrustLineFlagsResultCode::SetTrustLineFlagsSuccess => Self::SetTrustLineFlagsSuccess,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for SetTrustLineFlagsResult {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::SetTrustLineFlagsSuccess => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// LiquidityPoolDepositResultCode is an XDR Enum defines as:
//
//   enum LiquidityPoolDepositResultCode
//    {
//        // codes considered as "success" for the operation
//        LIQUIDITY_POOL_DEPOSIT_SUCCESS = 0,
//
//        // codes considered as "failure" for the operation
//        LIQUIDITY_POOL_DEPOSIT_MALFORMED = -1,      // bad input
//        LIQUIDITY_POOL_DEPOSIT_NO_TRUST = -2,       // no trust line for one of the
//                                                    // assets
//        LIQUIDITY_POOL_DEPOSIT_NOT_AUTHORIZED = -3, // not authorized for one of the
//                                                    // assets
//        LIQUIDITY_POOL_DEPOSIT_UNDERFUNDED = -4,    // not enough balance for one of
//                                                    // the assets
//        LIQUIDITY_POOL_DEPOSIT_LINE_FULL = -5,      // pool share trust line doesn't
//                                                    // have sufficient limit
//        LIQUIDITY_POOL_DEPOSIT_BAD_PRICE = -6,      // deposit price outside bounds
//        LIQUIDITY_POOL_DEPOSIT_POOL_FULL = -7       // pool reserves are full
//    };
//
// enum
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum LiquidityPoolDepositResultCode {
    LiquidityPoolDepositSuccess = 0,
    LiquidityPoolDepositMalformed = -1,
    LiquidityPoolDepositNoTrust = -2,
    LiquidityPoolDepositNotAuthorized = -3,
    LiquidityPoolDepositUnderfunded = -4,
    LiquidityPoolDepositLineFull = -5,
    LiquidityPoolDepositBadPrice = -6,
    LiquidityPoolDepositPoolFull = -7,
}

impl TryFrom<i32> for LiquidityPoolDepositResultCode {
    type Error = Error;

    fn try_from(i: i32) -> std::result::Result<Self, Self::Error> {
        let e = match i {
            0 => LiquidityPoolDepositResultCode::LiquidityPoolDepositSuccess,
            -1 => LiquidityPoolDepositResultCode::LiquidityPoolDepositMalformed,
            -2 => LiquidityPoolDepositResultCode::LiquidityPoolDepositNoTrust,
            -3 => LiquidityPoolDepositResultCode::LiquidityPoolDepositNotAuthorized,
            -4 => LiquidityPoolDepositResultCode::LiquidityPoolDepositUnderfunded,
            -5 => LiquidityPoolDepositResultCode::LiquidityPoolDepositLineFull,
            -6 => LiquidityPoolDepositResultCode::LiquidityPoolDepositBadPrice,
            -7 => LiquidityPoolDepositResultCode::LiquidityPoolDepositPoolFull,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<LiquidityPoolDepositResultCode> for i32 {
    fn from(e: LiquidityPoolDepositResultCode) -> Self {
        e as Self
    }
}

impl ReadXDR for LiquidityPoolDepositResultCode {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXDR for LiquidityPoolDepositResultCode {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// LiquidityPoolDepositResult is an XDR Union defines as:
//
//   union LiquidityPoolDepositResult switch (LiquidityPoolDepositResultCode code)
//    {
//    case LIQUIDITY_POOL_DEPOSIT_SUCCESS:
//        void;
//    default:
//        void;
//    };
//
// union with discriminant LiquidityPoolDepositResultCode
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LiquidityPoolDepositResult {
    LiquidityPoolDepositSuccess,
}

impl LiquidityPoolDepositResult {
    fn discriminant(&self) -> LiquidityPoolDepositResultCode {
        match self {
            Self::LiquidityPoolDepositSuccess => {
                LiquidityPoolDepositResultCode::LiquidityPoolDepositSuccess
            }
        }
    }
}

impl ReadXDR for LiquidityPoolDepositResult {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: LiquidityPoolDepositResultCode =
            <LiquidityPoolDepositResultCode as ReadXDR>::read_xdr(r)?;
        let v = match dv {
            LiquidityPoolDepositResultCode::LiquidityPoolDepositSuccess => {
                Self::LiquidityPoolDepositSuccess
            }
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for LiquidityPoolDepositResult {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::LiquidityPoolDepositSuccess => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// LiquidityPoolWithdrawResultCode is an XDR Enum defines as:
//
//   enum LiquidityPoolWithdrawResultCode
//    {
//        // codes considered as "success" for the operation
//        LIQUIDITY_POOL_WITHDRAW_SUCCESS = 0,
//
//        // codes considered as "failure" for the operation
//        LIQUIDITY_POOL_WITHDRAW_MALFORMED = -1,    // bad input
//        LIQUIDITY_POOL_WITHDRAW_NO_TRUST = -2,     // no trust line for one of the
//                                                   // assets
//        LIQUIDITY_POOL_WITHDRAW_UNDERFUNDED = -3,  // not enough balance of the
//                                                   // pool share
//        LIQUIDITY_POOL_WITHDRAW_LINE_FULL = -4,    // would go above limit for one
//                                                   // of the assets
//        LIQUIDITY_POOL_WITHDRAW_UNDER_MINIMUM = -5 // didn't withdraw enough
//    };
//
// enum
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum LiquidityPoolWithdrawResultCode {
    LiquidityPoolWithdrawSuccess = 0,
    LiquidityPoolWithdrawMalformed = -1,
    LiquidityPoolWithdrawNoTrust = -2,
    LiquidityPoolWithdrawUnderfunded = -3,
    LiquidityPoolWithdrawLineFull = -4,
    LiquidityPoolWithdrawUnderMinimum = -5,
}

impl TryFrom<i32> for LiquidityPoolWithdrawResultCode {
    type Error = Error;

    fn try_from(i: i32) -> std::result::Result<Self, Self::Error> {
        let e = match i {
            0 => LiquidityPoolWithdrawResultCode::LiquidityPoolWithdrawSuccess,
            -1 => LiquidityPoolWithdrawResultCode::LiquidityPoolWithdrawMalformed,
            -2 => LiquidityPoolWithdrawResultCode::LiquidityPoolWithdrawNoTrust,
            -3 => LiquidityPoolWithdrawResultCode::LiquidityPoolWithdrawUnderfunded,
            -4 => LiquidityPoolWithdrawResultCode::LiquidityPoolWithdrawLineFull,
            -5 => LiquidityPoolWithdrawResultCode::LiquidityPoolWithdrawUnderMinimum,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<LiquidityPoolWithdrawResultCode> for i32 {
    fn from(e: LiquidityPoolWithdrawResultCode) -> Self {
        e as Self
    }
}

impl ReadXDR for LiquidityPoolWithdrawResultCode {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXDR for LiquidityPoolWithdrawResultCode {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// LiquidityPoolWithdrawResult is an XDR Union defines as:
//
//   union LiquidityPoolWithdrawResult switch (LiquidityPoolWithdrawResultCode code)
//    {
//    case LIQUIDITY_POOL_WITHDRAW_SUCCESS:
//        void;
//    default:
//        void;
//    };
//
// union with discriminant LiquidityPoolWithdrawResultCode
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LiquidityPoolWithdrawResult {
    LiquidityPoolWithdrawSuccess,
}

impl LiquidityPoolWithdrawResult {
    fn discriminant(&self) -> LiquidityPoolWithdrawResultCode {
        match self {
            Self::LiquidityPoolWithdrawSuccess => {
                LiquidityPoolWithdrawResultCode::LiquidityPoolWithdrawSuccess
            }
        }
    }
}

impl ReadXDR for LiquidityPoolWithdrawResult {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: LiquidityPoolWithdrawResultCode =
            <LiquidityPoolWithdrawResultCode as ReadXDR>::read_xdr(r)?;
        let v = match dv {
            LiquidityPoolWithdrawResultCode::LiquidityPoolWithdrawSuccess => {
                Self::LiquidityPoolWithdrawSuccess
            }
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for LiquidityPoolWithdrawResult {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::LiquidityPoolWithdrawSuccess => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// OperationResultCode is an XDR Enum defines as:
//
//   enum OperationResultCode
//    {
//        opINNER = 0, // inner object result is valid
//
//        opBAD_AUTH = -1,            // too few valid signatures / wrong network
//        opNO_ACCOUNT = -2,          // source account was not found
//        opNOT_SUPPORTED = -3,       // operation not supported at this time
//        opTOO_MANY_SUBENTRIES = -4, // max number of subentries already reached
//        opEXCEEDED_WORK_LIMIT = -5, // operation did too much work
//        opTOO_MANY_SPONSORING = -6  // account is sponsoring too many entries
//    };
//
// enum
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum OperationResultCode {
    OpInner = 0,
    OpBadAuth = -1,
    OpNoAccount = -2,
    OpNotSupported = -3,
    OpTooManySubentries = -4,
    OpExceededWorkLimit = -5,
    OpTooManySponsoring = -6,
}

impl TryFrom<i32> for OperationResultCode {
    type Error = Error;

    fn try_from(i: i32) -> std::result::Result<Self, Self::Error> {
        let e = match i {
            0 => OperationResultCode::OpInner,
            -1 => OperationResultCode::OpBadAuth,
            -2 => OperationResultCode::OpNoAccount,
            -3 => OperationResultCode::OpNotSupported,
            -4 => OperationResultCode::OpTooManySubentries,
            -5 => OperationResultCode::OpExceededWorkLimit,
            -6 => OperationResultCode::OpTooManySponsoring,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<OperationResultCode> for i32 {
    fn from(e: OperationResultCode) -> Self {
        e as Self
    }
}

impl ReadXDR for OperationResultCode {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXDR for OperationResultCode {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// OperationResultTr is an XDR NestedUnion defines as:
//
//   union switch (OperationType type)
//        {
//        case CREATE_ACCOUNT:
//            CreateAccountResult createAccountResult;
//        case PAYMENT:
//            PaymentResult paymentResult;
//        case PATH_PAYMENT_STRICT_RECEIVE:
//            PathPaymentStrictReceiveResult pathPaymentStrictReceiveResult;
//        case MANAGE_SELL_OFFER:
//            ManageSellOfferResult manageSellOfferResult;
//        case CREATE_PASSIVE_SELL_OFFER:
//            ManageSellOfferResult createPassiveSellOfferResult;
//        case SET_OPTIONS:
//            SetOptionsResult setOptionsResult;
//        case CHANGE_TRUST:
//            ChangeTrustResult changeTrustResult;
//        case ALLOW_TRUST:
//            AllowTrustResult allowTrustResult;
//        case ACCOUNT_MERGE:
//            AccountMergeResult accountMergeResult;
//        case INFLATION:
//            InflationResult inflationResult;
//        case MANAGE_DATA:
//            ManageDataResult manageDataResult;
//        case BUMP_SEQUENCE:
//            BumpSequenceResult bumpSeqResult;
//        case MANAGE_BUY_OFFER:
//            ManageBuyOfferResult manageBuyOfferResult;
//        case PATH_PAYMENT_STRICT_SEND:
//            PathPaymentStrictSendResult pathPaymentStrictSendResult;
//        case CREATE_CLAIMABLE_BALANCE:
//            CreateClaimableBalanceResult createClaimableBalanceResult;
//        case CLAIM_CLAIMABLE_BALANCE:
//            ClaimClaimableBalanceResult claimClaimableBalanceResult;
//        case BEGIN_SPONSORING_FUTURE_RESERVES:
//            BeginSponsoringFutureReservesResult beginSponsoringFutureReservesResult;
//        case END_SPONSORING_FUTURE_RESERVES:
//            EndSponsoringFutureReservesResult endSponsoringFutureReservesResult;
//        case REVOKE_SPONSORSHIP:
//            RevokeSponsorshipResult revokeSponsorshipResult;
//        case CLAWBACK:
//            ClawbackResult clawbackResult;
//        case CLAWBACK_CLAIMABLE_BALANCE:
//            ClawbackClaimableBalanceResult clawbackClaimableBalanceResult;
//        case SET_TRUST_LINE_FLAGS:
//            SetTrustLineFlagsResult setTrustLineFlagsResult;
//        case LIQUIDITY_POOL_DEPOSIT:
//            LiquidityPoolDepositResult liquidityPoolDepositResult;
//        case LIQUIDITY_POOL_WITHDRAW:
//            LiquidityPoolWithdrawResult liquidityPoolWithdrawResult;
//        }
//
// union with discriminant OperationType
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OperationResultTr {
    CreateAccount(CreateAccountResult),
    Payment(PaymentResult),
    PathPaymentStrictReceive(PathPaymentStrictReceiveResult),
    ManageSellOffer(ManageSellOfferResult),
    CreatePassiveSellOffer(ManageSellOfferResult),
    SetOptions(SetOptionsResult),
    ChangeTrust(ChangeTrustResult),
    AllowTrust(AllowTrustResult),
    AccountMerge(AccountMergeResult),
    Inflation(InflationResult),
    ManageData(ManageDataResult),
    BumpSequence(BumpSequenceResult),
    ManageBuyOffer(ManageBuyOfferResult),
    PathPaymentStrictSend(PathPaymentStrictSendResult),
    CreateClaimableBalance(CreateClaimableBalanceResult),
    ClaimClaimableBalance(ClaimClaimableBalanceResult),
    BeginSponsoringFutureReserves(BeginSponsoringFutureReservesResult),
    EndSponsoringFutureReserves(EndSponsoringFutureReservesResult),
    RevokeSponsorship(RevokeSponsorshipResult),
    Clawback(ClawbackResult),
    ClawbackClaimableBalance(ClawbackClaimableBalanceResult),
    SetTrustLineFlags(SetTrustLineFlagsResult),
    LiquidityPoolDeposit(LiquidityPoolDepositResult),
    LiquidityPoolWithdraw(LiquidityPoolWithdrawResult),
}

impl OperationResultTr {
    fn discriminant(&self) -> OperationType {
        match self {
            Self::CreateAccount(_) => OperationType::CreateAccount,
            Self::Payment(_) => OperationType::Payment,
            Self::PathPaymentStrictReceive(_) => OperationType::PathPaymentStrictReceive,
            Self::ManageSellOffer(_) => OperationType::ManageSellOffer,
            Self::CreatePassiveSellOffer(_) => OperationType::CreatePassiveSellOffer,
            Self::SetOptions(_) => OperationType::SetOptions,
            Self::ChangeTrust(_) => OperationType::ChangeTrust,
            Self::AllowTrust(_) => OperationType::AllowTrust,
            Self::AccountMerge(_) => OperationType::AccountMerge,
            Self::Inflation(_) => OperationType::Inflation,
            Self::ManageData(_) => OperationType::ManageData,
            Self::BumpSequence(_) => OperationType::BumpSequence,
            Self::ManageBuyOffer(_) => OperationType::ManageBuyOffer,
            Self::PathPaymentStrictSend(_) => OperationType::PathPaymentStrictSend,
            Self::CreateClaimableBalance(_) => OperationType::CreateClaimableBalance,
            Self::ClaimClaimableBalance(_) => OperationType::ClaimClaimableBalance,
            Self::BeginSponsoringFutureReserves(_) => OperationType::BeginSponsoringFutureReserves,
            Self::EndSponsoringFutureReserves(_) => OperationType::EndSponsoringFutureReserves,
            Self::RevokeSponsorship(_) => OperationType::RevokeSponsorship,
            Self::Clawback(_) => OperationType::Clawback,
            Self::ClawbackClaimableBalance(_) => OperationType::ClawbackClaimableBalance,
            Self::SetTrustLineFlags(_) => OperationType::SetTrustLineFlags,
            Self::LiquidityPoolDeposit(_) => OperationType::LiquidityPoolDeposit,
            Self::LiquidityPoolWithdraw(_) => OperationType::LiquidityPoolWithdraw,
        }
    }
}

impl ReadXDR for OperationResultTr {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: OperationType = <OperationType as ReadXDR>::read_xdr(r)?;
        let v = match dv {
            OperationType::CreateAccount => {
                Self::CreateAccount(<CreateAccountResult as ReadXDR>::read_xdr(r)?)
            }
            OperationType::Payment => Self::Payment(<PaymentResult as ReadXDR>::read_xdr(r)?),
            OperationType::PathPaymentStrictReceive => Self::PathPaymentStrictReceive(
                <PathPaymentStrictReceiveResult as ReadXDR>::read_xdr(r)?,
            ),
            OperationType::ManageSellOffer => {
                Self::ManageSellOffer(<ManageSellOfferResult as ReadXDR>::read_xdr(r)?)
            }
            OperationType::CreatePassiveSellOffer => {
                Self::CreatePassiveSellOffer(<ManageSellOfferResult as ReadXDR>::read_xdr(r)?)
            }
            OperationType::SetOptions => {
                Self::SetOptions(<SetOptionsResult as ReadXDR>::read_xdr(r)?)
            }
            OperationType::ChangeTrust => {
                Self::ChangeTrust(<ChangeTrustResult as ReadXDR>::read_xdr(r)?)
            }
            OperationType::AllowTrust => {
                Self::AllowTrust(<AllowTrustResult as ReadXDR>::read_xdr(r)?)
            }
            OperationType::AccountMerge => {
                Self::AccountMerge(<AccountMergeResult as ReadXDR>::read_xdr(r)?)
            }
            OperationType::Inflation => Self::Inflation(<InflationResult as ReadXDR>::read_xdr(r)?),
            OperationType::ManageData => {
                Self::ManageData(<ManageDataResult as ReadXDR>::read_xdr(r)?)
            }
            OperationType::BumpSequence => {
                Self::BumpSequence(<BumpSequenceResult as ReadXDR>::read_xdr(r)?)
            }
            OperationType::ManageBuyOffer => {
                Self::ManageBuyOffer(<ManageBuyOfferResult as ReadXDR>::read_xdr(r)?)
            }
            OperationType::PathPaymentStrictSend => {
                Self::PathPaymentStrictSend(<PathPaymentStrictSendResult as ReadXDR>::read_xdr(r)?)
            }
            OperationType::CreateClaimableBalance => Self::CreateClaimableBalance(
                <CreateClaimableBalanceResult as ReadXDR>::read_xdr(r)?,
            ),
            OperationType::ClaimClaimableBalance => {
                Self::ClaimClaimableBalance(<ClaimClaimableBalanceResult as ReadXDR>::read_xdr(r)?)
            }
            OperationType::BeginSponsoringFutureReserves => Self::BeginSponsoringFutureReserves(
                <BeginSponsoringFutureReservesResult as ReadXDR>::read_xdr(r)?,
            ),
            OperationType::EndSponsoringFutureReserves => Self::EndSponsoringFutureReserves(
                <EndSponsoringFutureReservesResult as ReadXDR>::read_xdr(r)?,
            ),
            OperationType::RevokeSponsorship => {
                Self::RevokeSponsorship(<RevokeSponsorshipResult as ReadXDR>::read_xdr(r)?)
            }
            OperationType::Clawback => Self::Clawback(<ClawbackResult as ReadXDR>::read_xdr(r)?),
            OperationType::ClawbackClaimableBalance => Self::ClawbackClaimableBalance(
                <ClawbackClaimableBalanceResult as ReadXDR>::read_xdr(r)?,
            ),
            OperationType::SetTrustLineFlags => {
                Self::SetTrustLineFlags(<SetTrustLineFlagsResult as ReadXDR>::read_xdr(r)?)
            }
            OperationType::LiquidityPoolDeposit => {
                Self::LiquidityPoolDeposit(<LiquidityPoolDepositResult as ReadXDR>::read_xdr(r)?)
            }
            OperationType::LiquidityPoolWithdraw => {
                Self::LiquidityPoolWithdraw(<LiquidityPoolWithdrawResult as ReadXDR>::read_xdr(r)?)
            }
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for OperationResultTr {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::CreateAccount(v) => v.write_xdr(w)?,
            Self::Payment(v) => v.write_xdr(w)?,
            Self::PathPaymentStrictReceive(v) => v.write_xdr(w)?,
            Self::ManageSellOffer(v) => v.write_xdr(w)?,
            Self::CreatePassiveSellOffer(v) => v.write_xdr(w)?,
            Self::SetOptions(v) => v.write_xdr(w)?,
            Self::ChangeTrust(v) => v.write_xdr(w)?,
            Self::AllowTrust(v) => v.write_xdr(w)?,
            Self::AccountMerge(v) => v.write_xdr(w)?,
            Self::Inflation(v) => v.write_xdr(w)?,
            Self::ManageData(v) => v.write_xdr(w)?,
            Self::BumpSequence(v) => v.write_xdr(w)?,
            Self::ManageBuyOffer(v) => v.write_xdr(w)?,
            Self::PathPaymentStrictSend(v) => v.write_xdr(w)?,
            Self::CreateClaimableBalance(v) => v.write_xdr(w)?,
            Self::ClaimClaimableBalance(v) => v.write_xdr(w)?,
            Self::BeginSponsoringFutureReserves(v) => v.write_xdr(w)?,
            Self::EndSponsoringFutureReserves(v) => v.write_xdr(w)?,
            Self::RevokeSponsorship(v) => v.write_xdr(w)?,
            Self::Clawback(v) => v.write_xdr(w)?,
            Self::ClawbackClaimableBalance(v) => v.write_xdr(w)?,
            Self::SetTrustLineFlags(v) => v.write_xdr(w)?,
            Self::LiquidityPoolDeposit(v) => v.write_xdr(w)?,
            Self::LiquidityPoolWithdraw(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// OperationResult is an XDR Union defines as:
//
//   union OperationResult switch (OperationResultCode code)
//    {
//    case opINNER:
//        union switch (OperationType type)
//        {
//        case CREATE_ACCOUNT:
//            CreateAccountResult createAccountResult;
//        case PAYMENT:
//            PaymentResult paymentResult;
//        case PATH_PAYMENT_STRICT_RECEIVE:
//            PathPaymentStrictReceiveResult pathPaymentStrictReceiveResult;
//        case MANAGE_SELL_OFFER:
//            ManageSellOfferResult manageSellOfferResult;
//        case CREATE_PASSIVE_SELL_OFFER:
//            ManageSellOfferResult createPassiveSellOfferResult;
//        case SET_OPTIONS:
//            SetOptionsResult setOptionsResult;
//        case CHANGE_TRUST:
//            ChangeTrustResult changeTrustResult;
//        case ALLOW_TRUST:
//            AllowTrustResult allowTrustResult;
//        case ACCOUNT_MERGE:
//            AccountMergeResult accountMergeResult;
//        case INFLATION:
//            InflationResult inflationResult;
//        case MANAGE_DATA:
//            ManageDataResult manageDataResult;
//        case BUMP_SEQUENCE:
//            BumpSequenceResult bumpSeqResult;
//        case MANAGE_BUY_OFFER:
//            ManageBuyOfferResult manageBuyOfferResult;
//        case PATH_PAYMENT_STRICT_SEND:
//            PathPaymentStrictSendResult pathPaymentStrictSendResult;
//        case CREATE_CLAIMABLE_BALANCE:
//            CreateClaimableBalanceResult createClaimableBalanceResult;
//        case CLAIM_CLAIMABLE_BALANCE:
//            ClaimClaimableBalanceResult claimClaimableBalanceResult;
//        case BEGIN_SPONSORING_FUTURE_RESERVES:
//            BeginSponsoringFutureReservesResult beginSponsoringFutureReservesResult;
//        case END_SPONSORING_FUTURE_RESERVES:
//            EndSponsoringFutureReservesResult endSponsoringFutureReservesResult;
//        case REVOKE_SPONSORSHIP:
//            RevokeSponsorshipResult revokeSponsorshipResult;
//        case CLAWBACK:
//            ClawbackResult clawbackResult;
//        case CLAWBACK_CLAIMABLE_BALANCE:
//            ClawbackClaimableBalanceResult clawbackClaimableBalanceResult;
//        case SET_TRUST_LINE_FLAGS:
//            SetTrustLineFlagsResult setTrustLineFlagsResult;
//        case LIQUIDITY_POOL_DEPOSIT:
//            LiquidityPoolDepositResult liquidityPoolDepositResult;
//        case LIQUIDITY_POOL_WITHDRAW:
//            LiquidityPoolWithdrawResult liquidityPoolWithdrawResult;
//        }
//        tr;
//    default:
//        void;
//    };
//
// union with discriminant OperationResultCode
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OperationResult {
    OpInner(OperationResultTr),
}

impl OperationResult {
    fn discriminant(&self) -> OperationResultCode {
        match self {
            Self::OpInner(_) => OperationResultCode::OpInner,
        }
    }
}

impl ReadXDR for OperationResult {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: OperationResultCode = <OperationResultCode as ReadXDR>::read_xdr(r)?;
        let v = match dv {
            OperationResultCode::OpInner => {
                Self::OpInner(<OperationResultTr as ReadXDR>::read_xdr(r)?)
            }
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for OperationResult {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::OpInner(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// TransactionResultCode is an XDR Enum defines as:
//
//   enum TransactionResultCode
//    {
//        txFEE_BUMP_INNER_SUCCESS = 1, // fee bump inner transaction succeeded
//        txSUCCESS = 0,                // all operations succeeded
//
//        txFAILED = -1, // one of the operations failed (none were applied)
//
//        txTOO_EARLY = -2,         // ledger closeTime before minTime
//        txTOO_LATE = -3,          // ledger closeTime after maxTime
//        txMISSING_OPERATION = -4, // no operation was specified
//        txBAD_SEQ = -5,           // sequence number does not match source account
//
//        txBAD_AUTH = -6,             // too few valid signatures / wrong network
//        txINSUFFICIENT_BALANCE = -7, // fee would bring account below reserve
//        txNO_ACCOUNT = -8,           // source account not found
//        txINSUFFICIENT_FEE = -9,     // fee is too small
//        txBAD_AUTH_EXTRA = -10,      // unused signatures attached to transaction
//        txINTERNAL_ERROR = -11,      // an unknown error occurred
//
//        txNOT_SUPPORTED = -12,         // transaction type not supported
//        txFEE_BUMP_INNER_FAILED = -13, // fee bump inner transaction failed
//        txBAD_SPONSORSHIP = -14,       // sponsorship not confirmed
//        txBAD_MIN_SEQ_AGE_OR_GAP =
//            -15, // minSeqAge or minSeqLedgerGap conditions not met
//        txMALFORMED = -16 // precondition is invalid
//    };
//
// enum
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum TransactionResultCode {
    TxFeeBumpInnerSuccess = 1,
    TxSuccess = 0,
    TxFailed = -1,
    TxTooEarly = -2,
    TxTooLate = -3,
    TxMissingOperation = -4,
    TxBadSeq = -5,
    TxBadAuth = -6,
    TxInsufficientBalance = -7,
    TxNoAccount = -8,
    TxInsufficientFee = -9,
    TxBadAuthExtra = -10,
    TxInternalError = -11,
    TxNotSupported = -12,
    TxFeeBumpInnerFailed = -13,
    TxBadSponsorship = -14,
    TxBadMinSeqAgeOrGap = -15,
    TxMalformed = -16,
}

impl TryFrom<i32> for TransactionResultCode {
    type Error = Error;

    fn try_from(i: i32) -> std::result::Result<Self, Self::Error> {
        let e = match i {
            1 => TransactionResultCode::TxFeeBumpInnerSuccess,
            0 => TransactionResultCode::TxSuccess,
            -1 => TransactionResultCode::TxFailed,
            -2 => TransactionResultCode::TxTooEarly,
            -3 => TransactionResultCode::TxTooLate,
            -4 => TransactionResultCode::TxMissingOperation,
            -5 => TransactionResultCode::TxBadSeq,
            -6 => TransactionResultCode::TxBadAuth,
            -7 => TransactionResultCode::TxInsufficientBalance,
            -8 => TransactionResultCode::TxNoAccount,
            -9 => TransactionResultCode::TxInsufficientFee,
            -10 => TransactionResultCode::TxBadAuthExtra,
            -11 => TransactionResultCode::TxInternalError,
            -12 => TransactionResultCode::TxNotSupported,
            -13 => TransactionResultCode::TxFeeBumpInnerFailed,
            -14 => TransactionResultCode::TxBadSponsorship,
            -15 => TransactionResultCode::TxBadMinSeqAgeOrGap,
            -16 => TransactionResultCode::TxMalformed,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<TransactionResultCode> for i32 {
    fn from(e: TransactionResultCode) -> Self {
        e as Self
    }
}

impl ReadXDR for TransactionResultCode {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXDR for TransactionResultCode {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// InnerTransactionResultResult is an XDR NestedUnion defines as:
//
//   union switch (TransactionResultCode code)
//        {
//        // txFEE_BUMP_INNER_SUCCESS is not included
//        case txSUCCESS:
//        case txFAILED:
//            OperationResult results<>;
//        case txTOO_EARLY:
//        case txTOO_LATE:
//        case txMISSING_OPERATION:
//        case txBAD_SEQ:
//        case txBAD_AUTH:
//        case txINSUFFICIENT_BALANCE:
//        case txNO_ACCOUNT:
//        case txINSUFFICIENT_FEE:
//        case txBAD_AUTH_EXTRA:
//        case txINTERNAL_ERROR:
//        case txNOT_SUPPORTED:
//        // txFEE_BUMP_INNER_FAILED is not included
//        case txBAD_SPONSORSHIP:
//        case txBAD_MIN_SEQ_AGE_OR_GAP:
//        case txMALFORMED:
//            void;
//        }
//
// union with discriminant TransactionResultCode
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InnerTransactionResultResult {
    TxSuccess(Vec<OperationResult>),
    TxFailed(Vec<OperationResult>),
    TxTooEarly,
    TxTooLate,
    TxMissingOperation,
    TxBadSeq,
    TxBadAuth,
    TxInsufficientBalance,
    TxNoAccount,
    TxInsufficientFee,
    TxBadAuthExtra,
    TxInternalError,
    TxNotSupported,
    TxBadSponsorship,
    TxBadMinSeqAgeOrGap,
    TxMalformed,
}

impl InnerTransactionResultResult {
    fn discriminant(&self) -> TransactionResultCode {
        match self {
            Self::TxSuccess(_) => TransactionResultCode::TxSuccess,
            Self::TxFailed(_) => TransactionResultCode::TxFailed,
            Self::TxTooEarly => TransactionResultCode::TxTooEarly,
            Self::TxTooLate => TransactionResultCode::TxTooLate,
            Self::TxMissingOperation => TransactionResultCode::TxMissingOperation,
            Self::TxBadSeq => TransactionResultCode::TxBadSeq,
            Self::TxBadAuth => TransactionResultCode::TxBadAuth,
            Self::TxInsufficientBalance => TransactionResultCode::TxInsufficientBalance,
            Self::TxNoAccount => TransactionResultCode::TxNoAccount,
            Self::TxInsufficientFee => TransactionResultCode::TxInsufficientFee,
            Self::TxBadAuthExtra => TransactionResultCode::TxBadAuthExtra,
            Self::TxInternalError => TransactionResultCode::TxInternalError,
            Self::TxNotSupported => TransactionResultCode::TxNotSupported,
            Self::TxBadSponsorship => TransactionResultCode::TxBadSponsorship,
            Self::TxBadMinSeqAgeOrGap => TransactionResultCode::TxBadMinSeqAgeOrGap,
            Self::TxMalformed => TransactionResultCode::TxMalformed,
        }
    }
}

impl ReadXDR for InnerTransactionResultResult {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: TransactionResultCode = <TransactionResultCode as ReadXDR>::read_xdr(r)?;
        let v = match dv {
            TransactionResultCode::TxSuccess => {
                Self::TxSuccess(<Vec<OperationResult> as ReadXDR>::read_xdr(r)?)
            }
            TransactionResultCode::TxFailed => {
                Self::TxFailed(<Vec<OperationResult> as ReadXDR>::read_xdr(r)?)
            }
            TransactionResultCode::TxTooEarly => Self::TxTooEarly,
            TransactionResultCode::TxTooLate => Self::TxTooLate,
            TransactionResultCode::TxMissingOperation => Self::TxMissingOperation,
            TransactionResultCode::TxBadSeq => Self::TxBadSeq,
            TransactionResultCode::TxBadAuth => Self::TxBadAuth,
            TransactionResultCode::TxInsufficientBalance => Self::TxInsufficientBalance,
            TransactionResultCode::TxNoAccount => Self::TxNoAccount,
            TransactionResultCode::TxInsufficientFee => Self::TxInsufficientFee,
            TransactionResultCode::TxBadAuthExtra => Self::TxBadAuthExtra,
            TransactionResultCode::TxInternalError => Self::TxInternalError,
            TransactionResultCode::TxNotSupported => Self::TxNotSupported,
            TransactionResultCode::TxBadSponsorship => Self::TxBadSponsorship,
            TransactionResultCode::TxBadMinSeqAgeOrGap => Self::TxBadMinSeqAgeOrGap,
            TransactionResultCode::TxMalformed => Self::TxMalformed,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for InnerTransactionResultResult {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::TxSuccess(v) => v.write_xdr(w)?,
            Self::TxFailed(v) => v.write_xdr(w)?,
            Self::TxTooEarly => ().write_xdr(w)?,
            Self::TxTooLate => ().write_xdr(w)?,
            Self::TxMissingOperation => ().write_xdr(w)?,
            Self::TxBadSeq => ().write_xdr(w)?,
            Self::TxBadAuth => ().write_xdr(w)?,
            Self::TxInsufficientBalance => ().write_xdr(w)?,
            Self::TxNoAccount => ().write_xdr(w)?,
            Self::TxInsufficientFee => ().write_xdr(w)?,
            Self::TxBadAuthExtra => ().write_xdr(w)?,
            Self::TxInternalError => ().write_xdr(w)?,
            Self::TxNotSupported => ().write_xdr(w)?,
            Self::TxBadSponsorship => ().write_xdr(w)?,
            Self::TxBadMinSeqAgeOrGap => ().write_xdr(w)?,
            Self::TxMalformed => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// InnerTransactionResultExt is an XDR NestedUnion defines as:
//
//   union switch (int v)
//        {
//        case 0:
//            void;
//        }
//
// union with discriminant i32
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InnerTransactionResultExt {
    V0,
}

impl InnerTransactionResultExt {
    fn discriminant(&self) -> i32 {
        match self {
            Self::V0 => 0,
        }
    }
}

impl ReadXDR for InnerTransactionResultExt {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: i32 = <i32 as ReadXDR>::read_xdr(r)?;
        let v = match dv.into() {
            0 => Self::V0,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for InnerTransactionResultExt {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::V0 => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// InnerTransactionResult is an XDR Struct defines as:
//
//   struct InnerTransactionResult
//    {
//        // Always 0. Here for binary compatibility.
//        int64 feeCharged;
//
//        union switch (TransactionResultCode code)
//        {
//        // txFEE_BUMP_INNER_SUCCESS is not included
//        case txSUCCESS:
//        case txFAILED:
//            OperationResult results<>;
//        case txTOO_EARLY:
//        case txTOO_LATE:
//        case txMISSING_OPERATION:
//        case txBAD_SEQ:
//        case txBAD_AUTH:
//        case txINSUFFICIENT_BALANCE:
//        case txNO_ACCOUNT:
//        case txINSUFFICIENT_FEE:
//        case txBAD_AUTH_EXTRA:
//        case txINTERNAL_ERROR:
//        case txNOT_SUPPORTED:
//        // txFEE_BUMP_INNER_FAILED is not included
//        case txBAD_SPONSORSHIP:
//        case txBAD_MIN_SEQ_AGE_OR_GAP:
//        case txMALFORMED:
//            void;
//        }
//        result;
//
//        // reserved for future use
//        union switch (int v)
//        {
//        case 0:
//            void;
//        }
//        ext;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InnerTransactionResult {
    pub fee_charged: Int64,
    pub result: InnerTransactionResultResult,
    pub ext: InnerTransactionResultExt,
}

impl ReadXDR for InnerTransactionResult {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            fee_charged: <Int64 as ReadXDR>::read_xdr(r)?,
            result: <InnerTransactionResultResult as ReadXDR>::read_xdr(r)?,
            ext: <InnerTransactionResultExt as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for InnerTransactionResult {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.fee_charged.write_xdr(w)?;
        self.result.write_xdr(w)?;
        self.ext.write_xdr(w)?;
        Ok(())
    }
}

// InnerTransactionResultPair is an XDR Struct defines as:
//
//   struct InnerTransactionResultPair
//    {
//        Hash transactionHash;          // hash of the inner transaction
//        InnerTransactionResult result; // result for the inner transaction
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InnerTransactionResultPair {
    pub transaction_hash: Hash,
    pub result: InnerTransactionResult,
}

impl ReadXDR for InnerTransactionResultPair {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            transaction_hash: <Hash as ReadXDR>::read_xdr(r)?,
            result: <InnerTransactionResult as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for InnerTransactionResultPair {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.transaction_hash.write_xdr(w)?;
        self.result.write_xdr(w)?;
        Ok(())
    }
}

// TransactionResultResult is an XDR NestedUnion defines as:
//
//   union switch (TransactionResultCode code)
//        {
//        case txFEE_BUMP_INNER_SUCCESS:
//        case txFEE_BUMP_INNER_FAILED:
//            InnerTransactionResultPair innerResultPair;
//        case txSUCCESS:
//        case txFAILED:
//            OperationResult results<>;
//        default:
//            void;
//        }
//
// union with discriminant TransactionResultCode
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TransactionResultResult {
    TxFeeBumpInnerSuccess(InnerTransactionResultPair),
    TxFeeBumpInnerFailed(InnerTransactionResultPair),
    TxSuccess(Vec<OperationResult>),
    TxFailed(Vec<OperationResult>),
}

impl TransactionResultResult {
    fn discriminant(&self) -> TransactionResultCode {
        match self {
            Self::TxFeeBumpInnerSuccess(_) => TransactionResultCode::TxFeeBumpInnerSuccess,
            Self::TxFeeBumpInnerFailed(_) => TransactionResultCode::TxFeeBumpInnerFailed,
            Self::TxSuccess(_) => TransactionResultCode::TxSuccess,
            Self::TxFailed(_) => TransactionResultCode::TxFailed,
        }
    }
}

impl ReadXDR for TransactionResultResult {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: TransactionResultCode = <TransactionResultCode as ReadXDR>::read_xdr(r)?;
        let v = match dv {
            TransactionResultCode::TxFeeBumpInnerSuccess => {
                Self::TxFeeBumpInnerSuccess(<InnerTransactionResultPair as ReadXDR>::read_xdr(r)?)
            }
            TransactionResultCode::TxFeeBumpInnerFailed => {
                Self::TxFeeBumpInnerFailed(<InnerTransactionResultPair as ReadXDR>::read_xdr(r)?)
            }
            TransactionResultCode::TxSuccess => {
                Self::TxSuccess(<Vec<OperationResult> as ReadXDR>::read_xdr(r)?)
            }
            TransactionResultCode::TxFailed => {
                Self::TxFailed(<Vec<OperationResult> as ReadXDR>::read_xdr(r)?)
            }
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for TransactionResultResult {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::TxFeeBumpInnerSuccess(v) => v.write_xdr(w)?,
            Self::TxFeeBumpInnerFailed(v) => v.write_xdr(w)?,
            Self::TxSuccess(v) => v.write_xdr(w)?,
            Self::TxFailed(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// TransactionResultExt is an XDR NestedUnion defines as:
//
//   union switch (int v)
//        {
//        case 0:
//            void;
//        }
//
// union with discriminant i32
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TransactionResultExt {
    V0,
}

impl TransactionResultExt {
    fn discriminant(&self) -> i32 {
        match self {
            Self::V0 => 0,
        }
    }
}

impl ReadXDR for TransactionResultExt {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: i32 = <i32 as ReadXDR>::read_xdr(r)?;
        let v = match dv.into() {
            0 => Self::V0,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for TransactionResultExt {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::V0 => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// TransactionResult is an XDR Struct defines as:
//
//   struct TransactionResult
//    {
//        int64 feeCharged; // actual fee charged for the transaction
//
//        union switch (TransactionResultCode code)
//        {
//        case txFEE_BUMP_INNER_SUCCESS:
//        case txFEE_BUMP_INNER_FAILED:
//            InnerTransactionResultPair innerResultPair;
//        case txSUCCESS:
//        case txFAILED:
//            OperationResult results<>;
//        default:
//            void;
//        }
//        result;
//
//        // reserved for future use
//        union switch (int v)
//        {
//        case 0:
//            void;
//        }
//        ext;
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransactionResult {
    pub fee_charged: Int64,
    pub result: TransactionResultResult,
    pub ext: TransactionResultExt,
}

impl ReadXDR for TransactionResult {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            fee_charged: <Int64 as ReadXDR>::read_xdr(r)?,
            result: <TransactionResultResult as ReadXDR>::read_xdr(r)?,
            ext: <TransactionResultExt as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for TransactionResult {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.fee_charged.write_xdr(w)?;
        self.result.write_xdr(w)?;
        self.ext.write_xdr(w)?;
        Ok(())
    }
}

// Hash is an XDR Typedef defines as:
//
//   typedef opaque Hash[32];
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Hash(pub [u8; 32]);

impl From<Hash> for [u8; 32] {
    fn from(x: Hash) -> Self {
        x.0
    }
}

impl From<[u8; 32]> for Hash {
    fn from(x: [u8; 32]) -> Self {
        Hash(x)
    }
}

impl ReadXDR for Hash {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let i = <[u8; 32] as ReadXDR>::read_xdr(r)?;
        let v = Hash(i);
        Ok(v)
    }
}

impl WriteXDR for Hash {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.0.write_xdr(w)
    }
}

// Uint256 is an XDR Typedef defines as:
//
//   typedef opaque uint256[32];
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Uint256(pub [u8; 32]);

impl From<Uint256> for [u8; 32] {
    fn from(x: Uint256) -> Self {
        x.0
    }
}

impl From<[u8; 32]> for Uint256 {
    fn from(x: [u8; 32]) -> Self {
        Uint256(x)
    }
}

impl ReadXDR for Uint256 {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let i = <[u8; 32] as ReadXDR>::read_xdr(r)?;
        let v = Uint256(i);
        Ok(v)
    }
}

impl WriteXDR for Uint256 {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.0.write_xdr(w)
    }
}

// Uint32 is an XDR Typedef defines as:
//
//   typedef unsigned int uint32;
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Uint32(pub u32);

impl From<Uint32> for u32 {
    fn from(x: Uint32) -> Self {
        x.0
    }
}

impl From<u32> for Uint32 {
    fn from(x: u32) -> Self {
        Uint32(x)
    }
}

impl ReadXDR for Uint32 {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let i = <u32 as ReadXDR>::read_xdr(r)?;
        let v = Uint32(i);
        Ok(v)
    }
}

impl WriteXDR for Uint32 {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.0.write_xdr(w)
    }
}

// Int32 is an XDR Typedef defines as:
//
//   typedef int int32;
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Int32(pub i32);

impl From<Int32> for i32 {
    fn from(x: Int32) -> Self {
        x.0
    }
}

impl From<i32> for Int32 {
    fn from(x: i32) -> Self {
        Int32(x)
    }
}

impl ReadXDR for Int32 {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let i = <i32 as ReadXDR>::read_xdr(r)?;
        let v = Int32(i);
        Ok(v)
    }
}

impl WriteXDR for Int32 {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.0.write_xdr(w)
    }
}

// Uint64 is an XDR Typedef defines as:
//
//   typedef unsigned hyper uint64;
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Uint64(pub u64);

impl From<Uint64> for u64 {
    fn from(x: Uint64) -> Self {
        x.0
    }
}

impl From<u64> for Uint64 {
    fn from(x: u64) -> Self {
        Uint64(x)
    }
}

impl ReadXDR for Uint64 {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let i = <u64 as ReadXDR>::read_xdr(r)?;
        let v = Uint64(i);
        Ok(v)
    }
}

impl WriteXDR for Uint64 {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.0.write_xdr(w)
    }
}

// Int64 is an XDR Typedef defines as:
//
//   typedef hyper int64;
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Int64(pub i64);

impl From<Int64> for i64 {
    fn from(x: Int64) -> Self {
        x.0
    }
}

impl From<i64> for Int64 {
    fn from(x: i64) -> Self {
        Int64(x)
    }
}

impl ReadXDR for Int64 {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let i = <i64 as ReadXDR>::read_xdr(r)?;
        let v = Int64(i);
        Ok(v)
    }
}

impl WriteXDR for Int64 {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.0.write_xdr(w)
    }
}

// ExtensionPoint is an XDR Union defines as:
//
//   union ExtensionPoint switch (int v)
//    {
//    case 0:
//        void;
//    };
//
// union with discriminant i32
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExtensionPoint {
    V0,
}

impl ExtensionPoint {
    fn discriminant(&self) -> i32 {
        match self {
            Self::V0 => 0,
        }
    }
}

impl ReadXDR for ExtensionPoint {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: i32 = <i32 as ReadXDR>::read_xdr(r)?;
        let v = match dv.into() {
            0 => Self::V0,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for ExtensionPoint {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::V0 => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// CryptoKeyType is an XDR Enum defines as:
//
//   enum CryptoKeyType
//    {
//        KEY_TYPE_ED25519 = 0,
//        KEY_TYPE_PRE_AUTH_TX = 1,
//        KEY_TYPE_HASH_X = 2,
//        KEY_TYPE_ED25519_SIGNED_PAYLOAD = 3,
//        // MUXED enum values for supported type are derived from the enum values
//        // above by ORing them with 0x100
//        KEY_TYPE_MUXED_ED25519 = 0x100
//    };
//
// enum
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum CryptoKeyType {
    KeyTypeEd25519 = 0,
    KeyTypePreAuthTx = 1,
    KeyTypeHashX = 2,
    KeyTypeEd25519SignedPayload = 3,
    KeyTypeMuxedEd25519 = 256,
}

impl TryFrom<i32> for CryptoKeyType {
    type Error = Error;

    fn try_from(i: i32) -> std::result::Result<Self, Self::Error> {
        let e = match i {
            0 => CryptoKeyType::KeyTypeEd25519,
            1 => CryptoKeyType::KeyTypePreAuthTx,
            2 => CryptoKeyType::KeyTypeHashX,
            3 => CryptoKeyType::KeyTypeEd25519SignedPayload,
            256 => CryptoKeyType::KeyTypeMuxedEd25519,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<CryptoKeyType> for i32 {
    fn from(e: CryptoKeyType) -> Self {
        e as Self
    }
}

impl ReadXDR for CryptoKeyType {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXDR for CryptoKeyType {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// PublicKeyType is an XDR Enum defines as:
//
//   enum PublicKeyType
//    {
//        PUBLIC_KEY_TYPE_ED25519 = KEY_TYPE_ED25519
//    };
//
// enum
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum PublicKeyType {
    PublicKeyTypeEd25519 = 0,
}

impl TryFrom<i32> for PublicKeyType {
    type Error = Error;

    fn try_from(i: i32) -> std::result::Result<Self, Self::Error> {
        let e = match i {
            0 => PublicKeyType::PublicKeyTypeEd25519,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<PublicKeyType> for i32 {
    fn from(e: PublicKeyType) -> Self {
        e as Self
    }
}

impl ReadXDR for PublicKeyType {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXDR for PublicKeyType {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// SignerKeyType is an XDR Enum defines as:
//
//   enum SignerKeyType
//    {
//        SIGNER_KEY_TYPE_ED25519 = KEY_TYPE_ED25519,
//        SIGNER_KEY_TYPE_PRE_AUTH_TX = KEY_TYPE_PRE_AUTH_TX,
//        SIGNER_KEY_TYPE_HASH_X = KEY_TYPE_HASH_X,
//        SIGNER_KEY_TYPE_ED25519_SIGNED_PAYLOAD = KEY_TYPE_ED25519_SIGNED_PAYLOAD
//    };
//
// enum
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum SignerKeyType {
    SignerKeyTypeEd25519 = 0,
    SignerKeyTypePreAuthTx = 1,
    SignerKeyTypeHashX = 2,
    SignerKeyTypeEd25519SignedPayload = 3,
}

impl TryFrom<i32> for SignerKeyType {
    type Error = Error;

    fn try_from(i: i32) -> std::result::Result<Self, Self::Error> {
        let e = match i {
            0 => SignerKeyType::SignerKeyTypeEd25519,
            1 => SignerKeyType::SignerKeyTypePreAuthTx,
            2 => SignerKeyType::SignerKeyTypeHashX,
            3 => SignerKeyType::SignerKeyTypeEd25519SignedPayload,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<SignerKeyType> for i32 {
    fn from(e: SignerKeyType) -> Self {
        e as Self
    }
}

impl ReadXDR for SignerKeyType {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXDR for SignerKeyType {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// PublicKey is an XDR Union defines as:
//
//   union PublicKey switch (PublicKeyType type)
//    {
//    case PUBLIC_KEY_TYPE_ED25519:
//        uint256 ed25519;
//    };
//
// union with discriminant PublicKeyType
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PublicKey {
    PublicKeyTypeEd25519(Uint256),
}

impl PublicKey {
    fn discriminant(&self) -> PublicKeyType {
        match self {
            Self::PublicKeyTypeEd25519(_) => PublicKeyType::PublicKeyTypeEd25519,
        }
    }
}

impl ReadXDR for PublicKey {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: PublicKeyType = <PublicKeyType as ReadXDR>::read_xdr(r)?;
        let v = match dv {
            PublicKeyType::PublicKeyTypeEd25519 => {
                Self::PublicKeyTypeEd25519(<Uint256 as ReadXDR>::read_xdr(r)?)
            }
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for PublicKey {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::PublicKeyTypeEd25519(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// SignerKeyEd25519SignedPayload is an XDR NestedStruct defines as:
//
//   struct
//        {
//            /* Public key that must sign the payload. */
//            uint256 ed25519;
//            /* Payload to be raw signed by ed25519. */
//            opaque payload<64>;
//        }
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SignerKeyEd25519SignedPayload {
    pub ed25519: Uint256,
    pub payload: Vec<u8>,
}

impl ReadXDR for SignerKeyEd25519SignedPayload {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            ed25519: <Uint256 as ReadXDR>::read_xdr(r)?,
            payload: <Vec<u8> as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for SignerKeyEd25519SignedPayload {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.ed25519.write_xdr(w)?;
        self.payload.write_xdr(w)?;
        Ok(())
    }
}

// SignerKey is an XDR Union defines as:
//
//   union SignerKey switch (SignerKeyType type)
//    {
//    case SIGNER_KEY_TYPE_ED25519:
//        uint256 ed25519;
//    case SIGNER_KEY_TYPE_PRE_AUTH_TX:
//        /* SHA-256 Hash of TransactionSignaturePayload structure */
//        uint256 preAuthTx;
//    case SIGNER_KEY_TYPE_HASH_X:
//        /* Hash of random 256 bit preimage X */
//        uint256 hashX;
//    case SIGNER_KEY_TYPE_ED25519_SIGNED_PAYLOAD:
//        struct
//        {
//            /* Public key that must sign the payload. */
//            uint256 ed25519;
//            /* Payload to be raw signed by ed25519. */
//            opaque payload<64>;
//        } ed25519SignedPayload;
//    };
//
// union with discriminant SignerKeyType
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SignerKey {
    SignerKeyTypeEd25519(Uint256),
    SignerKeyTypePreAuthTx(Uint256),
    SignerKeyTypeHashX(Uint256),
    SignerKeyTypeEd25519SignedPayload(SignerKeyEd25519SignedPayload),
}

impl SignerKey {
    fn discriminant(&self) -> SignerKeyType {
        match self {
            Self::SignerKeyTypeEd25519(_) => SignerKeyType::SignerKeyTypeEd25519,
            Self::SignerKeyTypePreAuthTx(_) => SignerKeyType::SignerKeyTypePreAuthTx,
            Self::SignerKeyTypeHashX(_) => SignerKeyType::SignerKeyTypeHashX,
            Self::SignerKeyTypeEd25519SignedPayload(_) => {
                SignerKeyType::SignerKeyTypeEd25519SignedPayload
            }
        }
    }
}

impl ReadXDR for SignerKey {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: SignerKeyType = <SignerKeyType as ReadXDR>::read_xdr(r)?;
        let v = match dv {
            SignerKeyType::SignerKeyTypeEd25519 => {
                Self::SignerKeyTypeEd25519(<Uint256 as ReadXDR>::read_xdr(r)?)
            }
            SignerKeyType::SignerKeyTypePreAuthTx => {
                Self::SignerKeyTypePreAuthTx(<Uint256 as ReadXDR>::read_xdr(r)?)
            }
            SignerKeyType::SignerKeyTypeHashX => {
                Self::SignerKeyTypeHashX(<Uint256 as ReadXDR>::read_xdr(r)?)
            }
            SignerKeyType::SignerKeyTypeEd25519SignedPayload => {
                Self::SignerKeyTypeEd25519SignedPayload(
                    <SignerKeyEd25519SignedPayload as ReadXDR>::read_xdr(r)?,
                )
            }
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXDR for SignerKey {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        match self {
            Self::SignerKeyTypeEd25519(v) => v.write_xdr(w)?,
            Self::SignerKeyTypePreAuthTx(v) => v.write_xdr(w)?,
            Self::SignerKeyTypeHashX(v) => v.write_xdr(w)?,
            Self::SignerKeyTypeEd25519SignedPayload(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// Signature is an XDR Typedef defines as:
//
//   typedef opaque Signature<64>;
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Signature(pub Vec<u8>);

impl From<Signature> for Vec<u8> {
    fn from(x: Signature) -> Self {
        x.0
    }
}

impl From<Vec<u8>> for Signature {
    fn from(x: Vec<u8>) -> Self {
        Signature(x)
    }
}

impl ReadXDR for Signature {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let i = <Vec<u8> as ReadXDR>::read_xdr(r)?;
        let v = Signature(i);
        Ok(v)
    }
}

impl WriteXDR for Signature {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.0.write_xdr(w)
    }
}

// SignatureHint is an XDR Typedef defines as:
//
//   typedef opaque SignatureHint[4];
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SignatureHint(pub [u8; 4]);

impl From<SignatureHint> for [u8; 4] {
    fn from(x: SignatureHint) -> Self {
        x.0
    }
}

impl From<[u8; 4]> for SignatureHint {
    fn from(x: [u8; 4]) -> Self {
        SignatureHint(x)
    }
}

impl ReadXDR for SignatureHint {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let i = <[u8; 4] as ReadXDR>::read_xdr(r)?;
        let v = SignatureHint(i);
        Ok(v)
    }
}

impl WriteXDR for SignatureHint {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.0.write_xdr(w)
    }
}

// NodeId is an XDR Typedef defines as:
//
//   typedef PublicKey NodeID;
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NodeId(pub PublicKey);

impl From<NodeId> for PublicKey {
    fn from(x: NodeId) -> Self {
        x.0
    }
}

impl From<PublicKey> for NodeId {
    fn from(x: PublicKey) -> Self {
        NodeId(x)
    }
}

impl ReadXDR for NodeId {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let i = <PublicKey as ReadXDR>::read_xdr(r)?;
        let v = NodeId(i);
        Ok(v)
    }
}

impl WriteXDR for NodeId {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.0.write_xdr(w)
    }
}

// Curve25519Secret is an XDR Struct defines as:
//
//   struct Curve25519Secret
//    {
//        opaque key[32];
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Curve25519Secret {
    pub key: [u8; 32],
}

impl ReadXDR for Curve25519Secret {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            key: <[u8; 32] as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for Curve25519Secret {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.key.write_xdr(w)?;
        Ok(())
    }
}

// Curve25519Public is an XDR Struct defines as:
//
//   struct Curve25519Public
//    {
//        opaque key[32];
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Curve25519Public {
    pub key: [u8; 32],
}

impl ReadXDR for Curve25519Public {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            key: <[u8; 32] as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for Curve25519Public {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.key.write_xdr(w)?;
        Ok(())
    }
}

// HmacSha256Key is an XDR Struct defines as:
//
//   struct HmacSha256Key
//    {
//        opaque key[32];
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HmacSha256Key {
    pub key: [u8; 32],
}

impl ReadXDR for HmacSha256Key {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            key: <[u8; 32] as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for HmacSha256Key {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.key.write_xdr(w)?;
        Ok(())
    }
}

// HmacSha256Mac is an XDR Struct defines as:
//
//   struct HmacSha256Mac
//    {
//        opaque mac[32];
//    };
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HmacSha256Mac {
    pub mac: [u8; 32],
}

impl ReadXDR for HmacSha256Mac {
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            mac: <[u8; 32] as ReadXDR>::read_xdr(r)?,
        })
    }
}

impl WriteXDR for HmacSha256Mac {
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.mac.write_xdr(w)?;
        Ok(())
    }
}
