# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

### Added
- Visual packager GUI (`packager-gui`) — Slint-based installer builder with 8 screens: App Info, Install Settings, Files, UI & Branding, Shortcuts, Registry, Run Hooks, Build
- Safe in-place upgrade: reinstalling the same app cleans previous files, shortcuts, registry, and PATH entries before proceeding
- Streaming payload assembly: packager writes compressed chunks to a temp file instead of holding everything in RAM
- Release profile: `opt-level = "z"` with fat LTO and strip for runtime stubs; `opt-level = 3` for packager/GUI
- CI: parallel jobs for fmt, clippy, Linux build+test, Windows build+test
- Comprehensive unit tests for `InstallConfig` validation (22 tests) and `PackagedInstaller` manifest format (8 tests)
- `CONTRIBUTING.md` with build, test, and lint instructions
- This changelog

### Changed
- CI workflow renamed from "Rust" to "CI" and split into 4 parallel jobs
- `ProgressContext` struct replaces repeated progress parameters in runtime engine
