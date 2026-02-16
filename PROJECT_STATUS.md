# Neon Snake - Project Status

## Project Overview

**Goal:** Build a classic snake game as a pet project with neon retro pixel art aesthetics.

**Tech Stack:** Rust + SDL2, cross-platform (macOS/Windows/Linux), native desktop app.

---

## Initial Requirements

- Native desktop app (no web)
- Simple graphics (no heavy engines like Unity/Unreal)
- Cross-platform: Windows + macOS
- Classic snake gameplay
- Main menu with Start/Leaderboard/Exit
- Local leaderboard (top 5 scores)
- Configurable game speed progression
- Keyboard + gamepad support
- Neon retro pixel art visual style
- Grid-based movement with smooth interpolation
- Sound effects (crunch on food eat, click on menu)
- Input buffering for responsive controls

---

## ✅ COMPLETED - Phase 1

### Core Functionality
- [x] SDL2 setup with cross-platform build system
- [x] cargo-make task runner (works on all platforms)
- [x] System SDL2 (Homebrew on macOS, vcpkg on Windows)
- [x] Latest dependencies (SDL2 0.38, rand 0.10, toml 1.0)
- [x] Main menu with keyboard/gamepad navigation
- [x] Core game loop and state machine (MainMenu/Playing/Leaderboard)
- [x] Snake movement with grid-based logic
- [x] Food spawning and collision detection
- [x] Wall and self-collision
- [x] Score tracking
- [x] Configurable speed progression (config.toml)
- [x] Local leaderboard persistence (JSON)
- [x] Input buffering (1 input queue for responsive controls)
- [x] Pause functionality (ESC)
- [x] Game over + restart flow

### Architecture (Refactored)
- [x] **Modular rendering system:**
  - `rendering/colors.rs` - Color constants
  - `rendering/text_renderer.rs` - Text rendering (placeholder, ready for SDL2_ttf)
  - `rendering/menu_renderer.rs` - Menu rendering
  - `rendering/game_renderer.rs` - Game + leaderboard rendering
- [x] **Clean separation:**
  - `main.rs` - Entry point (20 lines)
  - `app.rs` - SDL context + game loop
  - `input_handler.rs` - Input mapping logic
  - `game_state.rs` - Game logic
  - `config.rs` - Configuration loading
- [x] Grid rendering with semi-transparent lines
- [x] Neon color palette implemented
- [x] Snake segments with 2px gap (segmented look)

### Infrastructure
- [x] Git repository initialized
- [x] GitHub remote configured (git@github.com:dmytrodudnyk-rgb/snake-game.git)
- [x] SSH authentication configured (~/.ssh/id_ed25519_openvpn)
- [x] Comprehensive .gitignore
- [x] README.md with installation instructions
- [x] TODO.md for future enhancements

### Windows Build Environment (COMPLETED)
- [x] **vcpkg as git submodule** - `third-party/vcpkg/` with SDL2 installed
- [x] **Automated dependency management** - No manual environment variables needed
- [x] **.cargo/config.toml** - Windows-specific linker paths configured
- [x] **Makefile.toml updated** - Windows PATH handling for SDL2.dll at runtime
- [x] **.gitignore updated** - vcpkg build artifacts excluded
- [x] **README.md updated** - Complete Windows setup instructions with vcpkg submodule
- [x] **Build verified** - `cargo make build` works out of the box
- [x] **Runtime verified** - `cargo make run` launches game successfully

### Current State
- Game builds and runs successfully on both macOS and Windows
- All core gameplay functional
- Visual rendering uses placeholder rectangles for text
- Audio system stubbed (prints to console)
- Leaderboard uses hardcoded "PLAYER" name

---

---

## ✅ COMPLETED - Windows Platform Setup (2026-02-16)

### Objective
Set up Windows development environment with vcpkg as a git submodule for consistent cross-platform builds.

### What Was Done
1. **vcpkg Integration:**
   - Added vcpkg as git submodule at `third-party/vcpkg/`
   - Bootstrapped vcpkg on Windows
   - Installed SDL2 via vcpkg: `sdl2:x64-windows@2.32.10`

2. **Build Configuration:**
   - Created `.cargo/config.toml` with Windows MSVC linker search paths
   - Updated `Makefile.toml` to auto-configure SDL2.dll PATH on Windows
   - No manual environment variables required

3. **Documentation:**
   - Updated `README.md` with complete vcpkg submodule setup instructions
   - Added `.gitignore` entries for vcpkg build artifacts
   - Clarified Windows shell requirements (PowerShell/cmd/Git Bash)

4. **Verification:**
   - Installed cargo-make on Windows
   - Build successful: `cargo make build`
   - Runtime successful: `cargo make run`

