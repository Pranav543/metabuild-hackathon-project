#!/bin/bash
./build.sh
near dev-deploy res/element.wasm
source neardev/dev-account.env
cat neardev/dev-account.env > .env

echo "Initializing element contract '$CONTRACT_NAME'"
near call $CONTRACT_NAME new '{"owner_id": "'$CONTRACT_NAME'", "total_supply": "100000000", "metadata": { "spec": "ft-1.0.0", "name": "Element Token", "symbol": "ELE", "decimals": 0 }}' --accountId $CONTRACT_NAME


INVEST_CONTRACT_NAME=invest.$CONTRACT_NAME
echo "Creating '$INVEST_CONTRACT_NAME' account"

near create-account $INVEST_CONTRACT_NAME  --masterAccount $CONTRACT_NAME --initialBalance 50
near call $CONTRACT_NAME storage_deposit '' --accountId $INVEST_CONTRACT_NAME --amount 0.00125

echo "Deploying invest contract '$INVEST_CONTRACT_NAME'"
near deploy $INVEST_CONTRACT_NAME res/invest.wasm new '{"fungible_token_account_id": "'$CONTRACT_NAME'", "max_investment_hex": 12345678, "maturity_days":7, "measurement_window": 7}'

element=$CONTRACT_NAME
invest=$INVEST_CONTRACT_NAME

echo "element='$CONTRACT_NAME'"
echo "invest='$INVEST_CONTRACT_NAME'"