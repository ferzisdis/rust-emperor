# 🔧 Developer Continuation Guide - Dark Emperor Project

## Quick Context Refresh

This is a Rust + htmx rewrite of the Java ME game "DarkEmperorMD". It's a medieval strategy game where players advance from Baron to Emperor by managing resources, building structures, and meeting rank requirements.

**Current Status:** ✅ Fully playable with core mechanics working
**Location:** `/Users/ferzisdis/rustprojs/rust-emperor`

---

## 🚀 Quick Start (Next Session)

```bash
cd /Users/ferzisdis/rustprojs/rust-emperor

# Test the current build
cargo run

# Open browser to http://127.0.0.1:3000
# Play a round to refresh your memory on mechanics
```

---

## 📁 Project Structure Quick Reference

```
src/
├── main.rs                 # Axum server setup, entry point
├── game/
│   ├── mod.rs             # Module exports
│   ├── state.rs           # GameState struct - ALL game logic here
│   └── events.rs          # 15 random events (EventGenerator)
└── routes/
    ├── mod.rs             # Route exports
    ├── menu.rs            # Menu routes (/, /about, /highscores)
    └── game.rs            # Game action routes (/game/*, round processing)

templates/                  # Askama HTML templates
├── base.html              # Base layout with htmx script
├── menu.html              # Main menu
├── new_game_form.html     # Game setup form
├── game.html              # MAIN GAME INTERFACE (most complex)
├── about.html             # About page
└── highscores.html        # Scores display

static/css/
└── style.css              # 666 lines of medieval-themed CSS

RustEmperorOriginal/       # Original Java source (reference only)
```

---

## 🔑 Key Code Locations

### Where to Find Things:

**Game State & Logic:**
- `src/game/state.rs` → All resource management, building logic, requirements
- `GameState` struct (line ~10) → All game variables
- `apply_difficulty_modifier()` → Starting resource adjustments
- `get_grade_requirements()` → Rank advancement requirements (line ~295)
- `finish_round()` → Round completion logic (line ~355)

**Round Processing:**
- `src/routes/game.rs` → `finish_round()` handler (line ~142)
- `apply_round_effects()` → Farm production, taxes, food consumption (line ~175)

**Random Events:**
- `src/game/events.rs` → All 15 event types
- `generate_random_event()` → Event selection (30% chance none) (line ~26)

**UI Templates:**
- `templates/game.html` → Main game interface (most important)
- Lines 1-50: Header, resources bar
- Lines 51-70: Castle display
- Lines 82-100: Tax/Food settings
- Lines 102-150: Buildings panel
- Lines 175-189: Rank requirements display

**Styling:**
- `static/css/style.css`
- Lines 1-15: CSS variables (colors)
- Lines 280-350: Resources bar
- Lines 350-450: Game panels
- Lines 450-550: Buildings, buttons

---

## 🎯 Priority TODO Items

### HIGH PRIORITY (Implement These First)

1. **Session Management** ⭐⭐⭐
   - **Problem:** All players share one game state (SharedGameState in main.rs)
   - **Solution:** Use session cookies + HashMap of game states
   - **Files to Modify:**
     - `src/main.rs` → Add session layer
     - `src/routes/game.rs` → Extract session ID, load correct game
   - **Crates Needed:** `axum-sessions` or `tower-sessions`
   ```rust
   // Add to Cargo.toml:
   tower-sessions = "0.9"
   
   // In main.rs, replace SharedGameState with:
   type GameStates = Arc<RwLock<HashMap<String, GameState>>>;
   ```

2. **Event Display Modal** ⭐⭐
   - **Problem:** Events happen silently, no user feedback
   - **Solution:** Create event modal template, show before redirecting
   - **Files to Create:**
     - `templates/event_modal.html`
   - **Files to Modify:**
     - `src/routes/game.rs` → `finish_round()` returns modal instead of redirect
   - **htmx Pattern:**
   ```html
   <!-- Show modal, then redirect after user clicks OK -->
   <div id="modal" hx-get="/game" hx-trigger="click" hx-swap="outerHTML">
     <h2>Event: Plague!</h2>
     <p>Lost 100 citizens</p>
     <button>OK</button>
   </div>
   ```

