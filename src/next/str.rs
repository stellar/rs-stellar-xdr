//# Custom string representations of the following types, also used for JSON
//# formatting.
//#
//# The types that has impls in this file are given to the xdrgen
//# --rust-types-custom-str-impl cli option, so that xdrgen does not generate
//# FromStr and Display impls for them.
//#
//# ## Strkey Types (SEP-23)
//# - PublicKey
//# - AccountId
//# - MuxedAccount
//# - MuxedAccountMed25519
//# - SignerKey
//# - SignerKeyEd25519SignedPayload
//# - NodeId
//#
//# ## Asset Codes
//# - AssetCode
//# - AssetCode4
//# - AssetCode12
//#
//# ## Integers
//# - Int128Parts
//# - UInt128Parts
//# - Int256Parts
//# - UInt256Parts
//#
//# ## Other
//# - ClaimableBalanceId
//# - PoolId
#![cfg(feature = "alloc")]

use super::{
    super::num::{
        i256_str_from_pieces, i256_str_into_pieces, u256_str_from_pieces, u256_str_into_pieces,
    },
    AccountId, AssetCode, AssetCode12, AssetCode4, ClaimableBalanceId, ContractId, Error, Hash,
    Int128Parts, Int256Parts, MuxedAccount, MuxedAccountMed25519, MuxedEd25519Account, NodeId,
    PoolId, PublicKey, ScAddress, SignerKey, SignerKeyEd25519SignedPayload, UInt128Parts,
    UInt256Parts, Uint256,
};

impl From<stellar_strkey::DecodeError> for Error {
    fn from(_: stellar_strkey::DecodeError) -> Self {
        Error::Invalid
    }
}

impl core::fmt::Display for PublicKey {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            PublicKey::PublicKeyTypeEd25519(Uint256(k)) => {
                let k = stellar_strkey::ed25519::PublicKey::from_payload(k)
                    .map_err(|_| core::fmt::Error)?;
                let s = k.to_string();
                f.write_str(&s)?;
            }
        }
        Ok(())
    }
}

impl core::str::FromStr for PublicKey {
    type Err = Error;
    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        let stellar_strkey::ed25519::PublicKey(k) =
            stellar_strkey::ed25519::PublicKey::from_str(s)?;
        Ok(PublicKey::PublicKeyTypeEd25519(Uint256(k)))
    }
}

impl core::fmt::Display for AccountId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

impl core::str::FromStr for AccountId {
    type Err = Error;
    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        Ok(AccountId(PublicKey::from_str(s)?))
    }
}

impl core::fmt::Display for ContractId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let k = stellar_strkey::Contract(self.0 .0);
        let s = k.to_string();
        f.write_str(&s)?;
        Ok(())
    }
}

impl core::str::FromStr for ContractId {
    type Err = Error;
    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        let stellar_strkey::Contract(h) = stellar_strkey::Contract::from_str(s)?;
        Ok(ContractId(Hash(h)))
    }
}

impl core::fmt::Display for PoolId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let PoolId(Hash(p_id)) = self.clone();
        let key = stellar_strkey::Strkey::LiquidityPool(stellar_strkey::LiquidityPool(p_id));
        key.fmt(f)
    }
}

impl core::str::FromStr for PoolId {
    type Err = Error;
    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        let pool_key = stellar_strkey::LiquidityPool::from_str(s)?;
        Ok(PoolId(Hash(pool_key.0)))
    }
}

impl core::fmt::Display for MuxedAccountMed25519 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let MuxedAccountMed25519 {
            ed25519: Uint256(ed25519),
            id,
        } = self;
        let k = stellar_strkey::ed25519::MuxedAccount {
            ed25519: *ed25519,
            id: *id,
        };
        let s = k.to_string();
        f.write_str(&s)?;
        Ok(())
    }
}

impl core::str::FromStr for MuxedAccountMed25519 {
    type Err = Error;
    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        let stellar_strkey::ed25519::MuxedAccount { ed25519, id } =
            stellar_strkey::ed25519::MuxedAccount::from_str(s)?;
        Ok(MuxedAccountMed25519 {
            ed25519: Uint256(ed25519),
            id,
        })
    }
}

