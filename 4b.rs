use std::collections::HashSet;

fn main() {
    let mut i = 0;
    let lines: Vec<Result<String, std::io::Error>> = std::io::stdin().lines().collect();
    let mut bonuses: Vec<usize> = vec![1; lines.len()];
    for rline in lines {
        let line = rline.unwrap();
        let mut stage1 = true;
        let mut matches = 0;
        let mut winning = HashSet::new();
        for token in line.split_whitespace().skip(2) {
            if token == "|" {
                stage1 = false;
            } else if stage1 {
                winning.insert(token);
            } else if winning.contains(token) {
                matches += 1;
                if i + matches < bonuses.len() {
                    bonuses[i + matches] += bonuses[i];
                }
            }
        }
        i += 1;
    }
    println!("{}", bonuses.iter().sum::<usize>());
}
