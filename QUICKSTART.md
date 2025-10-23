# 🚀 Quick Start Guide - Dark Emperor

Get up and running in under 2 minutes!

## Prerequisites

- **Rust** installed on your system
  - Don't have it? Install from: https://rustup.rs/
  - Takes ~5 minutes to install

## Running the Game

### Option 1: Using the Run Script (Easiest)

```bash
cd rust-emperor
./run.sh
```

### Option 2: Using Cargo Directly

```bash
cd rust-emperor
cargo run --release
```

### Option 3: Development Mode (Faster Compilation)

```bash
cd rust-emperor
cargo run
```

## Access the Game

Once the server starts, you'll see:
```
🏰 Dark Emperor server starting on http://127.0.0.1:3000
```

Open your browser and navigate to: **http://127.0.0.1:3000**

## First Time Playing?

### Starting Your Reign

1. Click **"🎮 Start New Game"**
2. Enter your name (e.g., "Arthur")
3. Choose your gender:
   - **Male**: Baron → Emperor
   - **Female**: Baroness → Empress
4. Select difficulty:
   - **Easy**: Great for learning (recommended first time)
   - **Medium**: Balanced challenge
   - **Hard**: For experienced players
5. Click **"⚔️ Begin Your Reign"**

### Basic Strategy for Beginners

**Round 1-5: Establish Your Economy**
- Build 2-3 Farms for food production
- Build 1-2 Markets for passive gold income
- Keep taxes at Medium (level 3)
- Keep food rations at Medium (level 3)
- Upgrade castle when you have 5000 gold

**Round 6-15: Expand Your Kingdom**
- Build Mines for iron production
- Build Smithies to create weapons
- Build more Farms to support growing population
- Upgrade castle whenever possible
- Adjust taxes/rations based on popularity

**Round 16+: Build Your Empire**
- Focus on meeting advancement requirements
- Check the Advancement panel regularly
- Keep population growing
- Maintain high popularity (70%+)
- Save gold for castle upgrades

### Quick Tips

✅ **DO:**
- Check your advancement requirements each round
- Upgrade your castle as soon as you can afford it
- Keep population fed (watch food quantity)
- Balance taxes and popularity
- Build markets for passive income

❌ **DON'T:**
- Set taxes too high (kills popularity)
- Let food run out (citizens starve)
- Ignore castle upgrades (needed for advancement)
- Forget to finish rounds (nothing happens until you do)
- Rush - you have until year 1500!

## Understanding the Interface

### Top Bar (Resources)
```
💰 Gold      - Buy buildings, pay soldiers
🌾 Food      - Feed your citizens
👥 Citizens  - Population (needed for advancement)
⚔️ Soldiers  - Military strength
❤️ Popularity - Citizen happiness (affects growth)
```

### Settings Panel
```
Taxes         - Higher = more gold, lower popularity
Food Rations  - Higher = better morale, more food consumed
```

### Buildings
```
🌾 Farms     - Produce food each round
⛏️ Mines     - Extract iron
🔨 Smithies  - Convert iron to weapons
🏪 Markets   - Generate passive gold income
🏰 Castle    - Must upgrade to advance ranks
```

## Winning Conditions

**Goal:** Reach Emperor/Empress rank before year 1500

**Rank Progression:**
1. Baron/Baroness (Starting rank)
2. Count/Countess (Need: 1400 citizens, 65% popularity)
3. Duke/Duchess (Need: 2000 citizens, 70% popularity, castle level 1, 10 soldiers)
4. Prince/Princess (Need: 3000 citizens, 75% popularity, castle level 2, 25 soldiers)
5. King/Queen (Need: 5000 citizens, 80% popularity, castle level 6, 200 soldiers, 100k gold)
6. Emperor/Empress (Need: 10000 citizens, 90% popularity, castle level 8, 500 soldiers, 1M gold)

## Common Issues

### Port Already in Use
If you get "Address already in use" error:
```bash
# Kill the existing process
pkill rust-emperor
# Then start again
cargo run --release
```

### Compilation Errors
```bash
# Clean and rebuild
cargo clean
cargo build --release
```

### Browser Doesn't Load
- Check the server is running (look for "server starting" message)
- Try http://localhost:3000 instead of 127.0.0.1
- Clear browser cache and reload
- Try a different browser

## Next Steps

Once you're comfortable with the basics:
- Read the full [README.md](README.md) for detailed mechanics
- Try different difficulty levels
- Experiment with different strategies
- Check out [About] in-game for more information

## Stopping the Server

Press `Ctrl+C` in the terminal where the server is running.

---

**Enjoy your reign, Your Majesty! 👑**

Need help? Check the full README.md for detailed information.