# Trade Panel Redesign - Implementation Documentation

## Overview

The Trade panel has been completely redesigned according to the technical specification to improve user experience, information density, and visual feedback.

## Changes Made

### 1. CSS Styles (`static/css/style.css`)

Added comprehensive styling for the redesigned trade panel (lines ~700-985):

#### New CSS Classes:
- `.trade-header` - Panel header with flexbox layout
- `.trade-header-left` - Left side of header (icon, title, market count)
- `.trade-header-right` - Right side with balance display
- `.trade-balance` - Player's current gold balance (prominent display)
- `.trade-market-count` - Market counter with muted styling
- `.trade-goods-list` - Container for all trade goods
- `.trade-good-item` - Individual good card with hover effects
- `.trade-good-info` - Top row with icon, name, price, and inventory
- `.trade-good-icon` - Icon container
- `.trade-good-name` - Bold good name
- `.trade-good-lock` - Lock icon for unavailable items
- `.trade-good-details` - Price and inventory information
- `.trade-good-controls` - Bottom row with input and buttons
- `.trade-input-section` - Number input with quick buttons
- `.trade-quick-buttons` - Container for 100, 1K, 10K, MAX buttons
- `.btn-quick-amount` - Individual quick amount button
- `.trade-action-buttons` - Buy/Sell button container
- `.btn-trade-buy` - Buy button (green/success color)
- `.btn-trade-sell` - Sell button (orange/warning color)
- `.trade-good-locked` - Locked state styling (50% opacity)
- `.trade-locked-text` - Centered "Requires X+ markets" text

#### Responsive Design:
- **768px breakpoint**: Stack header elements, adjust input sizes
- **480px breakpoint**: Stack all controls vertically for mobile

### 2. Template Changes (`templates/game.html`)

Completely rewrote the Trade panel section (lines 303-482):

#### Structure:
```
Trade Panel
├── Header
│   ├── Left: Icon + "Trade" + Market Count
│   └── Right: Gold Balance
└── Goods List
    ├── Food (always available if 1+ markets)
    │   ├── Info Row: Icon + Name + Price + Inventory
    │   └── Controls Row
    │       ├── Number Input + Quick Buttons (100, 1K, 10K, MAX)
    │       └── Buy/Sell Buttons
    ├── Iron (unlocks at 5+ markets)
    │   └── [Same structure or locked state]
    └── Weapons (unlocks at 10+ markets)
        └── [Same structure or locked state]
```

#### Key Features:
- **Header**: Shows trade icon, title, market count, and player balance
- **Good Cards**: Each good has its own card with:
  - Icon and name
  - Price per unit and current inventory
  - Number input for amount
  - Quick amount buttons (100, 1K, 10K, MAX)
  - Buy and Sell buttons
- **Locked State**: Unavailable goods show lock icon and requirement text
- **HTMX Integration**: All buttons use HTMX for async updates
- **Inventory Display**: Shows current stock with trade limits where applicable

### 3. Number Formatting Module (`src/filters.rs`)

Created a new filters module with a `format_number` function for adding thousand separators:

```rust
pub fn format_number<T>(value: &T) -> askama::Result<String>
```

**Features:**
- Adds commas every 3 digits (e.g., 1000 → "1,000")
- Handles negative numbers
- Generic over any type implementing `Display`
- Includes unit tests

**Note**: Currently not used in templates but available for future enhancement. To use in Askama templates, you would need to:

1. Add to Cargo.toml:
```toml
[dependencies]
askama = { version = "0.12", features = ["with-axum"] }
```

2. Use in templates:
```html
{{ state.gold|format_number }}
```

### 4. Main Module (`src/main.rs`)

Registered the new filters module:
```rust
mod filters;
```

## Design Decisions

### Visual Hierarchy
1. **Gold balance** is prominently displayed in the header (bold, gold color)
2. **Good names** are bold and larger for easy scanning
3. **Prices and inventory** use secondary text color
4. **Locked items** have 50% opacity and muted background

### User Experience
- **Quick amount buttons** reduce clicks for common quantities
- **MAX button** automatically fills current inventory for selling
- **Input fields** allow custom amounts with number spinners
- **Touch-friendly**: All buttons meet 44px minimum height on mobile
- **Hover effects**: Cards lift slightly on hover (desktop only)

### Color Coding
- **Buy buttons**: Green (success color) - `#7a9b6d`
- **Sell buttons**: Orange (warning color) - `#c89860`
- Maintains existing parchment theme

### Accessibility
- All buttons have clear labels
- Icons have alt text
- Color is not the only indicator (icons + text)
- Disabled states are visually distinct
- Keyboard navigation supported (standard HTML)

