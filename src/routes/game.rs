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
    // Calculate harvest
    let base_harvest = game.farm_quantity as i32 * 500;
    let harvest_bonus = match game.food_supply {
        0 => -200,
        1 => -100,
        2 => 0,
        3 => 100,
        4 => 200,
        5 => 300,
        _ => 0,
    };
    game.harvest_value = base_harvest + harvest_bonus;

    // Calculate harvest percentage (for display)
    let expected_harvest = game.farm_quantity as i32 * 500;
    if expected_harvest > 0 {
        game.harvest_percent =
            ((game.harvest_value as f32 / expected_harvest as f32) * 100.0) as i32;
    } else {
        game.harvest_percent = 100;
    }

    game.change_food(game.harvest_value);

    // Calculate food consumption
    let food_per_person = match game.food_supply {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => 4,
        5 => 5,
        _ => 3,
    };
    let food_consumed = game.man_quantity * food_per_person;
    game.change_food(-food_consumed);

    // Calculate taxes
    let tax_rate = match game.taxes_level {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => 5,
        5 => 8,
        _ => 3,
    };
    game.taxes_value = game.man_quantity * tax_rate;
    game.change_gold(game.taxes_value);

    // Calculate market income
    game.market_place_value = game.market_quantity as i32 * 50;
    game.change_gold(game.market_place_value);

    // Calculate mine production
    let iron_produced = game.mine_quantity as i16 * 10;
    game.iron_quantity += iron_produced;

    // Calculate weapon production
    let weapons_produced = game.smithy_quantity.min(game.iron_quantity / 5);
    game.weapon_quantity += weapons_produced;
    game.iron_quantity -= weapons_produced * 5;

    // Popularity changes
    let tax_penalty = match game.taxes_level {
        0 => 2,
        1 => 1,
        2 => 0,
        3 => -1,
        4 => -3,
        5 => -5,
        _ => 0,
    };

    let food_penalty = match game.food_supply {
        0 => -10,
        1 => -5,
        2 => -2,
        3 => 0,
        4 => 1,
        5 => 2,
        _ => 0,
    };

    let popularity_change = tax_penalty + food_penalty;
    game.change_popularity(popularity_change);

    // Population growth
    let growth_rate = if game.popularity_percent > 70 {
        (game.man_quantity as f32 * 0.05) as i32
    } else if game.popularity_percent > 50 {
        (game.man_quantity as f32 * 0.02) as i32
    } else if game.popularity_percent > 30 {
        0
    } else {
        -((game.man_quantity as f32 * 0.03) as i32)
    };

    game.change_population(growth_rate);

    // Soldier maintenance cost
    let soldier_cost = game.soldier_quantity as i32 * 10;
    game.change_gold(-soldier_cost);

    // Food shortage effects
    if game.food_quantity < game.man_quantity / 2 {
        let starvation = (game.man_quantity as f32 * 0.1) as i32;
        game.change_population(-starvation);
        game.change_popularity(-10);
    }
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
}
