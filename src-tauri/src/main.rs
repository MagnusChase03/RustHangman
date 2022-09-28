#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

extern crate rand;

use rand::thread_rng;
use rand::Rng;

use std::env;
use std::path;
use std::fs;
use std::io::Write;

fn read_words() -> Vec<String> {

    let mut words: Vec<String> = Vec::new();
    let word_path: String = env::current_dir().unwrap().display().to_string();

    if !path::Path::new(format!("{}\\words.txt", word_path).as_str()).exists() {

        let mut word_file: fs::File = fs::File::create(format!("{}\\words.txt", word_path).as_str()).unwrap();

        let mut _result = writeln!(word_file, "coding");
        _result = writeln!(word_file, "rust");
        _result = writeln!(word_file, "cargo");
        _result = writeln!(word_file, "crate");
        _result = writeln!(word_file, "ownership");
        _result = writeln!(word_file, "abstraction");
        _result = writeln!(word_file, "lifetime");
        _result = writeln!(word_file, "namespace");
        
    }
    
    let word_list = std::fs::read_to_string(format!("{}\\words.txt", word_path)).expect("Invalid word path.");
    for word in (&word_list).split("\n") {
        // Remove extra new line character at the end
        if word != "" {

            words.push(word[0..word.len() - 1].to_string());

        }
    }

    words
}

#[tauri::command]
fn get_word() -> String {
    let mut rng = thread_rng();

    let mut words: Vec<String> = read_words();
    let word: String = words.remove(rng.gen_range(0..words.len()));

    word
}

#[tauri::command]
fn is_solved(word_display: &str) -> bool {
    let word_display_no_whitespace: Vec<&str> = word_display.split_whitespace().collect();
    for c in word_display_no_whitespace {
        if c.chars().nth(0).unwrap() == '_' {
            return false;
        }
    }

    true
}

#[tauri::command]
fn can_guess(word: &str, user_guess: &str) -> bool {
    let macthing: char = user_guess.chars().nth(0).unwrap();
    for c in word.chars() {
        if c == macthing {
            return true;
        }
    }

    false
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
                new_string.push_str(
                    format!("{} ", word_display_no_whitespace[i].chars().nth(0).unwrap()).as_str(),
                );
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
        .invoke_handler(tauri::generate_handler![
            get_word, is_solved, can_guess, guess
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