## Backend Integration

### Current Implementation
The backend routes (`src/routes/game.rs`) currently return full-page redirects:

```rust
async fn buy_food(...) -> impl IntoResponse {
    // ... execute trade
    Redirect::to("/game")
}
```

### HTMX Attributes Used
- `hx-post` - POST request to trade endpoint
- `hx-vals` - JavaScript expression to get input value
- `hx-target="body"` - Replace entire page (matches redirect behavior)
- `hx-swap="innerHTML"` - Replace content
- `hx-disabled-elt="this"` - Disable button during request

### Future Enhancements (Optional)

To implement true partial updates as specified:

1. **Return HTML Fragments** instead of redirects:
```rust
async fn buy_food(...) -> Html<String> {
    // ... execute trade
    Html(format!(
        r#"<div id="player-balance">{}<img src="/static/images/deg_gold.png" alt="Gold"></div>"#,
        game.gold
    ))
}
```

2. **Add validation endpoints** for real-time feedback:
```rust
#[derive(Template)]
#[template(path = "partials/trade_buttons.html")]
struct TradeButtonsTemplate {
    can_buy: bool,
    can_sell: bool,
    buy_cost: i32,
    sell_income: i32,
}
```

3. **Update button labels** dynamically:
```html
<button hx-get="/game/trade/preview-food"
        hx-trigger="input from:#trade_food_qty"
        hx-target="#food-buttons">
    Buy (-{{ cost }})
</button>
```

## Testing

### Manual Testing Checklist
- [ ] Trade panel displays correctly with 0 markets (build prompt)
- [ ] Trade panel displays correctly with 1+ markets (Food available)
- [ ] Food trading: Buy and Sell work correctly
- [ ] Iron unlocks at 5+ markets
- [ ] Weapons unlock at 10+ markets
- [ ] Quick amount buttons set correct values
- [ ] MAX button sets inventory amount
- [ ] Number input accepts custom values
- [ ] Balance updates after trades
- [ ] Inventory counts update after trades
- [ ] Locked items show correct requirement text
- [ ] Responsive layout works on mobile (< 768px)
- [ ] Hover effects work on desktop
- [ ] Buttons disable during HTMX requests

### Visual Testing
Compare with specification requirements:
- ✅ Header with icon, title, market count
- ✅ Gold balance displayed prominently
- ✅ Good cards with icon, name, lock icon
- ✅ Price and inventory display
- ✅ Number input with quick buttons
- ✅ Buy/Sell buttons with distinct colors
- ✅ Locked state with reduced opacity
- ✅ Centered requirement text for locked items
- ✅ Responsive design for mobile

## Known Limitations

1. **Number Formatting**: Numbers are not formatted with thousand separators yet. The filter exists but needs Askama configuration to enable.

2. **Cost Preview**: Button labels show "Buy" and "Sell" without dynamic cost calculation (would require additional HTMX endpoints).

3. **Price Trends**: No up/down arrow indicators yet (would require tracking price history in GameState).

4. **Full Page Reload**: HTMX currently triggers full page reload instead of partial updates (matches existing redirect behavior).

5. **Validation Feedback**: No inline error messages yet (errors would appear after page reload).

## Future Improvements

### High Priority
- [ ] Add dynamic cost preview in button labels: "Buy (-400)" / "Sell (+400)"
- [ ] Implement partial HTML fragment returns for HTMX
- [ ] Add inline validation messages
- [ ] Add loading indicators during trades

### Medium Priority
- [ ] Add price trend indicators (↑↓) based on price changes
- [ ] Add trade confirmation dialog for large transactions
- [ ] Add trade history/log
- [ ] Implement optimistic UI updates

### Low Priority
- [ ] Add number formatting with thousand separators
- [ ] Add trade statistics/achievements
- [ ] Add bulk trade slider controls
- [ ] Add keyboard shortcuts for quick trading

## Files Modified

1. `static/css/style.css` - Added ~285 lines of new CSS
2. `templates/game.html` - Replaced lines 303-368 with new structure
3. `src/filters.rs` - New file with number formatting
4. `src/main.rs` - Added filters module import

## Compatibility

- ✅ Works with existing backend (full page reload)
- ✅ Uses existing color palette and design system
- ✅ Uses existing project icons
- ✅ Compatible with HTMX 1.x
- ✅ Responsive design (mobile, tablet, desktop)
- ✅ No breaking changes to game logic

## References

- Original specification: See project requirements document
- HTMX documentation: https://htmx.org/
- Askama documentation: https://djc.github.io/askama/
- Project icons: See `ICON_REFERENCE.md`
- Trading feature: See `TRADING_FEATURE.md`
