use std::cmp::Ordering;
use rand::Rng;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use console::Term;
use dialoguer::{Input, Select};

fn show_game_title() {
    let term = Term::stdout();
    term.clear_screen().unwrap();
    
    println!("{}", "╔══════════════════════════════════════════════════════════════╗".bright_blue());
    println!("{}", "║                                                              ║".bright_blue());
    println!("{}", "║    ██████╗ ██╗   ██╗███████╗███████╗███████╗██╗███╗   ██╗ ██╗  ║".cyan());
    println!("{}", "║   ██╔════╝ ██║   ██║██╔════╝██╔════╝██╔════╝██║████╗  ██║ ██║  ║".cyan());
    println!("{}", "║   ██║  ███╗██║   ██║███████╗███████╗███████╗██║██╔██╗ ██║ ██║  ║".cyan());
    println!("{}", "║   ██║   ██║██║   ██║╚════██║╚════██║╚════██║██║██║╚██╗██║ ╚═╝  ║".cyan());
    println!("{}", "║   ╚██████╔╝╚██████╔╝███████║███████║███████║██║██║ ╚████║ ██╗  ║".cyan());
    println!("{}", "║    ╚═════╝  ╚═════╝ ╚══════╝╚══════╝╚══════╝╚═╝╚═╝  ╚═══╝ ╚═╝  ║".cyan());
    println!("{}", "║                                                              ║".bright_blue());
    println!("{}", "║                    🎯 THE ULTIMATE GAME 🎯                    ║".bright_yellow());
    println!("{}", "║                                                              ║".bright_blue());
    println!("{}", "║              Can you find the secret number?                 ║".white());
    println!("{}", "║                                                              ║".bright_blue());
    println!("{}", "╚══════════════════════════════════════════════════════════════╝".bright_blue());
    println!();
}

fn show_loading_animation() {
    let pb = ProgressBar::new(100);
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {wide_msg}")
            .unwrap()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
    );
    
    pb.set_message("🎮 Initializing game...");
    for _ in 0..100 {
        pb.inc(1);
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
    pb.finish_with_message("🎮 Game ready!");
    println!();
}

fn show_win_screen() {
    println!();
    println!("{}", "╔══════════════════════════════════════════════════════════════╗".bright_green());
    println!("{}", "║                                                              ║".bright_green());
    println!("{}", "║    ██╗   ██╗██╗ ██████╗████████╗ ██████╗ ██████╗ ██╗   ██╗  ║".green());
    println!("{}", "║    ╚██╗ ██╔╝██║██╔════╝╚══██╔══╝██╔═══██╗██╔══██╗╚██╗ ██╔╝  ║".green());
    println!("{}", "║     ╚████╔╝ ██║██║        ██║   ██║   ██║██████╔╝ ╚████╔╝   ║".green());
    println!("{}", "║      ╚██╔╝  ██║██║        ██║   ██║   ██║██╔══██╗ ╚██╔╝    ║".green());
    println!("{}", "║       ██║   ██║╚██████╗   ██║   ╚██████╔╝██║  ██║  ██║     ║".green());
    println!("{}", "║       ╚═╝   ╚═╝ ╚═════╝   ╚═╝    ╚═════╝ ╚═╝  ╚═╝  ╚═╝     ║".green());
    println!("{}", "║                                                              ║".bright_green());
    println!("{}", "║                    🏆 CONGRATULATIONS! 🏆                    ║".bright_yellow());
    println!("{}", "║                                                              ║".bright_green());
    println!("{}", "║                    You are a winner!                         ║".white());
    println!("{}", "║                                                              ║".bright_green());
    println!("{}", "╚══════════════════════════════════════════════════════════════╝".bright_green());
    println!();
}

fn show_game_over_screen() {
    println!();
    println!("{}", "╔══════════════════════════════════════════════════════════════╗".bright_red());
    println!("{}", "║                                                              ║".bright_red());
    println!("{}", "║     ██████╗  █████╗ ███╗   ███╗███████╗     ██████╗ ██╗   ██╗ ║".red());
    println!("{}", "║    ██╔════╝ ██╔══██╗████╗ ████║██╔════╝    ██╔═══██╗██║   ██║ ║".red());
    println!("{}", "║    ██║  ███╗███████║██╔████╔██║█████╗      ██║   ██║██║   ██║ ║".red());
    println!("{}", "║    ██║   ██║██╔══██║██║╚██╔╝██║██╔══╝      ██║   ██║╚██╗ ██╔╝ ║".red());
    println!("{}", "║    ╚██████╔╝██║  ██║██║ ╚═╝ ██║███████╗    ╚██████╔╝ ╚████╔╝  ║".red());
    println!("{}", "║     ╚═════╝ ╚═╝  ╚═╝╚═╝     ╚═╝╚══════╝     ╚═════╝  ╚═══╝   ║".red());
    println!("{}", "║                                                              ║".bright_red());
    println!("{}", "║                    💀 GAME OVER! 💀                          ║".bright_red());
    println!("{}", "║                                                              ║".bright_red());
    println!("{}", "║                    Better luck next time!                   ║".white());
    println!("{}", "║                                                              ║".bright_red());
    println!("{}", "╚══════════════════════════════════════════════════════════════╝".bright_red());
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

fn main() {
    show_game_title();
    show_loading_animation();
    
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
                break;
            }
        }

        // Check if player ran out of lives
        if current_lives == 0 {
            show_game_over_screen();
            println!("{}", format!("🎯 The secret number was: {}", secret_number).bright_red());
            println!("{}", format!("📊 Total attempts: {}", attempts).cyan());
            println!("{}", "=".repeat(50).bright_red());
            break;
        }
    }
}
