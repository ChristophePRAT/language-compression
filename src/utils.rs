use std::collections::HashMap;

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

pub fn preprocess_text(text: &str) -> String {
    preprocess_text_with_options(text, false)
}

pub fn preprocess_text_with_options(text: &str, trim_whitespace: bool) -> String {
    let lowercased = text.to_lowercase();

    if trim_whitespace {
        // Trim consecutive whitespace characters to single instances
        let mut result = String::new();
        let mut prev_was_whitespace = false;

        for c in lowercased.chars() {
            if c.is_whitespace() {
                if !prev_was_whitespace {
                    result.push(c);
                    prev_was_whitespace = true;
                }
            } else {
                result.push(c);
                prev_was_whitespace = false;
            }
        }
        result
    } else {
        lowercased.chars().collect()
    }
}
pub fn compute_table_complexity(pairs_replaced: &Vec<(char, char)>) -> f64 {
    let len = pairs_replaced.len() as f64;
    len * 2.0 * (30_f64 + len).log2() // 1 pair = 2 chars, each char is in the alphabet or " ", ",", ".", etc.
}

pub fn compute_text_complexity(text: &String, table_len: u16) -> f64 {
    let len = text.chars().count() as f64;
    len * ((30 + table_len) as f64).log2()
}
pub fn pretty_print_pairs(pairs_replaced: &Vec<(char, char)>) {
    println!("{} replacements", pairs_replaced.len());
    pairs_replaced.iter().for_each(|(a, b)| {
        print!("`");
        decrypt_and_print(*a, pairs_replaced);
        decrypt_and_print(*b, pairs_replaced);
        print!("`");
        println!()
    });
}
/// Finds the most common byte pair in the text and replaces all occurrences.
///
/// This function is optimized to scan the text only twice:
/// 1. First pass: count all pairs using a HashMap directly from text iterator - O(n)
/// 2. Second pass: replace all occurrences of the most common pair - O(n)
///
/// Total complexity: O(n) where n is the text length
pub fn find_common_byte_pair(text: &str, index: u16) -> ((char, char), String) {
    // First pass: Count all pairs using HashMap directly from iterator
    let mut pairs: HashMap<(char, char), usize> = HashMap::new();
    let mut chars_iter = text.chars().peekable();

    while let Some(first) = chars_iter.next() {
        if let Some(&second) = chars_iter.peek() {
            let pair = (first, second);
            // Only count pairs where neither character is whitespace or punctuation
            if !(pair.0.is_whitespace()
                || pair.0.is_ascii_punctuation()
                || pair.1.is_whitespace()
                || pair.1.is_ascii_punctuation())
            {
                *pairs.entry(pair).or_insert(0) += 1;
            }
        }
    }

    // Find the most common pair - O(m) where m is number of unique pairs
    let (a, b) = pairs
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(pair, _)| pair)
        .unwrap_or(('\0', '\0'));

    // Create replacement character that won't conflict with original text
    let replacement = char::from_u32(128 + index as u32).unwrap(); // Use extended ASCII range to avoid conflicts

    // Second pass: Replace all occurrences of the most common pair
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preprocess_text_no_trim() {
        let input = "Hello   World\n\n\nTest";
        let result = preprocess_text(input);
        assert_eq!(result, "hello   world\n\n\ntest");
    }

    #[test]
    fn test_preprocess_text_with_trim() {
        let input = "Hello   World\n\n\nTest";
        let result = preprocess_text_with_options(input, true);
        assert_eq!(result, "hello world\ntest");
    }

    #[test]
    fn test_find_common_byte_pair() {
        let text = "hello hello world";
        let ((a, b), result) = find_common_byte_pair(text, 0);
        // Should find "ll" or "he" as most common pair
        assert!(a != '\0' && b != '\0');
        // Result should have fewer characters than original (not bytes, but chars)
        assert!(result.chars().count() < text.chars().count());
    }
}

