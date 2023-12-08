use anyhow::{Ok, Result};

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day07.txt")?;
    let start = std::time::Instant::now();

    let mut card_lut = [0u8; 256];
    for (i, b) in b"J23456789T_QKA".iter().enumerate() {
        card_lut[*b as usize] = i as u8;
    }

    let mut hands = Vec::new();
    for line in input.lines() {
        let mut bit_hand = 0; // Bits 4*i..4*i+4 contain the card code.
        let mut count = 0; // Bits 4*i..4*i+4 track number of times card i was counted.
        let mut achieved_count = 5; // Bits 8*c..8*c+8 track number of times count c was achieved.
        let mut is_joker = 0;
        for card_byte in line[..5].as_bytes() {
            let card = card_lut[*card_byte as usize];
            achieved_count -= 1u64 << ((count >> 4 * card) & 0b1111) * 8;
            count += 1u64 << 4 * card;
            achieved_count += 1u64 << ((count >> 4 * card) & 0b1111) * 8;
            bit_hand = (bit_hand << 4) | card as u64;
            is_joker = (is_joker << 4) | (card == 0) as u64;
        }

        let find_best_count = |x: u64| (63 - x.leading_zeros() as u64) / 8;
        let p1_rank1 = find_best_count(achieved_count);
        let p1_rank2 = find_best_count(achieved_count - (1 << 8 * p1_rank1));
        let num_jokers = count & 0b1111;
        achieved_count -= 1u64 << num_jokers * 8;
        let p2_rank1 = find_best_count(achieved_count);
        let p2_rank2 = find_best_count(achieved_count - (1 << 8 * p2_rank1));

        let part1_hand = ((p1_rank1 * 3 + p1_rank2) << 20) | (bit_hand + is_joker * 10);
        let part2_hand = (((p2_rank1 + num_jokers) * 3 + p2_rank2) << 20) | bit_hand;
        let bid: usize = line[6..].parse()?;
        hands.push((part1_hand as u32, part2_hand as u32, bid));
    }

    hands.sort_unstable_by_key(|h| h.0);
    let part1: usize = hands.iter().enumerate().map(|(r, h)| (r + 1) * h.2).sum();
    hands.sort_unstable_by_key(|h| h.1);
    let part2: usize = hands.iter().enumerate().map(|(r, h)| (r + 1) * h.2).sum();

    println!("part1: {part1}");
    println!("part2: {part2}");
    println!("time: {:?}", start.elapsed());
    Ok(())
}
