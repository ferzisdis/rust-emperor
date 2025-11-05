# Templates Quick Start Guide

## 🎨 View All Updated Templates

All 9 templates have been updated with the Fantasy Manuscript (Light Parchment) design. Here's how to view each page:

---

## 🚀 Start the Server

```bash
cd rust-emperor
cargo run
```

Server starts at: **http://localhost:3000**

---

## 📄 Template Navigation Guide

### 1. **Main Menu** (`menu.html`)
- **URL**: `http://localhost:3000`
- **What to see**:
  - Warm parchment background
  - EB Garamond title font
  - Gold accent buttons
  - Smooth hover effects

### 2. **About Page** (`about.html`)
- **Access**: Click "ℹ️ About" from main menu
- **What to see**:
  - Parchment sections with soft shadows
  - Gold border-left accents on lists
  - Hover effects on gameplay items
  - Ranks grid with medieval styling

### 3. **New Game Form** (`new_game_form.html`)
- **Access**: Click "🎮 Start New Game" from main menu
- **What to see**:
  - Parchment form container
  - Radio button cards with hover
  - Gold focus states on inputs
  - Medieval typography

### 4. **Game View** (`game.html`)
- **Access**: Fill form and start game
- **What to see**:
  - Decorative frame around kingdom map (main feature!)
  - "Your Kingdom" in Uncial Antiqua font
  - Resource cards with parchment styling
  - Gold upgrade buttons
  - Panel cards with dual shadows

### 5. **Army Page** (`army.html`)
- **Access**: Click "🎖️ Manage Army" from game view
- **What to see**:
  - Two-column layout (info + actions)
  - Stats card with parchment styling
  - Available recruitment card (green accent)
  - Quick select buttons with hover
  - Help panel with gold border
  - Action forms with medieval inputs

### 6. **Trade Page** (`trade.html`)
- **Access**: Click "🏪 Open Market" from game view (needs 1+ market)
- **What to see**:
  - Similar layout to army page
  - Trade cards for Buy/Sell
  - Form inputs with gold focus
  - Quick select buttons
  - Price information cards

### 7. **Highscores** (`highscores.html`)
- **Access**: Click "🏆 Highscores" from main menu
- **What to see**:
  - Parchment table design
  - Gold rank badges
  - Difficulty badges (Easy/Medium/Hard)
  - Hover effects on rows
  - "No scores" state if empty

### 8. **Round Report** (`report.html`)
- **Access**: Click "⏭️ Finish Round" in game
- **What to see**:
  - Report sections with parchment styling
  - Positive values (green)
  - Negative values (red)
  - Neutral values (gold)
  - Event list with border-left accents

---

## 🎯 Key Features to Check

### Typography
- [ ] **Headings**: EB Garamond serif font
- [ ] **Body text**: Inter sans-serif font
- [ ] **Kingdom title**: Uncial Antiqua medieval font

