use askama::Template;
use axum::{
    extract::{Form, State},
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use std::sync::{Arc, RwLock};

use crate::game::{EventGenerator, GameState};

// Shared game state (in a real app, use proper session management)
pub type SharedGameState = Arc<RwLock<Option<GameState>>>;

#[derive(Template)]
#[template(path = "game.html")]
struct GameTemplate {
    state: GameState,
}

#[derive(Template)]
#[template(path = "report.html")]
struct ReportTemplate {
    state: GameState,
}

#[derive(Deserialize)]
pub struct TaxesForm {
    taxes_level: u8,
}

#[derive(Deserialize)]
pub struct FoodSupplyForm {
    food_supply: u8,
}

#[derive(Deserialize)]
pub struct TradeForm {
    quantity: i32,
}

#[derive(Template)]
#[template(path = "army.html")]
struct ArmyTemplate {
    state: GameState,
}

async fn game_view(State(game_state): State<SharedGameState>) -> impl IntoResponse {
    let state = game_state.read().unwrap();

    if let Some(ref game) = *state {
        let template = GameTemplate {
            state: game.clone(),
        };
        Html(template.render().unwrap())
    } else {
        Html("<h1>No active game. Please start a new game.</h1>".to_string())
    }
}

async fn set_taxes(
    State(game_state): State<SharedGameState>,
    Form(form): Form<TaxesForm>,
) -> impl IntoResponse {
    let mut state = game_state.write().unwrap();

    if let Some(ref mut game) = *state {
        game.taxes_level = form.taxes_level.min(5);
    }

    drop(state);
    Redirect::to("/game")
}

async fn set_food_supply(
    State(game_state): State<SharedGameState>,
    Form(form): Form<FoodSupplyForm>,
) -> impl IntoResponse {
    let mut state = game_state.write().unwrap();

    if let Some(ref mut game) = *state {
        game.food_supply = form.food_supply.min(5);
    }

    drop(state);
    Redirect::to("/game")
}

async fn build_farm(State(game_state): State<SharedGameState>) -> impl IntoResponse {
    let mut state = game_state.write().unwrap();

    if let Some(ref mut game) = *state {
        let _ = game.build_farm();
    }

    drop(state);
    Redirect::to("/game")
}

async fn build_mine(State(game_state): State<SharedGameState>) -> impl IntoResponse {
    let mut state = game_state.write().unwrap();

    if let Some(ref mut game) = *state {
        let _ = game.build_mine();
    }

    drop(state);
    Redirect::to("/game")
}

async fn build_smithy(State(game_state): State<SharedGameState>) -> impl IntoResponse {
    let mut state = game_state.write().unwrap();

    if let Some(ref mut game) = *state {
        let _ = game.build_smithy();
    }

    drop(state);
    Redirect::to("/game")
}

async fn build_market(State(game_state): State<SharedGameState>) -> impl IntoResponse {
    let mut state = game_state.write().unwrap();

    if let Some(ref mut game) = *state {
        let _ = game.build_market();
    }

    drop(state);
    Redirect::to("/game")
}

async fn upgrade_castle(State(game_state): State<SharedGameState>) -> impl IntoResponse {
    let mut state = game_state.write().unwrap();

    if let Some(ref mut game) = *state {
        let _ = game.upgrade_castle();
    }

    drop(state);
    Redirect::to("/game")
}

async fn buy_food(
    State(game_state): State<SharedGameState>,
    Form(form): Form<TradeForm>,
) -> impl IntoResponse {
    let mut state = game_state.write().unwrap();

    if let Some(ref mut game) = *state {
        let _ = game.buy_food(form.quantity);
    }

    drop(state);
    Redirect::to("/game")
}

async fn sell_food(
    State(game_state): State<SharedGameState>,
    Form(form): Form<TradeForm>,
) -> impl IntoResponse {
    let mut state = game_state.write().unwrap();

    if let Some(ref mut game) = *state {
        let _ = game.sell_food(form.quantity);
    }

    drop(state);
    Redirect::to("/game")
}

async fn buy_iron(
    State(game_state): State<SharedGameState>,
    Form(form): Form<TradeForm>,
) -> impl IntoResponse {
    let mut state = game_state.write().unwrap();

    if let Some(ref mut game) = *state {
        let _ = game.buy_iron(form.quantity as i16);
    }

    drop(state);
    Redirect::to("/game")
}

async fn buy_weapons(
    State(game_state): State<SharedGameState>,
    Form(form): Form<TradeForm>,
) -> impl IntoResponse {
    let mut state = game_state.write().unwrap();

    if let Some(ref mut game) = *state {
        let _ = game.buy_weapons(form.quantity as i16);
    }

    drop(state);
    Redirect::to("/game")
}

async fn army_view(State(game_state): State<SharedGameState>) -> impl IntoResponse {
    let state = game_state.read().unwrap();

    if let Some(ref game) = *state {
        let template = ArmyTemplate {
            state: game.clone(),
        };
        Html(template.render().unwrap())
    } else {
        Html("<h1>No active game. Please start a new game.</h1>".to_string())
    }
}

async fn recruit_soldiers(
    State(game_state): State<SharedGameState>,
    Form(form): Form<TradeForm>,
) -> impl IntoResponse {
    let mut state = game_state.write().unwrap();

    if let Some(ref mut game) = *state {
        let _ = game.recruit_soldiers(form.quantity as i16);
    }

    drop(state);
    Redirect::to("/game/army")
}

async fn discharge_soldiers(
    State(game_state): State<SharedGameState>,
    Form(form): Form<TradeForm>,
) -> impl IntoResponse {
    let mut state = game_state.write().unwrap();

    if let Some(ref mut game) = *state {
        let _ = game.discharge_soldiers(form.quantity as i16);
    }

    drop(state);
    Redirect::to("/game/army")
}

async fn finish_round(State(game_state): State<SharedGameState>) -> impl IntoResponse {
    let mut state = game_state.write().unwrap();

    if let Some(ref mut game) = *state {
        // Store previous popularity for comparison
        game.previous_popularity_percent = game.popularity_percent;

        // Apply round effects
        apply_round_effects(game);

        // Generate random event (30% chance)
        use rand::Rng;
        let mut rng = rand::thread_rng();
        if rng.gen::<f32>() < 0.3 {
            if let Some(event) = EventGenerator::generate_random_event(game) {
                // Store event for display in report
                game.last_event_title = Some(event.title.clone());
                game.last_event_description = Some(event.description.clone());
                event.apply_to_state(game);
            }
        } else {
            game.last_event_title = None;
            game.last_event_description = None;
        }

        // Check for grade advancement
        let _advanced = game.finish_round();

        // Check for game over
        if game.is_game_ended {
            drop(state);
            return Redirect::to("/game-over");
        }
    }

    drop(state);
    Redirect::to("/game/report")
}

async fn report_view(State(game_state): State<SharedGameState>) -> impl IntoResponse {
    let state = game_state.read().unwrap();

    if let Some(ref game) = *state {
        let template = ReportTemplate {
            state: game.clone(),
        };
        Html(template.render().unwrap())
    } else {
        Html("<h1>No active game. Please start a new game.</h1>".to_string())
    }
}

async fn continue_from_report(State(game_state): State<SharedGameState>) -> impl IntoResponse {
    // Clear event data after viewing report
    let mut state = game_state.write().unwrap();
    if let Some(ref mut game) = *state {
        game.last_event_title = None;
        game.last_event_description = None;
    }
    drop(state);
    Redirect::to("/game")
}

fn apply_round_effects(game: &mut GameState) {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    // 1. Calculate taxes (with randomness)
    if game.taxes_level != 0 && game.man_quantity > 0 {
        let random_value =
            rng.gen_range(-(game.man_quantity / 30 + 1)..=(game.man_quantity / 30 + 1));
        game.taxes_value = game.man_quantity * game.taxes_level as i32 * 10 / 30 + random_value;
        if game.taxes_value < 0 {
            game.taxes_value = 0;
        }
    } else {
        game.taxes_value = 0;
    }

    // 2. Calculate market income (with randomness)
    if game.market_quantity > 0 {
        let random_value =
            rng.gen_range(-(game.market_quantity as i32 + 1)..=(game.market_quantity as i32 + 1));
        game.market_place_value = game.market_quantity as i32 * 200 + random_value;
    } else {
        game.market_place_value = 0;
    }

    // 3. Calculate weapon production (happens BEFORE mine production)
    if game.smithy_quantity > 0 {
        let mut var3 = game.smithy_quantity as i32 * 8;
        if var3 > game.iron_quantity as i32 {
            var3 = game.iron_quantity as i32;
        }
        if game.weapon_quantity as i32 + var3 > game.trade_limit as i32 {
            var3 = game.trade_limit as i32 - game.weapon_quantity as i32;
        }
        game.weapon_quantity += var3 as i16;
        game.iron_quantity -= var3 as i16;
    }

    // 4. Calculate mine production (happens AFTER weapon production)
    if game.mine_quantity > 0 {
        if game.iron_quantity as i32 + game.mine_quantity as i32 * 10 > game.trade_limit as i32 {
            game.iron_quantity = game.trade_limit;
        } else {
            game.iron_quantity += game.mine_quantity * 10;
        }
    }

    // 5. Calculate harvest (with randomness)
    if game.farm_quantity > 0 {
        let random_value = rng.gen_range(0..64);
        game.harvest_value =
            game.farm_quantity as i32 * 116 + game.farm_quantity as i32 * random_value * 12;
        game.harvest_percent = game.harvest_value / (game.farm_quantity as i32 * 5);
    } else {
        game.harvest_value = 0;
        game.harvest_percent = 0;
    }

    // 6. Calculate food consumption
    let mut food_needed = game.man_quantity * (game.food_supply as i32 * 34 - 2) / 100;
    if food_needed < 0 {
        food_needed = 0;
    }

    let actual_food_level: i32;
    if game.food_quantity >= food_needed {
        game.food_quantity -= food_needed;
        actual_food_level = game.food_supply as i32;
    } else {
        actual_food_level = game.food_quantity * 100 / game.man_quantity / 34;
        game.food_quantity = 0;
        game.change_popularity(-4);
    }

    // 7. Food supply effects on popularity and population
    let (food_popularity_change, food_population_change) = match actual_food_level {
        0 => (-5, -(game.man_quantity * 8 / 100)),
        1 => (-2, -(game.man_quantity * 6 / 100)),
        2 => (-1, -(game.man_quantity * 3 / 100)),
        3 => (0, game.man_quantity * 1 / 100),
        4 => (1, game.man_quantity * 4 / 100),
        5 => (3, game.man_quantity * 7 / 100),
        _ => (0, 0),
    };

    game.change_popularity(food_popularity_change);
    game.change_population(food_population_change);

    // 8. Add harvest to food
    game.food_quantity += game.harvest_value;

    // 9. Tax level effects on popularity and population (with randomness)
    let random_base = rng.gen_range(0..4) + 4; // Generates 4-7
    let (tax_popularity_change, tax_population_multiplier) = match game.taxes_level {
        0 => (5, 10),
        1 => (3, 8),
        2 => (2, 4),
        3 => (0, 1),
        4 => (-3, -5),
        5 => (-5, -9),
        _ => (0, 1),
    };

    game.change_popularity(tax_popularity_change);
    game.change_population(random_base * tax_population_multiplier);

    // 10. Random population and popularity fluctuations
    let random_population = rng.gen_range(-4..=4);
    game.change_population(random_population);

    let random_popularity = rng.gen_range(-4..=4);
    game.change_popularity(random_popularity);

    // 11. Add gold from taxes and markets
    game.gold += game.taxes_value + game.market_place_value;

    // 12. Adjust food prices based on harvest
    if game.farm_quantity > 0 {
        let mut price_for_food =
            game.price_for_food_rate_constant as i32 * 100 / game.harvest_percent;
        let random_value = rng.gen_range(0..(price_for_food / 5));

        if price_for_food < 25 {
            price_for_food = 25;
        }
        if price_for_food > 100 {
            price_for_food = 100;
        }

        price_for_food -= random_value;
        game.price_for_food = price_for_food;
    } else {
        let random_value = rng.gen_range(
            -(game.price_for_food_rate_constant as i32 * 50 / 100)
                ..=(game.price_for_food_rate_constant as i32 * 50 / 100),
        );
        game.price_for_food = random_value + game.price_for_food_rate_constant as i32;
    }

    // 13. Adjust armor prices (with randomness)
    let var6 = game.price_for_armor_rate_constant as i32 * 20 / 100;
    let random_value = rng.gen_range(-var6..=var6);
    game.price_for_armor = game.price_for_armor_rate_constant as i32 + random_value;

    // 14. Adjust weapon prices (with randomness)
    let var6 = game.price_for_weapon_rate_constant as i32 * 20 / 100;
    let random_value = rng.gen_range(-var6..=var6);
    game.price_for_weapon = game.price_for_weapon_rate_constant as i32 + random_value;
}

pub fn game_routes() -> Router<SharedGameState> {
    Router::new()
        .route("/game", get(game_view))
        .route("/game/report", get(report_view))
        .route("/game/continue-from-report", post(continue_from_report))
        .route("/game/set-taxes", post(set_taxes))
        .route("/game/set-food-supply", post(set_food_supply))
        .route("/game/build-farm", post(build_farm))
        .route("/game/build-mine", post(build_mine))
        .route("/game/build-smithy", post(build_smithy))
        .route("/game/build-market", post(build_market))
        .route("/game/upgrade-castle", post(upgrade_castle))
        .route("/game/finish-round", post(finish_round))
        .route("/game/trade/buy-food", post(buy_food))
        .route("/game/trade/sell-food", post(sell_food))
        .route("/game/trade/buy-iron", post(buy_iron))
        .route("/game/trade/buy-weapons", post(buy_weapons))
        .route("/game/army", get(army_view))
        .route("/game/army/recruit", post(recruit_soldiers))
        .route("/game/army/discharge", post(discharge_soldiers))
}
