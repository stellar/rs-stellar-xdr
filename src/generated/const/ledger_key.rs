#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// LedgerKey is a borrowing equivalent of [`LedgerKey`](super::super::LedgerKey)
/// over `'static` data, for const XDR encoding.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(clippy::large_enum_variant)]
pub enum LedgerKey {
    Account(LedgerKeyAccount),
    Trustline(LedgerKeyTrustLine),
    Offer(LedgerKeyOffer),
    Data(LedgerKeyData),
    ClaimableBalance(LedgerKeyClaimableBalance),
    LiquidityPool(LedgerKeyLiquidityPool),
    ContractData(LedgerKeyContractData),
    ContractCode(LedgerKeyContractCode),
    ConfigSetting(LedgerKeyConfigSetting),
    Ttl(LedgerKeyTtl),
}

impl LedgerKey {
    #[must_use]
    pub const fn discriminant(&self) -> LedgerEntryType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Account(_) => LedgerEntryType::Account,
            Self::Trustline(_) => LedgerEntryType::Trustline,
            Self::Offer(_) => LedgerEntryType::Offer,
            Self::Data(_) => LedgerEntryType::Data,
            Self::ClaimableBalance(_) => LedgerEntryType::ClaimableBalance,
            Self::LiquidityPool(_) => LedgerEntryType::LiquidityPool,
            Self::ContractData(_) => LedgerEntryType::ContractData,
            Self::ContractCode(_) => LedgerEntryType::ContractCode,
            Self::ConfigSetting(_) => LedgerEntryType::ConfigSetting,
            Self::Ttl(_) => LedgerEntryType::Ttl,
        }
    }
}

impl LedgerKey {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_ledger_key(self);
        w.len()
    }

    /// Serialize this value as XDR into a fixed-size `[u8; N]` using only const
    /// operations. This is the const counterpart to
    /// [`WriteXdr::to_xdr`](super::super::WriteXdr::to_xdr).
    ///
    /// `N` must equal [`Self::const_xdr_len`]. It is intended for callers, such
    /// as a proc-macro, that compute the length with `const_xdr_len` and pass
    /// it as `N`; `const_to_xdr` itself does not need to call `const_xdr_len`.
    ///
    /// # Panics
    ///
    /// Panics if `N` does not equal the value's [`Self::const_xdr_len`].
    #[must_use]
    pub const fn const_to_xdr<const N: usize>(&self) -> [u8; N] {
        let mut buf = [0u8; N];
        let mut w = ConstWriter::new(&mut buf);
        w.write_type_ledger_key(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

impl ConstWriter<'_> {
    /// Serializes a [`LedgerKey`], mirroring `<LedgerKey as WriteXdr>::write_xdr`.
    pub const fn write_type_ledger_key(&mut self, v: &LedgerKey) {
        let d = v.discriminant();
        self.write_type_ledger_entry_type(&d);
        #[allow(clippy::match_same_arms)]
        match v {
            LedgerKey::Account(value) => {
                self.write_type_ledger_key_account(value);
            }
            LedgerKey::Trustline(value) => {
                self.write_type_ledger_key_trust_line(value);
            }
            LedgerKey::Offer(value) => {
                self.write_type_ledger_key_offer(value);
            }
            LedgerKey::Data(value) => {
                self.write_type_ledger_key_data(value);
            }
            LedgerKey::ClaimableBalance(value) => {
                self.write_type_ledger_key_claimable_balance(value);
            }
            LedgerKey::LiquidityPool(value) => {
                self.write_type_ledger_key_liquidity_pool(value);
            }
            LedgerKey::ContractData(value) => {
                self.write_type_ledger_key_contract_data(value);
            }
            LedgerKey::ContractCode(value) => {
                self.write_type_ledger_key_contract_code(value);
            }
            LedgerKey::ConfigSetting(value) => {
                self.write_type_ledger_key_config_setting(value);
            }
            LedgerKey::Ttl(value) => {
                self.write_type_ledger_key_ttl(value);
            }
        }
    }

    /// Serializes a variable-length array of [`LedgerKey`], mirroring `<VecM<LedgerKey, MAX> as WriteXdr>::write_xdr`.
    pub const fn write_type_vec_ledger_key<const MAX: u32>(&mut self, v: &VecM<LedgerKey, MAX>) {
        let s = v.as_slice();
        let len = s.len();
        self.write_len(len);
        let mut i = 0usize;
        while i < len {
            self.write_type_ledger_key(&s[i]);
            i += 1;
        }
    }
}
