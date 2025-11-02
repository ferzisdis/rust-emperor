# Testing the Win/Lose Screen

This guide provides quick methods to test the win/lose screen functionality without having to play through an entire game.

## Method 1: Test via Browser Console (Easiest)

### Testing the Victory Screen

1. Start the game and navigate to the game view (`/game`)
2. Open browser developer tools (F12)
3. In the browser's address bar, navigate to: `http://localhost:3000/game-over`
4. You should see the game-over screen

To test with actual game state:
1. Start a game and play at least one round
2. The game state will be stored in server memory
3. Navigate to `/game-over` to see the screen with your current game data

### Testing the Defeat Screen

Same process as above - the screen will show defeat if `is_won = false`.

## Method 2: Modify Game State (For Development)

### Quick Win Test

Temporarily modify `src/routes/game.rs` in the `finish_round` handler:

```rust
async fn finish_round(State(game_state): State<SharedGameState>) -> impl IntoResponse {
    let mut state = game_state.write().unwrap();

    if let Some(ref mut game) = *state {
        // === TEST: Force a win ===
        game.is_game_ended = true;
        game.is_won = true;
        game.year = 1450; // For score calculation
        // === END TEST ===
        
        // Store previous popularity for comparison
        game.previous_popularity_percent = game.popularity_percent;
        // ... rest of the code
```

Now when you click "Finish Round", you'll be redirected to the victory screen.

### Quick Loss Test

```rust
async fn finish_round(State(game_state): State<SharedGameState>) -> impl IntoResponse {
    let mut state = game_state.write().unwrap();

    if let Some(ref mut game) = *state {
        // === TEST: Force a loss ===
        game.is_game_ended = true;
        game.is_won = false;
        game.year = 1501;
        // === END TEST ===
        
        // ... rest of the code
```

### Remember to Remove Test Code

After testing, remove the test code before committing!

## Method 3: Play to Victory (Natural Test)

To naturally reach the victory screen:

1. Start a new game on Easy difficulty
2. Focus on these requirements for Grade 5 (Emperor):
   - Castle Level 8
   - At least 4000 citizens
   - At least 500 soldiers
   - At least 100 weapons
   - Popularity at least 20%

3. Strategy for quick victory:
   - Set taxes to level 2-3 (balance gold and popularity)
   - Set food supply to level 4-5 (grow population fast)
   - Build farms first (10+)
   - Build markets for income (5+)
   - Build mines for iron (5+)
   - Build smithies for weapons (3+)
   - Upgrade castle whenever possible
   - Recruit soldiers when you have enough population and weapons
   - Trade to buy iron and weapons if needed

4. Once you meet all Grade 5 requirements and advance, the game ends with victory

## Method 4: Play to Defeat (Natural Test)

To naturally reach the defeat screen:

1. Start a new game
2. Play poorly:
   - Set high taxes (level 5)
   - Set low food supply (level 0-1)
   - Don't build anything
   - Don't upgrade castle
   - Let your population die off

3. Keep clicking "Finish Round" until year > 1500
4. Game ends with defeat

## Method 5: Add Debug Route (Advanced)

Add a temporary debug route to `src/routes/game.rs`:

```rust
async fn debug_end_game(
    State(game_state): State<SharedGameState>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>
) -> impl IntoResponse {
    let mut state = game_state.write().unwrap();
    
    if let Some(ref mut game) = *state {
        game.is_game_ended = true;
        game.is_won = params.get("win").map(|v| v == "true").unwrap_or(false);
        if game.is_won {
            game.year = 1450; // Good score
            game.man_quantity = 5000;
        } else {
            game.year = 1501;
        }
    }
    
    drop(state);
    Redirect::to("/game-over")
}
```

Register the route:
```rust
.route("/debug/end-game", get(debug_end_game))
```

Then navigate to:
- Victory: `http://localhost:3000/debug/end-game?win=true`
- Defeat: `http://localhost:3000/debug/end-game?win=false`

**Important:** Remove this debug route before production!

## Expected Behavior

### Victory Screen Should Show:
- 🎉 Congratulations heading
- "You have won!" message
- Final score (calculated based on year, difficulty, population)
- Player name
- Green/positive styling
- Menu buttons (Start New Game, Highscores, About)

### Defeat Screen Should Show:
- 😔 Game Over heading
- "You have lost!" message
- Final score: 0 points
- Player name
- Red/negative styling
- Menu buttons (Start New Game, Highscores, About)

## Verifying Score Calculation

For a victory at year 1450 on Medium difficulty with 5000 citizens:

```
Score = (1501 - 1450) × 100 + 1200 + (5000 ÷ 50)
Score = 51 × 100 + 1200 + 100
Score = 5100 + 1200 + 100
Score = 6400 points
```

Victory scores should always be > 0.
Defeat scores should always be 0.

## Common Issues

### Issue: Game-over screen shows no game state
**Solution:** Make sure you start a game first before navigating to `/game-over`

### Issue: Screen doesn't show after clicking "Finish Round"
**Solution:** Check that `is_game_ended` is being set to true in the game state

### Issue: Score is always 0
**Solution:** Check that `is_won` is set to true for victories

### Issue: Redirect doesn't happen
**Solution:** Verify the check in `finish_round` handler:
```rust
if game.is_game_ended {
    drop(state);
    return Redirect::to("/game-over");
}
```

## Automated Testing (Future Enhancement)

Consider adding integration tests:

```rust
#[tokio::test]
async fn test_victory_screen() {
    // Create game state with victory conditions
    // Navigate to /game-over
    // Assert response contains "Congratulations"
    // Assert score > 0
}

#[tokio::test]
async fn test_defeat_screen() {
    // Create game state with defeat conditions
    // Navigate to /game-over
    // Assert response contains "Game Over"
    // Assert score == 0
}
```
