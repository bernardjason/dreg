rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown 
cp src/index.html mq_js_bundle.js  assets/* target/wasm32-unknown-unknown/debug
cd target/wasm32-unknown-unknown/debug
python3 -m http.server
