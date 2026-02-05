mod text_utils;
use std::collections::HashMap;
use std::fs;
use std::io::Read;

fn find_common_byte_pair(text: &str, index: u8) -> ((char, char), String) {
    let mut pairs: HashMap<(char, char), usize> = HashMap::new();
    let chars: Vec<char> = text.chars().collect();

    for window in chars.windows(2) {
        let pair = (window[0], window[1]);
        if !(pair.0.is_whitespace()
            || pair.0.is_ascii_punctuation()
            || pair.1.is_whitespace()
            || pair.1.is_ascii_punctuation())
        {
            *pairs.entry(pair).or_insert(0) += 1;
        }
    }

    let (a, b) = pairs
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(pair, _)| pair)
        .unwrap_or(('\0', '\0'));

    // Create replacement character that won't conflict with original text
    let replacement = char::from_u32(128 + index as u32).unwrap(); // Use extended ASCII range to avoid conflicts

    // More efficient replacement by iterating through characters
    let mut result = String::new();
    let mut chars = text.chars().peekable();

    while let Some(current) = chars.next() {
        if current == a && chars.peek() == Some(&b) {
            result.push(replacement);
            chars.next(); // Skip the next character since we've matched a pair
        } else {
            result.push(current);
        }
    }

    ((a, b), result)
}

fn compute_table_complexity(pairs_replaced: &Vec<(char, char)>) -> f64 {
    let len = pairs_replaced.len() as f64;
    len * 2.0 * (30_f64 + len).log2() // 1 pair = 2 chars, each char is in the alphabet or " ", ",", ".", etc.
}

fn compute_text_complexity(text: &String, table_len: u8) -> f64 {
    let len = text.chars().count() as f64;
    len * ((30 + table_len) as f64).log2()
}

fn iterate(txt: String, times: u16) {
    let mut text = txt.clone();

    let mut pairs_replaced: Vec<(char, char)> = Vec::new();

    let mut complexities: Vec<f64> = Vec::new();

    for i in 0..times as usize {
        let ((a, b), new_text) = find_common_byte_pair(&text, i as u8);
        text = new_text;
        pairs_replaced.push((a, b));
        println!("{}.{} => {}", a, b, i as u8);
        let total_complexity =
            compute_table_complexity(&pairs_replaced) + compute_text_complexity(&text, i as u8 + 1);
        complexities.push(total_complexity);
    }

    // Replace all non-latin characters with their ASCII code between backticks
    // text = text
    //     .chars()
    //     .map(|c| {
    //         if c.is_ascii_alphabetic() || c == ' ' || c == '.' || c == ',' || c == '\n' {
    //             c.to_string()
    //         } else {
    //             format!("`{}`", c as u8)
    //         }
    //     })
    //     .collect();

    for i in 0..complexities.len() {
        println!(
            "After {} replacements, total complexity = {}",
            i + 1,
            complexities[i].round()
        );
    }

    // println!("Final encoded text:\n{}", text);
    pretty_print_pairs(&pairs_replaced);
}

fn decrypt_and_print(start_char: char, pairs_replaced: &Vec<(char, char)>) {
    // If the character is in the original character set, print it directly
    if start_char.is_ascii_alphabetic()
        || start_char == ' '
        || start_char == '.'
        || start_char == ','
        || start_char == '\n'
    {
        print!("{}", start_char);
        return;
    }

    // If the character is a replacement (>= 128), decode it recursively
    let idx = start_char as usize;
    if idx >= 128 && (idx - 128) < pairs_replaced.len() {
        let (a, b) = pairs_replaced[idx - 128];
        // Recursively decode both parts of the pair
        decrypt_and_print(a, pairs_replaced);
        decrypt_and_print(b, pairs_replaced);
    } else {
        // If it's not a recognized replacement, print as-is
        print!("{}", start_char);
    }
}

fn pretty_print_pairs(pairs_replaced: &Vec<(char, char)>) {
    println!("Replaced pairs:");
    pairs_replaced.iter().for_each(|(a, b)| {
        print!("`");
        decrypt_and_print(*a, pairs_replaced);
        decrypt_and_print(*b, pairs_replaced);
        print!("`");
        println!()
    });
}

fn optimal(txt: String) {
    let mut text = txt.clone();

    let mut pairs_replaced: Vec<(char, char)> = Vec::new();

    let mut complexities: Vec<f64> = Vec::new();

    let mut i = 0;
    loop {
        let ((a, b), new_text) = find_common_byte_pair(&text, i as u8);
        let mut temp_pairs_replaced = pairs_replaced.clone();
        temp_pairs_replaced.push((a, b));
        let total_complexity = compute_table_complexity(&temp_pairs_replaced)
            + compute_text_complexity(&new_text, i as u8 + 1);

        complexities.push(total_complexity);

        if total_complexity
            >= compute_table_complexity(&pairs_replaced) + compute_text_complexity(&text, i as u8)
        {
            break;
        }

        text = new_text;
        pairs_replaced.push((a, b));
        i += 1;
    }

    // Replace all non-latin characters with their ASCII code between backticks
    // text = text
    //     .chars()
    //     .map(|c| {
    //         if c.is_ascii_alphabetic() || c == ' ' || c == '.' || c == ',' || c == '\n' {
    //             c.to_string()
    //         } else {
    //             format!("`{}`", c as u8)
    //         }
    //     })
    //     .collect();

    for i in 0..complexities.len() {
        println!(
            "After {} replacements, total complexity = {}",
            i + 1,
            complexities[i].round()
        );
    }

    // println!("Final encoded text:\n{}", text);

    pretty_print_pairs(&pairs_replaced)
}

static TIMES: i16 = 150; // Number of iterations (-1 for optimal)
static TEXT: &str = "alice.txt";

fn main() {
    let mut file = fs::File::open(TEXT).expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file");

    contents = text_utils::preprocess_text(&contents);
    if TIMES > 0 {
        iterate(contents, TIMES as u16);
    } else {
        optimal(contents);
    }
}
