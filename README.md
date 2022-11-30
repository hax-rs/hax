# 🧪 hax

The versatile and intuitive memory hacking framework.

## 🤔 About

[hax](https://github.com/hax-rs/hax) is a Rust crate designed to make memory hacking, game hacking, cheat development, and any other low level memory based development *simple, yet intuitive*.

So far the project is in it's infancy, although many things are [planned](https://github.com/hax-rs/hax/projects) and we appreciate any form of contribution! We're on Discord and primarily discuss the project there, please note we request you not talk about cheat development as it is against Discord's terms of service.

## 💎 Features

- [x] Various macros aimed to eliminate boilerplate

## 🪛 Installation & Usage

Adding hax to your project is simple, however it requires a bit of setup depending on your desired output.

### External

To add hax to your external project and produce an executable (.exe), you must:

1. Add [hax](https://crates.io/crates/hax) to your project with cargo: `cargo add hax -F external`
2. Inside of `main.rs`, attribute your entrypoint with `#[hax::main]`

### Internal

To add hax to your internal project and produce a shared library (.dll, .dylib, .so), you must:

1. Add [hax](https://crates.io/crates/hax) to your project with cargo: `cargo add hax -F internal`
2. Set the `crate-type` to `["cdylib"]` inside of `Cargo.toml`
3. Inside of `lib.rs`, attribute your entrypoint with `#[hax::main]`

### Kernel

👀
