# Testing Guide: Round Report Feature

## Quick Test Steps

### 1. Start the Server
```bash
cd rust-emperor
cargo run
```

The server should start at `http://localhost:3000`

### 2. Create a New Game
1. Open browser to `http://localhost:3000`
2. Click "New Game"
3. Fill in:
   - Name: TestPlayer
   - Gender: Male or Female
   - Difficulty: Medium
4. Click "Start Game"

### 3. Test Basic Round Report
1. On the main game screen, click **"Finish Round"** without making any changes
2. You should be redirected to `/game/report`
3. Verify the report shows:
   - ✅ "Round Report - Year 1440" title
   - ✅ Taxes Collected: Shows a value (e.g., +3000 💰)
   - ✅ Market Income: Shows a value (e.g., +0 💰)
   - ✅ Harvest: Shows harvest amount and percentage (e.g., +500 🌾 (100%))
   - ✅ Current Popularity: Shows percentage (e.g., 50%)
   - ✅ Popularity Change: Shows change from previous round
4. Click **"Continue"** button
5. You should return to `/game` (main game view)
6. Verify year changed from 1440 to 1441

### 4. Test with Tax Changes
1. Change "Taxes Level" to "High" (level 4 or 5)
2. Click "Finish Round"
3. On the report:
   - ✅ Taxes Collected should be HIGHER than before
   - ✅ Popularity Change should be NEGATIVE (red text with minus sign)
4. Click "Continue"

### 5. Test with Building Construction
1. Build a Farm (if you have enough gold)
2. Click "Finish Round"
3. On the report:
   - ✅ Harvest value should be HIGHER than previous round
   - ✅ Harvest percentage should show correctly
4. Click "Continue"

### 6. Test Random Events
Since events have a 30% chance, you may need to finish multiple rounds:
1. Click "Finish Round" several times
2. Eventually, you should see an **Event section** on the report with:
   - ✅ Event title (e.g., "⚡ Event: Good Harvest")
   - ✅ Event description in amber-colored box
   - ✅ Event effects should be visible in the statistics
3. Click "Continue"
4. On next round report, event section should NOT appear (it's cleared)

### 7. Test Report Calculations
Set specific values and verify calculations:

**Test Case A: High Taxes**
- Set Taxes to level 5 (Oppressive)
- Citizens: 1000
- Expected taxes: ~8000 gold
- Expected popularity: Should decrease

**Test Case B: Multiple Farms**
- Build 3 farms
- Expected harvest: ~1500 (3 farms × 500)
- Harvest percentage: ~100% with normal food supply

**Test Case C: Build Markets**
- Build 2 markets
- Expected market income: 100 gold (2 × 50)

## What to Check

### Visual Elements
- [ ] Report page has medieval styling matching the game
- [ ] Positive values are shown in GREEN
- [ ] Negative values are shown in RED
- [ ] Event section (when present) has AMBER/YELLOW background
- [ ] All sections are clearly separated
- [ ] "Continue" button is prominent and centered

### Functionality
- [ ] Report appears AFTER every "Finish Round" action
- [ ] Report shows correct values for taxes, market, harvest
- [ ] Popularity change calculation is correct
- [ ] Random events appear occasionally (~30% of the time)
- [ ] Event data is cleared after viewing report
- [ ] Clicking "Continue" returns to game view
- [ ] Year counter increments correctly
- [ ] Round counter increments correctly

### Edge Cases
- [ ] Report works when gold is 0
- [ ] Report works when popularity is very low
- [ ] Report works when harvest is negative (due to low food supply)
- [ ] Report works when no buildings are constructed
- [ ] Report handles very large numbers properly

## Expected Values Reference

### Taxes (per citizen)
- Level 0 (None): 0 gold
- Level 1 (Low): 1 gold
- Level 2 (Fair): 2 gold
- Level 3 (Normal): 3 gold
- Level 4 (High): 5 gold
- Level 5 (Oppressive): 8 gold

### Market Income
- Per market: 50 gold

### Harvest
- Per farm: 500 food (base)
- Food supply bonus:
  - Level 0: -200
  - Level 1: -100
  - Level 2: 0
  - Level 3: +100
  - Level 4: +200
  - Level 5: +300

### Popularity Changes
- Tax penalty ranges from +2 (no taxes) to -5 (oppressive)
- Food supply penalty ranges from -10 (none) to +2 (generous)

## Comparison with Original

The new implementation matches the original Java ME version:
- Shows taxes, market income, harvest with percentage
- Shows popularity and its change
- Displays random events
- Uses a modal/separate screen (now a web page)
- Requires user to acknowledge before continuing

## Troubleshooting

### Report doesn't show
- Check browser console for errors
- Verify server is running and no Rust errors
- Check that `/game/report` route is registered
- Verify game state is not null

### Values seem wrong
- Check that round effects are being applied in `apply_round_effects()`
- Verify calculations match expected formulas
- Check that `previous_popularity_percent` is being stored before changes

### Events never appear
- Events have 30% chance, may need 5-10 rounds to see one
- Check that `EventGenerator::generate_random_event()` is being called
- Verify event data is being stored in `last_event_title` and `last_event_description`

### Can't continue from report
- Verify `/game/continue-from-report` route is registered
- Check that form POST method is correct
- Verify button is inside the form tags

## Success Criteria
✅ Report appears after every round  
✅ All values are calculated correctly  
✅ Visual styling matches game theme  
✅ Events appear occasionally  
✅ Continue button works properly  
✅ Game state is preserved correctly  

If all checks pass, the Round Report feature is working correctly!