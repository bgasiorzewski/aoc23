use std::collections::HashMap;

fn main() {
    let caps = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let mut total = 0;
    let mut game = 1;
    for rline in std::io::stdin().lines() {
        let mut bounty = true;
        let line = rline.unwrap();
        let cline = line.replace([':', ',', ';', '\n'], "");
        let tokens: Vec<&str> = cline.split_whitespace().collect();
        for i in 1..tokens.len()/2 {
            let count: i64 = tokens[i*2].parse().unwrap();
            let colour = tokens[i*2+1];
            if count > caps[colour] {
                bounty = false;
                break;
            }
        }
        if bounty {
            total += game;
        }
        game += 1;
    }
    println!("{}", total);
}
