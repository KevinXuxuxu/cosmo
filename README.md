![eg](eg.gif)
```
cd rust
cargo run -- -f scenes/panorama.cos -s 200,30 -d 20 --fr 60
```

#### Build
- Build to binary (CLI mode):
```shell
cargo build --release
# or just run directly
cargo run -- -f scenes/cube.cos -s 80,40 -d 10 --fr 60
```
- Build to WASM (result in `./pkg`). To use in browser, see [this script](https://github.com/KevinXuxuxu/blog/blob/main/static/script/cosmo_load.js) as an example.
```shell
wasm-pack build --target web --features wasm
```
- Build to WASM with Rayon multi-thread support
```shell
wasm-pack build --target web --features wasm_rayon --config .cargo/wasm-rayon-config.toml
```
