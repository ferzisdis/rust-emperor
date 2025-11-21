# Taxes and Food Rations Preview Feature

## Overview
This feature adds pre-calculated preview values to the Taxes and Food Rations dropdowns in the Settings panel. Players can now see exactly how much gold or food each setting level will generate/consume before making their selection.

## Implementation Date
Implemented: 2024

## What Changed

### Visual Changes
The Settings panel now displays estimated values in parentheses for each option:

**Taxes:**
- None (0 gold)
- Very Low (333 gold)
- Low (666 gold)
- Medium (1000 gold)
- High (1333 gold)
- Very High (1666 gold)

**Food Rations:**
- None (0 food)
- Very Low (320 food)
- Low (660 food)
- Medium (1000 food)
- High (1340 food)
- Very High (1680 food)

*Note: Actual values depend on current population (man_quantity)*

## Visual Example

When a player opens the Settings panel with 1000 citizens, they will see:

### Taxes Dropdown:
```
None (0 gold)
Very Low (333 gold)
Low (666 gold)
Medium (1000 gold)          ← Currently selected
High (1333 gold)
Very High (1666 gold)
```

### Food Rations Dropdown:
```
None (0 food)
Very Low (320 food)
Low (660 food)
Medium (1000 food)          ← Currently selected
High (1340 food)
Very High (1680 food)
```

As the population grows or shrinks throughout the game, these preview values automatically update to reflect the current situation, allowing players to make informed decisions about their kingdom's economy.

## Technical Implementation

### 1. GameState Structure (src/game/state.rs)

#### Added Fields
```rust
pub estimated_taxes: i32,
pub estimated_food_consumption: i32,
```

#### Added Methods
```rust
/// Calculate estimated taxes based on current settings (without randomness)
pub fn calculate_estimated_taxes(&self) -> i32 {
    if self.taxes_level != 0 && self.man_quantity > 0 {
        self.man_quantity * self.taxes_level as i32 * 10 / 30
    } else {
        0
    }
}

/// Calculate estimated food consumption based on current settings
pub fn calculate_estimated_food_consumption(&self) -> i32 {
    let mut food_needed = self.man_quantity * (self.food_supply as i32 * 34 - 2) / 100;
    if food_needed < 0 {
        food_needed = 0;
    }
    food_needed
}

/// Update the preview values for display
pub fn update_preview_values(&mut self) {
    self.estimated_taxes = self.calculate_estimated_taxes();
    self.estimated_food_consumption = self.calculate_estimated_food_consumption();
}
```

### 2. Route Handlers (src/routes/game.rs)

#### Updated: `game_view()`
- Now calls `update_preview_values()` before rendering to ensure values are current

#### Updated: `set_taxes()`
- Calls `update_preview_values()` after changing tax level

#### Updated: `set_food_supply()`
- Calls `update_preview_values()` after changing food supply level

### 3. Menu Routes (src/routes/menu.rs)

#### Updated: `start_game()`
- Initializes preview values when creating a new game

### 4. Template (templates/game.html)

#### Updated Dropdowns
Both Taxes and Food Rations dropdowns now calculate and display preview values inline:

```html
<!-- Taxes Example -->
<option value="3">Medium ({{ (state.man_quantity * 3 * 10 / 30) }} gold)</option>

<!-- Food Rations Example -->
<option value="3">Medium ({{ (state.man_quantity * (3 * 34 - 2) / 100) }} food)</option>
```

## Formula Documentation

### Taxes Formula
```
estimated_taxes = (man_quantity * taxes_level * 10) / 30
```

This matches the formula in `apply_round_effects()`, excluding the random variance component.

**Actual taxes collected** includes random variance:
```
random_value = rand(-man_quantity/30-1 to +man_quantity/30+1)
actual_taxes = estimated_taxes + random_value
```

### Food Consumption Formula
```
food_consumption = (man_quantity * (food_supply * 34 - 2)) / 100
```

This is the exact formula used in `apply_round_effects()` to calculate food needed.

## User Experience Benefits

1. **Better Planning**: Players can see exactly how much each setting will cost/earn
2. **Informed Decisions**: No more guessing about the impact of setting changes
3. **Dynamic Updates**: Values update based on current population automatically
4. **No Surprises**: Preview values use the same formulas as the actual round calculations

## Example Scenarios

### Early Game (1000 citizens)
- Medium Taxes: ~1000 gold per round
- Medium Food Rations: ~1000 food per round

### Mid Game (3000 citizens)
- Medium Taxes: ~3000 gold per round
- Medium Food Rations: ~3000 food per round

### Late Game (10000 citizens)
- Medium Taxes: ~10000 gold per round
- Medium Food Rations: ~10000 food per round

## Integration with Existing Systems

### Compatibility
- Fully compatible with existing game mechanics
- Preview values do not affect actual game calculations
- The actual round calculations in `apply_round_effects()` remain unchanged

### Performance
- Minimal performance impact
- Calculations are simple integer arithmetic
- Only calculated when needed (on page load and setting changes)

## Future Enhancements

Possible improvements:
1. Show estimated popularity impact for each tax/food level
2. Add color coding (green for positive, red for negative impacts)
3. Display warning if food consumption exceeds current food supply
4. Show estimated population change based on settings

## Files Modified

```
src/game/state.rs          - Added preview fields and calculation methods
src/routes/game.rs         - Updated handlers to recalculate previews
src/routes/menu.rs         - Initialize previews on new game
templates/game.html        - Display preview values in dropdowns
```

## Testing Notes

To test this feature:
1. Start a new game
2. Open the Settings panel
3. Observe the values in parentheses for each option
4. Change population (via building farms, recruiting soldiers, etc.)
5. Return to Settings and verify the preview values have updated
6. Change tax/food settings and verify values recalculate
7. Finish a round and verify actual values collected match estimates (±random variance)

## Known Limitations

1. **Random Variance**: Actual taxes collected will vary slightly from the estimate due to the random component in `apply_round_effects()`
2. **Food Shortage**: If food supply is insufficient, actual consumption may be lower than estimated (citizens go hungry instead)
3. **Rounding**: Integer division may cause minor rounding differences

## Conclusion

This feature significantly improves the user experience by providing transparent, real-time feedback about the economic impact of player decisions. The preview values use the exact same formulas as the actual game mechanics, ensuring accuracy and trust.