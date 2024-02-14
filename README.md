[![Tests](https://github.com/omni001s/front_test/actions/workflows/Basic.yml/badge.svg)](https://github.com/omni001s/front_test/actions/workflows/Basic.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![CosmWasm](https://img.shields.io/badge/CosmWasm-green)
![Xion](https://img.shields.io/badge/Xion-black)
# HayPay Contract
## Compiling and testing
### Requirements
This project is bootstrapped from CosmWasm Starter Pack ([minimal code repo](https://github.com/CosmWasm/cw-template?tab=readme-ov-file#creating-a-new-repo-from-template)), you need rust to build and test the contract, instructions for installing rust, could be found [here](https://www.rust-lang.org/tools/install). Additionally, you need the Wasm rust compiler to build Wasm binaries:
```sh
$ rustup target add wasm32-unknown-unknown
```
If you want to use scripts and run them like `yarn script`, you could install the `cargo-run-script` crate, we use this crate and [docker](https://docs.docker.com/engine/install/) to run the wasm binary optimizer, besides that you could use the minimal deploy script as well:
```sh
$ cargo install cargo-run-script
```
### Test locally
You could run the tests for this contract by runing the following command:
```sh
$ cargo test
```
The wasm binaries could be build by invoking
```sh
$ cargo wasm
```
If you want to generate the opotimized binaries, you could use the following command, you need to have docker beforehand.
```sh
$ cargo optimize
```
This generates the optimized wasm binary which could be deployed to local testnet or Xion testnet, the binary could be found in `artifacts` folder.
# Deploying to Xion testnet and testing the contract
## Deploy
These is a [Deploy](./Scripts/deploy.sh) script which could be used to deploy the contract on Xion Testnet, you need some requirements to run this script.
### Set environment variables
The requirement variables required for `deploy` and `test` script could be found in [sample environment variables](./.env.ex) file, you could set your variables there and then apply it. It is advised that you make a .env file from this file and then set your environment variables there, since .env is added to `.gitignore` file and and it is not manage by git.
```sh
$ cp .env.ex .env
# make your changes in the .env file and then use the following command
$ source .env
```
The required environment variables for deploy script are
```sh
$DEPLOYER_ADDRESS, $RPC, $CHAIN_ID, $GAS_PRICE
# GAS_PRICE could be set to 0uxion for xion testnet
```
### (Optional) Install the `cargo-run-script`
### Deploy the contract
you could invoke the deploy scripts directly or using cargo
- Directly run the script
  ```sh
  $ ./Scripts/deploy.sh
  ```
- Use Cargo (`cargo-run-script` is required)
  ```sh
  $ cargo deploy
  ```
After deploying the contract, the script gives you the `code_id`, `contract_address` and `tx_hash` for both binary saving and contract instantiating. You could set the received `contract_address` in `.env` file to be able to run `test` script.
## Testing
You could use the [test](./Scripts/test.sh) script in scripts folder to test the contract on testnet, but it requires jwt and email address, the email address and your `xion_address` shall be in jwt, preparing this jwt is kindof difficult but you could check it out in our demo [video](https://youtu.be/xxxxxxxxxxx)
## Deployed contract in Xion Testnet
The contract is deployed to Xion testnet at `xionxxxx` with code_id of `1xx`.
