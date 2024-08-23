#!/bin/bash
source .env
INSTANTIATE_MSG="
{
  \"keys_msg\": {
    \"key1\": \"d2d444cf8c5e3ae8386d66a13a316976aa369991\",
    \"n1\": \"onV5tzUbqyPfkM6MwUqCtrqun9x20hEUbIUlmAYYuPuMhsaNHJqs1AVzRt2TzaNjmPVddEbU7VMDmeFWUt7vgDi7Xu0leevuIN4VSPbAMGBa0oj9Qopqkn9ePO_7DvIN13ktHgfQqatNBu6uXH6zkUl3VtXnubXrUhx7uyF22dARDc1-pJoj2NnsvgxDRElPMyDkU-siVv3c6cgIEwLEZZPWOcwplPTUB4qeTK6prrPBGQshuE1PWK2ZrYpIvXfzHyEbkGdPnrhcxgCzbKBUFvr8n_sfSurLRoDBLjkURKmgB8T8iRzLyXsCu9D3Hw61LKuex1aeSQLdwOFLuUEBdw\",
    \"e1\": \"AQAB\",
    \"key2\": \"a49391bf52b58c1d560255c2f2a04e59e22a7b65\",
    \"n2\": \"v7hTj49pNGYjxKbgMx_iDyjeErhfJFepMl306IV_TW5T_CEGE4lWFfBe9w0cwpi5KD6XlC1GO1AsrtzcYF29wJ283GNBZRkbl8iTe-LQYdjQsBtf_1fLIVt6LR7H2U1RPqa3pY16Kq6i6yC2osVg6tD7ApBCGw1WKe8uU3cm28biJzuV4gv6PzcbOdErd-hb4Cv6n2SoMPYlBfT4pWee75poQh8DYoQ1KJwowz3U1MaxOBMP260hmDK-QK0q4LYabCQiBNsz4FWWcaAAFxZFbiqGY5Gdu18uOkpMbdAN5FoZ_6nMDMSTmlf0CHv7gZe_cL38kZvTaynkWwDxqsW_Xw\",
    \"e2\": \"AQAB\",
    \"key3\": \"4529c409f77a106fb67ee1a85d168fd2cfb7c0b7\",
    \"n3\": \"1crrYmsX8OVzrN9BTDD4RlVJDqSQIEbRby9ELqTmCpW1Qtt7y-pdmLPqlYG1ND5mprkTA83S7g_dcsxuV4wxK4_Vv5a8IBn86HfAX4VfCCOzqBYgACN6hlaffzPIWL1QA8yZ4w-D0fnN3xC5ULhtmtBG23qi__4yEo_FIY6irvbHrpRNI_-vjxFokm2X3ENP2ZOwgNhDIthwJo8l1KNbZa1riAJVcF86zWILQTy756hh8eH1Kt05wsGB3DeGPNV55zYv6sB2bzxARsVYAtCRJ8c28FYWwU8dCRJ70eJEmY4aKFOBO5g4fwYJlvMm9Le7qgAUH5-7wO52BayqXmqAOQ\",
    \"e3\": \"AQAB\"
  }
}"
optimize_wasm() {
    echo -n "Optimizing wasm binaries ..."
    OPTIMIZER=$(cargo run-script optimize 2>&1)
    echo -en "033[1A\033[2K\r"
}
store_wasm() {
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

instantiate_contract() {
    local label="$1"
    echo "Instantiate Contract"
    echo -n "Instantiating ..."
    RES=$(xiond tx wasm instantiate $CODE_ID "$INSTANTIATE_MSG" --from $DEPLOYER_ADDRESS --node $RPC --chain-id $CHAIN_ID --label $label --gas auto --gas-adjustment 1.4 --gas-prices $GAS_PRICE --yes --admin $DEPLOYER_ADDRESS --output json <<<"$password")
    echo -en "033[1A\033[2K\r"
    INS_TX_HASH=$(echo $RES | jq -r '.txhash')
    echo "Instantiating contract tx hash: https://explorer.burnt.com/xion-testnet-1/tx/$INS_TX_HASH"
    CONTRACT_ADDR=$(echo $res | jq -r '.tx_response.logs[0].events[-1].attributes[0].value')
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

get_contract_address() {
    echo -n "Getting Contract address ..."
    CONTRACT_ADDRESS=$(xiond q tx $INS_TX_HASH --node $RPC --output json | jq -r '.logs[0].events[-1].attributes[0].value')
    echo -en "033[1A\033[2K\r"
    echo "Contract Address: $CONTRACT_ADDRESS"
}

# Check for env vars
if [ -z "$DEPLOYER_ADDRESS" ] || [ -z "$RPC" ] || [ -z "$CHAIN_ID" ] || [ -z "$GAS_PRICE" ]; then
    echo "Error: Environment variables not set"
    echo "The environment variable are in .env.ex, the required ones for this script are: { DEPLOYER_ADDRESS, RPC, CHAIN_ID, GAS_PRICE }"
    exit 1
fi

clear
echo "Deploying and instantiating the contract"

optimize_wasm

if [[ "$OPTIMIZER" == *"status: 0" ]]; then
    read -s -p "Enter keyring passphrase (if any): " password
    echo -en "033[1A\033[2K\r"

    store_wasm "artifacts/heypay.wasm"
    wait_some 7
    get_code_id
    instantiate_contract "___"
    wait_some 7
    get_contract_address
else
    echo "There was an error optimizing the wasm, run cargo wasm to check for errors"
    exit 1
fi
