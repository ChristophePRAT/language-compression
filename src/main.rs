use std::{arch::aarch64::uint16x4_t, collections::HashMap};

static LOREM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nulla rhoncus nisl at tortor porta imperdiet. Nullam euismod sem nisl, nec pretium libero facilisis tincidunt. Pellentesque vitae faucibus metus. In ullamcorper nisl sed lorem aliquam blandit. Pellentesque eu malesuada lectus. Donec gravida et metus et varius. Etiam sodales eros et faucibus vestibulum. Integer ornare mollis massa, quis lobortis enim mattis id. Integer pulvinar semper posuere.

    Quisque sit amet mollis urna, a finibus felis. Phasellus euismod erat sit amet elit tempor, ut mollis magna euismod. Suspendisse sagittis vehicula mauris et volutpat. Duis eu malesuada sapien, sed fringilla tellus. Morbi tincidunt, quam a auctor semper, ante metus tempor tellus, nec varius risus lacus a arcu. Phasellus ut fermentum enim. In velit est, placerat sit amet dictum eu, lobortis ut urna. Aliquam luctus bibendum congue. Curabitur bibendum sagittis semper. Nullam ut dui elit. Morbi in tortor at nisl convallis rhoncus mollis eu risus. Ut venenatis, justo quis fermentum venenatis, tellus enim sagittis dui, tempor condimentum massa arcu nec sem. Vestibulum condimentum ornare velit id pulvinar. Pellentesque justo lacus, condimentum mollis odio aliquet, posuere sodales est. Sed lacinia id lectus in varius. Praesent malesuada sed nibh ut laoreet.

    Fusce varius erat et est pharetra, at elementum odio sodales. Donec porta, sem id egestas tempor, purus leo posuere nunc, a fermentum lacus erat sed magna. In sagittis at massa eu rutrum. Curabitur turpis augue, lacinia non nisl nec, condimentum tempus velit. Curabitur malesuada tortor eget lacus commodo, a imperdiet libero volutpat. Vivamus porttitor, velit nec suscipit elementum, neque eros eleifend quam, eu laoreet eros libero at nulla. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae; Vivamus urna massa, cursus sed turpis at, egestas dignissim felis. Morbi placerat, velit in dignissim elementum, urna lacus gravida leo, sit amet rutrum est magna quis lorem. Vestibulum interdum nibh neque, et congue odio porttitor non. Aenean porttitor viverra nibh nec congue. Maecenas quis cursus nisi. Curabitur sit amet pulvinar odio. Aliquam vel enim fringilla, aliquet risus et, aliquam lacus. In cursus metus vitae nisi condimentum blandit.

    Nulla facilisis enim at ligula euismod suscipit. Pellentesque dignissim ante sed vulputate suscipit. Sed eleifend dapibus lacus laoreet pellentesque. Donec sed velit nisl. Vestibulum viverra leo eget posuere malesuada. Nunc efficitur luctus placerat. Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nulla semper vehicula urna ut luctus. Ut et nunc purus.

    Sed eget ultrices sem, quis scelerisque nibh. Suspendisse urna nunc, bibendum eget eleifend non, ultricies id libero. Duis vel lectus vitae mi aliquam blandit eget et ligula. Aenean at eros non turpis pellentesque vehicula sed ut massa. Nunc sed magna fringilla, ultrices dolor id, ullamcorper felis. Phasellus sed ligula sed nulla feugiat posuere. Nullam laoreet ultricies ligula, eu maximus nisl cursus vel. Nulla semper mauris nec nulla vehicula scelerisque. Cras nibh metus, faucibus eu nunc non, mattis laoreet lorem. Cras eget libero non elit pharetra dictum.";