### Files Modified
- `.gitignore` - Added vcpkg artifacts
- `.cargo/config.toml` - Created with Windows SDL2 paths
- `Makefile.toml` - Added Windows PATH configuration
- `README.md` - Updated Windows prerequisites section
- `PROJECT_STATUS.md` - This file

---

## 🚧 TODO - Phase 2

### Required Tasks

#### 1. Text Rendering (SDL2_ttf)
- [ ] Add `sdl2-ttf` dependency to Cargo.toml
- [ ] Install system SDL2_ttf library (brew/vcpkg)
- [ ] Update Makefile.toml library paths
- [ ] Find/download pixel font (Press Start 2P or similar)
- [ ] Implement font loading in `text_renderer.rs`
- [ ] Replace placeholder rectangles with real text rendering
- [ ] Test on all UI screens (menu, game, leaderboard)

#### 2. Audio System (SDL2_mixer)
- [ ] Add `sdl2-mixer` dependency to Cargo.toml
- [ ] Install system SDL2_mixer library (brew/vcpkg)
- [ ] Find/download sound effects:
  - `crunch.wav` - Food eaten sound
  - `click.wav` - Menu navigation sound
- [ ] Implement audio loading in `audio.rs`
- [ ] Wire up audio calls (already stubbed)
- [ ] Test audio playback

#### 3. Smooth Movement Animation
- [ ] Add interpolation state to GameState
- [ ] Store previous position + interpolation timer
- [ ] Render snake at interpolated position (linear interpolation over 100-150ms)
- [ ] Maintain grid-based movement logic

#### 4. Food Pulse Animation
- [ ] Add animation timer to food rendering
- [ ] Implement scale animation (0.9x to 1.1x, 1 second loop, sine wave)
- [ ] Apply transform when rendering food

#### 5. Name Input UI
- [ ] Create `name_input_state.rs` for high score entry
- [ ] Implement text input handling (capture keystrokes)
- [ ] Add input overlay rendering in `game_renderer.rs`
- [ ] Add cursor blink animation
- [ ] Integrate into game over flow (when high score achieved)
- [ ] Character limit: 12 chars max
- [ ] Allowed characters: alphanumeric + underscore
- [ ] ESC to skip (defaults to "PLAYER"), Enter to confirm
- [ ] Test full flow: game over → name input → leaderboard

### Optional Enhancements
- [ ] Particle effects on food consumption (3-5 particles burst)
- [ ] Screen flash effect on food eat
- [ ] Menu transition animations

---

## 📋 DEFERRED - Future Refactoring

### Extract Game Logic from App
**Priority:** Medium
**Description:** `app.rs` currently contains both SDL/windowing lifecycle AND game loop logic. Should be separated:
- **App struct** - SDL context, window, canvas, event pump, frame loop
- **Game struct** (new) - Game state machine, all game states, update logic, timing

See `TODO.md` for details.

---

## 🎯 Next Steps

**Status:** Windows setup complete. Ready to begin Phase 2.

**Phase 2 Tasks (6-9 hours):**
1. SDL2_ttf integration + font rendering
2. SDL2_mixer integration + sound effects
3. Smooth snake movement interpolation
4. Food pulse animation
5. Name input UI for leaderboard

**Before Starting Phase 2:**
- Commit Windows setup work
- Relaunch IDE to refresh cargo PATH
- Continue from updated PROJECT_STATUS.md

---

## Assets Needed (Phase 2)

- **Font:** Pixel/retro font (.ttf), e.g., Press Start 2P
- **Sounds:**
  - `crunch.wav` - Short, crisp food eat sound
  - `click.wav` - Subtle UI click sound

Create `assets/fonts/` and `assets/sounds/` directories.

---

## Build & Run

### Prerequisites
- Rust (via rustup)
- cargo-make (`cargo install cargo-make`)

### macOS
SDL2 via Homebrew:
```bash
brew install sdl2
```

### Windows
vcpkg submodule setup (one-time):
```bash
git submodule update --init --recursive
./third-party/vcpkg/bootstrap-vcpkg.bat
./third-party/vcpkg/vcpkg install sdl2:x64-windows
```

### All Platforms
```bash
# Build
cargo make build

# Run
cargo make run

# Clean
cargo make clean
```

**No manual environment variables needed** - paths configured automatically via `.cargo/config.toml` and `Makefile.toml`.

---

## Repository

- **GitHub:** https://github.com/dmytrodudnyk-rgb/snake-game
- **Branch:** master
- **Latest Commit (macOS):** Phase 1 complete (ae284f5)
- **Platform:** Windows 10 Pro (build environment active)

---

## Session Notes

**Current Platform:** Windows
**Development Started:** macOS → transitioned to Windows (2026-02-16)
**Windows Setup:** ✅ Complete - vcpkg submodule configured, build verified

---

*Last Updated: 2026-02-16 - Windows build environment setup complete, ready for Phase 2*
