soroban contract invoke \
    --source acc1 \
    --wasm target/wasm32-unknown-unknown/release-with-logs/soroban_safe_counter_contract.wasm \
    --id 1 \
    -- \
    increment \
    --user GDPT3C3KUNGMSJBAELIQ2NIV52LDG2XPW4EOANKGDCAKLG6OAH3RPB2D \
    --value 1