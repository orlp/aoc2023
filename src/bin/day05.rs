use anyhow::{Ok, Result};
use aoc2023::OptionSomeExt;
use itertools::Itertools;

fn min_location_rec(mut range: (i64, i64), maps: &[Vec<(i64, i64, i64)>]) -> i64 {
    if range.0 >= range.1 {
        return i64::MAX;
    }
    let Some(map) = maps.first() else {
        return range.0;
    };

    let mut bound = i64::MAX;
    for (dst, src, len) in map.iter().copied() {
        let (start, stop) = range;
        if start >= stop {
            return bound;
        }
        let before = (start.min(src), stop.min(src));
        let overlap = (start.max(src), stop.min(src + len));
        let after = (start.max(src + len), stop.max(src + len));

        bound = bound.min(min_location_rec(before, &maps[1..]));
        let map_overlap = (overlap.0 - src + dst, overlap.1 - src + dst);
        bound = bound.min(min_location_rec(map_overlap, &maps[1..]));
        range = after;
    }

    bound.min(min_location_rec(range, &maps[1..]))
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day05.txt")?;
    let start = std::time::Instant::now();

    let (seed_line, map_data) = input.split_once("\n").some()?;
    let seed_data = seed_line.split_once(":").some()?.1.split_ascii_whitespace();
    let seeds: Vec<i64> = seed_data.map(|s| s.parse()).try_collect()?;

    let mut maps: Vec<Vec<(i64, i64, i64)>> = Vec::new();
    let map_lines = map_data.split("\n").map(str::trim).filter(|s| s.len() > 0);
    for map_line in map_lines {
        if map_line.ends_with("map:") {
            maps.push(Vec::new());
            continue;
        }

        let nums = map_line.split_ascii_whitespace().map(str::parse);
        let (dst, src, len) = nums.collect_tuple().some()?;
        maps.last_mut().unwrap().push((dst?, src?, len?));
    }

    for map in maps.iter_mut() {
        map.sort_unstable_by_key(|(_dst, src, _len)| *src);
    }

    let min_loc = |r| min_location_rec(r, &maps);
    let part1 = seeds.iter().map(|s| min_loc((*s, *s + 1)));
    let part2 = seeds.chunks_exact(2).map(|c| min_loc((c[0], c[0] + c[1])));

    println!("part1: {}", part1.min().some()?);
    println!("part2: {}", part2.min().some()?);
    println!("time: {:?}", start.elapsed());
    Ok(())
}
