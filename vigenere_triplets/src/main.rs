use std::fs::{self, File};
use std::collections::HashSet;
use std::io::Write;

// vigenere implementation
fn vigenere_encrypt(word: &str, key: &str) -> String {
    let mut encrypted = String::new();
    let a = 'a' as u8;

    for (i, c) in word.chars().enumerate() {
        // only lowercase a-z characters
        if c.is_ascii_lowercase() {
            if let Some(key_char) = key.chars().nth(i % key.len()) {
                if key_char.is_ascii_lowercase() {
                    let word_char_val = c as u8 - a;
                    let key_char_val = key_char as u8 - a;
                    let encrypted_char = ((word_char_val + key_char_val) % 26) + a;
                    encrypted.push(encrypted_char as char);
                }
            }
        }
    }

    encrypted
}

// check for encrypted word in list
fn find_vigenere_matches(word_list: &HashSet<String>, combinations: u32) -> Vec<(String, String, String)> {
    let mut results = Vec::new();

    let mut count: u32 = 0;
    let mut mil_count: u32 = 0;
    let mut match_count: u32 = 0;

	println!("word list length: {}", word_list.len());
    for word in word_list.iter() {
        for key in word_list.iter() {
            count += 1;
	          if key.len() != word.len() { // skip if key and word length mismatch
                continue;
            }
            if count%1000000 == 0 {
                mil_count += count/1000000;
                count = 0;
                println!("{} million combinations ({:.3}%) checked and {} matches found…", mil_count, mil_count as f64/combinations as f64 * 100.0, match_count);
            }
            let encrypted_word = vigenere_encrypt(word, key);
            if (word_list.contains(&encrypted_word)) && (word != &encrypted_word) {
                results.push((word.clone(), key.clone(), encrypted_word.clone()));
                match_count += 1;
            }
        }
    }

    results
}

fn main() {
    let content = fs::read_to_string("wordlist.txt")
        .expect("unable to read word list");

    println!("generating hash set of words…");

    // use a hashset so it's at least a little efficient
    let word_list: HashSet<String> = content
        .lines()
        .map(|line| line.trim().to_lowercase())      // only lowercase
        .filter(|line| !line.is_empty())             // ignore empty lines
        .collect();

    println!("done");

    let combinations: u32 = ((word_list.len() as u32)/1000) * ((word_list.len() as u32)/1000);

    println!("number of combinations to check: {} million", combinations);

    println!("working…");

    // find all combinations of word, key, and encrypted word
    let matches = find_vigenere_matches(&word_list, combinations);

    // save to file
    let mut output_file = File::create("output.txt")
        .expect("unable to create output.txt");

    for (word, key, encrypted_word) in matches {
        let result = format!("w: {}; k: {}; e: {}\n", word, key, encrypted_word);
        output_file.write_all(result.as_bytes())
            .expect("error writing to file");
    }

    println!("results saved to ouptut.txt");
}
