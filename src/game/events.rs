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
    pub weapon_change: i16,
    pub farm_change: i16,
}

pub struct EventGenerator;

impl EventGenerator {
    /// Generates a random event based on the original game logic
    /// Parameters are calculated as in GameCanvas.ShowRandomEvent():
    /// - food_param: random(foodQuantity * 30 / 100) + 200
    /// - gold_param: random(gold * 40 / 100) + 50
    /// - man_param: random(manQuantity * 9 / 100) + 5
    /// - soldier_param: random(soldierQuantity * 10 / 100) + 2
    /// - farm_param: random(2) + 2
    pub fn generate_random_event(state: &GameState) -> Option<GameEvent> {
        let mut rng = rand::thread_rng();

        // Calculate parameters as in original game
        let food_param = rng.gen_range(0..(state.food_quantity * 30 / 100).max(1)) + 200;
        let gold_param = rng.gen_range(0..(state.gold * 40 / 100).max(1)) + 50;
        let man_param = (rng.gen_range(0..(state.man_quantity * 9 / 100).max(1)) + 5) as i16;
        let soldier_param =
            (rng.gen_range(0..(state.soldier_quantity * 10 / 100).max(1)) + 2) as i16;
        let farm_param = (rng.gen_range(0..3) + 2) as i16;

        // Choose random event (0-18, but we use 0-15 to avoid some edge cases)
        let event_id = rng.gen_range(0..19);

        Self::generate_event(
            event_id,
            state,
            farm_param,
            man_param,
            soldier_param,
            food_param,
            gold_param,
        )
    }

    fn generate_event(
        event_id: usize,
        state: &GameState,
        farm_param: i16,
        man_param: i16,
        soldier_param: i16,
        food_param: i32,
        gold_param: i32,
    ) -> Option<GameEvent> {
        match event_id {
            0 => Self::event_plague_rats(state, food_param),
            1 => Self::event_accountant_flees(state, gold_param),
            2 => Self::event_gold_transporter_robbed(state, gold_param),
            3 => Self::event_epidemic(state, man_param),
            4 => Self::event_refugees(state, man_param),
            5 => Self::event_fire_farms(state, farm_param),
            6 => Self::event_thieves_plundering(state, man_param),
            7 => Self::event_weapons_found(state, soldier_param),
            8 => Self::event_soldiers_flee(state, soldier_param),
            9 => Self::event_joyous_festival(state, gold_param),
            10 => Self::event_warehouse_fire(state, food_param),
            11 => Self::event_pay_soldiers(state),
            12 => Self::event_thieves_steal_gold(state, gold_param, man_param, soldier_param),
            13 => Self::event_give_to_poor(state, gold_param),
            14 => Self::event_warehouse_renovation(state, gold_param),
            15 => Self::event_battle_casualties(state, soldier_param, man_param),
            16 => Self::event_deserters(state, soldier_param),
            17 => Self::event_dark_emperor_attack(state, man_param),
            18 => Self::event_catch_thieves(state, man_param),
            _ => None,
        }
    }

    // Event 0: A plague of rats causes you to lose * food rations
    fn event_plague_rats(state: &GameState, food_param: i32) -> Option<GameEvent> {
        let food_loss = food_param.min(state.food_quantity);

        if food_loss < 3 {
            return None; // Not enough food to lose
        }

        Some(GameEvent {
            title: "Plague of Rats".to_string(),
            description: format!(
                "A plague of rats causes you to lose {} food rations.",
                food_loss
            ),
            effects: EventEffects {
                food_change: -food_loss,
                ..Default::default()
            },
        })
    }

    // Event 1: One of your accountants takes * pieces of gold and flees
    fn event_accountant_flees(state: &GameState, gold_param: i32) -> Option<GameEvent> {
        let gold_loss = gold_param.min(state.gold);

        if gold_loss == 0 {
            return None;
        }

        Some(GameEvent {
            title: "Accountant Flees".to_string(),
            description: format!(
                "One of your accountants takes {} pieces of gold and flees.",
                gold_loss
            ),
            effects: EventEffects {
                gold_change: -gold_loss,
                ..Default::default()
            },
        })
    }

    // Event 2: One of your gold transporter is robbed and * pieces of gold are stolen
    fn event_gold_transporter_robbed(state: &GameState, gold_param: i32) -> Option<GameEvent> {
        let gold_loss = gold_param.min(state.gold);

        if gold_loss == 0 {
            return None;
        }

        Some(GameEvent {
            title: "Gold Transporter Robbed".to_string(),
            description: format!(
                "One of your gold transporter is robbed and {} pieces of gold are stolen.",
                gold_loss
            ),
            effects: EventEffects {
                gold_change: -gold_loss,
                ..Default::default()
            },
        })
    }

    // Event 3: An epidemic kills * inhabitants
    fn event_epidemic(state: &GameState, man_param: i16) -> Option<GameEvent> {
        if state.man_quantity < 10 || man_param < 10 {
            return None;
        }

        let population_loss = man_param.min(state.man_quantity as i16);

        Some(GameEvent {
            title: "Epidemic".to_string(),
            description: format!("An epidemic kills {} inhabitants.", population_loss),
            effects: EventEffects {
                population_change: -(population_loss as i32),
                ..Default::default()
            },
        })
    }

