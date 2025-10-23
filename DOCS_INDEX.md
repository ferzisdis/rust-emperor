# 📚 Documentation Index - Dark Emperor Project

Welcome! This file helps you navigate all the project documentation.

## 🎯 Start Here

**Never worked on this project before?**
→ Read: [`QUICKSTART.md`](QUICKSTART.md) (2 minutes)

**Coming back to continue development?**
→ Read: [`DEV_CONTINUATION_GUIDE.md`](DEV_CONTINUATION_GUIDE.md) (5 minutes)

**Want to understand the full project?**
→ Read: [`README.md`](README.md) (15 minutes)

## 📖 Documentation Files

### For Players

| File | Purpose | When to Read |
|------|---------|--------------|
| [`QUICKSTART.md`](QUICKSTART.md) | Get the game running in 2 minutes | First time playing |
| [`GAMEPLAY_VISUAL.md`](GAMEPLAY_VISUAL.md) | Visual guide with ASCII art, strategies | Learning to play |
| [`README.md`](README.md) - "How to Play" section | Detailed gameplay mechanics | Need detailed rules |

### For Developers

| File | Purpose | When to Read |
|------|---------|--------------|
| [`DEV_CONTINUATION_GUIDE.md`](DEV_CONTINUATION_GUIDE.md) | **START HERE** - Continue development easily | Every dev session |
| [`PROJECT_SUMMARY.md`](PROJECT_SUMMARY.md) | Technical overview, what was built | Understanding architecture |
| [`README.md`](README.md) - "Technology Stack" section | Tech details, dependencies | Understanding tools used |

### Scripts

| File | Purpose | How to Use |
|------|---------|------------|
| [`run.sh`](run.sh) | Quick start script | `./run.sh` |

## 🗂️ File Purpose Summary

### Essential Files (Read First)

1. **`DEV_CONTINUATION_GUIDE.md`** ⭐⭐⭐
   - Your main reference for development
   - Code locations, TODOs, patterns
   - Session continuation checklist
   - 460 lines of detailed dev info

2. **`QUICKSTART.md`**
   - Get running in under 2 minutes
   - Basic gameplay strategy
   - Troubleshooting common issues
   - 175 lines

3. **`README.md`**
   - Complete project documentation
   - Installation, gameplay, technology
   - 300+ lines

### Reference Files

4. **`PROJECT_SUMMARY.md`**
   - What was built and why
   - Architecture decisions
   - Statistics and metrics
   - 300 lines

5. **`GAMEPLAY_VISUAL.md`**
   - ASCII art visualizations
   - Game flow diagrams
   - Strategy charts
   - Pro tips
   - 380 lines

6. **`DOCS_INDEX.md`** (this file)
   - Documentation navigation
   - File organization guide

## 🎮 Quick Navigation by Task

### "I want to..."

**...play the game:**
1. Read: [`QUICKSTART.md`](QUICKSTART.md) → Basic Setup
2. Run: `./run.sh` or `cargo run --release`
3. Open: http://127.0.0.1:3000

**...understand game mechanics:**
1. Read: [`GAMEPLAY_VISUAL.md`](GAMEPLAY_VISUAL.md) → Visual guides
2. Read: [`README.md`](README.md) → "Game Features" section
3. Play a few rounds to experience it

**...add a new feature:**
1. Read: [`DEV_CONTINUATION_GUIDE.md`](DEV_CONTINUATION_GUIDE.md) → "Priority TODO Items"
2. Read: [`DEV_CONTINUATION_GUIDE.md`](DEV_CONTINUATION_GUIDE.md) → "Key Code Locations"
3. Read: [`PROJECT_SUMMARY.md`](PROJECT_SUMMARY.md) → "Architecture" section
4. Start coding!

**...fix a bug:**
1. Read: [`DEV_CONTINUATION_GUIDE.md`](DEV_CONTINUATION_GUIDE.md) → "Known Issues to Fix"
2. Read: [`DEV_CONTINUATION_GUIDE.md`](DEV_CONTINUATION_GUIDE.md) → "Key Code Locations"
3. Test with: `cargo run`

**...understand the technology:**
1. Read: [`README.md`](README.md) → "Technology Stack"
2. Read: [`PROJECT_SUMMARY.md`](PROJECT_SUMMARY.md) → "Why These Technologies"
3. Read: [`DEV_CONTINUATION_GUIDE.md`](DEV_CONTINUATION_GUIDE.md) → "Important Patterns"

**...see what's been done:**
1. Read: [`PROJECT_SUMMARY.md`](PROJECT_SUMMARY.md) → Full summary
2. Read: [`README.md`](README.md) → "Features" section
3. Look at: Source code in `src/`

## 📂 Source Code Structure