impl core::str::FromStr for MuxedAccount {
    type Err = Error;
    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        let strkey = stellar_strkey::Strkey::from_str(s)?;
        match strkey {
            stellar_strkey::Strkey::PublicKeyEd25519(stellar_strkey::ed25519::PublicKey(k)) => {
                Ok(MuxedAccount::Ed25519(Uint256(k)))
            }
            stellar_strkey::Strkey::MuxedAccountEd25519(
                stellar_strkey::ed25519::MuxedAccount { ed25519, id },
            ) => Ok(MuxedAccount::MuxedEd25519(MuxedAccountMed25519 {
                ed25519: Uint256(ed25519),
                id,
            })),
            stellar_strkey::Strkey::PrivateKeyEd25519(_)
            | stellar_strkey::Strkey::PreAuthTx(_)
            | stellar_strkey::Strkey::HashX(_)
            | stellar_strkey::Strkey::SignedPayloadEd25519(_)
            | stellar_strkey::Strkey::Contract(_)
            | stellar_strkey::Strkey::LiquidityPool(_)
            | stellar_strkey::Strkey::ClaimableBalance(_) => Err(Error::Invalid),
        }
    }
}

impl core::fmt::Display for MuxedAccount {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            MuxedAccount::Ed25519(Uint256(k)) => {
                let k = stellar_strkey::ed25519::PublicKey(*k);
                let s = k.to_string();
                f.write_str(&s)?;
            }
            MuxedAccount::MuxedEd25519(m) => m.fmt(f)?,
        }
        Ok(())
    }
}

impl core::fmt::Display for NodeId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

impl core::str::FromStr for NodeId {
    type Err = Error;
    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        Ok(NodeId(PublicKey::from_str(s)?))
    }
}

impl core::fmt::Display for SignerKeyEd25519SignedPayload {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let SignerKeyEd25519SignedPayload {
            ed25519: Uint256(ed25519),
            payload,
        } = self;
        let k = stellar_strkey::ed25519::SignedPayload {
            ed25519: *ed25519,
            payload: payload.into(),
        };
        let s = k.to_string();
        f.write_str(&s)?;
        Ok(())
    }
}

impl core::str::FromStr for SignerKeyEd25519SignedPayload {
    type Err = Error;
    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        let stellar_strkey::ed25519::SignedPayload { ed25519, payload } =
            stellar_strkey::ed25519::SignedPayload::from_str(s)?;
        Ok(SignerKeyEd25519SignedPayload {
            ed25519: Uint256(ed25519),
            payload: payload.try_into()?,
        })
    }
}

impl core::str::FromStr for SignerKey {
    type Err = Error;
    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        let strkey = stellar_strkey::Strkey::from_str(s)?;
        match strkey {
            stellar_strkey::Strkey::PublicKeyEd25519(stellar_strkey::ed25519::PublicKey(k)) => {
                Ok(SignerKey::Ed25519(Uint256(k)))
            }
            stellar_strkey::Strkey::PreAuthTx(stellar_strkey::PreAuthTx(h)) => {
                Ok(SignerKey::PreAuthTx(Uint256(h)))
            }
            stellar_strkey::Strkey::HashX(stellar_strkey::HashX(h)) => {
                Ok(SignerKey::HashX(Uint256(h)))
            }
            stellar_strkey::Strkey::SignedPayloadEd25519(
                stellar_strkey::ed25519::SignedPayload { ed25519, payload },
            ) => Ok(SignerKey::Ed25519SignedPayload(
                SignerKeyEd25519SignedPayload {
                    ed25519: Uint256(ed25519),
                    payload: payload.try_into()?,
                },
            )),
            stellar_strkey::Strkey::PrivateKeyEd25519(_)
            | stellar_strkey::Strkey::Contract(_)
            | stellar_strkey::Strkey::MuxedAccountEd25519(_)
            | stellar_strkey::Strkey::LiquidityPool(_)
            | stellar_strkey::Strkey::ClaimableBalance(_) => Err(Error::Invalid),
        }
    }
}

