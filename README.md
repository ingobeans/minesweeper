# minesweeper
![image](https://github.com/user-attachments/assets/0606ea3c-270c-4272-af53-f35acdc0d320)


minesweeper clone in rust with macroquad !!! :)

it is cross platform and you can run it in your web browser [here](https://ingobeans.github.io/minesweeper/).

it doesnt have a "no guess" mode because i couldnt figure out how to make a minesweeper solver, but thats something i would want to add later on.

i also have not yet added options for different map sizes, this project is just an implementation of the core concept of minesweeper.

## building

for standalone do `cargo run`.

for web with `basic-http-server` do `cargo build --release --target wasm32-unknown-unknown && cp target/wasm32-unknown-unknown/release/minesweeper.wasm web/ && basic-http-server web/`
