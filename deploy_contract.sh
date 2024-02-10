#!/bin/bash
source .env
INSTANTIATE_MSG='{}'
clear
echo "1. optimizing haypay.wasm ..."
OPTIMIZER=$(cargo run-script optimize 2>&1)
if [[ "$OPTIMIZER" == *"status: 0" ]]; then
    echo "haypay.wasm is optimized"
    echo "----------------------------------------"
    
    echo "2. Saving bytecode ..."
    RES=$(xiond tx wasm store artifacts/haypay.wasm --from $DEPLOYER_ADDRESS --node $RPC --chain-id $CHAIN_ID --gas auto --gas-adjustment 1.4 --gas-prices $GAS_PRICE --output json --yes)
    CODE_ID=$(echo $RES | jq -r '.tx_response.logs[0].events[-1].attributes[1].value')
    CODE_ID_TX_HASH=$(echo $RES | jq -r '.tx_response.txhash')

    echo "Code id is: $CODE_ID"
    echo "Bytecode saving tx hash is: $CODE_ID_TX_HASH"

    echo "----------------------------------------"

    echo "3. Instantiating the contract ..."
    RES=$(xiond tx wasm instantiate $code_id "$INSTANTIATE_MSG" --from $DEPLOYER_ADDRESS --node $RPC --chain-id $CHAIN_ID --label "haypay" --gas auto --gas-adjustment 1.4 --gas-prices $GAS_PRICE --yes --no-admin --output json)
    CONTRACT_ADDR=$(echo $res | jq -r '.tx_response.logs[0].events[-1].attributes[0].value')
    INS_TX_HASH=$(echo $res | jq -r '.tx_response.txhash')

    echo "----------------------------------------"

    echo "Contract address is $CONTRACT_ADDR"
    echo "Contract instantiationb tx hash $INS_TX_HASH"
else
    echo "There was an error optimizing the wasm, run cargo wasm to check for errors"
fi

