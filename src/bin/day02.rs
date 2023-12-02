use anyhow::{Ok, Result};
use aoc2023::OptionSomeExt;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day02.txt")?;
    let start = std::time::Instant::now();

    let mut part1 = 0;
    let mut part2 = 0;
    for l in input.lines() {
        let (game, pulls) = l.trim().split_once(':').some()?;
        let game_id: i64 = game.strip_prefix("Game").some()?.trim().parse()?;

        let colors = ["red", "green", "blue"];
        let mut color_bounds = [0, 0, 0];
        for pull in pulls.split(';') {
            for cube in pull.split(',') {
                let (count, color) = cube.trim().split_once(' ').some()?;
                let color_idx = colors.iter().position(|c| *c == color).some()?;
                color_bounds[color_idx] = color_bounds[color_idx].max(count.parse()?);
            }
        }

        let possible = color_bounds[0] <= 12 && color_bounds[1] <= 13 && color_bounds[2] <= 14;
        part1 += if possible { game_id } else { 0 };
        part2 += color_bounds.iter().product::<i64>();
    }

    println!("part1: {part1}");
    println!("part2: {part2}");
    println!("time: {:?}", start.elapsed());
    Ok(())
}
