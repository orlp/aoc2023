use anyhow::{Ok, Result};
use aoc2023::OptionSomeExt;
use itertools::Itertools;

fn winning_ways(t: i64, r: i64) -> i64 {
    // Time t, record r, holding time h.
    // (t - h) * h > r    -->    r bounded by (t +/- sqrt(t^2 - 4r)) / 2.
    let d = f64::sqrt((t * t - (4 * r)) as f64);
    let mut lo = ((t as f64 - d) / 2.0).ceil() as i64;
    let mut hi = ((t as f64 + d) / 2.0).floor() as i64;
    lo += ((t - lo) * lo <= r) as i64; // We must beat the time, not equal it.
    hi -= ((t - hi) * hi <= r) as i64;
    hi - lo + 1
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day06.txt")?;
    let start = std::time::Instant::now();

    let mut line_data = input.lines().map(|l| {
        let nums = l.split_once(":").some()?.1.split_ascii_whitespace();
        Ok(nums.map(str::parse).try_collect()?)
    });
    let times: Vec<i64> = line_data.next().some()??;
    let distances: Vec<i64> = line_data.next().some()??;

    let races = times.iter().copied().zip(distances.iter().copied());
    let part1: i64 = races.map(|(t, r)| winning_ways(t, r)).product();
    let concat = |v: &[i64]| -> i64 { v.iter().join("").parse().unwrap() };
    let part2 = winning_ways(concat(&times), concat(&distances));

    println!("part1: {part1}");
    println!("part2: {part2}");
    println!("time: {:?}", start.elapsed());
    Ok(())
}
