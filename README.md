# rust-c-lto
Demonstrate link time optimization with rust and c

First run `source ./setup.sh` to add RUSTFLAGS and llvm path to environment (llvm v18 must be installed)

run `cargo run` to see the performace difference between compilation of a single language (rust) vs compiling multiple langages together (rust + c) without link time optimization

run `cargo run --release` to see the performace difference with link time optimization

See [Rust + C Link Time Optimizatiom](https://qformic.com/en-US/posts/rust-c-lto) for full details.