    // Event 4: A wave of * refugees arrives in your country
    fn event_refugees(_state: &GameState, man_param: i16) -> Option<GameEvent> {
        Some(GameEvent {
            title: "Refugees Arrive".to_string(),
            description: format!("A wave of {} refugees arrives in your country.", man_param),
            effects: EventEffects {
                population_change: man_param as i32,
                ..Default::default()
            },
        })
    }

    // Event 5: Because of a major fire * farms are destroyed
    fn event_fire_farms(state: &GameState, farm_param: i16) -> Option<GameEvent> {
        if farm_param > state.farm_quantity {
            return None;
        }

        Some(GameEvent {
            title: "Major Fire".to_string(),
            description: format!(
                "Because of a major fire {} farms are destroyed.",
                farm_param
            ),
            effects: EventEffects {
                farm_change: -farm_param,
                ..Default::default()
            },
        })
    }

    // Event 6: Thieves are plundering your villages. Your people require more protection.
    fn event_thieves_plundering(state: &GameState, man_param: i16) -> Option<GameEvent> {
        // If enough soldiers (soldierQuantity * 50 >= manQuantity), redirect to event 18
        if state.soldier_quantity * 50 >= state.man_quantity as i16 {
            return Self::event_catch_thieves(state, man_param);
        }

        Some(GameEvent {
            title: "Thieves Plundering".to_string(),
            description:
                "Thieves are plundering your villages. Your people require more protection."
                    .to_string(),
            effects: EventEffects {
                popularity_change: -8,
                ..Default::default()
            },
        })
    }

    // Event 7: In a secret hiding place your soldiers find * weapons
    fn event_weapons_found(state: &GameState, soldier_param: i16) -> Option<GameEvent> {
        if state.soldier_quantity <= 2 {
            return None;
        }

        Some(GameEvent {
            title: "Weapons Found".to_string(),
            description: format!(
                "In a secret hiding place your soldiers find {} weapons.",
                soldier_param
            ),
            effects: EventEffects {
                weapon_change: soldier_param,
                ..Default::default()
            },
        })
    }

    // Event 8: According to rumor the Dark Emperor controls an enormous army. * Of your soldiers flee.
    fn event_soldiers_flee(state: &GameState, soldier_param: i16) -> Option<GameEvent> {
        if state.soldier_quantity < 2 || soldier_param < 2 {
            return None;
        }

        let soldier_loss = soldier_param.min(state.soldier_quantity);

        Some(GameEvent {
            title: "Soldiers Flee".to_string(),
            description: format!("According to rumor the Dark Emperor controls an enormous army. {} of your soldiers flee.", soldier_loss),
            effects: EventEffects {
                soldier_change: -soldier_loss,
                ..Default::default()
            },
        })
    }

    // Event 9: You celebrate a joyous festival for * pieces of gold
    fn event_joyous_festival(state: &GameState, gold_param: i32) -> Option<GameEvent> {
        let gold_cost = gold_param.min(state.gold);

        if gold_cost == 0 {
            return None;
        }

        Some(GameEvent {
            title: "Joyous Festival".to_string(),
            description: format!(
                "You celebrate a joyous festival for {} pieces of gold.",
                gold_cost
            ),
            effects: EventEffects {
                gold_change: -gold_cost,
                ..Default::default()
            },
        })
    }

    // Event 10: Fire in your warehouse. * food rations burn
    fn event_warehouse_fire(state: &GameState, food_param: i32) -> Option<GameEvent> {
        let food_loss = food_param.min(state.food_quantity);

        if food_loss < 3 {
            return None;
        }

        Some(GameEvent {
            title: "Warehouse Fire".to_string(),
            description: format!("Fire in your warehouse. {} food rations burn.", food_loss),
            effects: EventEffects {
                food_change: -food_loss,
                ..Default::default()
            },
        })
    }

    // Event 11: To increase moral, you pay 10 pieces of gold to each soldier
    fn event_pay_soldiers(state: &GameState) -> Option<GameEvent> {
        if state.soldier_quantity < 2 {
            return None;
        }

        let total_cost = state.soldier_quantity as i32 * 10;

        // If not enough gold, redirect to event 16 (deserters)
        if total_cost >= state.gold {
            let soldier_param = (state.soldier_quantity * 10 / 100).max(2);
            return Self::event_deserters(state, soldier_param);
        }

        Some(GameEvent {
            title: "Pay Soldiers".to_string(),
            description: format!("To increase moral, you pay 10 pieces of gold to each soldier."),
            effects: EventEffects {
                gold_change: -total_cost,
                ..Default::default()
            },
        })
    }

