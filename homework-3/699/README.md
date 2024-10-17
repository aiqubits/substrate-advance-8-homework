## 编译
```bash
cd polkadot-sdk-solochain-template/
cargo test --package pallet-poe --features runtime-benchmarks
# cargo build --profile=release --features runtime-benchmarks
cargo build --profile=dev --features runtime-benchmarks

```

## 运行 benchmark
```bash
# download frame-weight-template.hbs [polkadot-sdk repo](https://github.com/paritytech/polkadot-sdk/blob/master/substrate/.maintain/frame-weight-template.hbs).

./target/debug/solochain-template-node benchmark pallet \
--chain dev \
--execution=wasm \
--wasm-execution=compiled \
--pallet pallet_kitties \
--extrinsic "*" \
--steps 20 \
--repeat 10 \
--output pallets/kitties/src/weights.rs \
--template .maintain/frame-weight-template.hbs

# 修改extrinsic weight 属性宏

cargo build --profile=dev
```

[text](./1.png)
[text](./2.png)
[text](./3.png)
[text](./4.png)
