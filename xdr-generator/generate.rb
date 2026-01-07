require 'xdrgen'
require_relative 'generator/generator'

puts "Generating..."

# Operate on the root directory of the repo.
Dir.chdir("..")

# Shared options for both compilations (curr, and next).
options = {
  custom_default_impl: [
    "TransactionEnvelope",
  ],
  custom_str_impl: [
    "PublicKey", "AccountId", "ContractId", "MuxedAccount",
    "MuxedAccountMed25519", "SignerKey", "SignerKeyEd25519SignedPayload",
    "NodeId", "ScAddress", "AssetCode", "AssetCode4", "AssetCode12",
    "ClaimableBalanceId", "PoolId", "MuxedEd25519Account", "Int128Parts",
    "UInt128Parts", "Int256Parts", "UInt256Parts",
  ],
  # Sparse types configuration: generate decode-only types that extract specific nested fields.
  # Each sparse type config specifies:
  #   - base_type: The original XDR type name to create a sparse version of
  #   - name: The name for the generated sparse type
  #   - paths: Array of paths to fields to extract (all other fields are skipped during decode)
  #     Path format: "VariantName.field_name.nested_field" for unions/structs
  #     Use "[]" suffix to indicate traversing through a Vec (e.g., "tx_processing[].result.transaction_hash")
  sparse_types: [
    {
      base_type: "TransactionEnvelope",
      name: "TransactionEnvelopeSparse",
      paths: [
        "Tx.tx.operations",           # TransactionEnvelope::Tx -> v1.tx.operations
        "TxV0.tx.operations",         # TransactionEnvelope::TxV0 -> v0.tx.operations
        "TxFeeBump.tx.innerTx.Tx.tx.operations",  # Through FeeBumpTransaction
      ],
    },
    {
      base_type: "LedgerCloseMeta",
      name: "LedgerCloseMetaTxHashes",
      paths: [
        "V0.tx_processing[].result.transaction_hash",
        "V1.tx_processing[].result.transaction_hash",
        "V2.tx_processing[].result.transaction_hash",
      ],
    },
  ],
}

# Compile the curr XDR into Rust.
Xdrgen::Compilation.new(
  Dir.glob("xdr/curr/*.x"),
  output_dir: "src/curr/",
  generator: Generator,
  namespace: "generated",
  options: options,
).compile

# Compile the next XDR into Rust.
Xdrgen::Compilation.new(
  Dir.glob("xdr/next/*.x"),
  output_dir: "src/next/",
  generator: Generator,
  namespace: "generated",
  options: options,
).compile
