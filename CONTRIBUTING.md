# Contributing

Thanks for helping improve appscope. Keep changes small and explain the Linux distributions or package managers they affect.

Before opening a pull request:

```bash
cargo fmt --all
cargo test --all-targets
cargo clippy --all-targets --all-features -- -D warnings
```

Do not add code that runs uninstall commands automatically. appscope is intentionally read-only.
