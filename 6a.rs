fn main() {
    let lines: Vec<Result<String, std::io::Error>> = std::io::stdin().lines().collect();
    let mut time: Vec<i64> = Vec::new();
    let mut record: Vec<i64> = Vec::new();
    let mut total = 1i64;
    let line0 = lines[0].as_ref().unwrap();
    let line1 = lines[1].as_ref().unwrap();
    for tok in line0.split_whitespace().skip(1) {
        time.push(tok.parse().unwrap())
    }
    for tok in line1.split_whitespace().skip(1) {
        record.push(tok.parse().unwrap())
    }
    for i in 0..time.len() {
        let mut count = 0i64;
        for holdt in 0..time[i] {
            let dist = (time[i] - holdt) * holdt;
            if dist > record[i] {
                count += 1;
            }
        }
        total *= count;
    }
    println!("{}", total);
}
