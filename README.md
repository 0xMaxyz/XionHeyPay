[![Tests](https://github.com/omni001s/front_test/actions/workflows/Basic.yml/badge.svg)](https://github.com/omni001s/front_test/actions/workflows/Basic.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![CosmWasm](https://img.shields.io/badge/CosmWasm-green)
![Xion](https://img.shields.io/badge/Xion-black)
# HeyPay Contract
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
Run the following command to execute the contract's tests:
```sh
$ cargo test
```
Build the Wasm binaries using
```sh
$ cargo wasm
```
To generate optimized binaries (requires Docker):
```sh
$ cargo optimize
```
This creates optimized Wasm binaries suitable for deployment to local or Xion testnets. You can find them in the `artifacts` folder.
# Deploying to Xion testnet and testing the contract
## Deployment Steps
The [Deploy](./Scripts/deploy.sh) script helps deploy the contract on the Xion Testnet. Make sure you meet the following requirements before running it:
### Set environment variables
The required environment variables for the `deploy` and `test` scripts are listed in the [sample environment variables](./.env.ex) file. Copy it to `.env` and set your specific values. (`.env` is excluded from version control).
```sh
$ cp .env.ex .env
# Edit your values in .env and then apply them:
$ source .env
```
The required variables for deployment are:
```sh
$DEPLOYER_ADDRESS
$RPC
$CHAIN_ID
$GAS_PRICE # set to 0uxion for Xion testnet
```
### (Optional) Install [cargo-run-script](#requirements)
### Deploy the contract
Run the deploy script directly or using Cargo:
- **Directly:**
  ```sh
  $ ./Scripts/deploy.sh
  ```
- **Using Cargo Cargo** (requires `cargo-run-script`)
  ```sh
  $ cargo deploy
  ```
Upon successful deployment, the script provides the `code_id`, `contract_address`, and `tx_hash` for both binary saving and contract instantiation. Set the `HEYPAY_ADDRESS` in your `.env` file before executing the `test` script.
## Testing
The `./Scripts/test.sh` script helps test the contract on the testnet, but it requires a JWT containing your email address and Xion address. Manual setup of this custom JWT can be complex, you have to inject email address and xion address in your request to receive a valid JWT with custom claims.
A video of our demo using our front app is [here](https://youtu.be/NIFiNzDLiOY).
## Deployed contract in Xion Testnet
The contract is deployed to Xion testnet at `xion1xg3elrmwhu0u0e2yq2vexxs383r0clcyyc2nvezygn3mmgdxx9kq3x5l9e`.