3. **Save/Load Game** ⭐⭐
   - **Files to Modify:**
     - `src/game/state.rs` → GameState already has Serialize/Deserialize!
     - `src/routes/menu.rs` → Add save/load routes
   - **Simple Implementation:**
   ```rust
   // Save to JSON file
   use std::fs;
   let json = serde_json::to_string(&game_state)?;
   fs::write("saves/player_save.json", json)?;
   
   // Load from JSON file
   let json = fs::read_to_string("saves/player_save.json")?;
   let game_state: GameState = serde_json::from_str(&json)?;
   ```

### MEDIUM PRIORITY

4. **Persistent Highscores**
   - Use SQLite (crate: `rusqlite`) or just JSON file
   - Store: name, score, difficulty, year, date
   - **Files:** 
     - Create `src/storage/highscores.rs`
     - Modify `src/routes/menu.rs` → Load real scores

5. **Trading System**
   - **Files:** `templates/trade_modal.html`, `src/routes/game.rs`
   - Allow buying/selling: Food, Iron, Weapons
   - Prices from `GameState`: price_for_food, price_for_armor, price_for_weapon

6. **Military Recruitment**
   - **Files:** `templates/military_modal.html`
   - Recruit soldiers: Cost = soldier_price (100 gold/soldier)
   - Need weapons: 1 weapon per soldier
   - Train soldiers from population

### LOW PRIORITY

7. **Better Error Handling**
   - Currently uses `unwrap()` in many places
   - Replace with proper error pages

8. **WebSocket for Multiplayer**
   - Real-time updates
   - Spectator mode

---

## 🐛 Known Issues to Fix

1. **Grade Advancement Display**
   - Rank up happens but no visual celebration
   - **Fix:** Show congratulations modal after `finish_round()` if grade increased
   - **Location:** `src/routes/game.rs` line ~150

2. **Settings Dropdowns Don't Update Instantly**
   - Changes taxes/food but reloads whole page
   - **Fix:** Change hx-target to just the resources bar
   - **Location:** `templates/game.html` lines 82-100

3. **Castle Upgrade Button Stays Disabled**
   - Need to check is_castle_upgrade_in_this_round correctly
   - **Location:** `src/game/state.rs` line ~227

4. **Food Shortage Doesn't Show Warning**
   - Citizens starve silently
   - **Fix:** Add alert/notification system

---

## 💡 Quick Wins (Easy Improvements)

1. **Add Tooltips** (15 minutes)
   - Add `title` attributes to buildings
   - Example: `title="Farms produce 500 food per round"`

2. **Keyboard Shortcuts** (30 minutes)
   - Press 'F' to finish round
   - Press '1-5' to select tax level
   ```javascript
   document.addEventListener('keypress', (e) => {
     if (e.key === 'f') document.querySelector('.btn-finish-round').click();
   });
   ```

3. **Add "Undo" Button** (1 hour)
   - Store previous GameState
   - Let player undo last round

4. **Dark Mode Toggle** (2 hours)
   - Add CSS variables for dark theme
   - Toggle button in header

---

## 🔧 Development Workflow

### Making Changes:

```bash
# 1. Make code changes in src/
# 2. Test immediately (fast recompile):
cargo run

# 3. For template changes (HTML), just refresh browser!
#    Askama reloads automatically in debug mode

# 4. For CSS changes, just refresh browser!
#    Static files served directly

# 5. For big changes, clean build:
cargo clean && cargo build --release
```

### Testing Checklist:
- [ ] Start new game (all 3 difficulties)
- [ ] Build each building type
- [ ] Upgrade castle
- [ ] Change taxes/food rations
- [ ] Finish 5-10 rounds
- [ ] Check requirements update
- [ ] Trigger grade advancement
- [ ] Play until win/loss condition

---

## 📚 Important Patterns & Conventions

### htmx Patterns Used:
```html
<!-- Standard form post -->
<button hx-post="/game/build-farm" 
        hx-target="body" 
        hx-swap="innerHTML">

<!-- Dropdown that triggers post on change -->
<select hx-post="/game/set-taxes" 
        hx-target="body">

<!-- Get content into modal -->
<button hx-get="/game/trade" 
        hx-target="#modal-container">
```

