# Trade Panel Redesign - Testing & Deployment Guide

## Quick Start

### Running the Application

```bash
cd rust-emperor
cargo run
```

Navigate to: `http://127.0.0.1:3000`

### Testing Flow

1. Start a new game
2. Build your first market (costs 2,000 gold)
3. Navigate to the Trade panel
4. Test trading functionality

## Pre-Deployment Checklist

### Build Verification

```bash
# Check for compilation errors
cargo check

# Run tests (if available)
cargo test

# Build release version
cargo build --release

# Check file sizes
ls -lh target/release/rust-emperor
```

### Expected Output
- ✅ Compilation successful
- ⚠️ Warning about unused `format_number` function (safe to ignore)
- ✅ Binary created in `target/release/`

## Manual Testing Scenarios

### Scenario 1: No Markets (Initial State)

**Setup:**
- Start new game (default: 2,000 gold, 0 markets)

**Expected:**
```
┌─────────────────────────────┐
│ Trade                       │
│                             │
│ Build a market to trade!    │
└─────────────────────────────┘
```

**Verify:**
- [ ] "Build a market to trade!" message displays
- [ ] No trade options visible
- [ ] Panel styling matches other panels

---

### Scenario 2: First Market (Food Trading)

**Setup:**
1. Build 1 Market (from Buildings section)
2. Refresh page or finish round
3. Navigate to Trade panel

**Expected:**
```
┌─────────────────────────────────────────────┐
│ [🏪] Trade • 1 market(s)     2,000 [💰]     │
│ =========================================== │
│                                             │
│ [🌾] Food                                   │
│ 40 [💰]/100 • In stock: 2,500               │
│                                             │
│ [Iron] 🔒  Requires 5+ markets              │
│ [Weapons] 🔒  Requires 10+ markets          │
└─────────────────────────────────────────────┘
```

**Verify:**
- [ ] Header shows correct market count
- [ ] Balance displays current gold
- [ ] Food card is unlocked
- [ ] Food shows correct price (40 gold/100)
- [ ] Food shows current inventory (2,500)
- [ ] Iron is locked with "Requires 5+ markets"
- [ ] Weapons locked with "Requires 10+ markets"
- [ ] Locked items have 50% opacity

---

### Scenario 3: Food Trading - Buy Flow

**Setup:**
- Have at least 1 market and 100+ gold

**Test Steps:**

1. **Default State**
   - Input shows: 100
   - Buy button shows: "Buy (-40)"
   - Verify button is enabled (not grayed)

2. **Click "1K" quick button**
   - Input changes to: 1000
   - Buy button updates to: "Buy (-400)"
   - Verify button is still enabled

3. **Type custom amount (500)**
   - Buy button updates to: "Buy (-200)"
   - Cost calculation is correct (500 ÷ 100 × 40 = 200)

4. **Click "MAX" button**
   - Input shows current food inventory (e.g., 2500)
   - Sell button shows correct income

5. **Click Buy button**
   - Button disables during request
   - Page refreshes
   - Gold decreases by cost
   - Food inventory increases

**Verify:**
- [ ] Quick buttons set correct values
- [ ] Buy button shows dynamic cost
- [ ] Cost calculation is accurate
- [ ] Button disables when insufficient gold
- [ ] Trade executes successfully
- [ ] Resources update correctly

---

### Scenario 4: Food Trading - Sell Flow

**Setup:**
- Have at least 1 market and 100+ food

**Test Steps:**

1. **Set amount to 1000**
   - Sell button shows: "Sell (+400)"

2. **Click Sell button**
   - Gold increases by 400
   - Food decreases by 1000

3. **Try to sell more than inventory**
   - Type amount > current food
   - Sell button becomes disabled (grayed out)
   - Cursor changes to "not-allowed"

**Verify:**
- [ ] Sell button shows dynamic income
- [ ] Trade executes successfully
- [ ] Button disables when insufficient food
- [ ] Cannot sell more than owned

---

### Scenario 5: Iron Trading (5+ Markets)

**Setup:**
- Build 5 markets total
- Have 100+ gold

**Expected:**
- Iron card becomes unlocked
- Shows price per unit
- Shows current inventory with trade limit (X/20,000)

**Test:**
1. **Buy 10 iron**
   - Cost: 10 × 60 = 600 gold
   - Iron inventory increases by 10

