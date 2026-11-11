# The Nine Worlds Project Summary

## Overview

This repository is a Rust workspace for a Norse mythology-inspired 3D action-adventure RPG prototype. The project is still early-stage and acts as a foundation for a game engine, gameplay systems, rendering, asset management, and world-building.

The project uses the Bevy game engine and is organized as a multi-crate workspace. The README describes it as:

> "A Norse mythology inspired 3D action-adventure RPG."

## Current state

The project is not a complete game yet. It already contains:

- a workspace-based Rust project structure
- multiple engine/gameplay crates
- an executable game app
- an editor app
- a rendering prototype scene
- basic character movement and physics behavior
- asset and world support directories

## Project structure

### Root-level files

- `Cargo.toml` — workspace manifest defining shared dependencies and crate members
- `README.md` — project overview, build instructions, and project structure notes
- `LICENCE` — project license file
- `Cargo.lock` — generated dependency lockfile
- `.cargo/` — cargo configuration
- `.vscode/` — editor settings

### Main folders

- `apps/` — game and editor executables
- `crates/` — engine, gameplay, and system crates
- `assets/` — models, textures, audio, fonts, world content, and UI art
- `config/` — configuration files
- `docs/` — design/technical documentation folders
- `tests/` — automated tests
- `scripts/` — development utilities

## Apps

### `apps/nw-game`

This is the main game executable. It depends on:

- `nw-core`
- `nw-renderer`
- `nw-audio`
- `nw-assets`
- `nw-world`
- `nw-gameplay`
- `nw-ui`
- `bevy`

This app is likely the entry point for the playable game prototype.

### `apps/nw-editor`

This is the editor/tooling application. It depends on:

- `nw-core`
- `nw-renderer`
- `nw-assets`
- `nw-world`
- `nw-tools`
- `egui`
- `tracing`

This is likely intended for editing, world tooling, or content workflow support.

## Crates

### `crates/nw-renderer`

This crate contains the active rendering and camera prototype logic. The file `crates/nw-renderer/src/lib.rs` sets up a Bevy scene with:

- a `Camera3d`
- directional lighting
- a large ground plane
- several primitive 3D meshes
- a basic humanoid avatar made from mesh parts

It includes a `NineWorldsPlugin` that registers startup and update systems:

- `setup_scene` on startup
- `control_camera` on update

The camera controller supports:

- WASD movement
- mouse-based look direction
- jump with Space
- quitting with Escape

The system uses fixed-step physics simulation and positions the camera behind the player while maintaining look direction.

### `crates/nw-physics`

This crate is built around physics support with:

- `oxijolt` as the physics backend
- `glam` for math
- `thiserror` for typed errors
- `tracing` for logging

It is responsible for the gravity, collision, and movement behavior used by the player character.

### `crates/nw-world`

This crate handles world logic, ECS integration, serialization, and tracing. It depends on:

- `glam`
- `bevy_ecs`
- `serde`
- `tracing`

This suggests world data and gameplay state are modeled as structured ECS/world components.

### `crates/nw-gameplay`

This crate contains gameplay systems. It depends on:

- `bevy_ecs`
- `glam`
- `serde`
- `tracing`
- `thiserror`

This likely contains interactions, rules, combat, progression, and game logic later in development.

### `crates/nw-ui`

This crate provides UI support using `egui`, likely for menus, HUDs, and editor tools.

### `crates/nw-assets`

This crate handles asset loading and processing. Its dependencies include:

- `gltf`
- `image`
- `serde`
- `ron`
- `anyhow`
- `tracing`

This is intended for loading models, textures, and structured asset data.

### `crates/nw-audio`

This crate is focused on audio support and logging.

### `crates/nw-core`

This is the shared core layer for reusable math and shared game logic. It depends on:

- `glam`
- `serde`
- `thiserror`
- `tracing`

### `crates/nw-tools`

This crate is for tooling utilities, likely file watching or dev tooling. It depends on:

- `notify`
- `anyhow`
- `tracing`

## Dependencies and technology stack

The workspace uses the following key technologies:

- Rust
- Bevy 0.20.1
- Jolt physics (`oxijolt`)
- `glam` for vector/matrix math
- `egui` for UI
- `serde`, `ron`, and `serde_json` for serialization
- `tracing` and `tracing-subscriber` for logging
- `notify` for file watching/tooling support
- GLTF and image processing libraries for 3D and texture asset support

## Assets and content

The project includes a large content tree under `assets/`, including:

- `animations/`
- `audio/`
- `characters/`
- `fonts/`
- `materials/`
- `models/`
- `particles/`
- `shaders/`
- `textures/`
- `ui/`
- `worlds/`

This indicates the project is designed to support a fairly complete world and art pipeline.

## Gameplay prototype features

Based on the renderer and README, the prototype currently demonstrates:

- third-person style 3D camera movement
- WASD character movement
- mouse look
- jumping
- floor collision and gravity using physics
- a humanoid test character
- a stylized scene with primitive walls, lights, and ground

## Build and run instructions

The README includes the following usage:

```powershell
cargo build
cargo run -p nw-game
```

There are also validation commands:

```powershell
cargo check --workspace
cargo fmt --all
cargo clippy --workspace
```

## Summary

The Nine Worlds project is a Rust + Bevy game-development workspace for a Norse-themed 3D RPG prototype. It already contains a strong foundation for rendering, camera control, player movement, physics, world data, UI, asset management, and content structure. While the game is still in an early prototype stage, the project clearly aims for a substantial action-adventure RPG with a broad content pipeline and modular engine architecture.

## Key files to inspect

- `README.md`
- `Cargo.toml`
- `apps/nw-game/Cargo.toml`
- `apps/nw-editor/Cargo.toml`
- `crates/nw-renderer/src/lib.rs`
- `crates/nw-physics/Cargo.toml`
- `crates/nw-world/Cargo.toml`
- `crates/nw-gameplay/Cargo.toml`
