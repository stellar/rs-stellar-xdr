require 'xdrgen'
require_relative 'generator/generator'
require_relative 'type_deduplicator'
require 'tempfile'
require 'fileutils'

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
puts "Generating curr..."
Xdrgen::Compilation.new(
  Dir.glob("xdr/curr/*.x"),
  output_dir: "src/curr/",
  generator: Generator,
  namespace: "generated",
  options: options,
).compile

# Generate next XDR into a temporary location first for comparison
puts "Generating next (temporary)..."
temp_dir = Dir.mktmpdir("next_temp")
temp_next_compilation = Xdrgen::Compilation.new(
  Dir.glob("xdr/next/*.x"),
  output_dir: temp_dir + "/",
  generator: Generator,
  namespace: "generated",
  options: options,
)
temp_next_compilation.compile

# Compare and create deduplicated version
puts "Deduplicating types..."
deduplicator = TypeDeduplicator.new(
  curr_file: "src/curr/generated.rs",
  next_file: temp_dir + "/generated.rs"
)

deduplicated_content = deduplicator.generate_deduplicated_next

# Write the deduplicated version
File.write("src/next/generated.rs", deduplicated_content)

# Clean up temporary directory
FileUtils.rm_rf(temp_dir)

puts "Generation complete with deduplication."
