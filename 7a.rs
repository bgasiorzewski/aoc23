use std::cmp::Ordering;
use std::convert::TryInto;

fn rank_hand(s: &[u8; 5]) -> i64 {
    let mut groups: Vec<i64> = Vec::new();
    let mut chars: [u8; 5] = s.clone();
    chars.sort();
    let mut last = 255;
    for i in 0..chars.len() {
        if chars[i] == last {
            *groups.last_mut().unwrap() += 1;
        } else {
            groups.push(1);
            last = chars[i];
        }
    }
    groups.sort();
    groups.reverse();
    if groups[0] == 5 {
        6
    } else if groups[0] == 4 {
        5
    } else if groups[0] == 3 && groups[1] == 2 {
        4
    } else if groups[0] == 3 {
        3
    } else if groups[0] == 2 && groups[1] == 2 {
        2
    } else if groups[0] == 2 {
        1
    } else {
        0
    }
}

fn to_rank(c: char) -> u8 {
    match c {
        'A' => 12,
        'K' => 11,
        'Q' => 10,
        'J' => 9,
        'T' => 8,
        _ => c as u8 - '2' as u8,
    }
}

fn cmp_hands(a: &([u8; 5], usize), b: &([u8; 5], usize)) -> Ordering {
    let ranka = rank_hand(&a.0);
    let rankb = rank_hand(&b.0);
    if ranka < rankb {
        Ordering::Less
    } else if ranka > rankb {
        Ordering::Greater
    } else {
        a.0.cmp(&b.0)
    }
}

fn main() {
    let lines: Vec<Result<String, std::io::Error>> = std::io::stdin().lines().collect();
    let mut pairs: Vec<([u8; 5], usize)> = Vec::new();
    for rline in &lines {
        let line: &str = &rline.as_ref().unwrap();
        let toks: Vec<_> = line.split_whitespace().collect();
        let cards = toks[0];
        let bid = toks[1].parse().unwrap();
        pairs.push((cards.chars().map(to_rank).collect::<Vec<_>>().try_into().unwrap(), bid));
    }
    pairs.sort_by(cmp_hands);
    let mut total = 0usize;
    for i in 0..pairs.len() {
        total += pairs[i].1 * (i + 1);
    }
    println!("{}", total);
}
