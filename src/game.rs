use std::cmp::Ordering;
use rand::Rng;
use colored::*;
use dialoguer::Input;
use crate::ui::{
    show_game_title, show_win_screen, show_game_over_screen, 
    show_difficulty_menu, show_statistics, show_recent_history, wait_for_enter
};
use crate::stats::{load_stats, save_stats, create_game_result};
use crate::audio;

/// Plays a single game
pub fn play_game() {
    audio::play_start_sound();
    // Load existing statistics
    let mut stats = load_stats();
    
    show_game_title();
    let lives = show_difficulty_menu();
    
    println!("{}", format!("🎮 You have {} lives. Good luck! 🎮", lives).bright_cyan());
    println!("{}", "─".repeat(50).bright_blue());

    let secret_number = rand::rng().random_range(1..=100);
    let mut attempts = 0;
    let mut current_lives = lives;

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
        audio::play_guess_sound();

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
                audio::play_win_sound();
                show_win_screen();
                println!("{}", format!("🎯 The secret number was: {}", secret_number).bright_green());
                println!("{}", format!("📊 Total attempts: {}", attempts).cyan());
                println!("{}", format!("💖 Lives remaining: {}", current_lives).green());
                println!("{}", "=".repeat(50).bright_green());
                
                // Check if this is a new best score
                if let Some(current_best) = stats.best_score {
                    if attempts < current_best {
                        println!("{}", "🏆 NEW BEST SCORE! 🏆".bright_green());
                    }
                } else {
                    println!("{}", "🏆 FIRST WIN! 🏆".bright_green());
                }
                
                // Add game result to statistics
                let game_result = create_game_result(attempts, current_lives, true, lives);
                stats.add_game_result(game_result);
                
                break;
            }
        }

        // Check if player ran out of lives
        if current_lives == 0 {
            audio::play_game_over_sound();
            show_game_over_screen();
            println!("{}", format!("🎯 The secret number was: {}", secret_number).bright_red());
            println!("{}", format!("📊 Total attempts: {}", attempts).cyan());
            println!("{}", "=".repeat(50).bright_red());
            
            // Add game result to statistics
            let game_result = create_game_result(attempts, current_lives, false, lives);
            stats.add_game_result(game_result);
            
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
    wait_for_enter();
} 