### Rust Patterns:
```rust
// Shared state pattern
type SharedGameState = Arc<RwLock<Option<GameState>>>;

// Reading state
let state = game_state.read().unwrap();
if let Some(ref game) = *state { /* use game */ }

// Writing state
let mut state = game_state.write().unwrap();
if let Some(ref mut game) = *state { game.gold += 100; }

// Always drop() before redirect to release lock!
drop(state);
Redirect::to("/game")
```

---

## 🎨 Design Decisions Made

1. **Why htmx?**
   - No JavaScript framework complexity
   - Server-side rendering = SEO friendly
   - Progressive enhancement
   - Perfect for this type of turn-based game

2. **Why Shared State Instead of Database?**
   - Simpler for MVP
   - Easy to migrate later
   - Fast development

3. **Why Emojis Instead of Images?**
   - No asset management
   - Works everywhere
   - Scales perfectly
   - Maintains medieval charm

4. **Color Scheme Choice**
   - Brown/gold = medieval parchment feel
   - High contrast for accessibility
   - Professional yet playful

---

## 🔮 Architecture Evolution Path

### Current: MVP Architecture
```
Browser ←→ Axum Server ←→ Shared Memory State
```

### Phase 2: Multi-User
```
Browser ←→ Axum Server ←→ Session Manager ←→ In-Memory HashMap
                                         ↓
                                    JSON File Backup
```

### Phase 3: Production
```
Browser ←→ Axum Server ←→ Session Manager ←→ PostgreSQL
                    ↓
              WebSocket for Real-time
```

---

## 📝 Code Style Guide

### Naming:
- Structs: `PascalCase` (GameState, EventGenerator)
- Functions: `snake_case` (build_farm, finish_round)
- Templates: `snake_case.html` (game.html, new_game_form.html)
- Routes: kebab-case (`/game/build-farm`)

### Comments:
- Add `//` for complex logic
- Use `///` for public API docs
- Keep comments updated!

### Error Handling:
- Use `Result<T, E>` for operations that can fail
- Use `Option<T>` for optional values
- Only `unwrap()` when you're 100% sure it won't panic

---

## 🧪 Testing Strategy

### Manual Testing Areas:
1. **Resource Overflow:** Try to spend more than you have
2. **Negative Values:** Check resources can't go negative
3. **Edge Cases:** 
   - Year 1500 (game end)
   - Grade 5 (final rank)
   - Castle level 8
   - 0 population
   - 0 food

### Future: Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_build_farm() {
        let mut state = GameState::default();
        state.build_farm().unwrap();
        assert_eq!(state.farm_quantity, 2);
    }
}
```

---

## 🎯 Session Continuation Checklist

When you come back to this project:

- [ ] Read this file (you're doing it! ✅)
- [ ] Run `cargo run` to test current state
- [ ] Play through one full game to remember mechanics
- [ ] Check git status: `git status`
- [ ] Review last commit: `git log -1`
- [ ] Pick one TODO item from "Priority TODO Items"
- [ ] Create a new branch: `git checkout -b feature/your-feature`
- [ ] Code, test, commit, repeat
- [ ] Have fun! 🎮

---

## 🔗 Useful Resources

**Rust Web:**
- Axum docs: https://docs.rs/axum/latest/axum/
- Askama: https://docs.rs/askama/latest/askama/
- Tower sessions: https://docs.rs/tower-sessions/

**htmx:**
- Main site: https://htmx.org/
- Examples: https://htmx.org/examples/
- Attributes reference: https://htmx.org/reference/

**Game Design:**
- Original game source in `RustEmperorOriginal/`
- Localization.java has all game text
- GameStorage.java has requirements and calculations

---

## 💬 Development Philosophy

**Keep it Simple:**
- Don't over-engineer early
- Working > Perfect
- Iterate, don't rewrite

**User First:**
- Fast loading (htmx helps)
- Clear feedback (events, errors)
- No surprises (show what will happen)

**Code Quality:**
- Readable > Clever
- Comment the "why" not the "what"
- Small functions, clear names

---

## 🎉 Motivation

You've built something cool! This is a fully functional game that:
- Preserves a classic mobile game
- Uses modern, clean technology
- Is fun to play and extend

Every feature you add makes it better. Every bug you fix makes it smoother.

**The code is good. The architecture is solid. You can build anything on top of this foundation.**

Now go make it even more awesome! 👑

---

**Last Updated:** 2024 (Initial development complete)
**Next Steps:** Pick one HIGH PRIORITY item and start coding!
