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
