# `Benchmark, running wasm in wasmi on the IC`

## 1. Build benchmark

```bash
cd src/bench
sh build.sh
```

## 2. Start replica

```bash
dfx start --background
```

## 3. Deploy canister

```bash
dfx deploy
```

## 4. Run benchmark

```bash
dfx canister call wasmi_backend run
``` 
