# 🏰 Dark Emperor - Rust Edition

A modern web-based rewrite of the classic Java ME mobile game "DarkEmperorMD" using Rust, Axum, and htmx.

## 📖 About

Dark Emperor is a medieval strategy and resource management game where you start as a Baron or Baroness and work your way up through noble ranks to become Emperor or Empress. Manage your resources wisely, build structures, maintain your army, keep your citizens happy, and navigate through random events to achieve the ultimate rank!

### Original Game

- **Original:** DarkEmperorMD 1.0.2 by Steffen Bott
- **English Translation:** Oliver Schuster
- **Original Platform:** Java ME (Mobile)
- **More Info:** www.xboot.de

### This Rewrite

- **Backend:** Rust with Axum web framework
- **Frontend:** HTML + htmx for dynamic interactions
- **Templates:** Askama template engine
- **Styling:** Custom CSS with medieval theme

## 🎮 Game Features

### Core Gameplay

- **6 Noble Ranks:** Baron/Baroness → Count/Countess → Duke/Duchess → Prince/Princess → King/Queen → Emperor/Empress
- **Resource Management:** Gold, Food, Citizens, Soldiers, Weapons, Iron
- **Buildings:** Castle (8 upgrade levels), Farms, Mines, Smithies, Market Places
- **Settings:** Adjustable Taxes and Food Rations to balance income and popularity
- **Time Limit:** Achieve Emperor/Empress status before the year 1500
- **Random Events:** 15+ different events that can help or hinder your progress
- **Difficulty Levels:** Easy, Medium, Hard

### Game Mechanics

- **Taxes:** Higher taxes increase gold income but decrease popularity
- **Food Supply:** Better rations improve morale but consume more food
- **Population Growth:** Depends on popularity and food availability
- **Buildings:**
  - **Farms:** Produce food each round
  - **Mines:** Extract iron for weapon production
  - **Smithies:** Convert iron into weapons
  - **Markets:** Generate passive gold income
  - **Castle:** Must be upgraded to advance through ranks

### Advancement Requirements

Each rank requires meeting specific criteria:
- Minimum population
- Minimum popularity percentage
- Minimum castle level
- Minimum army size
- Minimum gold (for higher ranks)

## 🚀 Installation & Running

### Prerequisites

- Rust 1.70 or higher
- Cargo (comes with Rust)

### Setup

1. **Clone or navigate to the project:**
   ```bash
   cd rust-emperor
   ```

2. **Build the project:**
   ```bash
   cargo build --release
   ```

3. **Run the server:**
   ```bash
   cargo run --release
   ```

4. **Open your browser:**
   Navigate to `http://127.0.0.1:3000`

### Development Mode

For development with faster compilation and more detailed logging:

```bash
cargo run
```

Set the logging level:
```bash
RUST_LOG=debug cargo run
```

## 🎯 How to Play

### Starting a New Game

1. Click "Start New Game" from the main menu
2. Enter your name (max 12 characters)
3. Choose your gender (determines titles: Baron/Baroness, etc.)
4. Select difficulty:
   - **Easy:** Full starting resources, Year 1440
   - **Medium:** Reduced resources (-800 food, -250 citizens, -500 gold), Year 1445
   - **Hard:** Minimal resources (-1600 food, -500 citizens, -1000 gold, -1 farm), Year 1450

### Game Interface

**Resources Bar:**
- Monitor your Gold, Food, Citizens, Soldiers, and Popularity

**Castle Section:**
- View your current castle level
- Upgrade your castle (required for rank advancement)

**Settings Panel:**
- Adjust tax rates (0-5 levels)
- Set food ration levels (0-5 levels)

**Buildings Panel:**
- Build Farms, Mines, Smithies, and Markets
- Each building increases in price after construction

**Military Panel:**
- View soldier and weapon counts

**Trade Panel:**
- Access marketplace when you have market buildings

**Advancement Panel:**
- Track your progress toward the next noble rank
- See which requirements you've met (✅) and which you still need (❌)

### Each Round

1. Adjust your taxes and food rations as needed
2. Build structures if you have enough gold
3. Check your advancement requirements
4. Click "Finish Round & Continue"

### Round Processing

When you finish a round:
- Farms produce food
- Citizens consume food based on ration levels
- Taxes are collected based on tax level
- Markets generate gold
- Mines produce iron
- Smithies create weapons from iron
- Population grows or declines based on happiness and food
- Random events may occur
- Year advances by 1
- Rank advancement is checked

