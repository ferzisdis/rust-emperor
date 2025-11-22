# Panels Consistency Guide

## Overview
This guide documents the unified design system applied across the Trade and Military panels, establishing patterns that should be followed for future panel designs in the game.

## Design Principles

### 1. Visual Consistency
All resource management panels should share:
- Common header structure
- Card-based action layout
- Consistent color coding
- Unified typography and spacing

### 2. Interaction Patterns
Users should experience:
- Predictable input methods
- Familiar button layouts
- Consistent feedback mechanisms
- Similar state management

### 3. Information Architecture
Data should be presented with:
- Clear hierarchy
- Contextual grouping
- Progressive disclosure
- At-a-glance summaries

---

## Component Specifications

### Panel Header

#### Structure
```html
<div class="trade-header">
    <div class="trade-header-left">
        <img src="[icon]" alt="[name]" style="height: 1.2em; vertical-align: middle;">
        <h3>[Panel Name]</h3>
        <span class="trade-market-count">• [Context Info]</span>
    </div>
    <div class="trade-balance" id="[unique-id]">
        [Resource Count]<img src="/static/images/deg_gold.png" alt="Gold">
    </div>
</div>
```

#### Usage Examples
- **Trade Panel:** Icon: Market | Context: "X market(s)" | Resource: Gold
- **Military Panel:** Icon: Soldier | Context: "X soldier(s)" | Resource: Gold

#### Guidelines
- Icon should be 1.2em for visual prominence
- Context info uses bullet separator (•)
- Resource count right-aligned for easy scanning
- Consistent vertical alignment across all elements

---

### Action Cards

#### Structure
```html
<div class="trade-goods-list">
    <div class="trade-good-item">
        <div class="trade-good-info">
            <div class="trade-good-icon">
                <img src="[icon]" alt="[name]">
            </div>
            <span class="trade-good-name">[Action Name]</span>
            <div class="trade-good-details">
                [Cost/Price Info] • [Stock/Status Info]
            </div>
        </div>
        <div class="trade-good-controls">
            <!-- Input section -->
            <!-- Action buttons -->
        </div>
    </div>
</div>
```

#### Card Content Guidelines
1. **Icon:** Should clearly represent the action/resource
2. **Name:** Clear, concise action description
3. **Details:** 
   - Use bullet (•) to separate multiple pieces of info
   - Include relevant icons (gold, resources)
   - Show current state/availability

---

### Input Controls

#### Structure
```html
<div class="trade-input-section">
    <input type="number" id="[unique-id]" name="quantity" value="[default]" min="0" step="[step]">
    <div class="trade-quick-buttons">
        <button type="button" class="btn-quick-amount" onclick="[set-and-dispatch]">
            [Amount]
        </button>
        <!-- More quick buttons -->
    </div>
</div>
```

#### Quick Button Patterns

**Standard Pattern:**
```javascript
onclick="let input = document.getElementById('[id]'); 
         input.value = [value]; 
         input.dispatchEvent(new Event('input'));"
```

**Common Presets by Resource Type:**

| Resource Type | Preset Buttons | Step Size |
|--------------|----------------|-----------|
| Bulk (Food) | 100, 1K, 10K, MAX BUY, MAX SELL | 100 |
| Units (Iron/Weapons) | 1, 10, 100, MAX BUY, MAX SELL | 1 |
| Military | 1, 10, 50, 100, MAX | 1 |
| Buildings | N/A (single builds) | N/A |

#### MAX Button Logic

**Trade MAX BUY:**
```javascript
Math.floor(currentGold / itemPrice) * units_per_purchase
```

**Trade MAX SELL:**
```javascript
currentInventory
```

**Military Recruit MAX:**
```javascript
Math.min(
    Math.floor(currentGold / soldierPrice),
    availableWeapons,
    Math.max(0, availableCitizens - reserveAmount)
)
```

**Military Discharge MAX:**
```javascript
currentSoldiers
```

---

### Action Buttons

#### Structure
```html
<div class="trade-action-buttons">
    <button 
        hx-post="[endpoint]" 
        hx-vals='js:{quantity: document.getElementById("[input-id]").value}' 
        hx-target="body" 
        hx-swap="innerHTML"
        hx-disabled-elt="this"
        class="btn [btn-type]"
        id="[unique-id]"
        [conditional-disabled]>
        [Label]
    </button>
</div>
```

#### Button Color Coding

| Action Type | CSS Class | Color | Usage |
|------------|-----------|-------|-------|
| Buy/Acquire | `btn-trade-buy` | Green | Purchasing, recruiting, building |
| Sell/Release | `btn-trade-sell` | Orange | Selling, discharging, demolishing |
| Neutral/Info | `btn` | Gray | Default actions |
| Disabled | `disabled` | Muted | Unavailable actions |

#### Button Label Patterns
- **Cost Actions:** `[Action] (-[Cost])` e.g., "Buy (-500)", "Recruit (-10)"
- **Income Actions:** `[Action] (+[Income])` e.g., "Sell (+500)"
- **Quantity Actions:** `[Action] ([Qty])` e.g., "Discharge (10)"
- **Simple Actions:** `[Action]` e.g., "Build", "Upgrade"