### Colors
- [ ] **Background**: Warm parchment (#F4ECD5)
- [ ] **Cards**: Lighter parchment (#F9F5EA)
- [ ] **Borders**: Soft tan (#CBBCA4)
- [ ] **Text**: Rich brown (#3F3426)
- [ ] **Accent**: Medieval gold (#C59A37)

### Interactive Elements
- [ ] **Buttons**: Hover lifts 1px with enhanced shadow
- [ ] **Cards**: Subtle shadow increase on hover
- [ ] **Inputs**: Gold border on focus with glow
- [ ] **Tables**: Row highlight on hover
- [ ] **Lists**: Hover effects on items

### Responsive
- [ ] Desktop (1920x1080): Multi-column layouts
- [ ] Tablet (768x1024): Flexible grids
- [ ] Mobile (375x667): Single column, stacked
- [ ] Small (320x568): Optimized spacing

---

## 🔍 Visual Inspection Checklist

### Main Menu
- [ ] Title in EB Garamond
- [ ] Gold buttons with hover
- [ ] Warm background
- [ ] Credits in italic

### Game View
- [ ] **IMPORTANT**: Kingdom map has decorative frame
- [ ] Uncial Antiqua for "Your Kingdom"
- [ ] Resources in parchment cards
- [ ] Gold upgrade button
- [ ] Panels with soft shadows

### Army/Trade Pages
- [ ] Two-column grid layout
- [ ] Info panel on left
- [ ] Action cards on right
- [ ] Quick select buttons work
- [ ] Help panel at bottom

### Highscores
- [ ] Table with parchment styling
- [ ] Rank badges visible
- [ ] Difficulty colors correct
- [ ] Hover highlights rows

### Report
- [ ] Sections clearly divided
- [ ] Colors show gains/losses
- [ ] Events listed clearly
- [ ] Continue button at bottom

---

## 📱 Mobile Testing

### How to Test
1. Open Chrome DevTools (F12)
2. Toggle device toolbar (Ctrl+Shift+M)
3. Select device or enter custom size
4. Navigate through pages

### Devices to Test
- iPhone SE (375x667)
- iPhone 12 Pro (390x844)
- Pixel 5 (393x851)
- iPad (768x1024)
- iPad Pro (1024x1366)

### What to Check
- [ ] Text readable (no tiny fonts)
- [ ] Buttons easy to tap (44px min)
- [ ] No horizontal scrolling
- [ ] Images scale properly
- [ ] Forms easy to fill

---

## 🎨 Color Verification

Open DevTools and inspect elements:

### Body Background
Should be: `#F4ECD5` (or `rgb(244, 236, 213)`)

### Panel Cards
Should be: `#F9F5EA` (or `rgb(249, 245, 234)`)

### Borders
Should be: `#CBBCA4` (or `rgb(203, 188, 164)`)

### Gold Buttons
Should be: `#C59A37` (or `rgb(197, 154, 55)`)

### Text
Should be: `#3F3426` (or `rgb(63, 52, 38)`)

---

## ⚡ Performance Check

### Load Time
- [ ] Initial page loads in < 2 seconds
- [ ] Fonts load without layout shift
- [ ] No flickering during load

### Animations
- [ ] Hover effects smooth (60fps)
- [ ] Transitions take 0.25s
- [ ] No lag or stutter

### Navigation
- [ ] Page changes instant (htmx)
- [ ] No full page reloads
- [ ] Smooth transitions

---

## 🐛 Common Issues

### Fonts Not Loading
**Problem**: Text appears in system fonts  
**Solution**: Check internet connection (Google Fonts)  
**Fallback**: EB Garamond → Georgia → serif

### Colors Look Wrong
**Problem**: Colors don't match specification  
**Solution**: Hard refresh (Ctrl+Shift+R)  
**Check**: Browser zoom at 100%

### Layout Broken
**Problem**: Elements overlapping or misaligned  
**Solution**: Clear cache and reload  
**Check**: Browser DevTools for CSS errors

### Pixel Art Blurry
**Problem**: Kingdom map looks fuzzy  
**Solution**: Verify `image-rendering: pixelated`  
**Check**: Browser zoom level

---

## ✅ Success Criteria

All templates successfully updated if:

- ✅ Every page has warm parchment background
- ✅ Medieval fonts load (EB Garamond, Inter)
- ✅ Gold buttons with hover effects
- ✅ Kingdom map has decorative frame
- ✅ All inline styles removed
- ✅ Consistent spacing throughout
- ✅ Mobile responsive
- ✅ No CSS errors in console

---

## 📊 Quick Comparison

### Before
- Generic web UI
- Inconsistent styling
- 687 lines inline CSS
- Different colors per page

### After
- Medieval manuscript theme
- Cohesive design system
- Global CSS in one file
- Warm parchment throughout

---

## 🎉 Testing Complete?

If all pages display correctly with:
- Warm parchment colors ✅
- Medieval typography ✅
- Gold accents ✅
- Smooth hover effects ✅
- Mobile responsive ✅

**Then the redesign is successful!** 🏰📜✨

---

## 📞 Need Help?

- Check `UI_REDESIGN.md` for design specs
- Read `TESTING_REDESIGN.md` for detailed testing
- See `ALL_TEMPLATES_UPDATED.md` for changes list

---

**Happy Testing!**

*Navigate your kingdom through a beautifully crafted parchment world!*