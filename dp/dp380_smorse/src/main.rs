/**
https://www.reddit.com/r/dailyprogrammer/comments/cmd1hb/20190805_challenge_380_easy_smooshed_morse_code_1/

smorse("sos") => "...---..."
smorse("daily") => "-...-...-..-.--"
smorse("programmer") => ".--..-.-----..-..-----..-."
smorse("bits") => "-.....-..."
smorse("three") => "-.....-..."
 */

use std::collections::HashMap;
use std::fs;

fn smorse(morse: &HashMap<char, &str>, text: &str) -> String {
    // print!("smorse({}) => \"", text);
    let mut rv = String::new();
    for c in text.chars() {
        // print!("{}", morse[&c]);
        rv.push_str(morse[&c]);
    }
    // println!("\"");
    rv
}

fn main() {
    let alphabet = (b'a'..=b'z')                               // Start as u8
        .filter_map(|c| {
            let c = c as char;                             // Convert to char
            if c.is_alphabetic() { Some(c) } else { None } // Filter only alphabetic chars
        })
        .collect::<Vec<_>>();

    let morse_codes = vec![".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..", "--", "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-", "-.--", "--.."];

    // println!("{} {} {:?} {:?}", alphabet.len(), morse.len(), alphabet, morse_codes);
    // let morse: HashMap<_, _> = alphabet.iter().zip(morse_codes.iter()).collect();
    let mut morse : HashMap<char, &str> = HashMap::new();
    for (k, v) in alphabet.iter().zip(morse_codes.iter()) {
        morse.insert(*k, v);
    }

    println!("{:?}", morse);
    smorse(&morse, "sos");
    smorse(&morse, "daily");
    smorse(&morse, "programmer");
    smorse(&morse, "bits");
    smorse(&morse, "three");

    let filename = "data/enable1.txt";
    let mut text_raw = fs::read_to_string(filename).unwrap().to_lowercase();
    text_raw.retain(|c| c.is_alphanumeric() || c.is_whitespace());
    let words = text_raw.split_whitespace();

    let mut word_to_smorse : HashMap<&str, String> = HashMap::new();
    let mut smorse_to_words : HashMap<String, Vec<&str>> = HashMap::new();

    for word in words {
        let smorse_word = smorse(&morse, word);
        word_to_smorse.insert(word, smorse_word);
    }

    for (word, smorse_word) in word_to_smorse {
        // smorse_to_words.entry(smorse_word).or_default().push(word);
        // let key : &str = word_to_smorse.get(word).unwrap();
        smorse_to_words.entry(smorse_word).or_default().push(word);
    }

    /* The sequence -...-....-.--. is the code for four different words (needing, nervate, niding, tiling).
     * Find the only sequence that's the code for 13 different words.
     */
    let mut smorse_counts : HashMap<usize, Vec<String>> = HashMap::new();
    for (smorse_word, words) in smorse_to_words.iter() {
        smorse_counts.entry(words.len()).or_default().push(smorse_word.to_string());
    }

    let mut counts = smorse_counts.keys().copied().collect::<Vec<usize>>();
    counts.sort();
    counts.reverse();

    for count in counts {
        let smorse_words = smorse_counts.get(&count).unwrap();
        println!("{}", count);
        for smorse_word in smorse_words {
            let words = smorse_to_words.get(smorse_word).unwrap();
            println!("{}  {:?}", smorse_word, words);
        }
        if count < 11 {
            break;
        }
    }

    /* autotomous encodes to .-..--------------..-..., which has 14 dashes in a row.
     * Find the only word that has 15 dashes in a row.
     */
    let mut smorse_lengths : HashMap<usize, Vec<String>> = HashMap::new();
    for smorse_word in smorse_to_words.keys() {
        smorse_lengths.entry(smorse_word.chars().count()).or_default().push(smorse_word.to_string());
    }

    let mut lengths = smorse_lengths.keys().copied().collect::<Vec<usize>>();
    lengths.sort();
    lengths.reverse();

    println!("lengths {}, {}", lengths.len(), lengths[0]);
    let mut best_dash_count = 0;
    let mut best_dash_word;
    for i in (20..lengths[0] + 1).rev() {
        if !smorse_lengths.contains_key(&i) {
            continue;
        }
        let smorse_words = smorse_lengths.get(&i).unwrap();
        println!("smorse length {}, num smorse words {}", i, smorse_words.len());
        for smorse_word in smorse_words {
            let mut dash_count = 0;
            for c in smorse_word.chars() {
                match c {
                    '-' => {
                        dash_count += 1;
                        if dash_count > best_dash_count {
                            println!("{} {} {:?}", best_dash_count, smorse_word,
                                     smorse_to_words.get(smorse_word).unwrap());
                            best_dash_count = dash_count;
                            best_dash_word = smorse_word;
                        }
                    }
                    '.' => dash_count = 0,
                    _ => panic!("unexpected non-morse char {} {}", c, smorse_word),
                }
            }
        }
    }

    println!("Call a word perfectly balanced if its code has the same number of dots as dashes.");
    println!("counterdemonstrations is one of two 21-letter words that's perfectly balanced. Find the other one.");
    for len in smorse_lengths.keys().filter(|len| *len % 2 == 0) {
        let smorse_words = smorse_lengths.get(len).unwrap();

        for smorse_word in smorse_words {
            let words = smorse_to_words.get(smorse_word).unwrap();
            let mut words21 = Vec::new();
            // TODO(lucasw) it would be more efficient to start with words of length 21
            for word in words {
                if word.chars().count() == 21 {
                    words21.push(word);
                }
            }
            if words21.len() == 0 {
                continue;
            }

            let mut dash_count = 0;
            let mut dot_count = 0;
            for c in smorse_word.chars() {
                match c {
                    '-' => dash_count += 1,
                    '.' => dot_count += 1,
                    _ => panic!("unexpected non-morse char {} {}", c, smorse_word),
                }
            }
            if dash_count == dot_count {
                println!("{} {} {} {:?}", dash_count, dot_count, smorse_word, words21);
            }
        }
    }

    /* protectorate is 12 letters long and encodes to .--..-.----.-.-.----.-..--.,
     * which is a palindrome (i.e. the string is the same when reversed).
     * Find the only 13-letter word that encodes to a palindrome.
     */

    /* --.---.---.-- is one of five 13-character sequences that does not appear in the encoding of any word.
     * Find the other four.
     */
}
