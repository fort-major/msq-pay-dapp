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
file_name="./backend/.env.$mode"

source $file_name

dfx deploy --network=$network payment_hub --verbose --argument "( \
    record { \
        supported_tokens = vec { \
            record { \
                id = principal \"ryjl3-tyaaa-aaaaa-aaaba-cai\"; \
                ticker = \"ICP\"; \
                decimals = 8 : nat8; \
                fee = 10_000 : nat; \
            }; \
            record { \
                id = principal \"mxzaz-hqaaa-aaaar-qaada-cai\"; \
                ticker = \"ckBTC\"; \
                decimals = 8 : nat8; \
                fee = 10 : nat; \
            }; \
            record { \
                id = principal \"ss2fx-dyaaa-aaaar-qacoq-cai\"; \
                ticker = \"ckETH\"; \
                decimals = 18 : nat8; \
                fee = 2_000_000_000_000 : nat; \
            }; \
            record { \
                id = principal \"pe5t5-diaaa-aaaar-qahwa-cai\"; \
                ticker = \"ckEURC\"; \
                decimals = 6 : nat8; \
                fee = 10_000 : nat; \
            }; \
            record { \
                id = principal \"xevnm-gaaaa-aaaar-qafnq-cai\"; \
                ticker = \"ckUSDC\"; \
                decimals = 6 : nat8; \
                fee = 10_000 : nat; \
            }; \
            record { \
                id = principal \"cngnf-vqaaa-aaaar-qag4q-cai\"; \
                ticker = \"ckUSDT\"; \
                decimals = 6 : nat8; \
                fee = 10_000 : nat; \
            }; \
        }; \
        fee_collector_account = opt record { \
            owner = principal \"qz32s-aqaaa-aaaag-alfya-cai\"; \
            subaccount = null; \
        }; \
    } \
)"

dfx deploy --network=$network invoice_history --argument "()"
