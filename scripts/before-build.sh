#!/usr/bin/env bash

if [[ -z "$1" ]]; then
    echo "Must provide network name (dev OR ic)" 1>&2
    exit 1
fi

mode=$1
if [ $mode = "dev" ]; then 
    network="local" 
else 
    network=$mode
fi

# check if canisters are created

dfx canister --network=$network create payment_hub && \
dfx canister --network=$network create invoice_history

# put env vars into backend

file_backend="./backend/.env.$mode"
rm -f $file_backend
touch $file_backend

echo "CAN_PAYMENT_HUB_CANISTER_ID=\"$(dfx canister --network=$network id payment_hub)\"" >> $file_backend
echo "CAN_INVOICE_HISTORY_CANISTER_ID=\"$(dfx canister --network=$network id invoice_history)\"" >> $file_backend
echo "CAN_ROOT_KEY=\"$(dfx ping $network | grep -oP '(?<="root_key": )\[.*\]')\"" >> $file_backend

mkdir -p /tmp/msq

sed "1 c const MODE: &str = \"$mode\";" ./backend/src/shared/build.rs >> /tmp/msq/build.rs
mv /tmp/msq/build.rs ./backend/src/shared/build.rs

cargo build --target wasm32-unknown-unknown --package shared

# pub env vars into frontend

file_frontend="./frontend/.env.$mode"
rm -f $file_frontend
touch $file_frontend

echo "VITE_PAYMENT_HUB_CANISTER_ID=\"$(dfx canister --network=$network id payment_hub)\"" >> $file_frontend
echo "VITE_INVOICE_HISTORY_CANISTER_ID=\"$(dfx canister --network=$network id invoice_history)\"" >> $file_frontend
echo "VITE_ROOT_KEY=\"$(dfx ping $network | grep -oP '(?<="root_key": )\[.*\]')\"" >> $file_frontend

if [ $mode = dev ]; then
    echo "VITE_IC_HOST=\"http://localhost:$(dfx info webserver-port)\"" >> $file_frontend
else
    echo "VITE_IC_HOST=\"https://icp-api.io\"" >> $file_frontend
fi
