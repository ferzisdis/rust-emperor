use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    // Player info
    pub user_name: String,
    pub user_sex: Gender,
    pub user_difficulty: Difficulty,

    // Time
    pub year: u16,
    pub round: u16,

    // Resources
    pub gold: i32,
    pub food_quantity: i32,
    pub man_quantity: i32,
    pub soldier_quantity: i16,
    pub weapon_quantity: i16,
    pub iron_quantity: i16,

    // Buildings
    pub castle_level: u8,
    pub farm_quantity: i16,
    pub mine_quantity: i16,
    pub smithy_quantity: i16,
    pub market_quantity: i16,

    // Prices
    pub price_for_castle: i32,
    pub price_for_farm: i32,
    pub price_for_mine: i32,
    pub price_for_smithy: i32,
    pub price_for_market: i32,
    pub price_for_food: i32,
    pub price_for_armor: i32,
    pub price_for_weapon: i32,
    pub soldier_price: i32,

    // Price rate constants (for price fluctuation)
    pub price_for_food_rate_constant: i16,
    pub price_for_armor_rate_constant: i16,
    pub price_for_weapon_rate_constant: i16,

    // Settings
    pub taxes_level: u8,
    pub food_supply: u8,

    // Stats
    pub popularity_percent: i8,
    pub previous_popularity_percent: i8,
    pub grade: u8,
    pub was_grade_up_before: bool,
    pub is_castle_upgrade_in_this_round: bool,

    // Round results
    pub taxes_value: i32,
    pub market_place_value: i32,
    pub harvest_value: i32,
    pub harvest_percent: i32,

    // Last event (for report display)
    pub last_event_title: Option<String>,
    pub last_event_description: Option<String>,

    // Game state
    pub is_game_ended: bool,
    pub is_won: bool,
    pub trade_limit: i16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Gender {
    Female,
    Male,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl GameState {
    pub fn new(user_name: String, user_sex: Gender, user_difficulty: Difficulty) -> Self {
        let mut state = Self {
            user_name,
            user_sex,
            user_difficulty,
            year: 1440,
            round: 0,
            gold: 2000,
            food_quantity: 2500,
            man_quantity: 1000,
            soldier_quantity: 0,
            weapon_quantity: 0,
            iron_quantity: 0,
            castle_level: 0,
            farm_quantity: 1,
            mine_quantity: 0,
            smithy_quantity: 0,
            market_quantity: 0,
            price_for_castle: 5000,
            price_for_farm: 1000,
            price_for_mine: 3000,
            price_for_smithy: 3000,
            price_for_market: 2000,
            price_for_food: 40,
            price_for_armor: 60,
            price_for_weapon: 150,
            soldier_price: 100,
            price_for_food_rate_constant: 40,
            price_for_armor_rate_constant: 60,
            price_for_weapon_rate_constant: 150,
            taxes_level: 3,
            food_supply: 3,
            popularity_percent: 72,
            previous_popularity_percent: 72,
            grade: 0,
            was_grade_up_before: false,
            is_castle_upgrade_in_this_round: false,
            taxes_value: 0,
            market_place_value: 0,
            harvest_value: 0,
            harvest_percent: 100,
            last_event_title: None,
            last_event_description: None,
            is_game_ended: false,
            is_won: false,
            trade_limit: 20000,
        };

        state.apply_difficulty_modifier();
        state
    }

    fn apply_difficulty_modifier(&mut self) {
        match self.user_difficulty {
            Difficulty::Easy => {
                // No modifications for easy
            }
            Difficulty::Medium => {
                self.food_quantity -= 800;
                self.man_quantity -= 250;
                self.popularity_percent -= 5;
                self.gold -= 500;
                self.year += 5;
            }
            Difficulty::Hard => {
                self.food_quantity -= 1600;
                self.man_quantity -= 500;
                self.popularity_percent -= 10;
                self.gold -= 1000;
                self.farm_quantity -= 1;
                self.year += 10;
            }
        }
    }

    pub fn get_grade_title(&self) -> &'static str {
        let grades = match self.user_sex {
            Gender::Female => [
                "Baroness", "Countess", "Duchess", "Princess", "Queen", "Empress",
            ],
            Gender::Male => ["Baron", "Count", "Duke", "Prince", "King", "Emperor"],
        };
        grades[self.grade as usize]
    }

    pub fn get_next_grade_title(&self) -> &'static str {
        let grades = match self.user_sex {
            Gender::Female => [
                "Baroness", "Countess", "Duchess", "Princess", "Queen", "Empress",
            ],
            Gender::Male => ["Baron", "Count", "Duke", "Prince", "King", "Emperor"],
        };
        let next_grade = (self.grade + 1).min(5) as usize;
        grades[next_grade]
    }

    pub fn change_gold(&mut self, amount: i32) -> i32 {
        self.gold += amount;
        if self.gold < 0 {
            let deficit = -self.gold;
            self.gold = 0;
            deficit
        } else {
            amount.abs()
        }
    }

    pub fn change_food(&mut self, amount: i32) -> i32 {
        self.food_quantity += amount;
        if self.food_quantity < 0 {
            let deficit = -self.food_quantity;
            self.food_quantity = 0;
            deficit
        } else {
            amount.abs()
        }
    }

    pub fn change_population(&mut self, amount: i32) -> i32 {
        self.man_quantity += amount;
        if self.man_quantity < 0 {
            let deficit = -self.man_quantity;
            self.man_quantity = 0;
            deficit
        } else {
            amount.abs()
        }
    }

    pub fn change_popularity(&mut self, amount: i8) -> i8 {
        self.popularity_percent += amount;
        if self.popularity_percent < 0 {
            self.popularity_percent = 0;
        } else if self.popularity_percent > 100 {
            self.popularity_percent = 100;
        }
        amount.abs()
    }

    pub fn change_soldiers(&mut self, amount: i16) -> i16 {
        self.soldier_quantity += amount;
        if self.soldier_quantity < 0 {
            let deficit = -self.soldier_quantity;
            self.soldier_quantity = 0;
            deficit
        } else {
            amount.abs()
        }
    }

    pub fn can_build_farm(&self) -> bool {
        self.gold >= self.price_for_farm as i32
    }

    pub fn can_build_mine(&self) -> bool {
        self.gold >= self.price_for_mine as i32
    }

    pub fn can_build_smithy(&self) -> bool {
        self.gold >= self.price_for_smithy as i32
    }

    pub fn can_build_market(&self) -> bool {
        self.gold >= self.price_for_market as i32
    }

    pub fn can_upgrade_castle(&self) -> bool {
        self.castle_level < 8
            && self.gold >= self.price_for_castle as i32
            && !self.is_castle_upgrade_in_this_round
    }

    pub fn build_farm(&mut self) -> Result<(), String> {
        if !self.can_build_farm() {
            return Err("Not enough gold!".to_string());
        }
        self.gold -= self.price_for_farm as i32;
        self.farm_quantity += 1;
        Ok(())
    }

    pub fn build_mine(&mut self) -> Result<(), String> {
        if !self.can_build_mine() {
            return Err("Not enough gold!".to_string());
        }
        self.gold -= self.price_for_mine as i32;
        self.mine_quantity += 1;
        Ok(())
    }

    pub fn build_smithy(&mut self) -> Result<(), String> {
        if !self.can_build_smithy() {
            return Err("Not enough gold!".to_string());
        }
        self.gold -= self.price_for_smithy as i32;
        self.smithy_quantity += 1;
        Ok(())
    }

    pub fn build_market(&mut self) -> Result<(), String> {
        if !self.can_build_market() {
            return Err("Not enough gold!".to_string());
        }
        self.gold -= self.price_for_market as i32;
        self.market_quantity += 1;
        Ok(())
    }

    pub fn upgrade_castle(&mut self) -> Result<(), String> {
        if !self.can_upgrade_castle() {
            return Err("Cannot upgrade castle!".to_string());
        }
        self.gold -= self.price_for_castle as i32;
        self.castle_level += 1;
        self.is_castle_upgrade_in_this_round = true;
        Ok(())
    }

    pub fn get_grade_requirements(&self) -> Vec<(String, i32, i32)> {
        let requirements = match self.grade {
            0 => vec![
                ("Citizens".to_string(), self.man_quantity, 1400),
                ("Popularity".to_string(), self.popularity_percent as i32, 65),
                ("Castle Level".to_string(), self.castle_level as i32, 0),
                ("Soldiers".to_string(), self.soldier_quantity as i32, 0),
                ("Gold".to_string(), self.gold, 0),
            ],
            1 => vec![
                ("Citizens".to_string(), self.man_quantity, 2000),
                ("Popularity".to_string(), self.popularity_percent as i32, 70),
                ("Castle Level".to_string(), self.castle_level as i32, 1),
                ("Soldiers".to_string(), self.soldier_quantity as i32, 10),
                ("Gold".to_string(), self.gold, 0),
            ],
            2 => vec![
                ("Citizens".to_string(), self.man_quantity, 3000),
                ("Popularity".to_string(), self.popularity_percent as i32, 75),
                ("Castle Level".to_string(), self.castle_level as i32, 2),
                ("Soldiers".to_string(), self.soldier_quantity as i32, 25),
                ("Gold".to_string(), self.gold, 0),
            ],
            3 => vec![
                ("Citizens".to_string(), self.man_quantity, 5000),
                ("Popularity".to_string(), self.popularity_percent as i32, 80),
                ("Castle Level".to_string(), self.castle_level as i32, 6),
                ("Soldiers".to_string(), self.soldier_quantity as i32, 200),
                ("Gold".to_string(), self.gold, 100000),
            ],
            4 => vec![
                ("Citizens".to_string(), self.man_quantity, 10000),
                ("Popularity".to_string(), self.popularity_percent as i32, 90),
                ("Castle Level".to_string(), self.castle_level as i32, 8),
                ("Soldiers".to_string(), self.soldier_quantity as i32, 500),
                ("Gold".to_string(), self.gold, 1000000),
            ],
            _ => vec![],
        };

        requirements
    }

    pub fn check_can_advance_grade(&self) -> bool {
        if self.grade >= 5 {
            return false;
        }

        let requirements = self.get_grade_requirements();
        requirements
            .iter()
            .all(|(_, current, required)| current >= required)
    }

    pub fn finish_round(&mut self) -> bool {
        self.round += 1;
        self.year += 1;

        if self.castle_level < 8 {
            self.is_castle_upgrade_in_this_round = false;
        }

        if self.year > 1500 {
            self.is_game_ended = true;
            self.is_won = false;
            return false;
        }

        if self.was_grade_up_before {
            self.was_grade_up_before = false;
            return false;
        }

        if self.check_can_advance_grade() {
            self.grade += 1;
            self.was_grade_up_before = true;

            if self.grade == 5 {
                self.is_game_ended = true;
                self.is_won = true;
            }

            return true;
        }

        false
    }

    pub fn calculate_score(&self) -> i32 {
        if !self.is_won {
            return 0;
        }

        let mut score = 0;
        score += (1501 - self.year as i32) * 100;
        score += match self.user_difficulty {
            Difficulty::Easy => 0,
            Difficulty::Medium => 1200,
            Difficulty::Hard => 2400,
        };
        score += self.man_quantity / 50;

        score
    }

    pub fn buy_food(&mut self, quantity: i32) -> Result<(), String> {
        if self.market_quantity == 0 {
            return Err("No markets available!".to_string());
        }

        let cost = (quantity / 100) * self.price_for_food as i32;
        if self.gold < cost {
            return Err("Not enough gold!".to_string());
        }

        self.gold -= cost;
        self.food_quantity += quantity;
        Ok(())
    }

    pub fn sell_food(&mut self, quantity: i32) -> Result<(), String> {
        if self.market_quantity == 0 {
            return Err("No markets available!".to_string());
        }

        if self.food_quantity < quantity {
            return Err("Not enough food!".to_string());
        }

        let price = (quantity / 100) * self.price_for_food as i32;
        self.food_quantity -= quantity;
        self.gold += price;
        Ok(())
    }

    pub fn buy_iron(&mut self, quantity: i16) -> Result<(), String> {
        if self.market_quantity <= 4 {
            return Err("Need more than 4 markets to trade iron!".to_string());
        }

        let cost = quantity as i32 * self.price_for_armor as i32;
        if self.gold < cost {
            return Err("Not enough gold!".to_string());
        }

        if (self.iron_quantity as i32 + quantity as i32) > self.trade_limit as i32 {
            return Err("Trade limit reached!".to_string());
        }

        self.gold -= cost;
        self.iron_quantity += quantity;
        Ok(())
    }

    pub fn sell_iron(&mut self, quantity: i16) -> Result<(), String> {
        if self.market_quantity <= 4 {
            return Err("Need more than 4 markets to trade iron!".to_string());
        }

        if self.iron_quantity < quantity {
            return Err("Not enough iron!".to_string());
        }

        let cost = quantity as i32 * self.price_for_armor as i32;
        self.gold += cost;
        self.iron_quantity -= quantity;
        Ok(())
    }

    pub fn buy_weapons(&mut self, quantity: i16) -> Result<(), String> {
        if self.market_quantity <= 9 {
            return Err("Need more than 9 markets to trade weapons!".to_string());
        }

        let cost = quantity as i32 * self.price_for_weapon as i32;
        if self.gold < cost {
            return Err("Not enough gold!".to_string());
        }

        if (self.weapon_quantity as i32 + quantity as i32) > self.trade_limit as i32 {
            return Err("Trade limit reached!".to_string());
        }

        self.gold -= cost;
        self.weapon_quantity += quantity;
        Ok(())
    }

    pub fn sell_weapons(&mut self, quantity: i16) -> Result<(), String> {
        if self.market_quantity <= 9 {
            return Err("Need more than 9 markets to trade weapons!".to_string());
        }

        if self.weapon_quantity < quantity {
            return Err("Not enough weapons!".to_string());
        }

        let cost = quantity as i32 * self.price_for_weapon as i32;
        self.gold += cost;
        self.weapon_quantity -= quantity;
        Ok(())
    }

    pub fn recruit_soldiers(&mut self, quantity: i16) -> Result<(), String> {
        if quantity <= 0 {
            return Err("Quantity must be positive!".to_string());
        }

        let cost = quantity as i32 * self.soldier_price as i32;
        if self.gold < cost {
            return Err("Not enough gold!".to_string());
        }

        if self.weapon_quantity < quantity {
            return Err("Not enough weapons!".to_string());
        }

        if self.man_quantity - 200 < self.soldier_quantity as i32 + quantity as i32 {
            return Err("Not enough citizens! (Must keep at least 200 citizens)".to_string());
        }

        if (self.soldier_quantity as i32 + quantity as i32) > self.trade_limit as i32 {
            return Err("Trade limit reached!".to_string());
        }

        self.gold -= cost;
        self.weapon_quantity -= quantity;
        self.man_quantity -= quantity as i32;
        self.soldier_quantity += quantity;
        Ok(())
    }

    pub fn discharge_soldiers(&mut self, quantity: i16) -> Result<(), String> {
        if quantity <= 0 {
            return Err("Quantity must be positive!".to_string());
        }

        if self.soldier_quantity < quantity {
            return Err("Not enough soldiers!".to_string());
        }

        let refund = quantity as i32 * self.soldier_price as i32;

        self.gold += refund;
        self.weapon_quantity += quantity;
        self.man_quantity += quantity as i32;
        self.soldier_quantity -= quantity;
        Ok(())
    }

    pub fn can_afford_soldier(&self) -> bool {
        self.gold >= self.soldier_price as i32
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new("Player".to_string(), Gender::Male, Difficulty::Easy)
    }
}
