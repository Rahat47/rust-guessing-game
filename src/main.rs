use std::cmp::Ordering;
use std::fs;
use std::io;
use rand::Rng;
use colored::*;
use console::Term;
use dialoguer::{Input, Select};
use serde::{Serialize, Deserialize};
use serde_json;
use chrono;

// This struct represents a single game result
#[derive(Serialize, Deserialize, Clone)]
struct GameResult {
    attempts: u32,
    lives_remaining: u32,
    won: bool,
    difficulty: String,
    timestamp: String,
}

// This struct holds all our game statistics
#[derive(Serialize, Deserialize)]
struct GameStats {
    total_games: u32,
    games_won: u32,
    games_lost: u32,
    total_attempts: u32,
    best_score: Option<u32>, // lowest attempts for a win
    game_history: Vec<GameResult>,
}

// Implementation block for GameStats
impl GameStats {
    fn new() -> Self {
        GameStats {
            total_games: 0,
            games_won: 0,
            games_lost: 0,
            total_attempts: 0,
            best_score: None,
            game_history: Vec::new(),
        }
    }
}

fn clear_screen() {
    let term = Term::stdout();
    term.clear_screen().unwrap();
}

fn show_game_title() {
    clear_screen();
    
    println!("{}", "=".repeat(60).bright_blue());
    println!("{}", "🎯 THE ULTIMATE GUESSING GAME 🎯".bright_yellow());
    println!("{}", "=".repeat(60).bright_blue());
    println!();
    println!("{}", "Can you find the secret number between 1 and 100?".white());
    println!();
}

fn show_win_screen() {
    clear_screen();

    println!();
    println!("{}", "🎉".repeat(30).bright_green());
    println!("{}", "🏆 CONGRATULATIONS! YOU WIN! 🏆".bright_green());
    println!("{}", "🎉".repeat(30).bright_green());
    println!();
}

fn show_game_over_screen() {
    clear_screen();

    println!();
    println!("{}", "💀".repeat(20).bright_red());
    println!("{}", "💀 GAME OVER! 💀".bright_red());
    println!("{}", "💀".repeat(20).bright_red());
    println!();
}

