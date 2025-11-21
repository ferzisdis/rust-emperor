# Trade Panel Bugfix Log

## Bug #1: Quick Buttons Not Updating Cost Preview

**Date:** 2024  
**Severity:** Medium  
**Status:** ✅ Fixed  

### Description

The quick amount buttons (100, 1K, 10K, MAX) were not triggering the dynamic cost preview updates in the Buy/Sell buttons. When users clicked a quick button, the input value changed but the button labels still showed the old costs.

### Root Cause

The quick buttons used simple `onclick` handlers that only set the input value:
```javascript
onclick="document.getElementById('trade_food_qty').value = 100"
```

However, the JavaScript event listeners were only attached to `input` and `change` events:
```javascript
foodInput.addEventListener('input', updateFoodButtons);
foodInput.addEventListener('change', updateFoodButtons);
```

Programmatically setting the `.value` property does NOT trigger these events automatically in JavaScript.

### Solution

Updated all quick button `onclick` handlers to dispatch an `input` event after setting the value:

**Before:**
```javascript
onclick="document.getElementById('trade_food_qty').value = 100"
```

**After:**
```javascript
onclick="let input = document.getElementById('trade_food_qty'); input.value = 100; input.dispatchEvent(new Event('input'));"
```

### Files Changed

- `templates/game.html` (lines 337-340, 384-387, 443-446)
  - Food quick buttons (4 buttons)
  - Iron quick buttons (4 buttons)
  - Weapons quick buttons (4 buttons)
  - Total: 12 button handlers updated

### Testing

**Test Cases:**
1. Click "100" button → Cost preview updates to "Buy (-40)"
2. Click "1K" button → Cost preview updates to "Buy (-400)"
3. Click "10K" button → Cost preview updates to "Buy (-4000)"
4. Click "MAX" button → Cost preview shows sell value based on inventory
5. Type custom amount → Cost preview still updates (existing functionality preserved)

**Expected Behavior:**
- ✅ Quick buttons instantly update input value
- ✅ Cost preview updates immediately
- ✅ Button disabled state updates if insufficient resources
- ✅ Smooth user experience with no lag

### Impact

**User Experience:**
- Quick buttons now work as expected
- Immediate visual feedback on cost
- Validation works correctly (buttons disable appropriately)
- No confusion about trade costs

**Technical:**
- No performance impact (event dispatch is instant)
- Clean, maintainable solution
- Follows standard JavaScript event patterns

### Related Issues

None - this was the only issue with quick button functionality.

### Verification

```bash
# Start server
cargo run

# Manual test:
# 1. Navigate to Trade panel
# 2. Click each quick button
# 3. Verify cost preview updates
# 4. Test with insufficient gold (button should disable)
# 5. Test MAX button with sell
```

**Status:** ✅ Verified and working

### Lessons Learned

- Always dispatch appropriate events when programmatically changing input values
- Test all interactive elements, not just the main input field
- Quick buttons are critical UX feature - need to work flawlessly

---

## Future Improvements

Consider refactoring to use a more centralized approach:

```javascript
// Define reusable function
function setInputValue(inputId, value) {
    const input = document.getElementById(inputId);
    input.value = value;
    input.dispatchEvent(new Event('input'));
}

// Use in onclick:
onclick="setInputValue('trade_food_qty', 100)"
```

This would make the code more DRY and easier to maintain.

---

**Bugfix Version:** 1.0.1  
**Base Version:** 1.0  
**Status:** Production Ready  
**Tested:** ✅ Manually verified  
**Documentation Updated:** ✅ This log  

---

## Enhancement #1: Split MAX Button into MAX BUY and MAX SELL

**Date:** 2024  
**Type:** Enhancement / UX Improvement  
**Status:** ✅ Implemented  

### Description

The single MAX button has been split into two separate buttons for better user experience:
- **MAX BUY** - Sets input to maximum affordable amount based on current gold
- **MAX SELL** - Sets input to current inventory amount

### Rationale

The original MAX button was ambiguous:
- When buying, users wanted to know "how much can I afford?"
- When selling, users wanted "sell everything I have"
- Single button couldn't serve both purposes clearly

### Implementation

**Food (traded in 100-unit increments):**
- MAX BUY: `Math.floor(gold / price_for_food) * 100`
- MAX SELL: `food_quantity`

**Iron (per unit):**
- MAX BUY: `Math.floor(gold / price_for_armor)`
- MAX SELL: `iron_quantity`

**Weapons (per unit):**
- MAX BUY: `Math.floor(gold / price_for_weapon)`
- MAX SELL: `weapon_quantity`

### Files Changed

- `templates/game.html` (lines 337-341, 384-389, 443-448)
  - Food quick buttons (added MAX BUY and MAX SELL)
  - Iron quick buttons (added MAX BUY and MAX SELL)
  - Weapons quick buttons (added MAX BUY and MAX SELL)
  - Total: 3 goods × 2 buttons = 6 buttons

### User Experience

**Before:**
```
[100] [1K] [10K] [MAX]
```
MAX was ambiguous - unclear what it meant

**After:**
```
[100] [1K] [10K] [MAX BUY] [MAX SELL]
```
Clear intent - users know exactly what each button does

### Benefits

1. **Clearer Intent** - No ambiguity about button purpose
2. **Better UX** - Users can quickly buy maximum affordable or sell all inventory
3. **Prevents Errors** - No confusion about which "max" is being used
4. **Convenience** - Both common use cases covered with one click

### Examples

**Scenario 1: Buying Food**
- Player has 1,000 gold
- Food costs 40 gold per 100 units
- Click MAX BUY → Sets input to 2,500 (1000 ÷ 40 × 100)
- Click Buy → Spends all gold on food

**Scenario 2: Selling Food**
- Player has 5,000 food
- Click MAX SELL → Sets input to 5,000
- Click Sell → Sells entire food inventory

### CSS Considerations

Existing CSS handles the longer button text:
- `white-space: nowrap` prevents text wrapping
- `flex: 1` distributes space evenly
- `font-size: clamp(0.75rem, 1.6vw, 0.82rem)` scales appropriately
- Buttons may be slightly wider but remain touch-friendly

### Testing

**Test Cases:**
1. ✅ MAX BUY calculates correct affordable amount for each good
2. ✅ MAX SELL shows current inventory
3. ✅ Cost preview updates when clicking either button
4. ✅ Buttons work on all three goods (Food, Iron, Weapons)
5. ✅ Layout remains responsive on mobile
6. ✅ Button text is readable and doesn't wrap

### Impact

**User Experience:** Major improvement - eliminates confusion  
**Technical:** Minimal - just button labels and calculations  
**Performance:** No impact - calculations are instant  
**Accessibility:** Better - clearer button labels  

### Verification

```bash
# Start server
cargo run

# Manual test:
# 1. Navigate to Trade panel
# 2. Click MAX BUY - verify it shows affordable amount
# 3. Click MAX SELL - verify it shows inventory
# 4. Test with low gold (MAX BUY should be low)
# 5. Test with high inventory (MAX SELL should match)
```

**Status:** ✅ Verified and working

---

**Enhancement Version:** 1.1  
**Previous Version:** 1.0.1  
**Status:** Production Ready  
**Type:** UX Enhancement  
**Breaking Changes:** None  
