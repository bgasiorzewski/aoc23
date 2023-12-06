fn main() {
    let lines: Vec<Result<String, std::io::Error>> = std::io::stdin().lines().collect();
    let line0 = lines[0].as_ref().unwrap().split_whitespace().skip(1).collect::<Vec<&str>>().join("");
    let line1 = lines[1].as_ref().unwrap().split_whitespace().skip(1).collect::<Vec<&str>>().join("");
    let time: i64 = line0.parse().unwrap();
    let record: i64 = line1.parse().unwrap();
    let mut count = 0i64;
    for holdt in 0..time {
        let dist = (time - holdt) * holdt;
        if dist > record {
            count += 1;
        }
    }
    println!("{}", count);
}
