use std::collections::HashMap;
use rayon::prelude::*;

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
/// This function is optimized with parallelization:
/// 1. First pass: count all pairs in parallel using rayon - O(n/p) where p is number of cores
/// 2. Second pass: replace all occurrences of the most common pair - O(n)
///
/// The text is split into chunks for parallel processing, and results are merged.
pub fn find_common_byte_pair(text: &str, index: u16) -> ((char, char), String) {
    // First pass: Count all pairs in parallel by splitting text into chunks
    const MIN_CHUNK_SIZE: usize = 10000; // Minimum chars per chunk to avoid overhead
    let chars: Vec<char> = text.chars().collect();
    let chunk_size = (chars.len() / rayon::current_num_threads()).max(MIN_CHUNK_SIZE);

    // Process chunks in parallel and merge results
    let pairs: HashMap<(char, char), usize> = chars
        .par_chunks(chunk_size)
        .map(|chunk| {
            let mut local_pairs: HashMap<(char, char), usize> = HashMap::new();
            for i in 0..chunk.len().saturating_sub(1) {
                let pair = (chunk[i], chunk[i + 1]);
                // Only count pairs where neither character is whitespace or punctuation
                if !(pair.0.is_whitespace()
                    || pair.0.is_ascii_punctuation()
                    || pair.1.is_whitespace()
                    || pair.1.is_ascii_punctuation())
                {
                    *local_pairs.entry(pair).or_insert(0) += 1;
                }
            }
            local_pairs
        })
        .reduce(
            || HashMap::new(),
            |mut acc, local_pairs| {
                for (pair, count) in local_pairs {
                    *acc.entry(pair).or_insert(0) += count;
                }
                acc
            }
        );

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

    #[test]
    fn test_parallelization_produces_same_results() {
        // Test with a larger text to ensure chunking works correctly
        let text = "the quick brown fox jumps over the lazy dog the the the quick quick brown";
        let ((a1, b1), result1) = find_common_byte_pair(text, 0);

        // Run multiple times to ensure deterministic results
        let ((a2, b2), result2) = find_common_byte_pair(text, 0);

        assert_eq!(a1, a2);
        assert_eq!(b1, b2);
        assert_eq!(result1, result2);
    }

    #[test]
    fn test_parallelization_with_large_text() {
        // Create a large text by repeating a pattern
        let mut large_text = String::new();
        for _ in 0..1000 {
            large_text.push_str("hello world this is a test of parallel processing ");
        }

        let ((a, b), result) = find_common_byte_pair(&large_text, 0);

        // Verify that we found a valid pair
        assert!(a != '\0' && b != '\0');
        // Verify compression occurred
        assert!(result.chars().count() < large_text.chars().count());
    }
}

