# 🎮 Visual Gameplay Guide - Dark Emperor

## Game Flow Visualization

```
┌─────────────────────────────────────────────────────────────┐
│                     🏰 DARK EMPEROR                         │
│                                                             │
│  [🎮 Start New Game]  [🏆 Highscores]  [ℹ️ About]         │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│              📝 New Game Setup                              │
│                                                             │
│  Name: [________________]                                   │
│                                                             │
│  Gender: ○ Male (Baron→Emperor)                            │
│          ○ Female (Baroness→Empress)                       │
│                                                             │
│  Difficulty:                                                │
│    ○ Easy   - Full resources, Year 1440                    │
│    ○ Medium - Reduced resources, Year 1445                 │
│    ○ Hard   - Minimal resources, Year 1450                 │
│                                                             │
│              [⚔️ Begin Your Reign]                          │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│         Baron Arthur - Year 1440 | Round 1                  │
├─────────────────────────────────────────────────────────────┤
│  💰 Gold: 2000  🌾 Food: 2500  👥 Citizens: 1000           │
│  ⚔️ Soldiers: 0  ❤️ Popularity: 72%                        │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  🏰 Castle Level 0                                          │
│     [⬆️ Upgrade (5000 gold)]                               │
│                                                             │
│  ⚙️ Settings                                                │
│     Taxes: [Medium ▼]     Food Rations: [Medium ▼]        │
│                                                             │
│  🏗️ Buildings                                              │
│     🌾 Farms: 1      [Build (1000 💰)]                     │
│     ⛏️ Mines: 0      [Build (3000 💰)]                     │
│     🔨 Smithies: 0   [Build (3000 💰)]                     │
│     🏪 Markets: 0    [Build (2000 💰)]                     │
│                                                             │
│  📊 Advancement to Countess                                 │
│     ✅ Citizens: 1000 / 1400                                │
│     ✅ Popularity: 72% / 65%                                │
│     ❌ Castle Level: 0 / 0                                  │
│     ✅ Soldiers: 0 / 0                                      │
│     ✅ Gold: 2000 / 0                                       │
│                                                             │
│         [⏭️ Finish Round & Continue]                        │
└─────────────────────────────────────────────────────────────┘
```

## Resource Management Visual

```
┌──────────────────────────────────────────┐
│         ROUND PROCESSING                 │
└──────────────────────────────────────────┘

START OF ROUND:
  💰 Gold: 2000
  🌾 Food: 2500
  👥 Citizens: 1000
  ❤️ Popularity: 72%

            ⬇️ PLAYER ACTIONS ⬇️

1. Build Farm (-1000 gold)
2. Build Market (-2000 gold)
3. Set Taxes to High
4. Keep Food Rations at Medium
5. Click "Finish Round"

            ⬇️ AUTOMATIC PROCESSING ⬇️

PRODUCTION:
  🌾 Farms produce: +1000 food
  🏪 Markets generate: +50 gold
  
CONSUMPTION:
  👥 Citizens eat: -3000 food (1000 × 3)
  💰 Taxes collected: +5000 gold (1000 × 5)
  
CHANGES:
  ❤️ Popularity: -3% (high taxes penalty)
  👥 Population: +20 (good conditions)
  
RANDOM EVENT:
  🎉 "Excellent Harvest!"
  +800 food, +3% popularity

END OF ROUND:
  💰 Gold: 4050  (+2050)
  🌾 Food: 1300  (-1200)
  👥 Citizens: 1020  (+20)
  ❤️ Popularity: 72%  (0)
  📅 Year: 1441
```

## Castle Upgrade Progression

```
Level 0: 🏚️  (Starting)
         Cost: 5,000 gold

Level 1: 🏘️  (Village)
         Cost: 10,000 gold
         Required for: Count/Countess

Level 2: 🏰  (Small Castle)
         Cost: 20,000 gold
         Required for: Duke/Duchess

Level 3: 🏰  (Castle)
         Cost: 40,000 gold

Level 4: 🏰  (Strong Castle)
         Cost: 80,000 gold

Level 5: 🏯  (Fortress)
         Cost: 160,000 gold

Level 6: 🏯  (Great Fortress)
         Cost: 320,000 gold
         Required for: King/Queen

Level 7: 🏛️  (Palace)
         Cost: 640,000 gold

Level 8: 🏛️  (Grand Palace)
         Final level
         Required for: Emperor/Empress
```

## Building Strategy Flow

