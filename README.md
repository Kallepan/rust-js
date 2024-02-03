# Rust-JS

A test project to try out Rust and the v8 JavaScript engine.

```bash
# Start devcontainer
# get binaries from https://github.com/denoland/rusty_v8/releases
# and put them in the root of the project
# e.g:
curl -L https://github.com/denoland/rusty_v8/releases/download/v0.83.1/librusty_v8_release_x86_64-unknown-linux-gnu.a -o rusty_v8_binary.a

export RUSTY_V8_BIN_PATH=./workspaces/rust-js/rusty_v8_binary.a
cargo run -- test.ts
```
