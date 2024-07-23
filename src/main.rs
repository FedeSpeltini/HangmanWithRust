use std::io;

fn main() {
    let secret_word = "speltini";
    let mut attempts_left = 6;
    let mut guessed_letters = Vec::new();

    println!("Welcome to the hangman!");

    while attempts_left > 0 {
        display_progress(&secret_word, &guessed_letters);
        println!("Attempts left: {}", attempts_left);
        println!("Guess a letter: ");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess = guess.trim().chars().next().expect("Please enter a letter");

        if !guessed_letters.contains(&guess) {
            guessed_letters.push(guess);
        }

        if guessed_letters.contains(&guess) {
            println!("You already guessed that letter!");
            attempts_left -= 1;
            continue;
        }

        if secret_word.contains(guess) {
            println!("Good guess!");
        } else {
            println!("Wrong guess!");
            attempts_left -= 1;
        }

        if attempts_left == 0 {
            println!("You lose, the word was {}", secret_word)
        }
    }

}
fn display_progress(secret_word: &str, guessed_letters: &Vec<char>) {
    let mut display_string = String::new();

    for letter in secret_word.chars() {
        if guessed_letters.contains(&letter) {
            display_string.push(letter)
        } else {
            display_string.push('_');
        }
    }
    println!("current word: {}", display_string.trim());
}