#!/bin/bash
source .env
MIGRATE_MSG='{"default_gas_limit":1000000}'
optimize_wasm() {
    echo -n "Optimizing wasm binaries ..."
    OPTIMIZER=$(cargo run-script optimize 2>&1)
    echo -en "033[1A\033[2K\r"
}
store_code() {
    local c_addr="$1"
    echo "Save Binary"
    echo -n "Saving ..."
    RES=$(xiond tx wasm store $c_addr --from $DEPLOYER_ADDRESS --node $RPC --chain-id $CHAIN_ID --gas auto --gas-adjustment 1.4 --gas-prices $GAS_PRICE --output json --yes <<<"$password")
    echo -en "033[1A\033[2K\r"
    STORE_TX_HASH=$(echo $RES | jq -r '.txhash')
    echo "Save binary tx hash: https://explorer.burnt.com/xion-testnet-1/tx/$STORE_TX_HASH"
}

get_code_id() {
    echo -n "Getting the code id ..."
    CODE_ID=$(xiond q tx $STORE_TX_HASH --node $RPC --output json | jq -r '.logs[0].events[-1].attributes[1].value')
    echo -en "033[1A\033[2K\r"
    echo "Code id: $CODE_ID"
}

migrate() {
    echo "Migrating contract"
    echo -n "Migrating ..."
    RES=$(xiond tx wasm migrate $HEYPAY_ADDRESS $CODE_ID $MIGRATE_MSG --from $DEPLOYER_ADDRESS --node $RPC --chain-id $CHAIN_ID --gas auto --gas-adjustment 1.4 --gas-prices $GAS_PRICE --output json --yes <<<"$password")
    echo -en "033[1A\033[2K\r"
    STORE_TX_HASH=$(echo $RES | jq -r '.txhash')
    echo "Migration tx hash: https://explorer.burnt.com/xion-testnet-1/tx/$STORE_TX_HASH"
}

wait_some() {
    local Heights="$1"
    echo -n "Waiting for some blocks ($Heights)"
    INIT_HEIGHT=$(xiond query block --node $RPC | jq -r '.block.header.height')
    TARGET_HEIGHT=$((INIT_HEIGHT + Heights))
    while true; do
        CURRENT_HEIGHT=$(xiond query block --node "$RPC" | jq -r '.block.header.height')

        if [ "$CURRENT_HEIGHT" -ge "$TARGET_HEIGHT" ]; then
            break
        fi

        sleep 1
    done
    echo -en "033[1A\033[2K\r"
}

# Check for env vars
if [ -z "$DEPLOYER_ADDRESS" ] || [ -z "$RPC" ] || [ -z "$CHAIN_ID" ] || [ -z "$GAS_PRICE" ] || [ -z "$HEYPAY_ADDRESS" ]; then
    echo "Error: Environment variables not set"
    echo "The environment variable are in .env.ex, the required ones for this script are: { DEPLOYER_ADDRESS, RPC, CHAIN_ID, GAS_PRICE, HEYPAY_ADDRESS }"
    exit 1
fi

clear
echo "Migrating contract"

optimize_wasm

if [[ "$OPTIMIZER" == *"status: 0" ]]; then
    read -s -p "Enter keyring passphrase (if any): " password
    echo -en "033[1A\033[2K\r"

    store_code "artifacts/heypay.wasm"
    wait_some 7
    get_code_id
    migrate
else
    echo "There was an error optimizing the wasm, run cargo wasm to check for errors"
    exit 1
fi
