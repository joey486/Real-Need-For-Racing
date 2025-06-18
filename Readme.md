# Real Need For Racing

A 2D/3D racing game built with Rust and the Bevy game engine.

---

## About

This project is a racing game developed using [Bevy 0.11](https://bevyengine.org/), a modern Rust game engine. It features player movement, enemy cars, scrolling road lines, audio playback, and a game over UI.

---

## Features

- Player-controlled car movement
- Enemy spawning and movement
- Scrolling road lines for immersive effect
- Background music and sound effects using `bevy_kira_audio`
- Game Over screen with restart functionality
- Embedded assets support (optional)

---

## Dependencies & Versions

- Rust edition: 2021
- [Bevy](https://crates.io/crates/bevy): 0.11  
- [rand](https://crates.io/crates/rand): 0.9.1  
- [bevy_embedded_assets](https://crates.io/crates/bevy_embedded_assets): 0.8 (optional)  
- [bevy_kira_audio](https://crates.io/crates/bevy_kira_audio): 0.8.0  
- [winres](https://crates.io/crates/winres): 0.1 (build dependency for Windows resource management)  

---

## Getting Started

### Prerequisites

- Install Rust (latest stable recommended): https://rustup.rs/  
- Ensure you have a suitable graphics driver for running Bevy (Vulkan backend)  
- Windows users: Visual Studio Build Tools (for compiling)  

---

### How to Run

1. Clone the repository:

   ```bash
    git clone <your-repo-url>
    cd Real-Need-For-Racing
   ```
2. (Optional) Add embedded assets to the assets/ folder (if used).

3. Build and run the game:
```bash
cargo run
```