### Winning

- Advance to the highest rank (Emperor/Empress) before the year 1500
- Final score is based on:
  - Years remaining × 100
  - Difficulty bonus (Hard: 2400, Medium: 1200, Easy: 0)
  - Population / 50

### Losing

Game ends if:
- Year reaches 1501 without achieving Emperor/Empress rank
- (Future: Population drops to 0, Popularity reaches 0)

## 📁 Project Structure

```
rust-emperor/
├── src/
│   ├── main.rs              # Web server setup
│   ├── game/
│   │   ├── mod.rs           # Game module exports
│   │   ├── state.rs         # Game state and logic
│   │   └── events.rs        # Random event system
│   └── routes/
│       ├── mod.rs           # Route module exports
│       ├── menu.rs          # Menu and navigation routes
│       └── game.rs          # Game action routes
├── templates/
│   ├── base.html            # Base template with htmx
│   ├── menu.html            # Main menu screen
│   ├── new_game_form.html   # New game setup
│   ├── game.html            # Main game interface
│   ├── about.html           # About page
│   └── highscores.html      # High scores display
├── static/
│   └── css/
│       └── style.css        # Game styling
├── RustEmperorOriginal/     # Original Java source code
├── Cargo.toml               # Rust dependencies
└── README.md                # This file
```

## 🛠️ Technology Stack

- **Rust:** Safe, fast systems programming language
- **Axum:** Modern web framework for Rust
- **Tokio:** Async runtime
- **Tower:** Middleware for web services
- **Askama:** Type-safe template engine
- **htmx:** Dynamic HTML without JavaScript framework
- **Serde:** Serialization/deserialization
- **Rand:** Random number generation

## 🎨 Design Philosophy

### Why htmx?

This rewrite uses **htmx** instead of traditional mobile graphics or heavy JavaScript frameworks:

- **Lightweight:** No large JavaScript bundle, faster loading
- **Progressive Enhancement:** Works without JavaScript (fallback to full page loads)
- **Simplicity:** HTML attributes drive interactivity
- **SEO-Friendly:** Server-rendered HTML
- **Accessibility:** Standard HTML elements

### Modern Web vs. Original Mobile

**Original (Java ME):**
- Canvas-based rendering
- Custom UI components
- Limited screen real estate
- Bitmap images

**This Version:**
- HTML/CSS responsive design
- Native web controls
- Scalable to any screen size
- Unicode emoji icons (🏰💰⚔️)
- Modern web accessibility

## 🔮 Future Enhancements

### Planned Features

- [ ] Save/Load game functionality
- [ ] Persistent high scores (database or file storage)
- [ ] Trading system for buying/selling resources
- [ ] Military recruitment and training interface
- [ ] More detailed event modals
- [ ] Sound effects and music
- [ ] Multiplayer/competitive mode
- [ ] Extended game mode (beyond year 1500)
- [ ] Achievement system
- [ ] Game statistics and analytics

### Technical Improvements

- [ ] Session-based state management (currently uses shared memory)
- [ ] WebSocket support for real-time updates
- [ ] Docker containerization
- [ ] Unit and integration tests
- [ ] CI/CD pipeline
- [ ] Database integration (PostgreSQL/SQLite)
- [ ] REST API for external integrations

## 🐛 Known Issues

- Game state is stored in memory (lost on server restart)
- Multiple players share the same game state (needs session management)
- No actual trading implementation yet
- Military panel is display-only
- Events are shown but not displayed in a modal

## 🤝 Contributing

This is a personal rewrite project, but suggestions and improvements are welcome!

## 📄 License

This is a fan remake of the original DarkEmperorMD game. The original game is free for personal use.

**Original Credits:**
- Game Design & Development: Steffen Bott
- English Translation: Oliver Schuster

**This Rewrite:**
- Rust/htmx Implementation: 2024

## 🙏 Acknowledgments

- Steffen Bott for creating the original DarkEmperorMD
- Oliver Schuster for the English translation
- The Rust community for excellent tools and libraries
- htmx for making web development fun again

## 📞 Contact & Links

- Original Game Info: www.xboot.de
- Rust Language: https://www.rust-lang.org/
- Axum Framework: https://github.com/tokio-rs/axum
- htmx: https://htmx.org/

---

**Enjoy your reign, Your Majesty! 👑**