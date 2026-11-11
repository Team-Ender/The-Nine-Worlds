# The Nine Worlds

A Norse mythology inspired 3D action-adventure RPG.

The game runtime uses Bevy 0.20.1 for its application, windowing, input, and 3D rendering.
Use Rust 1.97.1 or newer.

The prototype starts at a title screen. Singleplayer begins the scene, which supports third-person
WASD movement, mouse look, and Space to jump. Press Escape to pause or resume; the menus provide
an option to return to the title screen. The title screen can exit the application. The humanoid
test character uses Jolt gravity and floor collision.

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
