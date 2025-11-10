# Icon Reference Guide

## Available Icons in `/static/images/`

This document provides a quick reference for all available pixel-art icons and their recommended usage throughout the Dark Emperor game templates.

## Core Resource Icons

### deg_gold.png
**Usage:** Gold/Money representation
- Resource header display
- Button costs
- Trade prices
- Income reports
**Size:** Typically `0.9em` to `1em`

### deg_food.png
**Usage:** Food resources
- Resource header display
- Farm production
- Trade section
- Harvest reports
**Size:** Typically `0.9em` to `1em`

### deg_man.png
**Usage:** Citizens/People
- Resource header display (Citizens count)
- Gender selection
- Discharge soldiers (returning to civilian life)
**Size:** Typically `1em`

### deg_soldier.png
**Usage:** Military/Army
- Resource header display (Army count)
- Military section header
- Recruit soldiers button
- Begin game button
**Size:** Typically `1em`

## Building Icons

### deg_castle0.png through deg_castle5.png
**Usage:** Castle progression levels
- `deg_castle0.png` - Starting castle (empty/minimal)
- `deg_castle1.png` - Basic castle (also used for general castle icon)
- `deg_castle2.png` - Castle towers
- `deg_castle3.png` - Castle wings
- `deg_castle4.png` - Castle upgrade
- `deg_castle5.png` - Highest spire (also used for crown/advancement symbols)
**Size:** Varies by context (kingdom view uses actual size, headers use `1em`)

### deg_farm.png
**Usage:** Farm buildings
- Farm building icon
- Food production representation
**Size:** `1em` for icons, actual size in kingdom view

### deg_mine.png
**Usage:** Mine buildings
- Mine building icon
- Iron/mineral extraction
**Size:** `1em` for icons, actual size in kingdom view

### deg_smithy.png
**Usage:** Smithy/Forge buildings
- Smithy building icon
- Weapon/tool production
**Size:** `1em` for icons, actual size in kingdom view

### deg_market.png
**Usage:** Market buildings and trading
- Market building icon
- Trade section header
- Commerce activities
**Size:** `1em` for icons, actual size in kingdom view

## Material Icons

### deg_iron.png
**Usage:** Iron resources
- Trade section (iron trading)
- Material representation
**Size:** Typically `1em`

### deg_weapons.png
**Usage:** Weapons/Arms
- Weapon trading
- Military equipment
- Smithy production
**Size:** Typically `1em`

## Game State Icons

### deg_win.png
**Usage:** Victory/Success
- Victory screen
- Highscores trophy
- Positive outcomes
**Size:** `1em` to `1.2em`

### deg_lost.png
**Usage:** Defeat/Failure
- Game over screen
- Loss notification
- Negative outcomes
**Size:** `1em` to `1.2em`

### deg_land.png
**Usage:** Background terrain
- Kingdom canvas background
- Full-size background image
**Size:** Full size as background

## Icon Sizing Guidelines

### Standard Sizes:
- **0.9em** - Small inline icons in buttons and compact text
- **1em** - Standard inline icons matching text height
- **1.2em** - Header icons for emphasis
- **Actual size** - Kingdom view buildings (no scaling)

### CSS Pattern:
```html
<img src="/static/images/[icon].png" alt="Description" style="height: [size]; vertical-align: middle;">
```

## Common Patterns

### Resource Display:
```html
<span class="resource-icon">
    <img src="/static/images/deg_gold.png" alt="Gold" style="height: 1em; vertical-align: middle;">
</span>
<span class="resource-value">{{ value }}</span>
```

### Section Header:
```html
<h3>
    <img src="/static/images/deg_castle1.png" alt="Buildings" style="height: 1em; vertical-align: middle;">
    Buildings
</h3>
```

### Button with Cost:
```html
<button>
    Build ({{ price }} <img src="/static/images/deg_gold.png" alt="Gold" style="height: 0.9em; vertical-align: middle;">)
</button>
```

## Multi-Purpose Icons

Some icons serve multiple purposes:

- **deg_castle1.png** - General castle, buildings section header, kingdom title
- **deg_castle5.png** - Highest castle level, crown symbol, advancement icon
- **deg_soldier.png** - Army count, military section, recruitment
- **deg_man.png** - Citizens, both genders, civilian activities
- **deg_win.png** - Victory screen, trophy/highscores
- **deg_market.png** - Market building, trade section header

## Accessibility

All icon images include:
- Descriptive `alt` text
- Appropriate sizing relative to text
- Vertical alignment for text flow
- Semantic context in surrounding HTML

## Missing Icons (Potential Future Additions)

The following concepts currently have no dedicated icons:
1. Info/About
2. Gamepad/Start Game
3. Heart/Popularity (using text symbol ♥)
4. Lock (using Unicode 🔒)
5. Trophy (using deg_win.png)
6. Crown (using deg_castle5.png)
7. Charts/Statistics (using deg_castle5.png)
8. Lightning/Events
9. Difficulty levels
10. Gender-specific characters
11. Scroll/Documents
12. Balance/Justice

See `EMOJI_REPLACEMENT_SUMMARY.md` for complete details on replacements and recommendations.