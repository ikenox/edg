## run emulator

```
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
cargo +nightly install miniserve
wasm-pack build --target web --out-name wasm --out-dir ./static
miniserve ./static --index index.html
```
