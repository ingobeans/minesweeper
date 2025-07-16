# minesweeper
![image](https://github.com/user-attachments/assets/2440ccda-b11f-4401-b3a9-2a45be7d27eb)

minesweeper clone in rust with macroquad !!! :)

## building

for standalone do `cargo run`.

for web with `basic-http-server` do `cargo build --release --target wasm32-unknown-unknown && cp target/wasm32-unknown-unknown/release/minesweeper.wasm web/ && basic-http-server web/`
