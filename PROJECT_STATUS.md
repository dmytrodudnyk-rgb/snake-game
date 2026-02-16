# Neon Snake - Project Status

**Last Updated:** 2026-02-16
**Current Platform:** Windows 10 Pro
**Session Status:** Phase 2 - 3 of 5 tasks complete

---

## Quick Summary

A classic snake game built with Rust + SDL2, featuring neon retro pixel art aesthetics. Cross-platform (Windows/macOS). Phase 1 complete, Phase 2 in progress.

**Tech Stack:** Rust, SDL2, SDL2_ttf, cargo-make
**Repository:** https://github.com/dmytrodudnyk-rgb/snake-game
**Branch:** master (2 commits ahead of origin)

---

## ✅ What's Working

### Core Game (Phase 1 - Complete)
- Full snake gameplay with grid-based movement
- Food spawning, collision detection (walls, self)
- Score tracking with configurable speed progression
- Main menu with keyboard/gamepad navigation
- Local leaderboard (top 5 scores, JSON persistence)
- Pause functionality (ESC)
- Input buffering for responsive controls

### Visual & Polish (Phase 2 - Partial)
- ✅ **SDL2_ttf text rendering** - Press Start 2P font @ 16pt
  - Menu, game HUD, leaderboard all render real text
  - Singleton resource manager (no lifetime cascading)
- ✅ **Smooth snake movement** - Head interpolates between grid cells at 60 FPS
  - Linear interpolation without cloning (calculates previous position)
  - Body segments stay on grid
- ✅ **Food pulse animation** - Sine wave scaling (0.9x to 1.1x, 1 sec loop)

### Build System
- **Windows:** vcpkg submodule with SDL2 + SDL2_ttf installed
- **Cross-platform:** cargo-make task runner handles everything
- **Zero manual config:** Automated linker paths and runtime DLL PATH

---

## 🚧 What's Left - Phase 2

### Remaining Tasks (Estimated: 2-3 hours)

#### 1. Audio System (SDL2_mixer) - ~45 min
**Status:** Not started
**What to do:**
- Install: `./third-party/vcpkg/vcpkg install sdl2-mixer:x64-windows`
- Add `mixer` feature to Cargo.toml sdl2 dependency
- Find/download sound files:
  - `crunch.wav` - Food eaten (short, crisp)
  - `click.wav` - Menu navigation (subtle UI sound)
- Update `audio.rs` - replace print stubs with real SDL2_mixer calls
- Create `assets/sounds/` directory for audio files

**Current state:** Audio stubs print "[Audio] Click" and "[Audio] Crunch"

#### 2. Name Input UI - ~1-2 hours
**Status:** Not started
**What to do:**
- Create `name_input_state.rs` module
- Implement text input handling (capture keystrokes)
- Render input overlay with cursor blink animation
- Integrate into game over flow when high score achieved
- Character limit: 12 chars, alphanumeric + underscore
- ESC to skip (defaults to "PLAYER"), Enter to confirm

**Current state:** Leaderboard hardcodes "PLAYER" name (app.rs:151)

---

## 📁 Project Structure

```
snake-game/
├── src/
│   ├── main.rs              # Entry point + TTF init
│   ├── app.rs               # SDL context + game loop
│   ├── resources.rs         # Singleton TTF context manager
│   ├── game_state.rs        # Game logic + interpolation
│   ├── rendering/
│   │   ├── text_renderer.rs # SDL2_ttf text rendering
│   │   ├── game_renderer.rs # Game + leaderboard rendering
│   │   └── menu_renderer.rs # Main menu rendering
│   ├── audio.rs             # Audio stubs (needs SDL2_mixer)
│   └── ...
├── third-party/
│   ├── fonts/
│   │   └── PressStart2P.ttf # Retro pixel font (SIL OFL)
│   └── vcpkg/               # Git submodule with SDL2 + SDL2_ttf
├── .cargo/config.toml       # Windows MSVC linker paths
├── Makefile.toml            # cargo-make build tasks
└── config.toml              # Game configuration
```

---

## 🔧 Build & Run

**Prerequisites:**
- Rust (rustup)
- cargo-make: `cargo install cargo-make`

**Windows (first-time setup):**
```bash
git submodule update --init --recursive
./third-party/vcpkg/bootstrap-vcpkg.bat
./third-party/vcpkg/vcpkg install sdl2:x64-windows sdl2-ttf:x64-windows
```

**All platforms:**
```bash
cargo make build  # Compile
cargo make run    # Launch game
```

**No manual environment variables needed** - automated via .cargo/config.toml and Makefile.toml.

---

## 🎯 Next Session - Quick Start

### Option A: Ship Audio (Fastest)
1. Install SDL2_mixer via vcpkg
2. Find free .wav sound files (freesound.org, OpenGameArt)
3. Update audio.rs to use SDL2_mixer
4. Test in-game sounds
5. **Result:** Game feels complete with audio feedback

### Option B: Build Name Input UI (More Complex)
1. Create name_input_state.rs module
2. Implement keyboard text input handling
3. Build UI overlay rendering
4. Wire into game over flow
5. **Result:** Leaderboard shows real player names

**Recommendation:** Start with audio (quick win), then tackle name input.

---

## 📝 Recent Commits

```
c9472e7 Add smooth movement and food pulse animations - Phase 2 animations complete
3eb0351 Add SDL2_ttf text rendering - Phase 2 text rendering complete
ae284f5 Add project status summary and include BMAD tooling
```

**Branch status:** 2 commits ahead of origin/master (not pushed yet)

---

## 🎮 Controls

**Keyboard:**
- Arrow Keys: Navigate menu / Move snake
- Enter: Select / Restart
- ESC: Pause / Back to menu / Exit

**Gamepad:**
- D-Pad: Navigate / Move
- A Button: Select / Restart
- Start: Pause

---

## 💡 Design Decisions

### Why Singleton for TTF Context?
Avoids cascading lifetimes through App → Renderers → TextRenderer. Using `once_cell` with `'static` fonts keeps code clean.

### Why No Snake Cloning for Interpolation?
Only the head needs interpolation. Previous position calculated from current position + direction. Zero allocation overhead.

### Why Sine Wave for Pulse?
TAU (2π) creates smooth continuous animation. Scale range 0.9-1.1 keeps food recognizable while adding visual interest.

---

## 🐛 Known Issues

None currently blocking. Audio and name input are planned features, not bugs.

---

*Ready to pick up Phase 2 tasks tomorrow. Start with audio for quick progress, or dive into name input UI for the full feature set.*
