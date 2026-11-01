# The Nine Worlds

A Norse mythology inspired 3D action-adventure RPG.

## Project structure

- apps/       Game and editor executables
- crates/     Engine and gameplay systems
- assets/     Models, textures, audio and world content
- config/     Game configuration
- docs/       Design and technical documentation
- tests/      Automated tests
- scripts/    Development utilities

## Build

```powershell
cargo build
cargo run -p nw-game
```

## Check

```powershell
cargo check --workspace
cargo fmt --all
cargo clippy --workspace
```
