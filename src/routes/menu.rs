use askama::Template;
use axum::{
    extract::{Form, State},
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
    Router,
};
use serde::Deserialize;

use crate::game::{Difficulty, GameState, Gender};
use crate::routes::game::SharedGameState;

#[derive(Template)]
#[template(path = "menu.html")]
struct MenuTemplate {
    show_game_over: bool,
    won: bool,
    score: i32,
    player_name: String,
}

#[derive(Template)]
#[template(path = "new_game_form.html")]
struct NewGameFormTemplate;

#[derive(Template)]
#[template(path = "about.html")]
struct AboutTemplate;

#[derive(Template)]
#[template(path = "highscores.html")]
struct HighscoresTemplate {
    scores: Vec<(String, i32, String)>, // (name, score, difficulty)
}

#[derive(Deserialize)]
pub struct NewGameForm {
    player_name: String,
    gender: String,
    difficulty: String,
}

async fn index() -> impl IntoResponse {
    let template = MenuTemplate {
        show_game_over: false,
        won: false,
        score: 0,
        player_name: String::new(),
    };
    Html(template.render().unwrap())
}

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

async fn new_game_form() -> impl IntoResponse {
    let template = NewGameFormTemplate;
    Html(template.render().unwrap())
}

async fn start_game(
    State(game_state): State<SharedGameState>,
    Form(form): Form<NewGameForm>,
) -> impl IntoResponse {
    let gender = match form.gender.as_str() {
        "Female" => Gender::Female,
        _ => Gender::Male,
    };

    let difficulty = match form.difficulty.as_str() {
        "Easy" => Difficulty::Easy,
        "Medium" => Difficulty::Medium,
        "Hard" => Difficulty::Hard,
        _ => Difficulty::Easy,
    };

    let state = GameState::new(form.player_name, gender, difficulty);

    // Store the game state
    {
        let mut game = game_state.write().unwrap();
        *game = Some(state);
    }

    Redirect::to("/game")
}

async fn about() -> impl IntoResponse {
    let template = AboutTemplate;
    Html(template.render().unwrap())
}

async fn highscores() -> impl IntoResponse {
    // Placeholder scores - in a real app, load from database/file
    let scores = vec![
        ("Emperor_Magnus".to_string(), 15000, "Hard".to_string()),
        ("Queen_Victoria".to_string(), 12500, "Medium".to_string()),
        ("King_Arthur".to_string(), 10000, "Medium".to_string()),
        ("Duke_William".to_string(), 8500, "Easy".to_string()),
        ("Baroness_Jane".to_string(), 5000, "Easy".to_string()),
    ];

    let template = HighscoresTemplate { scores };
    Html(template.render().unwrap())
}

pub fn menu_routes() -> Router<SharedGameState> {
    Router::new()
        .route("/", get(index))
        .route("/new-game-form", get(new_game_form))
        .route("/start-game", post(start_game))
        .route("/about", get(about))
        .route("/highscores", get(highscores))
        .route("/game-over", get(game_over))
}