2. **Verify trade limit**
   - Try to buy when near limit (19,990/20,000)
   - Should show error or prevent trade

**Verify:**
- [ ] Iron unlocks at exactly 5 markets
- [ ] Price per unit displays correctly
- [ ] Trade limit shows (current/20,000)
- [ ] Quick buttons work (1, 10, 100)
- [ ] MAX button uses current iron inventory
- [ ] Trade executes successfully
- [ ] Trade limit enforced

---

### Scenario 6: Weapons Trading (10+ Markets)

**Setup:**
- Build 10 markets total
- Have sufficient gold

**Test:**
1. Verify weapons card is unlocked
2. Check price display
3. Test buy functionality
4. Verify trade limit (X/20,000)

**Verify:**
- [ ] Weapons unlock at exactly 10 markets
- [ ] All controls work correctly
- [ ] Trade limit enforced

---

### Scenario 7: Insufficient Resources

**Test Cases:**

1. **Insufficient Gold for Buy**
   - Set gold to 30 (< 40 needed for 100 food)
   - Verify Buy button is disabled
   - Button should be grayed out
   - Cursor: not-allowed

2. **Insufficient Inventory for Sell**
   - Have 50 food
   - Try to sell 100 food
   - Verify Sell button is disabled

**Verify:**
- [ ] Buy button disables when gold < cost
- [ ] Sell button disables when inventory < amount
- [ ] Disabled state is visually clear
- [ ] Cannot click disabled buttons

---

### Scenario 8: Edge Cases

**Test Cases:**

1. **Zero Amount**
   - Type 0 in input
   - Buttons should show: "Buy (-0)" / "Sell (+0)"
   - Should be able to click (no-op expected)

2. **Negative Amount**
   - Try typing negative number
   - Should be prevented by min="0" attribute

3. **Very Large Amount**
   - Type 999999999
   - Cost calculation should handle large numbers
   - Button may be disabled if insufficient resources

4. **Non-numeric Input**
   - Try typing letters
   - Input should ignore or clear invalid characters

**Verify:**
- [ ] Zero amount handled gracefully
- [ ] Negative amounts prevented
- [ ] Large numbers calculated correctly
- [ ] Invalid input rejected

---

### Scenario 9: Responsive Design

**Desktop (> 768px):**
- [ ] Header is two-column (left/right)
- [ ] Input controls horizontal
- [ ] Buy/Sell buttons side-by-side
- [ ] Hover effects work
- [ ] Cards have shadows on hover

**Tablet (≤ 768px):**
- [ ] Header may stack
- [ ] Controls adjust size
- [ ] Quick buttons may wrap
- [ ] Touch-friendly sizes

**Mobile (≤ 480px):**
- [ ] Full vertical stacking
- [ ] Input full-width
- [ ] Quick buttons in 2×2 grid
- [ ] Buy/Sell buttons stacked
- [ ] All buttons ≥ 44px height
- [ ] No hover effects (touch only)

**Testing Methods:**
1. Browser DevTools responsive mode
2. Physical devices
3. Window resizing

---

### Scenario 10: Visual Design

