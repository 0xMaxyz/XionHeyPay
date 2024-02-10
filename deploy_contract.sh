#!/bin/bash
source .env
instantiate_msg='{}'
clear
echo "1. optimized haypay.wasm ..."
optimizer=$(cargo run-script optimize 2>&1)
if [[ "$optimizer" == *"status: 0" ]]; then
    echo "haypay.wasm is optimized"
    echo "----------------------------------------"
    
    echo "2. Saving bytecode ..."
    res=$(xiond tx wasm store artifacts/haypay.wasm --from $DEPLOYER_ADDRESS --node $RPC --chain-id $CHAIN_ID --gas auto --gas-adjustment 1.4 --gas-prices $GAS_PRICE --output json --yes)
    code_id=$(echo $res | jq -r '.tx_response.logs[0].events[-1].attributes[1].value')
    cid_tx_hash=$(echo $res | jq -r '.tx_response.txhash')

    echo "Code id is: $code_id"
    echo "Bytecode saving tx hash is: $cid_tx_hash"

    echo "----------------------------------------"

    echo "3. Instantiating the contract ..."
    res =$(xiond tx wasm instantiate $code_id "$instantiate_msg" --from $DEPLOYER_ADDRESS --node $RPC --chain-id $CHAIN_ID --label "haypay" --gas auto --gas-adjustment 1.4 --gas-prices $GAS_PRICE --yes --no-admin --output json)
    contract_address =$(echo $res | jq -r '.tx_response.logs[0].events[-1].attributes[0].value')
    ins_tx_hash =$(echo $res | jq -r '.tx_response.txhash')

    echo "----------------------------------------"

    echo "Contract address is $contract_address"
    echo "Contract instantiationb tx hash $ins_tx_hash"
else
    echo "There wa an error optimizing the wasm, run cargo wasm to check for errors"
fi

