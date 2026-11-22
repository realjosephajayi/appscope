# appscope

A small, dependency-free Rust CLI for inspecting Linux applications before uninstalling them.

## Why this project?

Removing software from Linux is not always straightforward: an application may come from `apt`, `dnf`, `pacman`, Flatpak, Snap, or a manually installed binary. **appscope** helps users identify the likely installation source and prints a removal command for review.

> **Safety:** appscope is read-only. It never runs an uninstall command and never changes your system.

## Features

- Checks Flatpak and Snap applications.
- Checks common native package managers: APT, DNF/RPM, and Pacman.
- Falls back to common binary locations.
- Handles multiple application names in one command.
- Uses only Rust's standard library, keeping the binary small and easy to audit.
- Includes unit tests and clear exit behavior.

## Build and run

```bash
cargo run -- scan cursor "Claude Desktop" mt5
cargo test
cargo build --release
./target/release/appscope scan firefox
```

Review every suggested command carefully. Package names and commands can vary by distribution.

## Supported systems

Linux distributions with Rust and at least one of `flatpak`, `snap`, `dpkg`, `rpm`, or `pacman`. The binary also works as a basic filesystem scanner when no package manager is available.

## Roadmap

- Add `--json` output for scripts.
- Add package-manager metadata and version reporting.
- Add opt-in interactive confirmation in a separate command.
- Add integration tests using mocked command output.

## License

MIT. See [LICENSE](LICENSE).

## Implementation

Built with Rust using only the standard library.
