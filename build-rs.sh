rm -rf ./out || exit
rm target/release/wasm32-unknown-unknown/vm.wat
cargo build --release || exit
# wasm2wat target/wasm32-unknown-unknown/release/vm.wasm -o target/wasm32-unknown-unknown/release/vm.wat || exit
mkdir out || exit
cp vm/target/wasm32-unknown-unknown/release/vm.wasm out/main.wasm