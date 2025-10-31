# 🏰 Kingdom Visual Representation Feature

## Overview

This feature implements a visual representation of the player's kingdom that dynamically displays buildings based on the player's construction progress. This matches the original Java game's `GameCanvas.paint()` method functionality.

## Implementation Details

### Files Modified/Created

1. **templates/game.html** - Replaced the simple castle display with a full kingdom canvas
2. **static/css/kingdom.css** - New CSS file containing all positioning and styling for kingdom visuals
3. **templates/base.html** - Added kingdom.css import

### Visual Components

#### Kingdom Canvas
- A 600x450px responsive canvas that displays the kingdom
- Background gradient simulating sky (top 35%) and ground (bottom 65%)
- Uses CSS absolute positioning to layer buildings and castle structures
- Buildings are positioned at various depths to create a 3D layered effect
- Castle positioned in the upper-middle area (bottom: 180px)
- Farms and markets spread across middle levels (bottom: 70-200px)
- Mines and smithies positioned in foreground (bottom: 25-35px)

#### Castle Structures (Based on `castle_level`)

The castle grows progressively as the player upgrades:

| Castle Level | Visual Elements |
|--------------|----------------|
| 0 | `deg_castle0.png` - Ruins |
| 1-4 | `deg_castle1.png` - Basic castle |
| 2+ | `deg_castle2.png` - Left tower added |
| 3+ | `deg_castle3.png` - Left wing added |
| 4+ | `deg_castle4.png` - Main castle upgraded |
| 5+ | `deg_castle3.png` - Right wing added |
| 6+ | `deg_castle2.png` - Right tower added |
| 7+ | `deg_castle5.png` - Left spire added |
| 8 | `deg_castle5.png` - Right spire added (max level) |

#### Buildings by Quantity

Buildings appear at specific positions when quantity thresholds are reached:

**Farms** (Left side of castle):
- 1+ farms: First farm appears
- 20+ farms: Second farm
- 30+ farms: Third farm
- 40+ farms: Fourth farm
- 50+ farms: Fifth farm
- 100+ farms: Sixth farm

**Markets** (Right side of castle):
- 1+ markets: First market appears
- 5+ markets: Second market
- 10+ markets: Third market
- 40+ markets: Fourth market
- 50+ markets: Fifth market
- 100+ markets: Sixth market

**Mines** (Bottom left area):
- 1+ mines: First mine appears
- 20+ mines: Second mine
- 30+ mines: Third mine

**Smithies** (Bottom right area):
- 1+ smithies: First smithy appears
- 20+ smithies: Second smithy
- 30+ smithies: Third smithy

### Original Java Code Reference

This implementation is based on the original `GameCanvas.java` paint method (lines 113-221):

```java
byte var3 = this.storage.castleLevel;
var1.drawImage(Images.land, ((Canvas)this).getWidth() / 2, someHeight1 * 3 + 8, 17);

if (this.storage.castleLevel == 0) {
    var1.drawImage(Images.castle0, ...);
} else if (var3 < 5) {
    var1.drawImage(Images.castle1, ...);
}
// ... additional castle structures based on level

if (this.storage.farmQuantity > 0) {
    var1.drawImage(Images.farm, ...);
}
// ... additional farms at different thresholds
```

### Technical Implementation

#### Template Logic (Tera)
- Uses `{% if condition %}` blocks to conditionally render buildings
- Each building threshold checks the quantity from `GameState`
- Images are positioned using CSS classes

#### CSS Positioning Strategy
The layout creates a balanced, layered 3D effect:

**Vertical Layers (bottom positions):**
- Castle main structure: 180px (upper-middle)
- Castle towers: 155px (slightly lower)
- Castle wings: 170px (between main and towers)
- Castle spires: 195px (highest points)
- Farms/Markets (level 1): 165px (upper tier)
- Farms/Markets (level 2): 135px (middle tier)
- Farms/Markets (level 3): 200px (back tier)
- Farms/Markets (level 4): 100px (lower tier)
- Farms/Markets (level 5-6): 70px (foreground tier)
- Mines/Smithies: 25-35px (bottom foreground)

**Horizontal Spread:**
- Uses `position: absolute` with center-based calculations
- Positions: `left: calc(50% ± offset)`
- Farms on left side: -60px to -200px from center
- Markets on right side: +60px to +200px from center
- Creates symmetrical layout around castle