impl core::fmt::Display for SignerKey {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            SignerKey::Ed25519(Uint256(k)) => {
                let k = stellar_strkey::ed25519::PublicKey(*k);
                let s = k.to_string();
                f.write_str(&s)?;
            }
            SignerKey::PreAuthTx(Uint256(h)) => {
                let k = stellar_strkey::PreAuthTx(*h);
                let s = k.to_string();
                f.write_str(&s)?;
            }
            SignerKey::HashX(Uint256(h)) => {
                let k = stellar_strkey::HashX(*h);
                let s = k.to_string();
                f.write_str(&s)?;
            }
            SignerKey::Ed25519SignedPayload(p) => p.fmt(f)?,
        }
        Ok(())
    }
}

impl core::str::FromStr for MuxedEd25519Account {
    type Err = Error;
    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        let strkey = stellar_strkey::Strkey::from_str(s)?;
        match strkey {
            stellar_strkey::Strkey::MuxedAccountEd25519(muxed_ed25519) => Ok(MuxedEd25519Account {
                id: muxed_ed25519.id,
                ed25519: Uint256(muxed_ed25519.ed25519),
            }),
            _ => Err(Error::Invalid),
        }
    }
}

impl core::fmt::Display for MuxedEd25519Account {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let k =
            stellar_strkey::Strkey::MuxedAccountEd25519(stellar_strkey::ed25519::MuxedAccount {
                ed25519: self.ed25519.0,
                id: self.id,
            });
        let s = k.to_string();
        f.write_str(&s)
    }
}

impl core::str::FromStr for ScAddress {
    type Err = Error;
    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        let strkey = stellar_strkey::Strkey::from_str(s)?;
        match strkey {
            stellar_strkey::Strkey::PublicKeyEd25519(stellar_strkey::ed25519::PublicKey(k)) => Ok(
                ScAddress::Account(AccountId(PublicKey::PublicKeyTypeEd25519(Uint256(k)))),
            ),
            stellar_strkey::Strkey::Contract(stellar_strkey::Contract(h)) => {
                Ok(ScAddress::Contract(ContractId(Hash(h))))
            }
            stellar_strkey::Strkey::MuxedAccountEd25519(muxed_ed25519) => {
                Ok(ScAddress::MuxedAccount(MuxedEd25519Account {
                    id: muxed_ed25519.id,
                    ed25519: Uint256(muxed_ed25519.ed25519),
                }))
            }
            stellar_strkey::Strkey::LiquidityPool(liquidity_pool) => {
                Ok(ScAddress::LiquidityPool(PoolId(Hash(liquidity_pool.0))))
            }
            stellar_strkey::Strkey::ClaimableBalance(stellar_strkey::ClaimableBalance::V0(
                claimable_balance,
            )) => Ok(ScAddress::ClaimableBalance(
                ClaimableBalanceId::ClaimableBalanceIdTypeV0(Hash(claimable_balance)),
            )),
            stellar_strkey::Strkey::PrivateKeyEd25519(_)
            | stellar_strkey::Strkey::PreAuthTx(_)
            | stellar_strkey::Strkey::HashX(_)
            | stellar_strkey::Strkey::SignedPayloadEd25519(_) => Err(Error::Invalid),
        }
    }
}

impl core::fmt::Display for ScAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ScAddress::Account(a) => a.fmt(f),
            ScAddress::Contract(ContractId(Hash(h))) => {
                let k = stellar_strkey::Contract(*h);
                let s = k.to_string();
                f.write_str(&s)
            }
            ScAddress::MuxedAccount(muxed_ed25519_account) => {
                let k = stellar_strkey::Strkey::MuxedAccountEd25519(
                    stellar_strkey::ed25519::MuxedAccount {
                        ed25519: muxed_ed25519_account.ed25519.0,
                        id: muxed_ed25519_account.id,
                    },
                );
                let s = k.to_string();
                f.write_str(&s)
            }
            ScAddress::ClaimableBalance(claimable_balance_id) => claimable_balance_id.fmt(f),
            ScAddress::LiquidityPool(pool_id) => pool_id.fmt(f),
        }
    }
}

impl core::str::FromStr for AssetCode4 {
    type Err = Error;
    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        let mut code = AssetCode4([0u8; 4]);
        escape_bytes::unescape_into(&mut code.0, s.as_bytes()).map_err(|_| Error::Invalid)?;
        Ok(code)
    }
}