fn show_difficulty_menu() -> u32 {
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

// Function to load existing statistics from file
fn load_stats() -> GameStats {
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

// Function to save statistics to file
fn save_stats(stats: &GameStats) -> Result<(), io::Error> {
    let json_content = serde_json::to_string_pretty(stats)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    fs::write("game_stats.json", json_content)
}

// Function to get difficulty name from lives count
fn get_difficulty_name(lives: u32) -> String {
    match lives {
        10 => "Easy".to_string(),
        5 => "Medium".to_string(),
        3 => "Hard".to_string(),
        _ => format!("Custom ({})", lives),
    }
}

// Function to display current game statistics
fn show_statistics(stats: &GameStats) {
    println!();
    println!("{}", "📊 GAME STATISTICS 📊".bright_cyan());
    println!("{}", "=".repeat(40).bright_blue());
    
    println!("{}", format!("🎮 Total Games: {}", stats.total_games).white());
    println!("{}", format!("🏆 Games Won: {}", stats.games_won).green());
    println!("{}", format!("💀 Games Lost: {}", stats.games_lost).red());
    
    if stats.total_games > 0 {
        let win_rate = (stats.games_won as f64 / stats.total_games as f64) * 100.0;
        println!("{}", format!("📈 Win Rate: {:.1}%", win_rate).yellow());
        
        let avg_attempts = stats.total_attempts as f64 / stats.total_games as f64;
        println!("{}", format!("📊 Average Attempts: {:.1}", avg_attempts).cyan());
    }
    
    if let Some(best) = stats.best_score {
        println!("{}", format!("🥇 Best Score: {} attempts", best).bright_green());
    } else {
        println!("{}", "🥇 Best Score: No wins yet".bright_yellow());
    }
    
    println!("{}", "=".repeat(40).bright_blue());
}

// Function to show recent game history
fn show_recent_history(stats: &GameStats) {
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

// Function to display and handle the main menu
fn show_main_menu() -> u32 {
    let options = vec!["🎮 Start New Game", "📊 View Statistics", "🔄 Reset Statistics", "🚪 Exit Game"];
    
    let selection = Select::new()
        .with_prompt("🎯 Welcome to The Ultimate Guessing Game!")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();
    
    selection as u32
}

// Function to handle the "View Statistics" option
fn handle_view_stats() {
    let stats = load_stats();
    
    clear_screen();
    show_game_title();
    
    if stats.total_games == 0 {
        println!("{}", "📊 No games played yet! Start a game to see statistics.".yellow());
    } else {
        show_statistics(&stats);
        show_recent_history(&stats);
    }
    
    println!();
    println!("{}", "Press Enter to return to main menu...".bright_cyan());
    let _ = std::io::stdin().read_line(&mut String::new());
}

// Function to handle the "Reset Statistics" option
fn handle_reset_stats() -> bool {
    let options = vec!["❌ No, keep my statistics", "✅ Yes, reset all statistics"];
    
    let selection = Select::new()
        .with_prompt("⚠️  Are you sure you want to reset all statistics?")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();
    
    if selection == 1 {
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

// Function to play a single game
fn play_game() {
    // Load existing statistics
    let mut stats = load_stats();
    
    show_game_title();
    let lives = show_difficulty_menu();
    
    println!("{}", format!("🎮 You have {} lives. Good luck! 🎮", lives).bright_cyan());
    println!("{}", "─".repeat(50).bright_blue());

    let secret_number = rand::rng().random_range(1..=100);
    let mut attempts = 0;
    let mut current_lives = lives;
    let difficulty_name = get_difficulty_name(lives);

    loop {
        attempts += 1;
        println!();
        
        // Create a visual lives indicator with colors
        let lives_display = "❤️ ".repeat(current_lives as usize);
        let lives_text = if current_lives > lives / 2 {
            format!("💖 Lives: {} ({})", lives_display, current_lives).green()
        } else if current_lives > lives / 4 {
            format!("💖 Lives: {} ({})", lives_display, current_lives).yellow()
        } else {
            format!("💖 Lives: {} ({})", lives_display, current_lives).red()
        };
        println!("{}", lives_text);
        
        println!("{}", format!("📊 Attempt: #{}", attempts).cyan());
        println!("{}", "─".repeat(30).bright_blue());
        
        let guess: u32 = Input::<u32>::new()
            .with_prompt("🎲 Enter your guess (1-100)")
            .validate_with(|input: &u32| {
                if *input >= 1 && *input <= 100 {
                    Ok(())
                } else {
                    Err("Please enter a number between 1 and 100")
                }
            })
            .interact()
            .unwrap();

        println!("{}", format!("🎯 You guessed: {}", guess).bright_white());

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                current_lives -= 1; 
                if current_lives > 0 {
                    println!("{}", "📈 Too small! Try a bigger number.".yellow());
                    println!("{}", format!("💡 Hint: The number is higher than {}", guess).blue());
                }
            },
            Ordering::Greater => {
                current_lives -= 1; 
                if current_lives > 0 {
                    println!("{}", "📉 Too big! Try a smaller number.".yellow());
                    println!("{}", format!("💡 Hint: The number is lower than {}", guess).blue());
                }
            },
            Ordering::Equal => {
                show_win_screen();
                println!("{}", format!("🎯 The secret number was: {}", secret_number).bright_green());
                println!("{}", format!("📊 Total attempts: {}", attempts).cyan());
                println!("{}", format!("💖 Lives remaining: {}", current_lives).green());
                println!("{}", "=".repeat(50).bright_green());
                
                // Update statistics for win
                stats.total_games += 1;
                stats.games_won += 1;
                stats.total_attempts += attempts;
                
                // Update best score if this is better
                if let Some(current_best) = stats.best_score {
                    if attempts < current_best {
                        stats.best_score = Some(attempts);
                        println!("{}", "🏆 NEW BEST SCORE! 🏆".bright_green());
                    }
                } else {
                    stats.best_score = Some(attempts);
                    println!("{}", "🏆 FIRST WIN! 🏆".bright_green());
                }
                
                // Add to game history
                let game_result = GameResult {
                    attempts,
                    lives_remaining: current_lives,
                    won: true,
                    difficulty: difficulty_name.clone(),
                    timestamp: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                };
                stats.game_history.push(game_result);
                
                break;
            }
        }

        // Check if player ran out of lives
        if current_lives == 0 {
            show_game_over_screen();
            println!("{}", format!("🎯 The secret number was: {}", secret_number).bright_red());
            println!("{}", format!("📊 Total attempts: {}", attempts).cyan());
            println!("{}", "=".repeat(50).bright_red());
            
            // Update statistics for loss
            stats.total_games += 1;
            stats.games_lost += 1;
            stats.total_attempts += attempts;
            
            // Add to game history
            let game_result = GameResult {
                attempts,
                lives_remaining: current_lives,
                won: false,
                difficulty: difficulty_name.clone(),
                timestamp: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            };
            stats.game_history.push(game_result);
            
            break;
        }
    }
    
    // Save updated statistics
    if let Err(e) = save_stats(&stats) {
        println!("{}", format!("⚠️  Failed to save statistics: {}", e).yellow());
    }
    
    // Show final statistics
    show_statistics(&stats);
    show_recent_history(&stats);
    
    println!();
    println!("{}", "Press Enter to return to main menu...".bright_cyan());
    let _ = std::io::stdin().read_line(&mut String::new());
}

fn main() {
    loop {
        clear_screen();
        show_game_title();
        
        let choice = show_main_menu();
        
        match choice {
            0 => {
                // Start New Game
                play_game();
            },
            1 => {
                // View Statistics
                handle_view_stats();
            },
            2 => {
                // Reset Statistics
                clear_screen();
                show_game_title();
                let _ = handle_reset_stats();
                
                println!();
                println!("{}", "Press Enter to return to main menu...".bright_cyan());
                let _ = std::io::stdin().read_line(&mut String::new());
            },
            3 => {
                // Exit Game
                clear_screen();
                println!();
                println!("{}", "🎮 Thanks for playing The Ultimate Guessing Game! 🎮".bright_cyan());
                println!("{}", "👋 Come back soon! 👋".bright_yellow());
                println!();
                break;
            },
            _ => {
                println!("{}", "Invalid choice! Please try again.".red());
            }
        }
    }
}
