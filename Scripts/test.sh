#!/bin/bash
source .env
clear

query() {
  local addr="$1"
  local name="$2"
  local contract="$3"

  echo -n "Querying..."
  QUERY_MSG="{\"balance\":{ \"address\": \"$addr\" }}"
  BASE64_Q_MSG=$(echo "$QUERY_MSG" | base64 | tr -d '[:space:]\n')
  RESP=$(xiond query wasm contract-state smart "$contract" "$BASE64_Q_MSG" --b64 --node "$RPC" --output json)

  INIT_BALANCE=$(echo "$RESP" | jq -r '.data.balance')
  echo -en "033[1A\033[2K\r"
  echo "$name Balance: $INIT_BALANCE"
}

transfer_token() {
  echo -n "Transfering tokens to test email..."
  local password="$1"
  TOKEN_MSG=$(echo -e "{ \"email\": \"$TEST_EMAIL\",\"memo\": \"test memo for this transaction\" }" | base64 -w 0)
  SEND_MSG="{
  \"send\": {
    \"contract\": \"$HAYPAY_ADDRESS\",
    \"amount\": \"1000\",
    \"msg\": \"$TOKEN_MSG\"
  }
}"
  RESP=$(xiond tx wasm execute $TOKEN_ADDRESS "$SEND_MSG" --from $DEPLOYER_ADDRESS --chain-id $CHAIN_ID --gas 600000 --gas-prices $GAS_PRICE --node $RPC --output json --yes <<<"$password")
  #echo -e "\033[2K"
  TX_HASH=$(echo "$RESP" | jq -r '.txhash')
  echo -en "033[1A\033[2K\r"
  echo "Tx Hash for transfering token to HayPay contract: https://explorer.burnt.com/xion-testnet-1/tx/${TX_HASH}"
}

claimable() {
  echo -n "Querying the claimable tokens for the test email..."
  QUERY_MSG="{\"claims\":{ \"email\": \"$TEST_EMAIL\" }}"
  BASE64_Q_MSG=$(echo "$QUERY_MSG" | base64)
  RESP=$(xiond query wasm contract-state smart $HAYPAY_ADDRESS $BASE64_Q_MSG --b64 --node $RPC --output json)
  BALANCE=$(echo "$RESP" | jq -r '.data.claims')
  echo -en "033[1A\033[2K\r"
  echo "Total amount of claimable tokens for test email is: {$BALANCE}"
}

claim() {
  local password="$1"
  echo -n "Claiming the balance for test email ..."
  CLAIM_MSG="
{
  \"claim\": {
    \"msg\": {
        \"jwt\": \"$JWT\",
        \"aud\": \"project-live-7e4a3221-79cd-4f34-ac1d-fedac4bde13e\"
    }
  }
}"

  RESP=$(xiond tx wasm execute $HAYPAY_ADDRESS "$CLAIM_MSG" --from $DEPLOYER_ADDRESS --chain-id $CHAIN_ID --gas 2000000 --gas-prices $GAS_PRICE --node $RPC --output json --yes <<<"$password")
  TX_HASH=$(echo "$RESP" | jq -r '.txhash')
  echo -en "033[1A\033[2K\r"
  echo "Tx Hash for claiming tokens: https://explorer.burnt.com/xion-testnet-1/tx/${TX_HASH}"
}

wait() {
  INIT_HEIGHT=$(xiond query block --node $RPC | jq -r '.block.header.height')
  TARGET_HEIGHT=$((INIT_HEIGHT + 20))
  while true; do
    CURRENT_HEIGHT=$(xiond query block --node "$RPC" | jq -r '.block.header.height')

    if [ "$CURRENT_HEIGHT" -ge "$TARGET_HEIGHT" ]; then
      break
    fi

    sleep 1
  done
}

read -s -p "Enter keyring passphrase (if any): " password
#echo "$password" >.t
echo -en "033[1A\033[2K\r"

query "$DEPLOYER_ADDRESS" "Deployer" "$TOKEN_ADDRESS"

transfer_token "$password"

echo -n "Waiting for some confirmations..."
wait
echo -en "\033[2K\r"

query "$DEPLOYER_ADDRESS" "Deployer" "$TOKEN_ADDRESS"

query "$HAYPAY_ADDRESS" "Haypay contract" "$TOKEN_ADDRESS"

claimable

claim "$password"

echo -n "Waiting for some confirmations..."
wait
echo -en "\033[2K\r"

claimable

query "$DEPLOYER_ADDRESS" "Deployer" "$TOKEN_ADDRESS"

query "$HAYPAY_ADDRESS" "Haypay contract" "$TOKEN_ADDRESS"

#rm .t
