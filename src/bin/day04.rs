use anyhow::{Ok, Result};
use aoc2023::OptionSomeExt;
use hashbrown::HashSet;
use itertools::{process_results, Itertools};
use std::collections::VecDeque;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day04.txt")?;
    let start = std::time::Instant::now();

    let mut num_cur_card = 1;
    let mut expiring_cards = VecDeque::new();
    let mut part1 = 0;
    let mut part2 = 0;
    for line in input.lines() {
        let (_card, numbers) = line.split_once(":").some()?;
        let (win_nums, our_nums) = numbers.split_once("|").some()?;
        let winning = win_nums.split_ascii_whitespace().map(|s| s.parse::<i64>());
        let ours = our_nums.split_ascii_whitespace().map(|s| s.parse::<i64>());
        let winning_set: HashSet<i64> = winning.try_collect()?;
        let num_wins = process_results(ours, |o| o.filter(|n| winning_set.contains(n)).count())?;

        if num_wins > 0 {
            part1 += 2_i64.pow(num_wins as u32 - 1)
        }
        part2 += num_cur_card;
        expiring_cards.resize(expiring_cards.len().max(num_wins + 1), 0);
        expiring_cards[num_wins] += num_cur_card;
        num_cur_card += num_cur_card;
        num_cur_card -= expiring_cards.pop_front().unwrap();
    }

    println!("part1: {part1}");
    println!("part2: {part2}");
    println!("time: {:?}", start.elapsed());
    Ok(())
}
