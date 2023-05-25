# vollerei
✨ Blazing fast Logging in Rust
# Examples
`examples/basic.rs`
```
fn main() {
    let logger = Logger::new();

    logger.debug("Hello, world!");
    logger.info("Hello, world!");
    logger.warn("Hello, world!");
    logger.error("Hello, world!");
    logger.critical("Hello, world!");
}
```