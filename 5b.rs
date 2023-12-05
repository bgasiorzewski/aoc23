fn main() {
    let lines: Vec<Result<String, std::io::Error>> = std::io::stdin().lines().collect();
    let mut cur: Vec<(i64, i64)> = Vec::new();
    let mut next: Vec<(i64, i64)> = Vec::new();
    let line0 = lines[0].as_ref().unwrap();
    let toks0: Vec<&str> = line0.split_whitespace().skip(1).collect();
    for i in 0..toks0.len()/2 {
        let beg: i64 = toks0[2*i].parse().unwrap();
        let len: i64 = toks0[2*i+1].parse().unwrap();
        cur.push((beg, beg+len));
    }
    for rline in lines.iter().skip(2) {
        let line = rline.as_ref().unwrap();
        if line.is_empty() {
            continue;
        }
        if line.contains(':') {
            next.extend(&cur);
            std::mem::swap(&mut cur, &mut next);
            next.clear();
            continue;
        }
        let toks: Vec<&str> = line.split_whitespace().collect();
        let next_start: i64 = toks[0].parse().unwrap();
        let cur_start: i64 = toks[1].parse().unwrap();
        let len: i64 = toks[2].parse().unwrap();
        for i in 0..cur.len() {
            let overlap_beg = std::cmp::max(cur_start, cur[i].0);
            let overlap_end = std::cmp::min(cur_start + len, cur[i].1);
            let overlap_len = overlap_end - overlap_beg;
            if overlap_len > 0 {
                let delta = overlap_beg - cur_start;
                next.push((next_start + delta, next_start + delta + overlap_len));
                cur.push((overlap_end, cur[i].1));
                cur[i] = (cur[i].0, overlap_beg);
            }
        }
    }
    next.extend(&cur);
    let mut minf = 5_000_000_000;
    for (beg, end) in next {
        if beg < end && beg < minf {
            minf = beg;
        }
    }
    println!("{}", minf);
}
