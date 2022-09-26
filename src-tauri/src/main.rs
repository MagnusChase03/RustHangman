#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

extern crate rand;

use rand::thread_rng;
use rand::Rng;

#[tauri::command]
fn get_word() -> String {
    let mut rng = thread_rng();

    let mut words: Vec<String> = vec!["rust".to_string(), "coding".to_string()];
    let word: String = words.remove(rng.gen_range(0..words.len()));

    word
}

#[tauri::command]
fn guess(word: &str, word_display: &str, user_guess: &str) -> String {
    
    let mut new_string: String = String::new();
    let word_display_no_whitespace: Vec<&str> = word_display.split_whitespace().collect();
    let macthing: char = user_guess.chars().nth(0).unwrap();
    for (i, c) in word.chars().enumerate() {

        if c == macthing {

            if i == word.len() - 1 {

                new_string.push(c);

            } else {

                new_string.push_str(format!("{} ", c).as_str());

            }

        } else if word_display_no_whitespace[i].chars().nth(0).unwrap() != '_' {

            if i == word.len() - 1 {

                new_string.push(word_display_no_whitespace[i].chars().nth(0).unwrap());

            } else {
                
                new_string.push_str(format!("{} ", word_display_no_whitespace[i].chars().nth(0).unwrap()).as_str());
            }

        } else {

            if i == word.len() - 1 {

                new_string.push('_');

            } else {
                
                new_string.push_str("_ ");
            }

        }

    }

    new_string

}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_word, guess])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
