use std::iter;

pub const MASK_CHAR: char = 'x';

pub fn gen_str(str: &str) -> String {
    iter::repeat(MASK_CHAR).take(str.len()).collect()
}

pub fn gen_pattern(pattern: &str) -> String {
    pattern
        .chars()
        .map(|c| match c {
            '#' => MASK_CHAR,
            other => other,
        })
        .collect()
}