---

### Locked/Unavailable States

#### Structure
```html
<div class="trade-good-item trade-good-locked">
    <div class="trade-good-info">
        <div class="trade-good-icon">
            <img src="[icon]" alt="[name]">
        </div>
        <span class="trade-good-name">[Item Name]</span>
        <span class="trade-good-lock">🔒</span>
    </div>
    <div class="trade-locked-text">[Unlock Requirement]</div>
</div>
```

#### Guidelines
- Use lock emoji (🔒) for visual indicator
- Clearly state unlock requirements
- Reduced opacity via CSS (trade-good-locked)
- No interactive elements when locked

---

## JavaScript Dynamic Updates

### Update Function Pattern

```javascript
const [resource]Input = document.getElementById('[input-id]');
const [action]Btn = document.getElementById('[button-id]');

if ([resource]Input && [action]Btn) {
    function update[Action]Buttons() {
        const qty = parseInt([resource]Input.value) || 0;
        const cost = [calculation];
        
        // Update button label
        [action]Btn.textContent = `[Action] (${cost})`;
        
        // Check requirements
        const canPerformAction = [validation-logic];
        
        // Enable/disable button
        if (!canPerformAction) {
            [action]Btn.classList.add('disabled');
            [action]Btn.disabled = true;
        } else {
            [action]Btn.classList.remove('disabled');
            [action]Btn.disabled = false;
        }
    }
    
    // Attach event listeners
    [resource]Input.addEventListener('input', update[Action]Buttons);
    [resource]Input.addEventListener('change', update[Action]Buttons);
    
    // Initial state
    update[Action]Buttons();
}
```

### Event Dispatch for Quick Buttons
Always dispatch 'input' event to trigger update functions:
```javascript
input.dispatchEvent(new Event('input'));
```

---

## CSS Class Reference

### Layout Classes
- `panel` - Main container for each panel
- `trade-header` - Panel header wrapper
- `trade-header-left` - Left section (icon, title, info)
- `trade-balance` - Right section (resource count)
- `trade-goods-list` - Container for action cards
- `trade-good-item` - Individual action card
- `trade-good-locked` - Modifier for locked/unavailable items

### Content Classes
- `trade-good-info` - Card header section
- `trade-good-icon` - Icon wrapper
- `trade-good-name` - Primary label
- `trade-good-details` - Secondary info/stats
- `trade-good-lock` - Lock indicator
- `trade-locked-text` - Unlock requirement text

### Control Classes
- `trade-good-controls` - Controls wrapper
- `trade-input-section` - Input field + quick buttons
- `trade-quick-buttons` - Quick amount button container
- `trade-action-buttons` - Action button container
- `btn-quick-amount` - Individual quick button
- `btn-trade-buy` - Buy/acquire action button
- `btn-trade-sell` - Sell/release action button

### Utility Classes
- `trade-market-count` - Context info badge
- `hint` - Helper text styling
- `disabled` - Disabled state

---

## Panel Comparison Matrix

| Feature | Trade Panel | Military Panel | Buildings Panel |
|---------|-------------|----------------|-----------------|
| **Header** | ✅ Enhanced | ✅ Enhanced | ⚠️ Basic |
| **Card Layout** | ✅ Yes | ✅ Yes | ❌ List |
| **Number Input** | ✅ Yes | ✅ Yes | ❌ N/A |
| **Quick Buttons** | ✅ Yes (5) | ✅ Yes (5) | ❌ No |
| **MAX Button** | ✅ Smart (Buy/Sell) | ✅ Smart (Multi-constraint) | ❌ N/A |
| **Dynamic Cost** | ✅ Yes | ✅ Yes | ⚠️ Static |
| **Color Coding** | ✅ Buy/Sell | ✅ Recruit/Discharge | ⚠️ Generic |
| **Locked States** | ✅ Yes | ❌ N/A | ❌ N/A |
| **HTMX Integration** | ✅ Full | ✅ Full | ✅ Full |

Legend:
- ✅ Fully implemented
- ⚠️ Partially implemented
- ❌ Not implemented

---

## Implementation Checklist

When creating a new panel or updating an existing one:

### Planning Phase
- [ ] Identify primary resources/actions
- [ ] Define header context info
- [ ] Determine appropriate quick button amounts
- [ ] Plan MAX button logic if applicable
- [ ] Define locked states if applicable

### HTML Structure
- [ ] Implement panel header with icon, title, context
- [ ] Add resource/gold balance display
- [ ] Create action cards with consistent structure
- [ ] Add input fields with number type
- [ ] Include quick amount buttons
- [ ] Implement action buttons with HTMX
- [ ] Add locked state variants if needed

### JavaScript Integration
- [ ] Define constants for prices/costs
- [ ] Create update functions for each action
- [ ] Calculate costs/requirements dynamically
- [ ] Update button labels in real-time
- [ ] Enable/disable based on validation
- [ ] Attach event listeners to inputs
- [ ] Call initial update on page load

