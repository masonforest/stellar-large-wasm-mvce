# Stellar Large Contract MVCE

Installing a transaction on the Soroban network requires sending the wasm binary of the contract you are deploying in a transaction. Since transaction size is limited to 70kb any contract larger than 70kb will fail to deploy using the soroban cli. This effectively limits contract size to 70kb where the upper limit set by the network is 250kb.

# Steps to reproduce

````
$ soroban contract build
$ soroban config identity generate --global alice
$ curl "https://friendbot.stellar.org/?addr=$(soroban config identity address alice)"
$ soroban contract install \
  --wasm target/wasm32-unknown-unknown/release/big_chungus_contract.wasm \
  --source alice \
  --network testnet

2024-01-21T01:24:39.506051Z ERROR soroban_cli::rpc: TXN failed:
 Ok(
    TxSorobanInvalid,
)
error: transaction submission failed: TxSorobanInvalid
````