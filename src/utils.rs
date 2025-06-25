use console::Term;

/// Clears the terminal screen
pub fn clear_screen() {
    let term = Term::stdout();
    term.clear_screen().unwrap();
}

/// Converts lives count to difficulty name
pub fn get_difficulty_name(lives: u32) -> String {
    match lives {
        10 => "Easy".to_string(),
        5 => "Medium".to_string(),
        3 => "Hard".to_string(),
        _ => format!("Custom ({})", lives),
    }
} 