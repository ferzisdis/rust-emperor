# Trading Feature Documentation

## Overview

The trading system has been fully restored based on the original Java ME game implementation. Players can now buy and sell resources through their markets once they build at least one market building.

## How Trading Works

### Accessing the Trade Interface

1. Build at least one Market building (costs 2,000 gold initially)
2. Navigate to the main game screen
3. In the "Trade" panel, click the "🏪 Open Market" button
4. This opens the dedicated trading interface at `/game/trade`

### Available Trade Options

Trading options unlock based on the number of markets you have built:

| Resource | Markets Required | Trading Type | Base Price |
|----------|-----------------|--------------|------------|
| 🌾 Food | 1+ | Buy & Sell | 40 gold per 100 units |
| ⚙️ Iron | 5+ | Buy only | 60 gold per unit |
| ⚔️ Weapons | 10+ | Buy only | Based on `price_for_weapon` |

### Food Trading

- **Trading Units**: Food is traded in multiples of 100 (100, 1,000, 10,000, 100,000)
- **Bidirectional**: You can both buy and sell food
- **Price**: 40 gold per 100 units (configurable via `price_for_food`)
- **Use Case**: Buy food when your farms can't produce enough, sell excess food for gold

### Iron Trading

- **Unlock Requirement**: More than 4 markets (5+ markets)
- **Trading Units**: 1, 10, 100, 1,000 units
- **Buy Only**: You cannot sell iron back to the market
- **Price**: Based on `price_for_armor` (default: 60 gold per unit)
- **Trade Limit**: Maximum iron holdings capped at `trade_limit` (default: 20,000)
- **Use Case**: Buy iron when mines aren't producing enough for your smithies

### Weapons Trading

- **Unlock Requirement**: More than 9 markets (10+ markets)
- **Trading Units**: 1, 10, 100, 1,000 units
- **Buy Only**: You cannot sell weapons back to the market
- **Price**: Based on `price_for_weapon` configuration
- **Trade Limit**: Maximum weapon holdings capped at `trade_limit` (default: 20,000)
- **Use Case**: Buy weapons to quickly recruit more soldiers

## Implementation Details

### State Methods Added

The following trading methods were added to `GameState` in `src/game/state.rs`:

- `can_buy_food(quantity: i32) -> bool` - Check if player can afford food purchase
- `buy_food(quantity: i32) -> Result<(), String>` - Execute food purchase
- `can_sell_food(quantity: i32) -> bool` - Check if player has enough food to sell
- `sell_food(quantity: i32) -> Result<(), String>` - Execute food sale
- `can_buy_iron(quantity: i16) -> bool` - Check if player can buy iron
- `buy_iron(quantity: i16) -> Result<(), String>` - Execute iron purchase
- `can_buy_weapons(quantity: i16) -> bool` - Check if player can buy weapons
- `buy_weapons(quantity: i16) -> Result<(), String>` - Execute weapon purchase
- `get_available_trade_options() -> Vec<TradeOption>` - Get list of unlocked trade options

### Routes Added

New routes in `src/routes/game.rs`:

- `GET /game/trade` - Display the trading interface
- `POST /game/trade/buy-food` - Buy food
- `POST /game/trade/sell-food` - Sell food
- `POST /game/trade/buy-iron` - Buy iron
- `POST /game/trade/buy-weapons` - Buy weapons

### Templates

- `templates/trade.html` - Full trading interface with all trade options
- `templates/game.html` - Updated to show "Open Market" button and trade preview

## Trading Strategy Tips

1. **Early Game**: Build 1 market as soon as possible to have emergency food access
2. **Mid Game**: Expand to 5+ markets to unlock iron trading if mines are slow
3. **Late Game**: Aim for 10+ markets to buy weapons for rapid military expansion
4. **Gold Management**: Always keep reserve gold for emergency food purchases
5. **Market Income**: Markets generate passive income each round (50 gold per market)

## Original Implementation Reference

This implementation is based on:
- `GameCanvas.java` - `OpenTradeMenu()` method (lines 453-475)
- `TradeCanvas.java` - Full trading canvas implementation
- Trading logic matches the original Java ME game mechanics

## Trade Limit System

The `trade_limit` field (default: 20,000) prevents unlimited resource accumulation:
- Applies to iron and weapon purchases only
- Does not apply to food (can accumulate unlimited food)
- Prevents exploiting the trading system to bypass building infrastructure

## Future Enhancements

Potential improvements for the trading system:

- [ ] Dynamic pricing based on supply/demand
- [ ] Price fluctuations between rounds
- [ ] Trade history/log
- [ ] Bulk trade interface with slider controls
- [ ] Market upgrades for better prices
- [ ] Trading achievements/statistics