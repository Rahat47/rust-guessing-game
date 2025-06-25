// Module declarations
mod models;
mod utils;
mod ui;
mod stats;
mod game;

// Main function - now much cleaner and focused
fn main() {
    use crate::ui::{show_game_title, show_main_menu, wait_for_enter};
    use crate::utils::clear_screen;
    use crate::game::play_game;
    use crate::stats::{handle_view_stats, handle_reset_stats};
    use colored::*;

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
                wait_for_enter();
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
