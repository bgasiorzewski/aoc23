use std::collections::HashMap;

fn main() {
    let mut total = 0;
    for rline in std::io::stdin().lines() {
        let mut mins = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
        let line = rline.unwrap();
        let cline = line.replace([':', ',', ';', '\n'], "");
        let tokens: Vec<&str> = cline.split_whitespace().collect();
        for i in 1..tokens.len()/2 {
            let count: i64 = tokens[i*2].parse().unwrap();
            let colour = tokens[i*2+1];
            let min_count = mins.get_mut(colour).unwrap();
            *min_count = std::cmp::max(*min_count, count);
        }
        total += mins["red"] * mins["green"] * mins["blue"];
    }
    println!("{}", total);
}
