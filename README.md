# A sandbox for Zero to Production in Rust

## Setup

```
cargo install cargo-watch
```

## Commands

### Start development

```
cargo watch -x check -x test -x run
```

### Measure code coverage

```
cargo install tarpaulin
cargo tarpaulin --ignore-tests
```

### Lint

```
cargo clippy
```

On CI

```
rustup component add clippy
// Failed if there are any warnings
cargo clippy -- -D warnings
```

### Format

```
cargo fmt
```

On CI

```
rustup component add rustfmt
cargo fmt -- --check
```

### Check Vulnerabilities

```
cargo install cargo-audit
cargo audit
```