fn find_common_byte_pair(text: String, index: u8) -> ((char, char), String) {
    let mut pairs: HashMap<(char, char), usize> = HashMap::new();
    let chars: Vec<char> = text.chars().collect();

    for window in chars.windows(2) {
        let pair = (window[0], window[1]);
        *pairs.entry(pair).or_insert(0) += 1;
    }

    let (a, b) = pairs
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(pair, _)| pair)
        .unwrap_or(('\0', '\0'));

    let replacement = (index as char).to_string();
    let new_text = text.replace(&format!("{}{}", a, b), replacement.as_str());

    ((a, b), new_text)
}

fn compute_table_complexity(pairs_replaced: &Vec<(char, char)>) -> f64 {
    let mut complexity: f64 = 0.0;
    for &(a, b) in pairs_replaced.iter() {
        complexity += 2.0 * (30_f64).log2(); // 1 pair = 2 chars, each char is in the alphabet or " ", ",", ".", etc.
    }
    complexity
}

fn compute_text_complexity(text: &String, table_len: u8) -> f64 {
    let len = text.len() as f64;
    len * ((30 + table_len) as f64).log2()
}

fn iterate(times: u8) {
    let mut text = LOREM.to_string();

    let mut pairs_replaced: Vec<(char, char)> = Vec::new();

    let n_iterations = 100;

    let mut complexities: Vec<f64> = Vec::new();

    for i in 0..n_iterations {
        let ((a, b), new_text) = find_common_byte_pair(text.clone(), i as u8);
        text = new_text;
        pairs_replaced.push((a, b));
        println!("{}.{} => {}", a, b, i as u8);
        let total_complexity =
            compute_table_complexity(&pairs_replaced) + compute_text_complexity(&text, i as u8 + 1);
        complexities.push(total_complexity);
    }

    // Replace all non-latin characters with their ASCII code between backticks
    text = text
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() || c == ' ' || c == '.' || c == ',' || c == '\n' {
                c.to_string()
            } else {
                format!("`{}`", c as u8)
            }
        })
        .collect();

    for i in 0..complexities.len() {
        println!(
            "After {} replacements, total complexity = {}",
            i + 1,
            complexities[i].round()
        );
    }

    println!("Final encoded text:\n{}", text);
}

fn decrypt_and_print(a: char, pairs_replaced: &Vec<(char, char)>) {
    let mut current_char = a;
    loop {
        if (a.is_ascii_alphabetic() || a == ' ' || a == '.' || a == ',' || a == '\n')
            || (current_char as usize) >= pairs_replaced.len()
        {
            print!("{}", current_char);
            break;
        } else {
            let (ap, bp) = pairs_replaced[current_char as usize];
            print!("{}", ap);
            current_char = bp;
        }
    }
}

fn pretty_print_pairs(pairs_replaced: &Vec<(char, char)>) {
    pairs_replaced.iter().for_each(|(a, b)| {
        decrypt_and_print(*a, pairs_replaced);
        print!(".");
        decrypt_and_print(*b, pairs_replaced);
        println!()
    });
}

static MODE: &str = "optimal"; // "fixed" or "optimal"

fn optimal() {
    let mut text = LOREM.to_string();

    let mut pairs_replaced: Vec<(char, char)> = Vec::new();

    let mut complexities: Vec<f64> = Vec::new();

    let mut i = 0;
    loop {
        let ((a, b), new_text) = find_common_byte_pair(text.clone(), i as u8);
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
    text = text
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() || c == ' ' || c == '.' || c == ',' || c == '\n' {
                c.to_string()
            } else {
                format!("`{}`", c as u8)
            }
        })
        .collect();

    for i in 0..complexities.len() {
        println!(
            "After {} replacements, total complexity = {}",
            i + 1,
            complexities[i].round()
        );
    }

    println!("Final encoded text:\n{}", text);

    pretty_print_pairs(&pairs_replaced)
}

fn main() {
    if MODE == "fixed" {
        iterate(100);
    } else if MODE == "optimal" {
        optimal();
    } else {
        println!("Unknown mode: {}", MODE);
    }
}
