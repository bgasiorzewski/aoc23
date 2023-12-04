use std::collections::HashSet;

fn main() {
    let mut total = 0i64;
    for rline in std::io::stdin().lines() {
        let line = rline.unwrap();
        let mut stage1 = true;
        let mut matches = 0;
        let mut winning = HashSet::new();
        for token in line.split_whitespace().skip(2) {
            if token == "|" {
                stage1 = false;
            } else if stage1 {
                winning.insert(token);
            } else {
                matches += winning.contains(token) as i64;
            }
        }
        total += 1i64 << matches >> 1;
    }
    println!("{}", total);
}
