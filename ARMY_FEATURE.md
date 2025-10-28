# Army Recruitment Feature

## Overview

The army recruitment feature has been successfully implemented in the Rust Emperor game, matching the functionality from the original Java version. Players can now recruit citizens into soldiers and discharge soldiers back to civilian life.

## Implementation Details

### 1. Game Logic (src/game/state.rs)

Added the following methods to `GameState`:

- **`recruit_soldiers(quantity: i16)`**: Recruits citizens into soldiers
  - Costs: `soldier_price` gold per soldier (default: 100 gold)
  - Requires: 1 weapon per soldier
  - Requires: 1 citizen per soldier
  - Constraint: Must maintain at least 200 citizens
  - Returns: `Result<(), String>` with appropriate error messages

- **`discharge_soldiers(quantity: i16)`**: Discharges soldiers back to civilian life
  - Refunds: `soldier_price` gold per soldier
  - Returns: 1 weapon per soldier
  - Returns: 1 citizen per soldier

- **Helper methods for UI**:
  - `can_recruit_soldiers(quantity: i16) -> bool`
  - `can_discharge_soldiers(quantity: i16) -> bool`
  - `max_recruitable_by_gold() -> i32`: Calculates maximum soldiers affordable
  - `max_recruitable_by_citizens() -> i32`: Calculates maximum soldiers by citizen availability
  - `can_afford_soldier() -> bool`: Simple check for template conditionals

### 2. Routes (src/routes/game.rs)

Added new routes:

- **GET `/game/army`**: Displays the army recruitment page
- **POST `/game/army/recruit`**: Handles soldier recruitment
- **POST `/game/army/discharge`**: Handles soldier discharge

Added `ArmyTemplate` struct for rendering the army management page.

### 3. Templates

#### templates/army.html
A comprehensive army management interface featuring:

- **Resource Display**: Shows gold, citizens, soldiers, and weapons
- **Statistics Panel**: 
  - Recruitment cost per soldier
  - Weapon requirements
  - Citizen requirements
  - Minimum citizen constraint
  - Maximum recruitable calculations by gold, weapons, and citizens

- **Recruitment Form**:
  - Number input with validation
  - Quick select buttons (1, 10, 50, 100)
  - Disabled state when requirements aren't met
  - HTMX integration for seamless updates

- **Discharge Form**:
  - Number input with validation
  - Quick select buttons (1, 10, 50, All)
  - Disabled state when no soldiers available

- **Help Section**: Tips and information about the army system

#### templates/game.html (Modified)
Added "Manage Army" button to the Military panel with:
- Link to `/game/army`
- Visual indicator showing readiness to recruit
- Preview text explaining the feature

### 4. Original Java Parity

The implementation matches the original Java code from:
- `GameCanvas.java` (lines 233-237): Menu option to open army recruitment
- `TradeCanvas.java` (lines 182-189): Soldier recruitment logic
- Key logic preserved:
  - Cost: 100 gold per soldier
  - 1 weapon consumed per soldier
  - 1 citizen converted per soldier
  - Minimum 200 citizens must be maintained
  - Trade limit applies (20,000 by default)

### 5. Game Balance

Soldiers are essential for:
- **Grade Advancement**: Required for ranks 1-5
  - Baron → Count: 10 soldiers
  - Count → Duke: 25 soldiers  
  - Duke → Prince: 200 soldiers
  - Prince → King: 500 soldiers

- **Game Events**: Several random events involve soldiers (desertions, attacks, etc.)

### 6. Weapon Production Chain

To recruit soldiers, players need weapons. The production chain:

1. **Mines** → Produce iron (10 iron per mine per round)
2. **Smithies** → Convert iron to weapons (8 weapons per smithy per round, limited by iron)
3. **Market** → Can buy iron (requires 5+ markets) or weapons (requires 10+ markets)

### 7. UI/UX Improvements

- Clean, modern interface with emoji icons
- Color-coded status indicators (green = ready, red = not ready)
- Quick select buttons for common quantities
- Disabled state styling for unavailable actions
- Responsive design that works on mobile devices
- HTMX integration for seamless page updates

## Testing

The feature compiles successfully with no errors (only minor warnings about unused variables in other parts of the codebase).

To test:
1. Start a new game
2. Build smithies and mines to produce weapons
3. Navigate to the Military panel in the main game view
4. Click "🎖️ Manage Army" button
5. Recruit soldiers (if you have weapons, gold, and enough citizens)
6. Optionally discharge soldiers to test the reverse operation

## Files Modified/Created

### Created:
- `templates/army.html` - Army management page

### Modified:
- `src/game/state.rs` - Added recruitment/discharge methods and helpers
- `src/routes/game.rs` - Added army routes and template
- `templates/game.html` - Added army management link in Military panel

## Future Enhancements

Potential improvements for the future:
- Training time mechanic (soldiers take X rounds to train)
- Different soldier types (archers, cavalry, etc.)
- Military campaigns or battles
- Soldier maintenance costs
- Experience/veteran system
- Military buildings (barracks, training grounds)

## Notes

- The implementation follows Rust best practices with proper error handling
- All type conversions handled correctly (i32 ↔ i16)
- Template syntax compatible with Askama template engine
- Maintains game state consistency with proper validation
- No breaking changes to existing functionality