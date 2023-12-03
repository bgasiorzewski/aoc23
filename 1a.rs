use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let mut total = 0;
    for res_line in stdin.lock().lines() {
        let line = res_line.unwrap();
        let mut opt_first = None;
        let mut last = '\0';
        // println!("{}", line);
        for ch in line.chars() {
            if ch < '0' || ch > '9' {
                continue;
            }
            if opt_first.is_none() {
                opt_first = Some(ch);
            }
            last = ch;
        }
        let first = opt_first.unwrap();
        // println!("{} {}", first, last);
        let val: i32 = format!("{}{}", first, last).parse().unwrap();
        // println!("{}", val);
        total += val;
    }
    println!("{}", total);
}
