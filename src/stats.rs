use std::fs;
use std::io;
use serde_json;
use chrono;
use colored::*;
use crate::models::{GameStats, GameResult};
use crate::utils::get_difficulty_name;

/// Loads existing statistics from file
pub fn load_stats() -> GameStats {
    match fs::read_to_string("game_stats.json") {
        Ok(content) => {
            // Try to parse the JSON content
            match serde_json::from_str(&content) {
                Ok(stats) => stats,
                Err(_) => {
                    println!("{}", "⚠️  Corrupted stats file, starting fresh".yellow());
                    GameStats::new()
                }
            }
        }
        Err(_) => {
            // File doesn't exist, create new stats
            GameStats::new()
        }
    }
}

/// Saves statistics to file
pub fn save_stats(stats: &GameStats) -> Result<(), io::Error> {
    let json_content = serde_json::to_string_pretty(stats)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    fs::write("game_stats.json", json_content)
}

/// Creates a new game result
pub fn create_game_result(
    attempts: u32,
    lives_remaining: u32,
    won: bool,
    lives: u32,
) -> GameResult {
    GameResult {
        attempts,
        lives_remaining,
        won,
        difficulty: get_difficulty_name(lives),
        timestamp: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    }
}

/// Handles the "View Statistics" menu option
pub fn handle_view_stats() {
    let stats = load_stats();
    
    use crate::ui::{show_game_title, show_statistics, show_recent_history, wait_for_enter};
    use crate::utils::clear_screen;
    
    clear_screen();
    show_game_title();
    
    if stats.total_games == 0 {
        println!("{}", "📊 No games played yet! Start a game to see statistics.".yellow());
    } else {
        show_statistics(&stats);
        show_recent_history(&stats);
    }
    
    println!();
    wait_for_enter();
}

/// Handles the "Reset Statistics" menu option
pub fn handle_reset_stats() -> bool {
    use crate::ui::show_reset_confirmation;
    
    if show_reset_confirmation() {
        // User confirmed reset
        let new_stats = GameStats::new();
        if let Err(e) = save_stats(&new_stats) {
            println!("{}", format!("❌ Failed to reset statistics: {}", e).red());
            return false;
        }
        println!("{}", "✅ Statistics have been reset!".green());
        true
    } else {
        println!("{}", "✅ Statistics kept unchanged.".green());
        false
    }
} 