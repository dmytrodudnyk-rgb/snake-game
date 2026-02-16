# Neon Snake

A classic snake game with neon retro pixel art aesthetics, built in Rust using SDL2.

## Features

- Classic snake gameplay with smooth movement
- Neon retro pixel art visual style
- Main menu with Start, Leaderboard, and Exit options
- Local leaderboard (top 5 scores)
- Configurable game speed progression
- Keyboard and gamepad support
- Cross-platform (Windows, macOS, Linux)

## Prerequisites

### Rust

Install Rust from [rustup.rs](https://rustup.rs/)

### SDL2

#### macOS
```bash
brew install sdl2
```

#### Linux (Ubuntu/Debian)
```bash
sudo apt install libsdl2-dev
```

#### Windows

**Prerequisites:**
- Rust installed via [rustup](https://rustup.rs/)
- Git Bash users: Add `%USERPROFILE%\.cargo\bin` to your bash PATH, or use PowerShell/cmd instead

**First-time setup:**

1. Initialize vcpkg submodule:
```bash
git submodule update --init --recursive
```

2. Bootstrap vcpkg:
```bash
./third-party/vcpkg/bootstrap-vcpkg.bat
```

3. Install SDL2:
```bash
./third-party/vcpkg/vcpkg install sdl2:x64-windows
```

**No manual environment variables needed** - `.cargo/config.toml` and `Makefile.toml` handle SDL2 paths automatically.

### cargo-make (optional but recommended)

```bash
cargo install cargo-make
```

## Building

### With cargo-make
```bash
cargo make build
```

### Without cargo-make
```bash
cargo build
```

### Release build
```bash
cargo make build-release
# or
cargo build --release
```

## Running

### With cargo-make
```bash
cargo make run
```

### Without cargo-make
```bash
cargo run
```

## Controls

### Keyboard
- **Arrow Keys** or **WASD**: Move snake / Navigate menu
- **Enter** or **Space**: Select menu item / Restart game
- **ESC**: Pause game / Back to menu / Exit from main menu

### Gamepad
- **D-Pad**: Move snake / Navigate menu
- **A Button**: Select menu item / Restart game
- **B Button**: Back
- **Start**: Pause

## Configuration

Edit `config.toml` to adjust game settings:

- **initial_speed_ms**: Starting movement delay (default: 150ms)
- **min_speed_ms**: Maximum speed cap (default: 50ms)
- **speed_increase_per_food**: Speed increase per food eaten (default: 5ms)
- **grid_size**: Grid dimensions (default: 30x30)
- **window_width/height**: Window size (default: 800x800)
- **grid_alpha**: Grid line transparency (default: 0.15)

## Project Structure

```
snake-game/
├── src/
│   ├── main.rs              # Entry point & game loop
│   ├── config.rs            # Configuration loading
│   ├── game_state.rs        # Game logic & snake mechanics
│   ├── menu_state.rs        # Main menu state
│   ├── leaderboard_state.rs # Leaderboard display
│   ├── renderer.rs          # All rendering logic
│   ├── input.rs             # Input handling
│   ├── audio.rs             # Audio system (TODO)
│   └── persistence.rs       # Score storage
├── config.toml              # Game configuration
├── Makefile.toml            # cargo-make tasks
└── README.md
```

## Development Tasks

Using cargo-make:
- `cargo make build` - Build the project
- `cargo make run` - Run the game
- `cargo make clean` - Clean build artifacts
- `cargo make check` - Check for errors
- `cargo make format` - Format code
- `cargo make test` - Run tests

## License

This is a personal educational project.
