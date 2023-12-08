use anyhow::{Ok, Result};

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day07.txt")?;
    let start = std::time::Instant::now();

    let mut hands = Vec::new();
    for l in input.lines() {
        let (hand_str, bid) = l.split_at(6);
        let strength = |c| b"J23456789T_QKA".iter().position(|b| *b == c).unwrap() as u8;
        let hand: [u8; 5] = std::array::from_fn(|i| strength(hand_str.as_bytes()[i]));

        let mut bit_hand = 0; // Bits 4*i..4*i+4 contain the card code.
        let mut count = 0; // Bits 4*i..4*i+4 track number of times card i was counted.
        let mut achieved_count = 5; // Bits 8*c..8*c+8 track number of times count c was achieved.
        let mut is_joker = 0;
        for s in hand {
            let w = 4 * s;
            achieved_count -= 1u64 << ((count >> w) & 0b1111) * 8;
            count += 1u64 << w;
            achieved_count += 1u64 << ((count >> w) & 0b1111) * 8;
            bit_hand = (bit_hand << 4) | s as u64;
            is_joker = (is_joker << 4) | (s == 0) as u64;
        }

        let find_best_count = |x: u64| (63 - x.leading_zeros() as u64) / 8;
        let p1_rank1 = find_best_count(achieved_count);
        let p1_rank2 = find_best_count(achieved_count - (1 << 8 * p1_rank1));
        let num_jokers = count & 0b1111;
        achieved_count -= 1u64 << num_jokers * 8;
        let p2_rank1 = find_best_count(achieved_count);
        let p2_rank2 = find_best_count(achieved_count - (1 << 8 * p2_rank1));

        let part1_hand = ((p1_rank1 * 3 + p1_rank2) << 32) | (bit_hand + is_joker * 10);
        let part2_hand = (((p2_rank1 + num_jokers) * 3 + p2_rank2) << 32) | bit_hand;
        hands.push((part1_hand, part2_hand, bid.parse()?));
    }

    let winnings = |hands: &[(u64, u64, usize)]| -> usize {
        hands.iter().enumerate().map(|(r, h)| (r + 1) * h.2).sum()
    };
    hands.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    let part1 = winnings(&hands);
    hands.sort_unstable_by(|a, b| a.1.cmp(&b.1));
    let part2 = winnings(&hands);

    println!("part1: {part1}");
    println!("part2: {part2}");
    println!("time: {:?}", start.elapsed());
    Ok(())
}
