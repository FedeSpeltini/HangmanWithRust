
# Hangman With Rust

This project is a console-based Hangman game implemented in Rust. It allows players to guess a secret word one letter at a time, providing feedback on correct and incorrect guesses until the word is fully guessed or the player runs out of attempts.

## Features

- Randomly selects a word for the player to guess.
- Tracks guessed letters and remaining attempts.
- Displays the current state of the word with underscores for unguessed letters.
- Provides feedback on correct and incorrect guesses.
- Ends the game when the word is fully guessed or the player runs out of attempts.

## Requirements

- Rust
- Cargo
- Internet connection (if using an API to fetch words)

## Installation

1. Clone the repository:
   ```sh
   git clone https://github.com/FedeSpeltini/HangmanWithRust.git
   cd HangmanWithRust
   ```

2. Build the project:
   ```sh
   cargo build
   ```

## Usage

1. Run the application:
   ```sh
   cargo run
   ```

2. Follow the on-screen instructions to guess the word letter by letter.

## Project Structure

- `src/main.rs`: Contains the main logic of the Hangman game.
- `Cargo.toml`: Project configuration and dependencies.

## Contributions

Contributions are welcome. Please open an issue or submit a pull request to discuss any changes.

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.

## Contact

Fede Speltini - fedesp71@gmail.com
