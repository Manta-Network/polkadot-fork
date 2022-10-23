paleblue:
  [[ $(git rev-parse --abbrev-ref HEAD) == "manta-staging" ]]
  cargo run --release -- build-spec --chain polkadot-staging --raw  > polkadot-relaychain-spec.json
  cp target/release/wbuild/polkadot-runtime/polkadot_runtime.compact.compressed.wasm polkadot_manta_staging_runtime.compact.compressed.wasm

seabird:
  [[ $(git rev-parse --abbrev-ref HEAD) == "manta-staging" ]]
  cargo run --release -- build-spec --chain kusama-staging --raw  > kusama-relaychain-spec.json
  cp target/release/wbuild/kusama-runtime/kusama_runtime.compact.compressed.wasm kusama_calamari_staging_runtime.compact.compressed.wasm

baikal:
  [[ $(git rev-parse --abbrev-ref HEAD) == "manta-staging" ]]
  cargo run --release --features=fast-runtime -- build-spec --chain rococo-local --raw  > baikal-relaychain-spec.json
  cp target/release/wbuild/kusama-runtime/kusama_runtime.compact.compressed.wasm kusama_baikal_runtime.compact.compressed.wasm

try-runtime-kusama-staging:
  [[ $(git rev-parse --abbrev-ref HEAD) == "manta-staging" ]]
  cargo run --release --features try-runtime -- \
  try-runtime --chain kusama-staging --wasm-execution=compiled --no-spec-check-panic \
  on-runtime-upgrade live --uri=wss://v3.internal.kusama.systems:443/1

emergency-solution chain="wss://v1.internal.kusama.systems:443/1":
  cargo build --release --locked --package staking-miner
  # RUSTC_LOG=trace target/release/staking-miner emergency-solution seq-phragmen --uri "{{chain}}"
  RUST_LOG=debug target/release/staking-miner emergency-solution phrag-mms --uri "{{chain}}"

pin v:
    rustup override set {{v}}

unpin :
    rustup override unset
