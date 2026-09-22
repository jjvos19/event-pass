# Soroban Project

## Project Structure

This repository uses the recommended structure for a Soroban project:

```text
.
├── contracts
│   └── hello_world
│       ├── src
│       │   ├── lib.rs
│       │   └── test.rs
│       └── Cargo.toml
├── Cargo.toml
└── README.md
```

- New Soroban contracts can be put in `contracts`, each in their own directory. There is already a `hello_world` contract in there to get you started.
- If you initialized this project with any other example contracts via `--with-example`, those contracts will be in the `contracts` directory as well.
- Contracts should have their own `Cargo.toml` files that rely on the top-level `Cargo.toml` workspace for their dependencies.
- Frontend libraries can be added to the top-level directory as well. If you initialized this project with a frontend template via `--frontend-template` you will have those files already included.

## Descripción

El ledger verifica que una address compro su pase y que lo uso una sola vez.

## Pruebas unitarias, obtener Address, publicado y prueba en stellar.expert

### Pruebas unitarias

Para hacer correr las pruebas unitarias, ejecutar el comando:

```bash
 cargo test --target-dir ./target
```

El resultado:

```bash
   Compiling event-pass v0.1.0 (D:\Desarrollo\blockchain\stellar\s-elite\prueba-contrato\event-pass\contracts\event-pass)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.89s
     Running unittests src\lib.rs (target\debug\deps\event_pass-a8216e9136b050c9.exe)

running 8 tests
test test::test_constructor_initializes_admin_correctly ... ok
test test::test_constructor_allows_multiple_contract_deployments ... ok
test test::test_check_pass_returns_default_for_unregistered_user ... ok
test test::test_check_pass_reflects_status_changes_correctly ... ok
test test::test_use_pass_fails_if_already_used_or_not_found ... ok
test test::test_buy_pass_fails_if_already_purchased ... ok
test test::test_use_pass_success ... ok
test test::test_buy_pass_success ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.10s

```

### Compilado

Para compilar:

```bash
stellar.exe contract build --out-dir ./target
```

El resultado:

```bash
ℹ️ CARGO_BUILD_RUSTFLAGS='--remap-path-prefix=C:/Users/jjvos19/.cargo/registry/src=' SOROBAN_SDK_BUILD_SYSTEM_SUPPORTS_SPEC_SHAKING_V2=1 cargo rustc '--manifest-path=contracts\event-pass\Cargo.toml' --crate-type=cdylib --target=wasm32v1-none --release
⚠️ A new release of Stellar CLI is available: 27.1.0 -> 28.0.0
   Compiling event-pass v0.1.0 (D:\Desarrollo\blockchain\stellar\s-elite\prueba-contrato\event-pass\contracts\event-pass)
    Finished `release` profile [optimized] target(s) in 6.19s
ℹ️ Build Summary:
   Wasm File: ./target\event_pass.wasm (2863 bytes optimized (original size was 3214 bytes))
   Wasm Hash: c573d1571055b7bcfbfc551fd943bec9c097a3fdfd8c80b41e49e17f5ed36628
   Wasm Size: 2863 bytes optimized (original size was 3214 bytes)
   Exported Functions: 4 found
     • __constructor
     • buy_pass
     • check_pass
     • use_pass
✅ Build Complete
```

### Obtenemos el Address de juanjo

Ejcutar el comando:

```bash
DIRECCION=$(stellar keys address juanjo)
```

El resultado:

```bash
echo $DIRECCION
GCA7QQ6FRH5AN4RCGIUU7WWBFM4RCLFKZK55K7BUCWNHOL4AA6NXTLUH
```

### Publicado

Para publicar usar el comando, en este caso por utilizar un constructor, se debe pasar tambien el parametro admin (direccion) para que se pueda publicar.

```bash
stellar contract deploy --wasm target/event_pass.wasm --source-account juanjo --network testnet --alias event-pass -- --admin $DIRECCION
```

El resultado:

```bash
ℹ️ Uploading contract WASM…
⚠️ A new release of Stellar CLI is available: 27.1.0 -> 28.0.0
ℹ️ Skipping install because wasm already installed
ℹ️ Deploying contract using wasm hash c573d1571055b7bcfbfc551fd943bec9c097a3fdfd8c80b41e49e17f5ed36628
ℹ️ Simulating transaction…
ℹ️ Signing transaction: 73c5d1962417c3871cce077cdf3cd399aec92a3f1be6cc853959d4a0b639bc68
🌎 Sending transaction…
✅ Transaction submitted successfully!
🔗 https://stellar.expert/explorer/testnet/tx/73c5d1962417c3871cce077cdf3cd399aec92a3f1be6cc853959d4a0b639bc68
🔗 https://lab.stellar.org/r/testnet/contract/CDXK3SR6264SGH2EYRYX6U3WJGN22WF6SJNOTRDQGFTOSH3CMFCBCXUK
✅ Deployed!
CDXK3SR6264SGH2EYRYX6U3WJGN22WF6SJNOTRDQGFTOSH3CMFCBCXUK
```

Para verificar que el contrato se publico, ingresar a la ruta: [Verificar: CDXK3SR6264SGH2EYRYX6U3WJGN22WF6SJNOTRDQGFTOSH3CMFCBCXUK](https://stellar.expert/explorer/testnet/contract/CDXK3SR6264SGH2EYRYX6U3WJGN22WF6SJNOTRDQGFTOSH3CMFCBCXUK)

## Pruebas en stellar

### Consultar ticket valido

Para consultar si el usuario tiene un ticket valido:

```bash
stellar contract invoke \
 --id CDXK3SR6264SGH2EYRYX6U3WJGN22WF6SJNOTRDQGFTOSH3CMFCBCXUK \
 --source-account juanjo \
 --network testnet \
 -- check_pass \
 --user $DIRECCION
```

El resutaldo es:

```bash
⚠️ A new release of Stellar CLI is available: 27.1.0 -> 28.0.0
ℹ️ Simulation identified as read-only. Send by rerunning with `--send=yes`.
{"purchased":false,"used":false}
```

### Comprar ticket

Para comprar ticket:

```bash
stellar contract invoke \
 --id CDXK3SR6264SGH2EYRYX6U3WJGN22WF6SJNOTRDQGFTOSH3CMFCBCXUK \
 --source-account juanjo \
 --network testnet \
 -- buy_pass \
 --user $DIRECCION
```

El resultado es:

```bash
⚠️ A new release of Stellar CLI is available: 27.1.0 -> 28.0.0
ℹ️ Simulating transaction…
ℹ️ Signing transaction: 34cfdc4c31137c396227a525d998c613d755367df9e45f0666e77e043c1d9585
🌎 Sending transaction…
✅ Transaction submitted successfully!
🔗 https://stellar.expert/explorer/testnet/tx/34cfdc4c31137c396227a525d998c613d755367df9e45f0666e77e043c1d9585
📅 CDXK3SR6264SGH2EYRYX6U3WJGN22WF6SJNOTRDQGFTOSH3CMFCBCXUK - Success - Event: PassPurchased (pass_purchased), user: "GCA7QQ6FRH5AN4RCGIUU7WWBFM4RCLFKZK55K7BUCWNHOL4AA6NXTLUH"
null
```

Una vez termiando este paso, volver a ejecutar el punto [Consultar ticket valido](#consultar-ticket-valido)
