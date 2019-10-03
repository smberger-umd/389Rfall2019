Do the following command to install Rustup, a tool for managing rust toolchains, and thus get the most recent version of Rust. It will only work if you don't have Rust and Cargo installed using APT.

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

Hit 2 to Customize installation
Hit enter to use default host triple.
Type "beta" since I'm using Rust beta features
Hit enter to make sure Rust is added to the Path.
Hit enter to do the installation.

Alternatively, you can do a default installation, then do a `rustup install beta` then `rustup default beta`.

Finally, just cd into the `wattsampshell` folder, and do `cargo run`, which will download any dependencies, build, and then run the program. The binary itself, which may be distributed, is at `target/debug/wattsampshell`.

CTRL-D, `quit`, or `exit` will leave the subshell portion back to the normal commands, or from the main shell back to bash.