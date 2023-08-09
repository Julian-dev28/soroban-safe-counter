soroban contract invoke \
    --source acc1 \
    --wasm target/wasm32-unknown-unknown/release-with-logs/soroban_safe_counter_contract.wasm \
    --id 1 \
    -- \
    get_count \
    --user GCSXUXZSA2VEXN5VGOWE5ODAJLC575JCMWRJ4FFRDWSTRCJYQK4ML6V3