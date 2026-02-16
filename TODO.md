# Snake Game TODO

## Phase 1 Cleanup (Deferred)

### Extract Game Logic from App
**Priority:** Medium
**Description:** `app.rs` currently contains both SDL/windowing lifecycle AND game loop logic. These should be separated:

- **App struct** - Should only own:
  - SDL context, window, canvas, event pump
  - Frame loop (event polling, render calls)
  - High-level state transitions

- **Game struct** (new) - Should own:
  - Game state machine (AppState)
  - All game states (MenuState, GameState, LeaderboardState)
  - Game update logic
  - Timing logic

**Approach:**
```
App::run() {
    loop {
        handle SDL events
        game.update()
        game.render(&mut canvas)
    }
}
```

**Files to modify:**
- `src/app.rs` - Extract game logic
- `src/game.rs` (new) - Game entity
- Possibly: `src/game_loop.rs` for update/render logic

---

## Phase 2 - Visual Polish & Audio

### Text Rendering
- [ ] Add SDL2_ttf dependency
- [ ] Implement proper font rendering in `text_renderer.rs`
- [ ] Load pixel font asset
- [ ] Replace placeholder rectangles with real text

### Audio System
- [ ] Add SDL2_mixer dependency
- [ ] Implement audio playback in `audio.rs`
- [ ] Add crunch sound effect asset
- [ ] Add menu click sound effect asset
- [ ] Wire up audio calls (already stubbed)

### Visual Effects
- [ ] Implement smooth snake movement interpolation
- [ ] Add food pulse animation
- [ ] Add particle effects on food consumption
- [ ] Screen flash effect on food eat (optional)

### Name Input UI
- [ ] Create `name_input_state.rs` for high score name entry
- [ ] Add name input rendering to `game_renderer.rs`
- [ ] Add name input handling to `input_handler.rs`
- [ ] Integrate into game over → leaderboard flow
- [ ] 12 character limit, alphanumeric + underscore only
- [ ] Blinking cursor after last character

---

## Future Enhancements (Optional)

### Gameplay
- [ ] Power-ups (speed boost, invincibility, etc.)
- [ ] Multiple difficulty levels
- [ ] Different game modes (classic, modern, timed)
- [ ] Obstacles/walls on the grid

### UI/UX
- [ ] Settings menu (volume, controls, difficulty)
- [ ] High score replay/ghost snake
- [ ] Better animations and transitions
- [ ] Controller vibration on collision

### Technical
- [ ] Unit tests for game logic
- [ ] Integration tests for state transitions
- [ ] Benchmarking for performance
- [ ] Release builds optimization
