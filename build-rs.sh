rm -rf ./out || exit
rm target/release/wasm32-unknown-unknown/ewasm_evm.wat
cargo +nightly build --release || exit
wasm2wat target/wasm32-unknown-unknown/release/ewasm_evm.wasm -o target/wasm32-unknown-unknown/release/ewasm_evm.wat || exit
mkdir out || exit
cp target/wasm32-unknown-unknown/release/ewasm_evm.wasm out/main.wasm