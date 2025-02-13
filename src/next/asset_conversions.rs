use super::{
    AlphaNum12, AlphaNum4, Asset, AssetCode12, AssetCode4, ChangeTrustAsset, ContractIdPreimage,
};
#[cfg(feature = "sha2")]
use super::{Hash, HashIdPreimage, HashIdPreimageContractId};

#[cfg(feature = "sha2")]
impl Asset {
    pub fn into_contract_id(
        self,
        network_passphrase: &str,
    ) -> Result<stellar_strkey::Contract, super::Error> {
        let network_id = Hash::hash_bytes(network_passphrase);
        HashIdPreimage::ContractId(HashIdPreimageContractId {
            network_id,
            contract_id_preimage: self.into(),
        })
        .try_into()
    }
}

impl From<Asset> for ContractIdPreimage {
    fn from(value: Asset) -> Self {
        ContractIdPreimage::Asset(value)
    }
}

impl From<Asset> for ChangeTrustAsset {
    fn from(asset: Asset) -> Self {
        match asset {
            Asset::CreditAlphanum4(asset) => ChangeTrustAsset::CreditAlphanum4(asset),
            Asset::CreditAlphanum12(asset) => ChangeTrustAsset::CreditAlphanum12(asset),
            Asset::Native => ChangeTrustAsset::Native,
        }
    }
}

impl From<&Asset> for ChangeTrustAsset {
    fn from(asset: &Asset) -> Self {
        match asset {
            Asset::CreditAlphanum4(asset) => ChangeTrustAsset::CreditAlphanum4(asset.into()),
            Asset::CreditAlphanum12(asset) => ChangeTrustAsset::CreditAlphanum12(asset.into()),
            Asset::Native => ChangeTrustAsset::Native,
        }
    }
}

impl From<&AssetCode4> for AssetCode4 {
    fn from(AssetCode4(inner): &AssetCode4) -> Self {
        AssetCode4(*inner)
    }
}

impl From<&AssetCode12> for AssetCode12 {
    fn from(AssetCode12(inner): &AssetCode12) -> Self {
        AssetCode12(*inner)
    }
}

impl From<&AlphaNum4> for AlphaNum4 {
    fn from(AlphaNum4 { asset_code, issuer }: &AlphaNum4) -> Self {
        AlphaNum4 {
            asset_code: asset_code.into(),
            issuer: issuer.into(),
        }
    }
}

impl From<&AlphaNum12> for AlphaNum12 {
    fn from(AlphaNum12 { asset_code, issuer }: &AlphaNum12) -> Self {
        AlphaNum12 {
            asset_code: asset_code.into(),
            issuer: issuer.into(),
        }
    }
}
