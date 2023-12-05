use std::collections::HashSet;

fn main() {
    let mut i = 0;
    let lines: Vec<Result<String, std::io::Error>> = std::io::stdin().lines().collect();
    let mut cur: Vec<i64> = Vec::new();
    let mut next: Vec<i64> = Vec::new();
    let line0 = lines[0].as_ref().unwrap().clone();
    for tok in line0.split_whitespace().skip(1) {
        cur.push(tok.parse().unwrap())
    }
    println!("{:?}", cur);
    for rline in lines.iter().skip(2) {
        let line = rline.as_ref().unwrap();
        println!("{:?}", line);
        if line.is_empty() {
            next.extend(cur);
            cur = next.clone();
            next.clear();
            continue;
        }
        if line.contains(':') {
            continue;
        }
        let toks: Vec<&str> = line.split_whitespace().collect();
        // println!("{:?}", toks);
        let to_start: i64 = toks[0].parse().unwrap();
        let from_start: i64 = toks[1].parse().unwrap();
        let len: i64 = toks[2].parse().unwrap();
        let mut rem: Vec<i64> = Vec::new();
        for v in &cur {
            if from_start <= *v && *v < from_start + len {
                rem.push(*v);
            }
        }
        for v in rem {
            cur.retain(|x| *x != v);
            next.push(to_start + v - from_start);
        }
        println!("{:?}", cur);
    }
    println!("{}", cur.iter().min().unwrap());
}