**Colors:**
- [ ] Buy button: Green (#7a9b6d)
- [ ] Sell button: Orange (#c89860)
- [ ] Gold balance: Gold color (#c59a37)
- [ ] Background: Parchment theme

**Typography:**
- [ ] Header: Bold, larger font
- [ ] Good names: Bold
- [ ] Details: Smaller, secondary color
- [ ] Locked text: Italic, muted

**Spacing:**
- [ ] Consistent padding in cards
- [ ] Proper gaps between elements
- [ ] Aligned text and icons

**Icons:**
- [ ] All icons display correctly
- [ ] Icons have proper alt text
- [ ] Icons scale with text

---

## Browser Compatibility Testing

### Required Browsers

**Desktop:**
- [ ] Chrome (latest)
- [ ] Firefox (latest)
- [ ] Safari (latest)
- [ ] Edge (latest)

**Mobile:**
- [ ] iOS Safari
- [ ] Chrome Mobile (Android)

### Features to Test

1. **CSS Grid/Flexbox**
   - Layout renders correctly

2. **HTMX**
   - Async requests work
   - Button disabling works
   - Page updates correctly

3. **JavaScript ES6**
   - Cost calculations work
   - Event listeners work
   - Arrow functions supported

4. **Number Input**
   - Stepper controls work
   - Min/max validation works

---

## Performance Testing

### Load Time
```bash
# Measure initial page load
curl -w "@curl-format.txt" -o /dev/null -s http://127.0.0.1:3000/game
```

**Expected:**
- Total time: < 200ms (local)
- CSS size: ~30-40KB
- JS inline: ~5KB

### Interaction Performance

**Metrics:**
- Input to button update: < 16ms (instant)
- Button click to disable: < 16ms
- Trade to page refresh: < 500ms (network dependent)

**Tools:**
- Browser DevTools Performance tab
- Network tab for HTMX requests

---

## Accessibility Testing

### Keyboard Navigation

**Test Steps:**
1. Press Tab to focus first input
2. Continue tabbing through all elements
3. Press Enter on focused buttons
4. Use arrow keys in number inputs

**Verify:**
- [ ] All interactive elements focusable
- [ ] Focus order is logical
- [ ] Focus indicators visible
- [ ] Enter key submits forms
- [ ] Arrow keys work in inputs

### Screen Reader Testing

**Tools:**
- macOS: VoiceOver (Cmd+F5)
- Windows: NVDA
- Chrome: ChromeVox extension

**Verify:**
- [ ] Images have alt text
- [ ] Buttons have descriptive labels
- [ ] Disabled states announced
- [ ] Form labels present

### Color Contrast

**Tools:**
- Browser DevTools (Lighthouse)
- WebAIM Contrast Checker

**Verify:**
- [ ] Text contrast ≥ 4.5:1 (WCAG AA)
- [ ] Button text contrast ≥ 4.5:1
- [ ] Disabled state distinguishable

---

## Integration Testing

### Game State Integration

**Test that trades affect game state:**

1. **Gold Balance**
   - Buy food → gold decreases
   - Sell food → gold increases
   - Header balance updates

2. **Resource Counts**
   - Trades update inventory
   - Header resources update
   - Other panels reflect changes

3. **Market Count**
   - Build market → count increases
   - Trade panel unlocks accordingly

### Round Progression

**Test that trade state persists:**

1. Make a trade
2. Finish round
3. Verify changes persist
4. Trade panel reflects new state

**Verify:**
- [ ] Trades persist across rounds
- [ ] Market count persists
- [ ] Resource counts persist

---

## Regression Testing

**Verify no breakage of existing features:**

### Other Panels
- [ ] Buildings panel works
- [ ] Army panel works
- [ ] Taxes panel works
- [ ] Food supply panel works

### Game Flow
- [ ] New game creation works
- [ ] Round finishing works
- [ ] Game over detection works
- [ ] Report display works

### Styling
- [ ] Other panels maintain styling
- [ ] No CSS conflicts or bleeding
- [ ] Consistent fonts and colors

---

## Bug Tracking

### Known Issues

1. **Number Formatting**
   - Numbers don't have thousand separators yet
   - Filter exists but not wired up to Askama
   - Low priority - doesn't affect functionality

2. **Full Page Reload**
   - HTMX causes full page refresh instead of partial
   - Matches existing backend behavior
   - Not a bug - just opportunity for enhancement

### Reporting New Issues

**Template:**
```
Title: [Brief description]
Severity: Critical/High/Medium/Low
Steps to Reproduce:
1. ...
2. ...
Expected: ...
Actual: ...
Browser: ...
Screenshot: [if applicable]
```

---

## Deployment Steps

### Development Deployment

```bash
# 1. Pull latest changes
cd rust-emperor

# 2. Verify compilation
cargo check

# 3. Run tests
cargo test

# 4. Start development server
cargo run

# 5. Manual testing (see scenarios above)

# 6. Verify all checklists complete
```

### Production Deployment

```bash
# 1. Build release binary
cargo build --release

# 2. Test release build
./target/release/rust-emperor
# Navigate to http://127.0.0.1:3000 and test

# 3. Stop production server
# (depends on your deployment)

# 4. Replace binary
cp target/release/rust-emperor /path/to/production/

# 5. Copy static files
cp -r static /path/to/production/

# 6. Copy templates
cp -r templates /path/to/production/

# 7. Restart server
# (depends on your deployment)

# 8. Verify production
# Navigate to production URL and test
```

### Rollback Plan

If issues are found in production:

```bash
# 1. Stop server

# 2. Restore previous files
git checkout HEAD~1 static/css/style.css
git checkout HEAD~1 templates/game.html
rm src/filters.rs
git checkout HEAD~1 src/main.rs

# 3. Rebuild
cargo build --release

# 4. Deploy old version

# 5. Restart server
```

---

## Post-Deployment Verification

### Smoke Tests (< 5 minutes)

- [ ] Application starts successfully
- [ ] Can create new game
- [ ] Trade panel displays correctly
- [ ] Can buy food
- [ ] Can sell food
- [ ] Page doesn't crash

### Full Verification (15-30 minutes)

- [ ] Run all manual testing scenarios
- [ ] Check browser console for errors
- [ ] Verify responsive design
- [ ] Test on mobile device
- [ ] Check all locked/unlocked states

---

## Monitoring

### Metrics to Watch

1. **Error Rate**
   - Trade endpoint errors
   - Page load errors
   - JavaScript console errors

2. **Performance**
   - Page load time
   - Trade execution time
   - JavaScript execution time

3. **User Behavior**
   - Trade frequency
   - Quick button usage
   - Input vs button usage

### Log Entries

Watch for:
```
ERROR: Trade failed: Not enough gold
ERROR: Trade failed: Not enough food
WARNING: Invalid quantity
```

---

## User Acceptance Testing

### UAT Checklist

Provide to testers:

- [ ] Clear instructions on how to access test environment
- [ ] List of test scenarios (see above)
- [ ] Feedback form/template
- [ ] Timeline for testing (e.g., 2-3 days)
- [ ] Contact for questions/issues

### Feedback Collection

**Questions to Ask:**
1. Is the layout intuitive?
2. Are the quick buttons useful?
3. Is the cost preview helpful?
4. Are locked items clearly indicated?
5. Does it work well on your device?
6. Any confusing elements?
7. Any suggestions for improvement?

---

## Success Criteria

### Launch Readiness

Application is ready to launch when:

- ✅ All critical tests pass
- ✅ No blocking bugs found
- ✅ Performance metrics acceptable
- ✅ Accessibility requirements met
- ✅ Responsive design verified
- ✅ Browser compatibility confirmed
- ✅ UAT feedback addressed
- ✅ Rollback plan tested

### Post-Launch Metrics (Week 1)

**Success indicators:**
- Zero critical bugs reported
- < 5 minor bugs reported
- No performance degradation
- Positive user feedback
- No rollbacks needed

---

## Future Testing Considerations

### Automated Testing (Future)

**Unit Tests:**
```rust
#[test]
fn test_buy_food_with_sufficient_gold() {
    let mut game = GameState::new(...);
    game.gold = 100;
    assert!(game.buy_food(100).is_ok());
    assert_eq!(game.gold, 60);
}
```

**Integration Tests:**
```rust
#[tokio::test]
async fn test_trade_endpoint() {
    let response = client.post("/game/trade/buy-food")
        .json(&json!({"quantity": 100}))
        .send()
        .await;
    assert_eq!(response.status(), 200);
}
```

**E2E Tests:**
- Selenium/Playwright for browser automation
- Test complete user flows
- Screenshot comparison testing

---

## Appendix: Testing Tools

### Browser Extensions
- **WAVE**: Accessibility testing
- **Lighthouse**: Performance, accessibility, SEO
- **React DevTools**: HTMX request inspection
- **ColorZilla**: Color contrast checking

### Command-Line Tools
```bash
# HTTP testing
curl -X POST http://127.0.0.1:3000/game/trade/buy-food \
  -d "quantity=100"

# Performance testing
ab -n 100 -c 10 http://127.0.0.1:3000/game
```

### Desktop Tools
- **Postman**: API testing
- **BrowserStack**: Cross-browser testing
- **Charles Proxy**: Network debugging

---

## Contact & Support

For questions or issues with this testing guide:
- Review `TRADE_PANEL_REDESIGN.md` for implementation details
- Review `TRADE_PANEL_SUMMARY.md` for feature overview
- Review `TRADE_PANEL_VISUAL_GUIDE.md` for UI reference
- Check GitHub issues for known problems
- Contact development team for clarification

---

## Test Sign-Off

**Tested By:** ___________________  
**Date:** ___________________  
**Environment:** Development / Staging / Production  
**Result:** Pass / Fail / Pass with Notes  

**Notes:**
_______________________________________________________
_______________________________________________________
_______________________________________________________

**Approved for Deployment:** Yes / No  
**Approver:** ___________________  
**Date:** ___________________