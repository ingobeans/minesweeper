# minesweeper
![image](https://github.com/user-attachments/assets/0606ea3c-270c-4272-af53-f35acdc0d320)


minesweeper clone in rust with macroquad !!! :)

## building

for standalone do `cargo run`.

for web with `basic-http-server` do `cargo build --release --target wasm32-unknown-unknown && cp target/wasm32-unknown-unknown/release/minesweeper.wasm web/ && basic-http-server web/`
