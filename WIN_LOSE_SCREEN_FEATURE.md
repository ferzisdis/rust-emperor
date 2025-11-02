# Win/Lose Screen Feature

## Overview

This document describes the win/lose screen feature that has been added to the Rust Emperor game. This feature displays an end-of-game screen when the player wins or loses the game, showing the final score and player information.

## Implementation

### Game End Conditions

The game can end in two ways:

1. **Victory (Win)**: The player reaches Grade 5 (Emperor rank)
   - `is_game_ended = true`
   - `is_won = true`
   - Score is calculated based on:
     - Years remaining (1501 - current_year) × 100
     - Difficulty bonus: Easy = 0, Medium = 1200, Hard = 2400
     - Population bonus: man_quantity ÷ 50

2. **Defeat (Lose)**: The game year exceeds 1500
   - `is_game_ended = true`
   - `is_won = false`
   - Score is set to 0

### Code Changes

#### 1. Menu Routes (`src/routes/menu.rs`)

Added a new `game_over` route handler that:
- Reads the game state
- Extracts the win/loss status, score, and player name
- Renders the menu template with `show_game_over = true`

```rust
async fn game_over(State(game_state): State<SharedGameState>) -> impl IntoResponse {
    let state = game_state.read().unwrap();

    let (won, score, player_name) = if let Some(ref game) = *state {
        let score = game.calculate_score();
        (game.is_won, score, game.user_name.clone())
    } else {
        (false, 0, String::new())
    };

    drop(state);

    let template = MenuTemplate {
        show_game_over: true,
        won,
        score,
        player_name,
    };
    Html(template.render().unwrap())
}
```

The route is registered at `/game-over`.

#### 2. Game Routes (`src/routes/game.rs`)

The `finish_round` handler already checks for game end conditions:

```rust
// Check for game over
if game.is_game_ended {
    drop(state);
    return Redirect::to("/game-over");
}
```

When the game ends, the player is automatically redirected to the `/game-over` route.

#### 3. Menu Template (`templates/menu.html`)

The menu template already had the win/lose screen UI built in:

```html
{% if show_game_over %}
<div class="game-over-panel {% if won %}won{% else %}lost{% endif %}">
    {% if won %}
    <h2>🎉 Congratulations!</h2>
    <p class="result-message">You have won!</p>
    {% else %}
    <h2>😔 Game Over</h2>
    <p class="result-message">You have lost!</p>
    {% endif %}
    <div class="score-display">
        <p><strong>Final Score:</strong> {{ score }} points</p>
        <p><strong>Player:</strong> {{ player_name }}</p>
    </div>
</div>
{% endif %}
```

The screen shows:
- Different messages and styling for win vs. loss
- Final score (only non-zero for wins)
- Player name
- Menu buttons to start a new game, view highscores, or see about info

## User Experience Flow

1. Player plays the game through multiple rounds
2. Player either:
   - Reaches Grade 5 (Emperor) → Victory
   - Exceeds year 1500 → Defeat
3. On the next round finish, `is_game_ended` is detected
4. Player is redirected to `/game-over`
5. Win/Lose screen is displayed with:
   - Congratulations or Game Over message
   - Final score
   - Player name
   - Option to start a new game

## Comparison to Original Game

In the original Java game (`RustEmperorOriginal/GameCanvas.java`):

```java
if (this.storage.isGameEnded) {
    if (this.storage.isWon) {
        this.ShowArmyCanvas();
    } else {
        this.EndGame();
    }
}
```

The new Rust implementation simplifies this by:
- Using a single route for both win and lose screens
- Conditionally rendering different content based on the `won` flag
- Maintaining the same game logic for determining win/lose conditions

## Testing

To test the win/lose screen:

### Test Victory Screen
1. Start a new game
2. Use browser dev tools or modify code to set `grade = 4`
3. Advance grade to 5 by meeting requirements
4. Click "Finish Round"
5. Should redirect to win screen with calculated score

### Test Defeat Screen
1. Start a new game
2. Use browser dev tools or modify code to set `year = 1500`
3. Click "Finish Round"
4. Should redirect to lose screen with score of 0

### Quick Test with Code Modification
Add temporary test data in `finish_round` handler:
```rust
// TEST: Force game end for testing
game.is_game_ended = true;
game.is_won = true; // or false for loss test
```

## Future Enhancements

Potential improvements to the win/lose screen:

1. **Highscore Integration**: Save and display if player's score makes the highscore list
2. **Statistics Display**: Show additional stats like total population, gold earned, etc.
3. **Achievements**: Display achievements unlocked during the game
4. **Replay Option**: Quick restart with same settings
5. **Share Functionality**: Share score on social media or export game summary
6. **Victory Animation**: Add celebratory animations or effects for wins
7. **Detailed Breakdown**: Show score calculation breakdown
8. **Comparison**: Compare with previous games or average scores

## Related Files

- `src/routes/menu.rs` - Menu and game-over route handlers
- `src/routes/game.rs` - Game flow and round finishing logic
- `src/game/state.rs` - Game state including `is_game_ended`, `is_won`, and `calculate_score()`
- `templates/menu.html` - Win/lose screen UI template
- `RustEmperorOriginal/GameCanvas.java` - Original game end logic reference