```
EARLY GAME (Rounds 1-10)
├── Build 2-3 Farms
│   └── Ensures stable food supply
├── Build 1-2 Markets
│   └── Passive gold income
└── Upgrade Castle to Level 1
    └── Required for Count/Countess

           ⬇️

MID GAME (Rounds 11-30)
├── Build Mines
│   └── Iron production for weapons
├── Build Smithies
│   └── Convert iron to weapons
├── Keep building Farms
│   └── Feed growing population
└── Upgrade Castle to Level 2+
    └── Progress toward higher ranks

           ⬇️

LATE GAME (Rounds 31-60)
├── Focus on Requirements
│   ├── Population growth
│   ├── Army building
│   └── Gold accumulation
├── Maximize Castle Level
│   └── Must reach Level 8
└── Maintain Balance
    └── High popularity + resources
```

## Tax & Food Ration Effects

```
TAXES:
None      ┃ +0 gold/citizen   ┃ +2% popularity
Very Low  ┃ +1 gold/citizen   ┃ +1% popularity
Low       ┃ +2 gold/citizen   ┃  0% popularity
Medium    ┃ +3 gold/citizen   ┃ -1% popularity
High      ┃ +5 gold/citizen   ┃ -3% popularity
Very High ┃ +8 gold/citizen   ┃ -5% popularity

FOOD RATIONS:
None      ┃ 0 food/citizen    ┃ -10% popularity ┃ Starvation risk!
Very Low  ┃ 1 food/citizen    ┃  -5% popularity ┃ Unhappiness
Low       ┃ 2 food/citizen    ┃  -2% popularity ┃ Discontent
Medium    ┃ 3 food/citizen    ┃   0% popularity ┃ Balanced
High      ┃ 4 food/citizen    ┃  +1% popularity ┃ Content
Very High ┃ 5 food/citizen    ┃  +2% popularity ┃ Very happy
```

## Rank Progression Chart

```
╔═══════════════════════════════════════════════════════════════╗
║  RANK              REQUIREMENTS                                ║
╠═══════════════════════════════════════════════════════════════╣
║  Baron/Baroness    ✅ Starting Rank                            ║
╟───────────────────────────────────────────────────────────────╢
║  Count/Countess    1,400 citizens  │ 65% popularity           ║
║                    Castle Lv 0     │ 0 soldiers               ║
║                    0 gold          │                          ║
╟───────────────────────────────────────────────────────────────╢
║  Duke/Duchess      2,000 citizens  │ 70% popularity           ║
║                    Castle Lv 1     │ 10 soldiers              ║
║                    0 gold          │                          ║
╟───────────────────────────────────────────────────────────────╢
║  Prince/Princess   3,000 citizens  │ 75% popularity           ║
║                    Castle Lv 2     │ 25 soldiers              ║
║                    0 gold          │                          ║
╟───────────────────────────────────────────────────────────────╢
║  King/Queen        5,000 citizens  │ 80% popularity           ║
║                    Castle Lv 6     │ 200 soldiers             ║
║                    100,000 gold    │                          ║
╟───────────────────────────────────────────────────────────────╢
║  Emperor/Empress   10,000 citizens │ 90% popularity           ║
║                    Castle Lv 8     │ 500 soldiers             ║
║                    1,000,000 gold  │ 🏆 VICTORY!              ║
╚═══════════════════════════════════════════════════════════════╝
```

## Random Event Examples

```
┌────────────────────────────────────────────┐
│  🎉 EVENT: Excellent Harvest!              │
│                                            │
│  The gods have blessed your fields!        │
│  Your farms produced 1200 extra food.      │
│                                            │
│  Effects:                                  │
│  🌾 +1200 Food                             │
│  ❤️ +3% Popularity                         │
└────────────────────────────────────────────┘

┌────────────────────────────────────────────┐
│  ⚠️ EVENT: Bandit Raid!                    │
│                                            │
│  Bandits raided your villages!             │
│                                            │
│  Effects:                                  │
│  💰 -350 Gold                              │
│  👥 -45 Citizens                           │
│  ❤️ -3% Popularity                         │
└────────────────────────────────────────────┘

┌────────────────────────────────────────────┐
│  ⚔️ EVENT: Raiders Repelled!               │
│                                            │
│  Enemy raiders attacked but your army      │
│  drove them back!                          │
│                                            │
│  Effects:                                  │
│  ⚔️ -12 Soldiers                           │
│  ❤️ +4% Popularity (gained prestige)       │
└────────────────────────────────────────────┘
```

