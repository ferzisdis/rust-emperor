use crate::game::state::GameState;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct GameEvent {
    pub title: String,
    pub description: String,
    pub effects: EventEffects,
}

#[derive(Debug, Clone, Default)]
pub struct EventEffects {
    pub gold_change: i32,
    pub food_change: i32,
    pub population_change: i32,
    pub popularity_change: i8,
    pub soldier_change: i16,
}

pub struct EventGenerator;

impl EventGenerator {
    pub fn generate_random_event(state: &GameState) -> Option<GameEvent> {
        let mut rng = rand::thread_rng();

        // 30% chance of no event
        if rng.gen_bool(0.3) {
            return None;
        }

        let event_type = rng.gen_range(0..15);

        let event = match event_type {
            0 => Self::plague_event(state),
            1 => Self::good_harvest_event(state),
            2 => Self::poor_harvest_event(state),
            3 => Self::merchants_event(state),
            4 => Self::bandits_event(state),
            5 => Self::festival_event(state),
            6 => Self::drought_event(state),
            7 => Self::immigrants_event(state),
            8 => Self::gold_mine_event(state),
            9 => Self::storm_event(state),
            10 => Self::deserters_event(state),
            11 => Self::tax_revolt_event(state),
            12 => Self::wedding_event(state),
            13 => Self::raiders_attack_event(state),
            14 => Self::bumper_crop_event(state),
            _ => return None,
        };

        Some(event)
    }

    fn plague_event(state: &GameState) -> GameEvent {
        let mut rng = rand::thread_rng();
        let loss = rng.gen_range(50..150);
        let popularity_loss = rng.gen_range(5..15) as i8;

        GameEvent {
            title: "Plague Outbreak!".to_string(),
            description: format!(
                "A plague has struck your lands! {} citizens have died. Popularity decreased by {}%.",
                loss, popularity_loss
            ),
            effects: EventEffects {
                population_change: -(loss as i32),
                popularity_change: -popularity_loss,
                ..Default::default()
            },
        }
    }

    fn good_harvest_event(state: &GameState) -> GameEvent {
        let mut rng = rand::thread_rng();
        let bonus = rng.gen_range(500..1500);

        GameEvent {
            title: "Excellent Harvest!".to_string(),
            description: format!(
                "The gods have blessed your fields! Your farms produced {} extra food.",
                bonus
            ),
            effects: EventEffects {
                food_change: bonus,
                popularity_change: 3,
                ..Default::default()
            },
        }
    }

    fn poor_harvest_event(state: &GameState) -> GameEvent {
        let mut rng = rand::thread_rng();
        let loss = rng.gen_range(300..800);

        GameEvent {
            title: "Poor Harvest".to_string(),
            description: format!("The harvest was poor this year. You lost {} food.", loss),
            effects: EventEffects {
                food_change: -loss,
                popularity_change: -2,
                ..Default::default()
            },
        }
    }

    fn merchants_event(state: &GameState) -> GameEvent {
        let mut rng = rand::thread_rng();
        let gold = rng.gen_range(200..600);

        GameEvent {
            title: "Wealthy Merchants".to_string(),
            description: format!(
                "A caravan of wealthy merchants passed through your lands and paid {} gold in tolls.",
                gold
            ),
            effects: EventEffects {
                gold_change: gold,
                ..Default::default()
            },
        }
    }

    fn bandits_event(state: &GameState) -> GameEvent {
        let mut rng = rand::thread_rng();
        let gold_loss = rng.gen_range(100..400);
        let pop_loss = rng.gen_range(20..60);

        GameEvent {
            title: "Bandit Raid!".to_string(),
            description: format!(
                "Bandits raided your villages! Lost {} gold and {} citizens.",
                gold_loss, pop_loss
            ),
            effects: EventEffects {
                gold_change: -gold_loss,
                population_change: -pop_loss,
                popularity_change: -3,
                ..Default::default()
            },
        }
    }

    fn festival_event(state: &GameState) -> GameEvent {
        let mut rng = rand::thread_rng();
        let cost = rng.gen_range(150..350);

        GameEvent {
            title: "Grand Festival".to_string(),
            description: format!(
                "You held a grand festival for your people. Cost {} gold but greatly improved morale!",
                cost
            ),
            effects: EventEffects {
                gold_change: -cost,
                popularity_change: 8,
                ..Default::default()
            },
        }
    }

    fn drought_event(state: &GameState) -> GameEvent {
        let mut rng = rand::thread_rng();
        let food_loss = rng.gen_range(400..1000);

        GameEvent {
            title: "Drought".to_string(),
            description: format!(
                "A severe drought has damaged your crops. Lost {} food.",
                food_loss
            ),
            effects: EventEffects {
                food_change: -food_loss,
                popularity_change: -4,
                ..Default::default()
            },
        }
    }