impl core::fmt::Display for AssetCode4 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if let Some(last_idx) = self.0.iter().rposition(|c| *c != 0) {
            for b in escape_bytes::Escape::new(&self.0[..=last_idx]) {
                write!(f, "{}", b as char)?;
            }
        }
        Ok(())
    }
}

impl core::str::FromStr for AssetCode12 {
    type Err = Error;
    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        let mut code = AssetCode12([0u8; 12]);
        escape_bytes::unescape_into(&mut code.0, s.as_bytes()).map_err(|_| Error::Invalid)?;
        Ok(code)
    }
}

impl core::fmt::Display for AssetCode12 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if let Some(last_idx) = self.0.iter().rposition(|c| *c != 0) {
            for b in escape_bytes::Escape::new(&self.0[..=last_idx]) {
                write!(f, "{}", b as char)?;
            }
        }
        Ok(())
    }
}

impl core::str::FromStr for AssetCode {
    type Err = Error;
    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        let mut code = [0u8; 12];
        let n = escape_bytes::unescape_into(&mut code, s.as_bytes()).map_err(|_| Error::Invalid)?;
        if n <= 4 {
            Ok(AssetCode::CreditAlphanum4(AssetCode4([
                code[0], code[1], code[2], code[3],
            ])))
        } else if n <= 12 {
            Ok(AssetCode::CreditAlphanum12(AssetCode12(code)))
        } else {
            Err(Error::Invalid)
        }
    }
}

impl core::fmt::Display for AssetCode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            AssetCode::CreditAlphanum4(c) => c.fmt(f),
            AssetCode::CreditAlphanum12(c) => c.fmt(f),
        }
    }
}

impl core::str::FromStr for ClaimableBalanceId {
    type Err = Error;
    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        let stellar_strkey::ClaimableBalance::V0(cb_id) =
            stellar_strkey::ClaimableBalance::from_str(s)?;
        Ok(ClaimableBalanceId::ClaimableBalanceIdTypeV0(Hash(cb_id)))
    }
}

impl core::fmt::Display for ClaimableBalanceId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let ClaimableBalanceId::ClaimableBalanceIdTypeV0(Hash(cb_id)) = self.clone();
        let key =
            stellar_strkey::Strkey::ClaimableBalance(stellar_strkey::ClaimableBalance::V0(cb_id));
        key.fmt(f)
    }
}

impl core::str::FromStr for Int128Parts {
    type Err = Error;
    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        let v = s.parse::<i128>().map_err(|_| Error::Invalid)?;
        let hi = (v >> 64) as i64;
        let lo = v as u64;
        Ok(Self { hi, lo })
    }
}

impl core::fmt::Display for Int128Parts {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let v = ((self.hi as i128) << 64) | (self.lo as i128);
        write!(f, "{v}")
    }
}

impl core::str::FromStr for UInt128Parts {
    type Err = Error;
    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        let v = s.parse::<u128>().map_err(|_| Error::Invalid)?;
        let hi = (v >> 64) as u64;
        let lo = v as u64;
        Ok(Self { hi, lo })
    }
}

impl core::fmt::Display for UInt128Parts {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let v = ((self.hi as u128) << 64) | (self.lo as u128);
        write!(f, "{v}")
    }
}

impl core::str::FromStr for Int256Parts {
    type Err = Error;
    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        let (hi_hi, hi_lo, lo_hi, lo_lo) = i256_str_into_pieces(s).map_err(|_| Error::Invalid)?;
        Ok(Self {
            hi_hi,
            hi_lo,
            lo_hi,
            lo_lo,
        })
    }
}

impl core::fmt::Display for Int256Parts {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let i256 = i256_str_from_pieces(self.hi_hi, self.hi_lo, self.lo_hi, self.lo_lo);
        write!(f, "{i256}")
    }
}

impl core::str::FromStr for UInt256Parts {
    type Err = Error;
    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        let (hi_hi, hi_lo, lo_hi, lo_lo) = u256_str_into_pieces(s).map_err(|_| Error::Invalid)?;
        Ok(Self {
            hi_hi,
            hi_lo,
            lo_hi,
            lo_lo,
        })
    }
}

impl core::fmt::Display for UInt256Parts {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let u256 = u256_str_from_pieces(self.hi_hi, self.hi_lo, self.lo_hi, self.lo_lo);
        write!(f, "{u256}")
    }
}