## Victory Screen

```
┌─────────────────────────────────────────────────────────────┐
│                                                             │
│             🎉 CONGRATULATIONS! 🎉                          │
│                                                             │
│                  YOU HAVE WON!                              │
│                                                             │
│             Empress Victoria reigns!                        │
│                                                             │
│  ╔═══════════════════════════════════════╗                 │
│  ║       FINAL STATISTICS                ║                 │
│  ╠═══════════════════════════════════════╣                 │
│  ║  Final Year:    1487                  ║                 │
│  ║  Years Used:    47 / 60               ║                 │
│  ║  Citizens:      12,450                ║                 │
│  ║  Gold:          1,250,000             ║                 │
│  ║  Army:          650 soldiers          ║                 │
│  ║  Popularity:    92%                   ║                 │
│  ║  Castle:        Level 8 (Max)         ║                 │
│  ╠═══════════════════════════════════════╣                 │
│  ║  FINAL SCORE:   18,549 points         ║                 │
│  ║                                       ║                 │
│  ║  Time Bonus:    +1,300 (13 yrs left)  ║                 │
│  ║  Difficulty:    +2,400 (Hard mode)    ║                 │
│  ║  Population:    +249                  ║                 │
│  ╚═══════════════════════════════════════╝                 │
│                                                             │
│         [🏆 View Highscores]  [🔄 Play Again]              │
└─────────────────────────────────────────────────────────────┘
```

## Interface Color Guide

```
PRIMARY COLORS:
  🟤 Primary:   #8b4513 (Saddle Brown) - Headers, borders
  🟡 Secondary: #daa520 (Goldenrod)    - Highlights, scores
  🟫 Accent:    #cd853f (Peru)         - Hover states

BACKGROUNDS:
  ⬜ Base:      #f5e6d3 (Parchment)    - Page background
  📄 Panel:     #fff8e7 (Cream)        - Card backgrounds

STATUS COLORS:
  ✅ Success:   #2e7d32 (Green)        - Met requirements
  ❌ Danger:    #c62828 (Red)          - Failed requirements
  📊 Info:      #1976d2 (Blue)         - Information

RESOURCE ICONS:
  💰 Gold        🌾 Food         👥 Citizens
  ⚔️ Soldiers    ❤️ Popularity   🏰 Castle
  🌾 Farm        ⛏️ Mine         🔨 Smithy
  🏪 Market      🎉 Event        📊 Stats
```

## Quick Decision Tree

```
EACH ROUND, ASK YOURSELF:

1. Food Supply?
   ├─ Running low? → Build Farms, Lower Taxes
   ├─ Surplus?     → Can reduce Food Rations
   └─ Balanced?    → Continue

2. Population Happy?
   ├─ < 50%  → CRISIS: Lower Taxes, Raise Rations
   ├─ 50-70% → Caution: Balance carefully
   └─ > 70%  → Good! Can push harder on taxes

3. Can Afford Castle Upgrade?
   ├─ Yes + Not upgraded this round → DO IT!
   └─ No → Save gold

4. Meeting Requirements?
   ├─ Yes → Prepare for next rank
   └─ No → Focus on missing requirements

5. Year > 1490?
   ├─ Yes → URGENT: Maximize everything
   └─ No → Steady progress is fine
```

## Pro Tips Visual

```
💡 TIP #1: The 70% Rule
   Keep popularity above 70% for steady growth
   ┌─────────────────────────────────┐
   │ 70%+ → Population grows +2-5%   │
   │ 50-70% → Stable population      │
   │ <50% → Population decline       │
   └─────────────────────────────────┘

💡 TIP #2: Early Market Strategy
   Markets pay for themselves in ~40 rounds
   Build 2-3 early for passive income

💡 TIP #3: Food Calculations
   Formula: (Farms × 500) - (Citizens × Rations)
   Example: (3 × 500) - (1000 × 3) = -1500/round
   ⚠️ You need ~6 farms per 1000 citizens!

💡 TIP #4: Castle Upgrade Timing
   One upgrade per round, so plan 8+ rounds
   for full castle. Start upgrading by Round 15!

💡 TIP #5: The Million Gold Challenge
   For Emperor/Empress, you need 1M gold
   Start saving heavily after reaching King/Queen
```

---

**Master these visuals and you'll rule the realm with wisdom!** 👑