# 🎯 The Ultimate Guessing Game

A feature-rich number guessing game built in Rust with a beautiful terminal interface, persistent statistics, and modular architecture.

Building this while learning new features of Rust programming language.

## 🎮 Game Features

### Core Gameplay

- **Number Guessing**: Guess a secret number between 1 and 100
- **Multiple Difficulty Levels**: Easy (10 lives), Medium (5 lives), Hard (3 lives), or Custom
- **Visual Lives Indicator**: Color-coded heart display showing remaining lives
- **Smart Hints**: Get helpful hints after each guess (can be improved more)
- **Beautiful UI**: Colored terminal interface with emojis and formatting

### Statistics & Progress Tracking

- **Persistent Statistics**: Game data saved to `game_stats.json`
- **Win/Loss Tracking**: Total games, wins, losses, and win rate
- **Best Score System**: Tracks your lowest attempts for a win
- **Game History**: Records details of each game with timestamps
- **Recent Games**: View your last 5 games with results

### User Interface

- **Main Menu System**: Clean menu-driven interface
- **Statistics Viewer**: View your progress without playing
- **Reset Functionality**: Clear all statistics with confirmation
- **Difficulty Selection**: Choose your preferred challenge level

## 🚀 Installation & Running

### Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

### Quick Start

```bash
# Clone or navigate to the project directory
cd guessing-game

# Build the project
cargo build

# Run the game
cargo run
```

### Development

```bash
# Build in release mode (faster)
cargo build --release

# Run tests (when implemented)
cargo test

# Check for warnings
cargo check
```

## 🎯 How to Play

1. **Start the Game**: Run `cargo run` in the project directory
2. **Choose Difficulty**: Select from Easy, Medium, Hard, or Custom
3. **Make Guesses**: Enter numbers between 1-100
4. **Use Hints**: Pay attention to the hints after each guess
5. **Manage Lives**: You have limited lives - use them wisely!
6. **View Statistics**: Check your progress from the main menu
7. **Try Again**: Play multiple games to improve your score

## 🏗️ Project Structure

```
guessing-game/
├── src/
│   ├── main.rs          # Entry point and main menu logic
│   ├── models.rs         # Data structures (GameStats, GameResult)
│   ├── ui.rs            # User interface functions
│   ├── game.rs          # Core game logic
│   ├── stats.rs         # Statistics management
│   └── utils.rs         # Utility functions
├── Cargo.toml           # Dependencies and project metadata
├── Cargo.lock           # Locked dependency versions
└── README.md           # This file
```

### Module Responsibilities

- **`main.rs`**: Application entry point, menu loop
- **`models.rs`**: Data structures and their implementations
- **`ui.rs`**: All user interface functions (screens, menus, display)
- **`game.rs`**: Core game logic and game loop
- **`stats.rs`**: Statistics loading, saving, and management
- **`utils.rs`**: Utility functions (screen clearing, helpers)

## 🛠️ Dependencies

- **`rand`**: Random number generation
- **`colored`**: Terminal text coloring
- **`console`**: Terminal manipulation
- **`dialoguer`**: Interactive prompts and menus
- **`serde`**: Data serialization/deserialization
- **`serde_json`**: JSON format support
- **`chrono`**: Date and time handling

## 🎓 Learning Objectives

This project demonstrates several important Rust concepts:

### Core Rust Concepts

- **Structs and Implementation**: `GameStats`, `GameResult` with methods
- **Enums and Pattern Matching**: Menu selection, game state handling
- **Error Handling**: `Result<T, E>`, `Option<T>`, proper error propagation
- **Ownership and Borrowing**: Understanding Rust's memory model
- **Collections**: `Vec<T>`, iterators, and data manipulation

### Advanced Features

- **Module System**: Code organization and separation of concerns
- **File I/O**: Reading/writing JSON files with error handling
- **Serialization**: Using Serde for data persistence
- **External Crates**: Managing dependencies and using third-party libraries
- **Documentation**: Code comments and documentation

### Software Engineering

- **Modular Architecture**: Clean separation of concerns
- **User Experience**: Intuitive interface design
- **Data Persistence**: Saving and loading game state
- **Error Recovery**: Graceful handling of file corruption
- **Code Maintainability**: Well-organized, readable code

## 🎨 Features in Detail

### Statistics System

- **Automatic Tracking**: Every game is automatically recorded
- **JSON Storage**: Human-readable statistics file
- **Error Recovery**: Handles corrupted files gracefully
- **Performance Metrics**: Win rate, average attempts, best scores

### User Interface

- **Color Coding**: Visual feedback with colors
- **Emoji Icons**: Engaging visual elements
- **Progress Indicators**: Live lives display with hearts
- **Confirmation Dialogs**: Safe operations with user confirmation

### Game Logic

- **Random Generation**: Secure random number generation
- **Input Validation**: Robust input handling and validation
- **State Management**: Proper game state tracking
- **Difficulty Scaling**: Multiple challenge levels

## 🔧 Customization

### Adding New Features

The modular structure makes it easy to add new features:

1. **New Game Modes**: Add to `game.rs`
2. **Additional Statistics**: Extend `models.rs`
3. **UI Improvements**: Modify `ui.rs`
4. **New Utilities**: Add to `utils.rs`

### Configuration

- **Difficulty Levels**: Modify `utils.rs` for new difficulty presets
- **UI Colors**: Change color schemes in `ui.rs`
- **Statistics File**: Modify file path in `stats.rs`

## 🐛 Troubleshooting

### Common Issues

- **Build Errors**: Ensure you have the latest Rust version
- **Permission Errors**: Check file write permissions for statistics
- **Display Issues**: Ensure your terminal supports colors and Unicode

### File Locations

- **Statistics File**: `game_stats.json` (created automatically)
- **Source Code**: `src/` directory
- **Dependencies**: `Cargo.toml`

## 🤝 Contributing

This is a learning project, but suggestions are welcome:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## 📝 Future Enhancements

Potential features to add:

- [ ] Achievement system
- [ ] Multiplayer support
- [ ] Sound effects
- [ ] Custom themes
- [ ] Export statistics
- [ ] Leaderboards
- [ ] Time attack mode
- [ ] Word guessing mode

## 📄 License

This project is created for educational purposes. Feel free to use and modify for learning Rust!

## 🎯 About

This project was created to learn Rust programming concepts through practical application. It demonstrates modern Rust development practices, including modular architecture, error handling, and user interface design.

---

**Happy Guessing! 🎮**
