use anyhow::{Ok, Result};
use aoc2023::OptionSomeExt;

const ENGLISH_NUMBERS: [&'static str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day01.txt")?;
    let start = std::time::Instant::now();

    let mut part1 = 0;
    let mut part2 = 0;
    for l in input.lines() {
        let bytes = l.as_bytes();
        let first_idx = bytes.iter().position(u8::is_ascii_digit).some()?;
        let last_idx = bytes.iter().rposition(u8::is_ascii_digit).some()?;
        let first_digit = usize::from(bytes[first_idx] - b'0');
        let last_digit = usize::from(bytes[last_idx] - b'0');
        part1 += first_digit * 10 + last_digit;

        let start_val = |s: &str| ENGLISH_NUMBERS.iter().position(|w| s.starts_with(w));
        let end_val = |s: &str| ENGLISH_NUMBERS.iter().position(|w| s.ends_with(w));
        let first_word = (0..first_idx).find_map(|i| start_val(&l[i..first_idx]));
        let last_word = (0..l.len() - last_idx).find_map(|i| end_val(&l[last_idx..l.len() - i]));
        part2 += first_word.map(|w| w + 1).unwrap_or(first_digit) * 10
            + last_word.map(|w| w + 1).unwrap_or(last_digit);
    }

    println!("part1: {part1}");
    println!("part2: {part2}");
    println!("time: {:?}", start.elapsed());
    Ok(())
}