### Styling
- [ ] Reuse existing CSS classes
- [ ] Apply color coding (buy/sell)
- [ ] Test hover/focus states
- [ ] Verify disabled states
- [ ] Check responsive layout

### Testing
- [ ] Test all quick buttons
- [ ] Verify MAX button calculations
- [ ] Test boundary conditions (0, max values)
- [ ] Validate disable logic
- [ ] Test HTMX submissions
- [ ] Verify no double-submissions
- [ ] Check cross-browser compatibility

---

## Best Practices

### Do's ✅
1. **Reuse existing CSS classes** - Maintain consistency
2. **Follow naming conventions** - Use descriptive IDs
3. **Dispatch input events** - Trigger updates from quick buttons
4. **Validate comprehensively** - Check all requirements
5. **Provide feedback** - Update labels, disable states
6. **Use semantic HTML** - Proper button types, labels
7. **Include alt text** - All images need descriptions
8. **Test edge cases** - Zero values, max limits

### Don'ts ❌
1. **Don't inline styles** - Use CSS classes
2. **Don't hardcode values** - Use template variables
3. **Don't skip validation** - Always check requirements
4. **Don't forget accessibility** - Labels, ARIA when needed
5. **Don't duplicate logic** - Reuse update functions
6. **Don't ignore disabled states** - Handle unavailable actions
7. **Don't break HTMX patterns** - Follow existing conventions
8. **Don't forget initial calls** - Run updates on page load

---

## Accessibility Guidelines

### Keyboard Navigation
- All interactive elements must be keyboard accessible
- Logical tab order through form elements
- Enter key should submit forms/trigger actions

### Screen Readers
- All images require alt text
- Button labels must be descriptive
- Dynamic updates should be announced (ARIA live regions)

### Visual Clarity
- Sufficient color contrast (WCAG AA minimum)
- Icons supplemented with text labels
- Disabled states clearly indicated
- Focus states visible

### Input Validation
- Clear error messages
- Inline validation feedback
- Prevent invalid submissions
- Guide users to corrections

---

## Migration Guide

### Updating Existing Panels

#### Step 1: Backup
```bash
git checkout -b panel-redesign-[name]
cp templates/game.html templates/game.html.backup
```

#### Step 2: Header Conversion
Replace simple header with enhanced layout:
```html
<!-- Old -->
<h3><img src="..." alt="..."> Title</h3>

<!-- New -->
<div class="trade-header">
    <div class="trade-header-left">
        <img src="..." alt="..." style="height: 1.2em; vertical-align: middle;">
        <h3>Title</h3>
        <span class="trade-market-count">• Context</span>
    </div>
    <div class="trade-balance">
        Balance<img src="...">
    </div>
</div>
```

#### Step 3: Card Conversion
Wrap actions in card structure:
```html
<!-- Old -->
<div class="action">
    <span>Name</span>
    <button>Action</button>
</div>

<!-- New -->
<div class="trade-goods-list">
    <div class="trade-good-item">
        <div class="trade-good-info">...</div>
        <div class="trade-good-controls">...</div>
    </div>
</div>
```

#### Step 4: Input Enhancement
Replace dropdowns with flexible inputs:
```html
<!-- Old -->
<select id="qty">
    <option value="1">1</option>
    <option value="10">10</option>
</select>

<!-- New -->
<div class="trade-input-section">
    <input type="number" id="qty" value="1" min="0" step="1">
    <div class="trade-quick-buttons">
        <button type="button" class="btn-quick-amount" onclick="...">1</button>
        <button type="button" class="btn-quick-amount" onclick="...">10</button>
        <button type="button" class="btn-quick-amount" onclick="...">MAX</button>
    </div>
</div>
```

#### Step 5: JavaScript Addition
Add dynamic update functions (see pattern above)

#### Step 6: Testing
- [ ] Visual comparison
- [ ] Functionality verification
- [ ] Accessibility audit
- [ ] Cross-browser testing

---

## Future Enhancements

### Potential Additions
1. **Animations:** Smooth transitions for updates
2. **Tooltips:** Hover info for complex requirements
3. **History:** Recent transactions log
4. **Trends:** Price/resource change indicators
5. **Bulk Actions:** Multi-select for related operations
6. **Templates:** Save common transaction amounts
7. **Keyboard Shortcuts:** Quick access to common actions

### Design System Evolution
As the game grows, consider:
- Component library documentation
- Storybook for UI components
- Design tokens for theming
- Responsive breakpoints standardization
- Dark mode support
- Internationalization patterns

---

## Version History

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2024 | Initial guide based on Trade/Military panels |

---

## Related Documentation
- `TRADE_PANEL_SUMMARY.md` - Trade panel redesign
- `TRADE_PANEL_CHANGELOG.md` - Trade panel changes
- `MILITARY_PANEL_CHANGELOG.md` - Military panel changes
- `MILITARY_PANEL_REDESIGN_SUMMARY.md` - Military panel summary
- `TRADE_PANEL_REDESIGN.md` - Full Trade redesign specification

---

*This is a living document. Update as patterns evolve and new panels are added.*