**Visual Layout Diagram:**
```
         Sky (35%)
    ┌────────────────────────┐
    │   Spires (195px)       │
    │     ▲         ▲        │
    │ F3  │  Tower  │  M3    │ Back Layer (165-200px)
    │ ░   ├─Castle──┤   ▓    │
    │ F1 ░│  Main   │▓ M1    │
    │    ░│ (180px) │▓        │ Castle Level (155-180px)
    ├─────┴─────────┴────────┤ --- Ground Line (35%) ---
    │ F2 ░    Land    ▓ M2    │ Middle Layer (100-165px)
    │                         │
    │F4F5 ░          ▓ M4M5   │ Foreground (70-100px)
    │ Mine           Smithy   │ Bottom (25-35px)
    └────────────────────────┘
    
    Legend:
    ░ = Farms (left side)
    ▓ = Markets (right side)
    F1-F6 = Farm positions 1-6
    M1-M6 = Market positions 1-6
```

**Z-Index Depth:**
- Background elements: z-index 1-9
- Mines/Smithies: z-index 9-10
- Farms/Markets (back): z-index 11-14
- Farms/Markets (front): z-index 15-16
- Castle structures: z-index 18-22
- Ensures proper overlap and depth perception

**Responsive Breakpoints:**
- Desktop (>768px): Full layout, 450px height
- Tablet (480-768px): Scaled down, 380px height
- Mobile (360-480px): Compact layout, 320px height
- Small mobile (<360px): Minimal layout, 280px height

#### Image Rendering
- All images use `image-rendering: pixelated` to preserve pixel art quality
- Images are upscaled versions from the original game
- Class `.pixel-art` ensures crisp rendering

### Responsive Design

The kingdom canvas adapts to different screen sizes:

- **Desktop (>768px)**: Full 600x450px canvas, 70px castle, 45-50px buildings
- **Tablet (480-768px)**: 380px height, 60px castle, 40px buildings
- **Mobile (360-480px)**: 320px height, 50px castle, 36-38px buildings, repositioned for compact layout
- **Small Mobile (<360px)**: 280px height, 45px castle, 32-34px buildings

Each breakpoint adjusts both sizes and vertical positions to maintain visual balance and prevent overlap.

### Animation

Buildings have a subtle appear animation:
```css
@keyframes buildingAppear {
    from { opacity: 0; transform: translateY(20px) scale(0.8); }
    to { opacity: 1; transform: translateY(0) scale(1); }
}
```

## Usage

The kingdom visual automatically updates when:
- Castle is upgraded (new structures appear)
- Buildings are constructed (more buildings appear at thresholds)
- Player navigates back to game view from other screens

No additional code changes needed in game logic - the template reads directly from `GameState` properties.

## Layout Summary

The improved layout creates a balanced, depth-based scene:
- **Top third**: Sky background with castle spires
- **Middle third**: Castle main structures with first tier of farms/markets
- **Bottom third**: Ground with additional farms/markets in foreground, mines/smithies at bottom

Buildings appear progressively further from castle as you build more, creating a sprawling kingdom effect.

## Testing

To test the visual representation:

1. Start a new game - observe the initial ruins (castle level 0)
2. Build 1 farm and 1 market - watch them appear on left and right sides
3. Upgrade castle to level 2 - see the basic castle plus left tower
4. Build 20+ farms - second farm appears further left
5. Build 30+ farms - third farm appears in back (higher up)
6. Upgrade castle to level 4+ - watch main castle upgrade
7. Build 5+ markets - second market appears
8. Build 10+ markets - third market appears in back
9. Build 50+ farms/markets - watch buildings spread to foreground
10. Upgrade to level 8 - see the complete palace with towers, wings, and spires
11. Build mines and smithies - watch them appear in bottom foreground
12. Build 100+ farms/markets - maximum of 6 visible per type

The layout should show:
- Castle dominating the upper-middle area
- Farms spreading across the left side at multiple depths
- Markets spreading across the right side at multiple depths
- Mines and smithies in the foreground at the bottom
- A balanced, layered 3D effect with proper depth perception

## Future Enhancements

Possible improvements:
- Animated weather effects (rain, snow)
- Day/night cycle
- Building animations when constructed
- Citizens/soldiers visible walking around
- Seasonal changes to the landscape
- Sound effects when buildings appear

## Credits

Based on the original Dark Emperor (Mrachnyy Vlastelin) J2ME game by Алексей Горевой (Alexey Gorevoy).