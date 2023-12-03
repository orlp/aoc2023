use anyhow::{Ok, Result};
use hashbrown::HashMap;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day03.txt")?;
    let start = std::time::Instant::now();

    let lines = input.lines().collect_vec();
    let (w, h) = (lines[0].len(), lines.len());

    let mut part1 = 0;
    let mut gear_values: HashMap<_, Vec<_>> = HashMap::new();
    for (y, l) in lines.iter().enumerate() {
        let mut x = 0;
        while x < w {
            let num_len = l[x..].find(|c: char| !c.is_ascii_digit()).unwrap_or(w - x);
            if num_len > 0 {
                let n = l[x..x + num_len].parse::<i64>().unwrap();

                let mut valid = false;
                for ny in y as i64 - 1..=y as i64 + 1 {
                    for nx in x as i64 - 1..=(x + num_len) as i64 {
                        if 0 <= ny && ny < h as i64 && 0 <= nx && nx < w as i64 {
                            let b = lines[ny as usize].as_bytes()[nx as usize];
                            if b == b'*' {
                                gear_values.entry((nx, ny)).or_default().push(n);
                            }
                            valid |= b != b'.' && !b.is_ascii_digit();
                        }
                    }
                }

                part1 += if valid { n } else { 0 };
            }

            x += num_len + 1;
        }
    }

    let mut part2 = 0;
    for val in gear_values.values() {
        if let &[a, b] = val.as_slice() {
            part2 += a * b;
        }
    }

    println!("part1: {part1}");
    println!("part2: {part2}");
    println!("time: {:?}", start.elapsed());
    Ok(())
}
