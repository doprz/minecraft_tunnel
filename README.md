# minecraft_tunnel

[![crates.io](https://img.shields.io/crates/v/minecraft_tunnel)](https://crates.io/crates/minecraft_tunnel)

A terminal-based Minecraft voxel tunnel renderer using DDA ray casting.

## Features

- Real-time procedural voxel rendering in your terminal
- DDA (Digital Differential Analyzer) ray casting for efficient 3D rendering
- Minecraft-inspired procedural textures (stone, grass, brick, wood, water, leaves)
- Full RGB color support
- Uses Unicode half-block characters for double the vertical resolution
- Automatic terminal size detection and adaptation
- High-performance Rust implementation
- Cross-platform

## Installation

### Cargo

Install directly from crates.io
```bash
cargo install minecraft_tunnel
```

### From Source
To build and install from source, first checkout the tag or branch you want to install, then run
```bash
cargo install --path .
```
This will build and install `minecraft_tunnel` in your ~/.cargo/bin. Make sure that ~/.cargo/bin is in your $PATH variable.

### Nix

Run without installing
```bash
nix run github:doprz/minecraft_tunnel
```

Local flake
```bash
nix run
```

## License
`minecraft_tunnel` is licensed under the MIT License

SPDX-License-Identifier: MIT