    // Event 12: Thieves steal * pieces of gold out of your treasure chamber
    fn event_thieves_steal_gold(
        state: &GameState,
        gold_param: i32,
        man_param: i16,
        _soldier_param: i16,
    ) -> Option<GameEvent> {
        // If enough soldiers (> 100), redirect to event 18
        if state.soldier_quantity > 100 {
            return Self::event_catch_thieves(state, man_param);
        }

        let gold_loss = gold_param.min(state.gold);

        if gold_loss == 0 {
            return None;
        }

        Some(GameEvent {
            title: "Thieves Steal Gold".to_string(),
            description: format!(
                "Thieves steal {} pieces of gold out of your treasure chamber.",
                gold_loss
            ),
            effects: EventEffects {
                gold_change: -gold_loss,
                ..Default::default()
            },
        })
    }

    // Event 13: To increase your popularity, you give out * pieces of gold to the poor
    fn event_give_to_poor(state: &GameState, gold_param: i32) -> Option<GameEvent> {
        if state.gold < 10 || state.man_quantity < 10 {
            return None;
        }

        let gold_given = gold_param.min(state.gold);
        let popularity_gain = if gold_given < 500 { 2 } else { 5 };

        Some(GameEvent {
            title: "Give to Poor".to_string(),
            description: format!(
                "To increase your popularity, you give out {} pieces of gold to the poor.",
                gold_given
            ),
            effects: EventEffects {
                gold_change: -gold_given,
                popularity_change: popularity_gain,
                ..Default::default()
            },
        })
    }

    // Event 14: Your warehouse must be renovated for * pieces of gold
    fn event_warehouse_renovation(state: &GameState, gold_param: i32) -> Option<GameEvent> {
        let gold_cost = gold_param.min(state.gold);

        if gold_cost == 0 {
            return None;
        }

        Some(GameEvent {
            title: "Warehouse Renovation".to_string(),
            description: format!(
                "Your warehouse must be renovated for {} pieces of gold.",
                gold_cost
            ),
            effects: EventEffects {
                gold_change: -gold_cost,
                ..Default::default()
            },
        })
    }

    // Event 15: At a fight against the Dark Emperor's army, * soldiers are killed
    fn event_battle_casualties(
        state: &GameState,
        soldier_param: i16,
        man_param: i16,
    ) -> Option<GameEvent> {
        // If not enough soldiers (< 10), redirect to event 17
        if state.soldier_quantity < 10 {
            return Self::event_dark_emperor_attack(state, man_param);
        }

        let soldier_loss = soldier_param.min(state.soldier_quantity);

        Some(GameEvent {
            title: "Battle Casualties".to_string(),
            description: format!(
                "At a fight against the Dark Emperor's army, {} soldiers are killed.",
                soldier_loss
            ),
            effects: EventEffects {
                soldier_change: -soldier_loss,
                weapon_change: -soldier_loss, // Also lose weapons (based on gameStorage.b() call)
                ..Default::default()
            },
        })
    }

    // Event 16: * of your soldiers desert the army
    fn event_deserters(state: &GameState, soldier_param: i16) -> Option<GameEvent> {
        if state.soldier_quantity < 2 || soldier_param < 2 {
            return None;
        }

        let deserters = soldier_param.min(state.soldier_quantity);

        Some(GameEvent {
            title: "Deserters".to_string(),
            description: format!("{} of your soldiers desert the army.", deserters),
            effects: EventEffects {
                soldier_change: -deserters,
                ..Default::default()
            },
        })
    }

    // Event 17: The Dark Emperor's soldiers kill * inhabitants. Your people are terrified.
    fn event_dark_emperor_attack(state: &GameState, man_param: i16) -> Option<GameEvent> {
        if state.man_quantity < 2 || man_param < 2 {
            return None;
        }

        let population_loss = man_param.min(state.man_quantity as i16);

        Some(GameEvent {
            title: "Dark Emperor Attack".to_string(),
            description: format!(
                "The Dark Emperor's soldiers kill {} inhabitants. Your people are terrified.",
                population_loss
            ),
            effects: EventEffects {
                population_change: -(population_loss as i32),
                popularity_change: -12,
                ..Default::default()
            },
        })
    }

    // Event 18: Your soldiers catch a band of thieves and retrieve * pieces of gold
    fn event_catch_thieves(_state: &GameState, man_param: i16) -> Option<GameEvent> {
        let gold_gained = man_param as i32 * 7;

        Some(GameEvent {
            title: "Thieves Caught".to_string(),
            description: format!(
                "Your soldiers catch a band of thieves and retrieve {} pieces of gold.",
                gold_gained
            ),
            effects: EventEffects {
                gold_change: gold_gained,
                popularity_change: 3,
                ..Default::default()
            },
        })
    }
}

impl GameEvent {
    pub fn apply_to_state(&self, state: &mut GameState) {
        state.change_gold(self.effects.gold_change);
        state.change_food(self.effects.food_change);
        state.change_population(self.effects.population_change);
        state.change_popularity(self.effects.popularity_change);
        state.change_soldiers(self.effects.soldier_change);

        // Handle farm changes
        if self.effects.farm_change != 0 {
            state.farm_quantity =
                (state.farm_quantity as i32 + self.effects.farm_change as i32).max(0) as i16;
        }

        // Handle weapon changes
        if self.effects.weapon_change != 0 {
            state.weapon_quantity =
                (state.weapon_quantity as i32 + self.effects.weapon_change as i32).max(0) as i16;
        }
    }
}
