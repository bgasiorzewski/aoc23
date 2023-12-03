use std::io;
use std::io::BufRead;

fn main() {
    let words: [(&str, i64); 19] = [
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];
    let stdin = io::stdin();
    let mut total = 0;
    for res_line in stdin.lock().lines() {
        let line = res_line.unwrap();
        let mut matches = Vec::new();
        for (word, digit) in &words {
            matches.push((line.find(word), digit));
            matches.push((line.rfind(word), digit));
        }
        matches.retain(|(pos, _digit)| pos.is_some());
        matches.sort();
        let first = matches[0].1;
        let last = matches.last().unwrap().1;
        let val: i32 = format!("{}{}", first, last).parse().unwrap();
        // println!("{}", val);
        total += val;
    }
    println!("{}", total);
}
