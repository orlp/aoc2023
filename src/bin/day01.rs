use anyhow::{Ok, Result};
use aoc2023::OptionSomeExt;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

const ENGLISH_NUMBERS: [&'static str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day01.txt")?;
    let start = std::time::Instant::now();

    let values: HashMap<_, _> = ENGLISH_NUMBERS
        .iter()
        .enumerate()
        .map(|(i, w)| (w.to_string(), i as i64))
        .chain((0..10).map(|d| (d.to_string(), d)))
        .collect();

    let first_re = Regex::new(&format!("({}).*$", values.keys().join("|")))?;
    let last_re = Regex::new(&format!("^.*({})", values.keys().join("|")))?;

    let mut part1 = 0;
    let mut part2 = 0;
    for l in input.lines() {
        let bytes = l.as_bytes();
        let first_digit = bytes.iter().position(u8::is_ascii_digit).some()?;
        let last_digit = bytes.iter().rposition(u8::is_ascii_digit).some()?;
        part1 += ((bytes[first_digit] - b'0') * 10 + (bytes[last_digit] - b'0')) as i64;

        let [first_word] = first_re.captures(l).some()?.extract().1;
        let [last_word] = last_re.captures(l).some()?.extract().1;
        part2 += values[first_word] * 10 + values[last_word];
    }

    println!("part1: {part1}");
    println!("part2: {part2}");
    println!("time: {:?}", start.elapsed());
    Ok(())
}
