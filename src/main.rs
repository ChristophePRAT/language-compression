mod bpe_strategies;
mod utils;
use std::fs;
use std::io::Read;

static TIMES: i16 = 500; // Number of iterations (-1 for optimal)
static TEXT: &str = "text/alice.txt";

fn main() {
    let mut file = fs::File::open(TEXT).expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file");

    contents = utils::preprocess_text(&contents);
    if TIMES > 0 {
        bpe_strategies::opti_search(contents, TIMES as u16);
    } else {
        bpe_strategies::optimal(contents);
    }
}
