extern crate rand;

use rand::thread_rng;
use rand::Rng;

// Returns a random word from a vector of possible game words
fn get_random_word() -> String {
    let mut rng = thread_rng();

    let mut words = vec![
        "coding".to_string(),
        "rust".to_string(),
        "ownership".to_string(),
    ];
    let word = words.remove(rng.gen_range(0..words.len()));

    word
}

// Checks if the user has found every letter in the word
fn is_unsolved(word_vec: &Vec<char>) -> bool {
    for c in word_vec {
        if *c == '_' {
            return true;
        }
    }

    false
}

// Uses the users guess to match characters in the word 
// and returns true if they found one
fn attempt(word: &String, word_vec: &mut Vec<char>, guess: &String) -> bool {
    let mut found = false;

    let matching = guess.chars().nth(0).unwrap();
    for (i, c) in word.chars().enumerate() {
        if c == matching {
            word_vec[i] = matching;
            found = true;
        }
    }

    found
}

fn game() {
    // Init word
    let word = get_random_word();
    let mut word_vec: Vec<char> = Vec::new();
    for _c in word.chars() {
        word_vec.push('_');
    }

    // Game loop
    let mut lives = 6;
    let mut guess = String::new();
    while is_unsolved(&word_vec) && lives > 0 {
        println!("{:?}", word_vec);
        println!("Lives: {}", lives);
        println!("Guess a letter:");

        // Read guess
        let _bytes_read = std::io::stdin().read_line(&mut guess).unwrap();

        if !attempt(&word, &mut word_vec, &guess) {
            lives -= 1;
        }

        guess.clear();
        print!("");
    }
}

fn main() {
    game();
}
