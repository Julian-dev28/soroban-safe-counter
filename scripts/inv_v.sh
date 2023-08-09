soroban -v contract invoke \
    --source acc1 \
    --wasm target/wasm32-unknown-unknown/release-with-logs/soroban_safe_counter_contract.wasm \
    --id 1 \
    -- \
    increment \
    --user GAGZIGR3PTOY7SXLDKM6E4NLBAJYJH2CD4GFB2QLALU6FR5GZA2ZA4XF \
    --value 1