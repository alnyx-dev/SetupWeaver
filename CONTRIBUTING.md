# Contributing to SetupWeaver

## Prerequisites

- **Rust stable** (latest)
- **Linux**: `pkg-config`, `libfontconfig1-dev`, `libxcb-render0-dev`, `libxcb-shape0-dev`, `libxcb-xfixes0-dev`, `libxkbcommon-dev`, `libwayland-dev`
- **Windows**: no extra system deps

Install Linux deps (Ubuntu/Debian):

```bash
sudo apt-get install -y pkg-config libfontconfig1-dev libxcb-render0-dev \
  libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libwayland-dev
```

## Build

```bash
cargo build --workspace
```

Release build (optimized stubs):

```bash
cargo build --release -p setupweaver-packager -p setupweaver-runtime -p setupweaver-runtime-admin
```

## Tests

```bash
cargo test --workspace
```

End-to-end tests in `packager/tests/e2e.rs` require Windows and run only on `windows-latest` CI.

## Linting

CI enforces both `clippy` and `rustfmt`:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
```

Fix formatting automatically:

```bash
cargo fmt --all
```

## Project layout

```
common/        — shared types: InstallConfig, PackagedInstaller, validation
packager/      — CLI: install.toml + files → setup.exe
packager-gui/  — Slint-based visual installer builder
runtime/       — embedded stub: extract + install engine + Slint wizard UI
runtime-admin/ — same stub with requireAdministrator manifest
examples/      — sample install.toml configs
docs/          — architecture notes
```

## Making changes

1. Fork and create a feature branch from `main`.
2. Keep commits focused — one logical change per commit.
3. Run `cargo fmt --all` and `cargo clippy --all-targets -- -D warnings` before pushing.
4. Add tests for new validation rules or payload format changes.
5. Open a PR against `main`. CI will run fmt, clippy, Linux build+test, and Windows build+test.

## Code style

- Follow existing conventions (naming, error handling with `thiserror`/`snafu`).
- No `unsafe` without justification.
- Keep the runtime stub lean — avoid adding heavy dependencies to `runtime/`.
