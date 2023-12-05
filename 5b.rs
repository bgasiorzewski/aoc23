use std::collections::HashSet;

fn main() {
    let mut i = 0;
    let lines: Vec<Result<String, std::io::Error>> = std::io::stdin().lines().collect();
    let mut cur: Vec<(i64, i64)> = Vec::new();
    let mut next: Vec<(i64, i64)> = Vec::new();
    let l0 = lines[0].as_ref().unwrap().clone();
    let line0: Vec<&str> = l0.split_whitespace().skip(1).collect();
    for i in 0..line0.len()/2 {
        let beg: i64 = line0[2*i].parse().unwrap();
        let len: i64 = line0[2*i+1].parse().unwrap();
        cur.push((beg, beg+len));
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
        for i in 0..cur.len() {
            let overlap_beg = std::cmp::max(from_start, cur[i].0);
            let overlap_end = std::cmp::min(from_start + len, cur[i].1);
            if overlap_beg < overlap_end {
                let delta = overlap_beg - from_start;
                next.push((to_start + delta, to_start + delta + (overlap_end - overlap_beg)));
                cur.push((overlap_end, cur[i].1));
                cur[i] = (cur[i].0, overlap_beg);
            }
        }
        println!("cur {:?}", cur);
        println!("nxt {:?}", next);
    }
    let mut minf = 5_000_000_000;
    for (beg, end) in cur {
        if beg < end && beg < minf {
            minf = beg;
        }
    }
    println!("{}", minf);
}
