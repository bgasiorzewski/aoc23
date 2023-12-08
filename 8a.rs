use std::collections::HashMap;

fn main() {
    let lines: Vec<Result<String, std::io::Error>> = std::io::stdin().lines().collect();
    let mut lefts: HashMap<String, String> = HashMap::new();
    let mut rights: HashMap<String, String> = HashMap::new();
    let line0: Vec<_> = lines[0].as_ref().unwrap().chars().collect();
    for rline in lines.iter().skip(2) {
        let line = rline.as_ref().unwrap();
        let line1 = line.replace(['=', '(', ',', ')'], "");
        let toks: Vec<_> = line1.split_whitespace().collect();
        lefts.insert(toks[0].to_string(), toks[1].to_string());
        rights.insert(toks[0].to_string(), toks[2].to_string());
    }
    let mut step = 0i64;
    let mut pos = "AAA";
    let mut dir = 0usize;
    while pos != "ZZZ" {
        if line0[dir] == 'L' {
            pos = &lefts[pos];
        } else {
            pos = &rights[pos];
        }
        step += 1;
        dir = (dir + 1) % line0.len();
    }
    println!("{}", step);
}
