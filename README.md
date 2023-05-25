# ![vollerei](assets/logo.png)

# vollerei

![language](https://img.shields.io/badge/language-Rust-ffc9bb)
![license](https://img.shields.io/badge/license-MIT-cafffe)

✨ Blazing fast Logging in Rust

## 🌼 Table of Contents
- [🥝 Statistics](#🥝-statistics)
- [💐 Install](#💐-install)
    - [📦 Cargo](#📦-cargo)
- [🍹 Examples](#🍹-examples)
    - [🍓 Basic](#🍓-basic)
- [🍸 Links](#🍸-links)

## 🥝 Statistics
Size: `~5.73` kB

Files: `8` \
Lines: `305` \
Blanks: `39` \
Comments: `133`

## 💐 Install

### 📦 Cargo
Run the following Cargo command in your project directory\
`cargo add vollerei`

Or add the following line to your Cargo.toml\
`vollerei = "0.1.7"`

## 🍹 Examples

### 🍓 Basic
`examples/basic.rs`
```
use vollerei::logger::Logger;

fn main() {
    let logger = Logger::new("example");

    logger.debug("Hello, world!");
    logger.info("Hello, world!");
    logger.warn("Hello, world!");
    logger.error("Hello, world!");
    logger.critical("Hello, world!");
}
```
`assets/basic.png`
![basic](assets/basic.png)

## 🍸 Links
### 🦀 Rust
[*Crates.io*](https://crates.io/crates/vollerei)\
[*Docs.rs*](https://docs.rs/vollerei)