```
rust-emperor/
├── 📄 Documentation (you are here!)
│   ├── README.md                    # Main documentation
│   ├── QUICKSTART.md                # Quick start guide
│   ├── DEV_CONTINUATION_GUIDE.md    # Developer guide ⭐
│   ├── PROJECT_SUMMARY.md           # Technical summary
│   ├── GAMEPLAY_VISUAL.md           # Visual gameplay guide
│   └── DOCS_INDEX.md                # This file
│
├── 🦀 Rust Source Code
│   └── src/
│       ├── main.rs                  # Server entry point
│       ├── game/
│       │   ├── mod.rs               # Module exports
│       │   ├── state.rs             # Core game logic ⭐
│       │   └── events.rs            # Random events
│       └── routes/
│           ├── mod.rs               # Route exports
│           ├── menu.rs              # Menu routes
│           └── game.rs              # Game routes ⭐
│
├── 🎨 Frontend
│   ├── templates/                   # HTML templates
│   │   ├── base.html                # Base layout
│   │   ├── menu.html                # Main menu
│   │   ├── new_game_form.html       # Setup form
│   │   ├── game.html                # Main game UI ⭐
│   │   ├── about.html               # About page
│   │   └── highscores.html          # Scores
│   └── static/css/
│       └── style.css                # All styling ⭐
│
├── 📜 Scripts
│   └── run.sh                       # Quick start script
│
├── 🗂️ Original Source (Reference)
│   └── RustEmperorOriginal/         # Java ME source
│
└── ⚙️ Configuration
    ├── Cargo.toml                   # Rust dependencies
    └── Cargo.lock                   # Locked versions
```

## 🔍 Finding Information Fast

### By Topic:

**Game Rules & Mechanics:**
- Rank requirements → `GAMEPLAY_VISUAL.md` line 195
- Building costs → `README.md` line 45
- Random events → `PROJECT_SUMMARY.md` line 93

**Code Locations:**
- Game state → `DEV_CONTINUATION_GUIDE.md` line 58
- Round processing → `DEV_CONTINUATION_GUIDE.md` line 67
- UI templates → `DEV_CONTINUATION_GUIDE.md` line 74

**How-To Guides:**
- Install & run → `QUICKSTART.md` line 11
- First game strategy → `QUICKSTART.md` line 58
- Development workflow → `DEV_CONTINUATION_GUIDE.md` line 222

**Technical Details:**
- Architecture → `PROJECT_SUMMARY.md` line 33
- Technology choices → `README.md` line 197
- Design decisions → `DEV_CONTINUATION_GUIDE.md` line 288

## 📊 Documentation Stats

- **Total Documentation:** ~2,400 lines
- **Code Comments:** Inline throughout source
- **README Size:** 301 lines
- **Dev Guide Size:** 460 lines
- **Files:** 6 markdown docs + source comments

## 🎯 Recommended Reading Order

### First Time (Player):
1. `QUICKSTART.md` (5 min)
2. `GAMEPLAY_VISUAL.md` - "Quick Decision Tree" (2 min)
3. Play the game!
4. `GAMEPLAY_VISUAL.md` - Full read when stuck (15 min)

### First Time (Developer):
1. `QUICKSTART.md` (5 min) - Get it running
2. `PROJECT_SUMMARY.md` (10 min) - Understand what was built
3. `DEV_CONTINUATION_GUIDE.md` (15 min) - Learn the codebase
4. `README.md` (15 min) - Full context
5. Start coding!

### Returning Developer:
1. `DEV_CONTINUATION_GUIDE.md` (5 min refresh)
2. Pick a TODO item
3. Reference specific sections as needed

## 💡 Pro Tips

- **Bookmark this file** for quick navigation
- **Keep DEV_CONTINUATION_GUIDE.md open** while coding
- **Use Cmd/Ctrl+F** to search within docs
- **Read code comments** - they explain the "why"
- **Play the game first** before changing code

## 🔗 External Resources

**Rust:**
- Book: https://doc.rust-lang.org/book/
- Axum: https://docs.rs/axum/

**htmx:**
- Docs: https://htmx.org/docs/
- Examples: https://htmx.org/examples/

**Original Game:**
- Source: `RustEmperorOriginal/` directory
- Website: www.xboot.de

## ✅ Documentation Maintenance

When you update code, update these docs:
- Code structure changes → Update `DEV_CONTINUATION_GUIDE.md`
- New features → Update `README.md` and `PROJECT_SUMMARY.md`
- New mechanics → Update `GAMEPLAY_VISUAL.md`
- Better strategies found → Update `QUICKSTART.md`

---

## 🚀 Ready to Start?

Choose your path:
- 🎮 **Player** → [`QUICKSTART.md`](QUICKSTART.md)
- 👨‍💻 **Developer** → [`DEV_CONTINUATION_GUIDE.md`](DEV_CONTINUATION_GUIDE.md)
- 📖 **Curious** → [`README.md`](README.md)

**Good luck with your reign, Your Majesty!** 👑

---

*Last Updated: 2024*
*Documentation is current as of initial project completion*