fn main() {
    let lines: Vec<Result<String, std::io::Error>> = std::io::stdin().lines().collect();
    let mut total = 0i64;
    for rline in &lines {
        let line: &str = &rline.as_ref().unwrap();
        let mut toks: Vec<i64> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
        let mut lasts: Vec<i64> = Vec::new();
        let mut nonzero = true;
        let mut n = 1;
        while nonzero {
            nonzero = false;
            lasts.push(toks[toks.len()-n]);
            for i in 0..toks.len()-n {
                let d = toks[i+1] - toks[i];
                toks[i] = d;
                if d != 0 {
                    nonzero = true;
                }
            }
            n += 1;
        }
        total += lasts.iter().sum::<i64>();
    }
    println!("{}", total);
}
