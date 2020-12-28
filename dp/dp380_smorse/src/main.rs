/**
https://www.reddit.com/r/dailyprogrammer/comments/cmd1hb/20190805_challenge_380_easy_smooshed_morse_code_1/

smorse("sos") => "...---..."
smorse("daily") => "-...-...-..-.--"
smorse("programmer") => ".--..-.-----..-..-----..-."
smorse("bits") => "-.....-..."
smorse("three") => "-.....-..."
 */

use std::collections::{HashMap, HashSet, VecDeque};
use std::convert::TryInto;
use std::fs;
use std::iter::FromIterator;

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

// TODO(lucasw) make a Smorse class and make these impl methods for it
fn find_balanced(target_len: &usize, word_lengths: &HashMap<usize, Vec<String>>,
                 word_to_smorse: &HashMap<String, String>) -> Vec<String> {
    let mut target_words = Vec::new();
    for word in word_lengths.get(target_len).unwrap() {
        let smorse_word = word_to_smorse.get(word).unwrap();
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
            target_words.push(word.to_string());
        }
    }
    target_words
}

fn find_palindromes(target_len: &usize, word_lengths: &HashMap<usize, Vec<String>>,
                    word_to_smorse: &HashMap<String, String>) -> Vec<String> {
    let mut target_words = Vec::new();
    if *target_len >= 2 {
        for word in word_lengths.get(target_len).unwrap() {
            let smorse_word = word_to_smorse.get(word).unwrap();
            let mut word_chars = VecDeque::from_iter(smorse_word.chars());
            loop {
                if word_chars.pop_front().unwrap() != word_chars.pop_back().unwrap() {
                    break;
                }
                if word_chars.len() <= 1 {
                    target_words.push(word.to_string());
                    break;
                }
            }
        }
    }
    target_words
}

fn find_possible(morse_codes: &Vec<&str>, len: usize, cur: String,
                 possible: &mut HashSet<String>,
                 searched: &mut HashSet<String>) {
    let cur_len = cur.chars().count();
    if cur_len > len {
        return;
    }
    if cur_len == len {
        possible.insert(cur);
        return;
    }

    if searched.contains(&cur) {
        return;
    }

    for code in morse_codes {
        // TODO(lucasw) it would be much more efficient to quit out if the current < len
        // sequence is a valid subsequence of something else that has already been searched.
        let mut cur2 = cur.clone();
        cur2.push_str(code);
        find_possible(morse_codes, len, cur2, possible, searched);
    }
    searched.insert(cur);
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

    let mut word_to_smorse : HashMap<String, String> = HashMap::new();
    let mut smorse_to_words : HashMap<String, Vec<String>> = HashMap::new();

    for word in words {
        let smorse_word = smorse(&morse, word);
        word_to_smorse.insert(word.to_string(), smorse_word);
    }

    for (word, smorse_word) in word_to_smorse.iter() {
        // smorse_to_words.entry(smorse_word).or_default().push(word);
        // let key : &str = word_to_smorse.get(word).unwrap();
        smorse_to_words.entry(smorse_word.to_string()).or_default().push(word.to_string());
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
    let mut best_dash_word = "";
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
                            println!("{} dashes in a row {} {:?}", best_dash_count, smorse_word,
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
    println!("best dash word {}", best_dash_word);

    println!("");
    println!("Call a word perfectly balanced if its code has the same number of dots as dashes.");
    println!("counterdemonstrations is one of two 21-letter words that's perfectly balanced. Find the other one.");
    let mut word_lengths : HashMap<usize, Vec<String>> = HashMap::new();
    for word in word_to_smorse.keys() {
        word_lengths.entry(word.chars().count()).or_default().push(word.to_string());
    }

    let target_len = 21;
    let target_words = find_balanced(&target_len, &word_lengths, &word_to_smorse);
    println!("balanced words of length {}: {:?}", target_len, target_words);

    println!("");
    println!("protectorate is 12 letters long and encodes to .--..-.----.-.-.----.-..--.,");
    println!("which is a palindrome (i.e. the string is the same when reversed).");
    println!("Find the only 13-letter word that encodes to a palindrome.");
    let target_len = 12;
    let target_words = find_palindromes(&target_len, &word_lengths, &word_to_smorse);
    println!("palindromes words of length {}: {:?}", target_len, target_words);
    let target_len = 13;
    let target_words = find_palindromes(&target_len, &word_lengths, &word_to_smorse);
    println!("palindromes words of length {}: {:?}", target_len, target_words);

    println!("");
    println!(" --.---.---.-- is one of five 13-character sequences that does not appear in the encoding of any word.");
    println!("Find the other four.");
    // generate all possible 13 character sequences (2^13)
    // TODO(lucasw) this isn't right, it wants valid sequences that could have come from real morse
    // sequences
    let mut all_seqs = Vec::new();
    let len: usize = 13;
    let possible: usize = 2usize.pow(len.try_into().unwrap());
    println!("{} used sequences of length {}", smorse_lengths.get(&len).unwrap().len(), len);
    for num in 0..possible {
        let mut seq = String::from("");
        for ind in 0..len {
            match (num & 1 << ind) >> ind {
                0 => seq.push('.'),
                1 => seq.push('-'),
                _ => panic!("shouldn't be possible {} {}", num, ind),
            }
        }
        all_seqs.push(seq);
    }
    // println!("{} sequences of length {} {:?}", all_seqs.len(), len, all_seqs);
    println!("{} sequences of length {}", all_seqs.len(), len);
    // TODO(lucasw) more memory efficient to do this in the above loop, don't store all seqs
    let mut unused_seqs = Vec::new();
    for seq in all_seqs.iter() {
        if !smorse_to_words.contains_key(seq) {
            unused_seqs.push(seq);
        }
    }
    // println!("{} unused sequences of length {} {:?}..", unused_seqs.len(), len, unused_seqs);
    println!("{} unused sequences of length {}", unused_seqs.len(), len);

    println!("now find just the {} length sequences that are valid smorse recursively", len);
    let mut possible = HashSet::new();
    let mut searched = HashSet::new();
    let cur = String::from("");
    find_possible(&morse_codes, len, cur, &mut possible, &mut searched);
    println!("{} possible", possible.len());

    // TODO(lucasw) I think the problem is actually to search all encodings of length >=13 for
    // unique length 13 encodings.
}
