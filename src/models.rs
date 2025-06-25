use serde::{Serialize, Deserialize};

/// Represents a single game result
#[derive(Serialize, Deserialize, Clone)]
pub struct GameResult {
    pub attempts: u32,
    pub lives_remaining: u32,
    pub won: bool,
    pub difficulty: String,
    pub timestamp: String,
}

/// Holds all game statistics
#[derive(Serialize, Deserialize)]
pub struct GameStats {
    pub total_games: u32,
    pub games_won: u32,
    pub games_lost: u32,
    pub total_attempts: u32,
    pub best_score: Option<u32>, // lowest attempts for a win
    pub game_history: Vec<GameResult>,
}

impl GameStats {
    /// Creates a new GameStats instance with default values
    pub fn new() -> Self {
        GameStats {
            total_games: 0,
            games_won: 0,
            games_lost: 0,
            total_attempts: 0,
            best_score: None,
            game_history: Vec::new(),
        }
    }
    
    /// Adds a game result to the statistics
    pub fn add_game_result(&mut self, result: GameResult) {
        self.total_games += 1;
        self.total_attempts += result.attempts;
        
        if result.won {
            self.games_won += 1;
            // Update best score if this is better
            if let Some(current_best) = self.best_score {
                if result.attempts < current_best {
                    self.best_score = Some(result.attempts);
                }
            } else {
                self.best_score = Some(result.attempts);
            }
        } else {
            self.games_lost += 1;
        }
        
        self.game_history.push(result);
    }
    
    /// Calculates the win rate as a percentage
    pub fn win_rate(&self) -> f64 {
        if self.total_games == 0 {
            0.0
        } else {
            (self.games_won as f64 / self.total_games as f64) * 100.0
        }
    }
    
    /// Calculates the average attempts per game
    pub fn average_attempts(&self) -> f64 {
        if self.total_games == 0 {
            0.0
        } else {
            self.total_attempts as f64 / self.total_games as f64
        }
    }
} 