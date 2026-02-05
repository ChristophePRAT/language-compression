#[path = "./utils.rs"]
mod utils;
use kdam::tqdm;

pub fn iterate(txt: String, times: u16) {
    let mut text = txt.clone();

    let mut pairs_replaced: Vec<(char, char)> = Vec::new();

    let mut complexities: Vec<f64> = Vec::new();

    for i in tqdm!(0..times as usize) {
        let ((a, b), new_text) = utils::find_common_byte_pair(&text, i as u16);
        text = new_text;
        pairs_replaced.push((a, b));
        let total_complexity = utils::compute_table_complexity(&pairs_replaced)
            + utils::compute_text_complexity(&text, i as u16 + 1);
        complexities.push(total_complexity);
    }

    utils::pretty_print_pairs(&pairs_replaced);
}
pub fn optimal(txt: String) {
    let mut text = txt.clone();

    let mut pairs_replaced: Vec<(char, char)> = Vec::new();

    let mut complexities: Vec<f64> = Vec::new();

    let mut i = 0;
    loop {
        let ((a, b), new_text) = utils::find_common_byte_pair(&text, i as u16);
        let mut temp_pairs_replaced = pairs_replaced.clone();
        temp_pairs_replaced.push((a, b));
        let total_complexity = utils::compute_table_complexity(&temp_pairs_replaced)
            + utils::compute_text_complexity(&new_text, i as u16 + 1);

        complexities.push(total_complexity);

        if total_complexity
            >= utils::compute_table_complexity(&pairs_replaced)
                + utils::compute_text_complexity(&text, i as u16)
        {
            break;
        }

        text = new_text;
        pairs_replaced.push((a, b));
        i += 1;
    }

    for i in 0..complexities.len() {
        println!(
            "After {} replacements, total complexity = {}",
            i + 1,
            complexities[i].round()
        );
    }
    utils::pretty_print_pairs(&pairs_replaced)
}

pub fn opti_search(txt: String, up_to_times: u16) {
    let mut text = txt.clone();

    let mut pairs_replaced: Vec<(char, char)> = Vec::new();

    let mut complexities: Vec<f64> = Vec::new();

    for i in tqdm!(0..up_to_times as usize) {
        let ((a, b), new_text) = utils::find_common_byte_pair(&text, i as u16);
        text = new_text;
        pairs_replaced.push((a, b));
        let total_complexity = utils::compute_table_complexity(&pairs_replaced)
            + utils::compute_text_complexity(&text, i as u16 + 1);
        complexities.push(total_complexity);
    }

    let best_idx = complexities
        .iter()
        .enumerate()
        .min_by_key(|&(_, complexity)| complexity.round() as i64)
        .map(|(idx, _)| idx);

    let end_range = best_idx.map(|i| i + 1).unwrap_or(0);
    let best_pairs = &pairs_replaced[..end_range];
    utils::pretty_print_pairs(&best_pairs.to_vec());
}