    fn immigrants_event(state: &GameState) -> GameEvent {
        let mut rng = rand::thread_rng();
        let immigrants = rng.gen_range(80..200);

        GameEvent {
            title: "Immigrants Arrive".to_string(),
            description: format!(
                "Word of your prosperous realm has spread! {} people have immigrated to your lands.",
                immigrants
            ),
            effects: EventEffects {
                population_change: immigrants,
                popularity_change: 2,
                ..Default::default()
            },
        }
    }

    fn gold_mine_event(state: &GameState) -> GameEvent {
        let mut rng = rand::thread_rng();
        let gold = rng.gen_range(400..1000);

        GameEvent {
            title: "Rich Vein Discovered!".to_string(),
            description: format!(
                "Your miners discovered a rich vein of gold! Gained {} gold.",
                gold
            ),
            effects: EventEffects {
                gold_change: gold,
                popularity_change: 1,
                ..Default::default()
            },
        }
    }

    fn storm_event(state: &GameState) -> GameEvent {
        let mut rng = rand::thread_rng();
        let gold_loss = rng.gen_range(200..500);
        let pop_loss = rng.gen_range(10..40);

        GameEvent {
            title: "Terrible Storm".to_string(),
            description: format!(
                "A terrible storm damaged buildings and crops. Lost {} gold and {} citizens.",
                gold_loss, pop_loss
            ),
            effects: EventEffects {
                gold_change: -gold_loss,
                population_change: -pop_loss,
                ..Default::default()
            },
        }
    }

    fn deserters_event(state: &GameState) -> GameEvent {
        if state.soldier_quantity < 10 {
            return Self::good_harvest_event(state);
        }

        let mut rng = rand::thread_rng();
        let deserters = rng.gen_range(5..15).min(state.soldier_quantity);

        GameEvent {
            title: "Deserters".to_string(),
            description: format!(
                "Some soldiers have deserted your army. Lost {} soldiers.",
                deserters
            ),
            effects: EventEffects {
                soldier_change: -deserters,
                popularity_change: -2,
                ..Default::default()
            },
        }
    }

    fn tax_revolt_event(state: &GameState) -> GameEvent {
        let mut rng = rand::thread_rng();
        let gold_loss = rng.gen_range(300..700);

        GameEvent {
            title: "Tax Revolt".to_string(),
            description: format!(
                "Citizens refused to pay taxes in protest! Lost {} gold and popularity.",
                gold_loss
            ),
            effects: EventEffects {
                gold_change: -gold_loss,
                popularity_change: -5,
                ..Default::default()
            },
        }
    }

    fn wedding_event(state: &GameState) -> GameEvent {
        let mut rng = rand::thread_rng();
        let cost = rng.gen_range(200..400);

        GameEvent {
            title: "Royal Wedding".to_string(),
            description: format!(
                "You celebrated a royal wedding! Cost {} gold but the people are overjoyed.",
                cost
            ),
            effects: EventEffects {
                gold_change: -cost,
                popularity_change: 6,
                ..Default::default()
            },
        }
    }

    fn raiders_attack_event(state: &GameState) -> GameEvent {
        let mut rng = rand::thread_rng();

        if state.soldier_quantity > 50 {
            // Successfully defended
            let casualties = rng.gen_range(5..20);
            GameEvent {
                title: "Raiders Repelled!".to_string(),
                description: format!(
                    "Enemy raiders attacked but your army drove them back! Lost {} soldiers but gained prestige.",
                    casualties
                ),
                effects: EventEffects {
                    soldier_change: -casualties,
                    popularity_change: 4,
                    ..Default::default()
                },
            }
        } else {
            // Poor defense
            let gold_loss = rng.gen_range(400..800);
            let pop_loss = rng.gen_range(50..100);
            GameEvent {
                title: "Raiders Attack!".to_string(),
                description: format!(
                    "Your weak defenses allowed raiders to pillage your lands! Lost {} gold and {} citizens.",
                    gold_loss, pop_loss
                ),
                effects: EventEffects {
                    gold_change: -gold_loss,
                    population_change: -pop_loss,
                    popularity_change: -6,
                    ..Default::default()
                },
            }
        }
    }

    fn bumper_crop_event(state: &GameState) -> GameEvent {
        let mut rng = rand::thread_rng();
        let bonus_food = rng.gen_range(800..2000);
        let bonus_gold = rng.gen_range(100..300);

        GameEvent {
            title: "Bumper Crop!".to_string(),
            description: format!(
                "An exceptional year for farming! Gained {} food and sold surplus for {} gold.",
                bonus_food, bonus_gold
            ),
            effects: EventEffects {
                food_change: bonus_food,
                gold_change: bonus_gold,
                popularity_change: 5,
                ..Default::default()
            },
        }
    }
}

impl GameEvent {
    pub fn apply_to_state(&self, state: &mut GameState) {
        state.change_gold(self.effects.gold_change);
        state.change_food(self.effects.food_change);
        state.change_population(self.effects.population_change);
        state.change_popularity(self.effects.popularity_change);
        state.change_soldiers(self.effects.soldier_change);
    }
}
