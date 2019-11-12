Do the following command to install Rustup, a tool for managing rust toolchains, and thus get the most recent version of Rust. It will only work if you don't have Rust and Cargo installed using APT.

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

Hit 1 to do Default Installation.

If it's already installed, do `rustup default stable` and `rustup update stable` to get on the most recent version of Rust.

Finally, just cd into the `fpffparse` folder, and do `cargo run -- [filename of fpff file]`, which will download any dependencies, build, and then run the program. The binary itself, which may be distributed, is at `target/debug/fpffparse`.
