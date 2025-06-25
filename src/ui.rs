use colored::*;
use dialoguer::{Input, Select};
use crate::models::GameStats;
use crate::utils::clear_screen;

/// Displays the game title screen
pub fn show_game_title() {
    clear_screen();
    
    println!("{}", "=".repeat(60).bright_blue());
    println!("{}", "🎯 THE ULTIMATE GUESSING GAME 🎯".bright_yellow());
    println!("{}", "=".repeat(60).bright_blue());
    println!();
    println!("{}", "Can you find the secret number between 1 and 100?".white());
    println!();
}

/// Displays the win screen
pub fn show_win_screen() {
    clear_screen();

    println!();
    println!("{}", "🎉".repeat(30).bright_green());
    println!("{}", "🏆 CONGRATULATIONS! YOU WIN! 🏆".bright_green());
    println!("{}", "🎉".repeat(30).bright_green());
    println!();
}

/// Displays the game over screen
pub fn show_game_over_screen() {
    clear_screen();

    println!();
    println!("{}", "💀".repeat(20).bright_red());
    println!("{}", "💀 GAME OVER! 💀".bright_red());
    println!("{}", "💀".repeat(20).bright_red());
    println!();
}

/// Shows the difficulty selection menu
pub fn show_difficulty_menu() -> u32 {
    let options = vec!["Easy (10 lives)", "Medium (5 lives)", "Hard (3 lives)", "Custom"];
    
    let selection = Select::new()
        .with_prompt("🎯 Choose your difficulty level")
        .items(&options)
        .default(1)
        .interact()
        .unwrap();
    
    match selection {
        0 => 10,
        1 => 5,
        2 => 3,
        3 => {
            Input::<u32>::new()
                .with_prompt("💭 How many lives do you want?")
                .validate_with(|input: &u32| {
                    if *input > 0 && *input <= 99 {
                        Ok(())
                    } else {
                        Err("Please enter a number between 1 and 99")
                    }
                })
                .interact()
                .unwrap()
        }
        _ => 5
    }
}

/// Shows the main menu and returns user selection
pub fn show_main_menu() -> u32 {
    let options = vec!["🎮 Start New Game", "📊 View Statistics", "🔄 Reset Statistics", "🚪 Exit Game"];
    
    let selection = Select::new()
        .with_prompt("🎯 Welcome to The Ultimate Guessing Game!")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();
    
    selection as u32
}

/// Displays current game statistics
pub fn show_statistics(stats: &GameStats) {
    println!();
    println!("{}", "📊 GAME STATISTICS 📊".bright_cyan());
    println!("{}", "=".repeat(40).bright_blue());
    
    println!("{}", format!("🎮 Total Games: {}", stats.total_games).white());
    println!("{}", format!("🏆 Games Won: {}", stats.games_won).green());
    println!("{}", format!("💀 Games Lost: {}", stats.games_lost).red());
    
    if stats.total_games > 0 {
        println!("{}", format!("📈 Win Rate: {:.1}%", stats.win_rate()).yellow());
        println!("{}", format!("📊 Average Attempts: {:.1}", stats.average_attempts()).cyan());
    }
    
    if let Some(best) = stats.best_score {
        println!("{}", format!("🥇 Best Score: {} attempts", best).bright_green());
    } else {
        println!("{}", "🥇 Best Score: No wins yet".bright_yellow());
    }
    
    println!("{}", "=".repeat(40).bright_blue());
}

/// Shows recent game history
pub fn show_recent_history(stats: &GameStats) {
    if stats.game_history.is_empty() {
        return;
    }
    
    println!();
    println!("{}", "📜 RECENT GAMES 📜".bright_magenta());
    println!("{}", "─".repeat(50).bright_blue());
    
    // Show last 5 games
    let recent_games = stats.game_history.iter().rev().take(5);
    
    for (i, game) in recent_games.enumerate() {
        let result_icon = if game.won { "🏆" } else { "💀" };
        
        println!("{}. {} {} - {} attempts, {} lives left ({})", 
            i + 1,
            result_icon,
            if game.won { "WON" } else { "LOST" },
            game.attempts,
            game.lives_remaining,
            game.difficulty
        );
    }
    
    println!("{}", "─".repeat(50).bright_blue());
}

/// Shows confirmation dialog for resetting statistics
pub fn show_reset_confirmation() -> bool {
    let options = vec!["❌ No, keep my statistics", "✅ Yes, reset all statistics"];
    
    let selection = Select::new()
        .with_prompt("⚠️  Are you sure you want to reset all statistics?")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();
    
    selection == 1
}

/// Waits for user to press Enter
pub fn wait_for_enter() {
    println!("{}", "Press Enter to continue...".bright_cyan());
    let _ = std::io::stdin().read_line(&mut String::new());
} 