<p align="center" width="100%">
  <a href="https://ajuna.io" target="_blank">
    <img src="docs/ajuna-banner.jpeg" alt="Ajuna Network">
  </a>
</p>

[![Build](https://github.com/ajuna-network/ajuna-runtimes-bajun/actions/workflows/check-pull-request.yml/badge.svg?branch=main)](https://github.com/ajuna-network/Ajuna/actions/workflows/check-pull-request.yml)
[![codecov](https://codecov.io/gh/ajuna-network/ajuna-runtimes-bajun/branch/main/graph/badge.svg?token=V2Y88ZUD6C)](https://codecov.io/gh/ajuna-network/Ajuna)
[![Docker Image Version (latest semver)](https://img.shields.io/docker/v/ajuna/parachain-bajun?label=bajun%20network&logo=docker&sort=semver&style=plastic)](https://hub.docker.com/repository/docker/ajuna/parachain-bajun/tags?page=1&ordering=last_updated)

A game platform [parachain](https://wiki.polkadot.network/docs/learn-parachains) built
with [Substrate](https://docs.substrate.io/).

## Chopsticks

Chopsticks can be used to create a local fork of a life network, copying all its storage values and override specific
storage values with a config file. The fork will create a block whenever an extrinsic is added to the tx-pool.
Otherwise, it will be stale, but exposes a fast-forward rpc to produce 'n' blocks, which is useful for scheduler
testing.

Chopsticks can be used to:

* Test runtime upgrades and functionality thereafter
* Setup XCM with multiple networks and allow for XCM testing
* Test execution of scheduled task, which is mostly execution of governance proposals.

**Preliminary Note**
Chopsticks will by default listen on the IPV6 localhost port 8000, which means we can either access it with:

* ws://localhost:8000
* ws://[::1]:8000

but ws://127.0.0.1:8000 doesn't work.

### Test Runtime Upgrades

Note: Currently, try-runtime seems to be a better solution to test the runtime upgrade itself, but chopsticks will be
good to test functionality after the runtime upgrade happened.

The following command overrides the runtime with the local build of the upgraded runtime and will run the migration
upon the next block, which means as soon as we send an arbitrary extrinsic.

#### Test Runtime Upgrades with chopsticks

We now have a zombienet setup that is intended to test runtime upgrades at `zombienet/`. See the documentation there.

```bash
nvm use 20
npx @acala-network/chopsticks@latest --config ./chopsticks/bajun.yml  --wasm-override ./target/release/wbuild/bajun-runtime/bajun_runtime.compact.compressed.wasm
```

Then we can send an arbitrary extrinsic to trigger the next block and observe the logs of the runtime upgrade.

### Test XCM stuff

Test Basilisk integration. Note: the tx links will work only if the local setup with chopsticks is running.

1. Setup local network
   with `npx @acala-network/chopsticks@latest xcm --p=chopsticks/bajun.yml --p=chopsticks/basilisk.yml`
2. Transfer from Bajun to Basilisk: Our token has token id 18 on basilisk, so you can send this
   tx: [Transfer Bajun to Alice on Basilisk](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2Flocalhost%3A8000#/extrinsics/decode/0x2300000010a5d4e8000000000000000000000003010200a9200100d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d00)
3. Transfer from Basilisk to
   Bajun: [Transfer Bajun to Alice Stash on Bajun](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2Flocalhost%3A8001#/extrinsics/decode/0x9a001200000000e87648170000000000000000000000030102001d210100be5ddb1579b72e84524fc29e78609e3caf42e85aa118ebfe0b0ad404b5bdd25f00)

### Test Governance execution

Simply create an external proposal and fast-track it with a small voting period and delay, e.g. 10 blocks. Then, in
polkadot-js/apps, go to Developer -> JavaScript and execute the following chopsticks-dev-rpc:
`api.rpc('dev_newBlock', { count: 10 })`. After this you can observe the blocks being created one by one until the
proposal is executed.

### Creating new production network chainspecs.

We have parametrized chain-spec generation to simplify creating new production network chain-specs without manual
copy-pasting.
This helps to prevent human errors, and the code also serves as documentation at the same time. How to create a new
production
chain-spec:

1. Add a new `*-fresh` entry in the [node/src/command], e.g., `bajun-westend-fresh`.
2. Ensure that the collator and the runtime is of the most recent build, and run the
   script: `scripts/dump_wasm_state_and_spec.sh bajun-westend-fresh`.
3. Copy the `chain_dumps/bajun-westend-fresh-raw.json`/`-.state`/`-.wasm` into `resources/bajun/westend`.
4. Rename the files to remove the `-fresh` suffix.
5. Delete the unsorted chain-spec.
6. Add a new function into `node/src/chain_spec.rs`

```rust
pub fn bajun_westend_config() -> Result<ChainSpec, String> {
    ChainSpec::from_json_bytes(&include_bytes!("../../resources/bajun/bajun-westend-raw.json")[..])
}
```

7. Refer to the new function in `node/src/command.rs`.
## Build

- Using `cargo`:

  ```bash
  # parachain with Bajun runtime
  cargo build --release
  ```
- Using `Docker`:

  ```bash
  # parachain with Bajun runtime
  docker build -f docker/Dockerfile -t ajuna/parachain-bajun:latest . --build-arg bin=bajun-node
  ```
