use std::collections::HashMap;

fn main() {
    let lines: Vec<Result<String, std::io::Error>> = std::io::stdin().lines().collect();
    let mut lefts: HashMap<String, String> = HashMap::new();
    let mut rights: HashMap<String, String> = HashMap::new();
    let line0: Vec<_> = lines[0].as_ref().unwrap().chars().collect();
    let mut pos: Vec<_> = Vec::new();
    for rline in lines.iter().skip(2) {
        let line = rline.as_ref().unwrap();
        let line1 = line.replace(['=', '(', ',', ')'], "");
        let toks: Vec<_> = line1.split_whitespace().collect();
        lefts.insert(toks[0].to_string(), toks[1].to_string());
        rights.insert(toks[0].to_string(), toks[2].to_string());
        if toks[0].chars().last() == Some('A') {
            pos.push(toks[0].to_string());
        }
    }
    let mut num: Vec<_> = vec![0usize; pos.len()];
    for i in 0..pos.len() {
        let mut dir = 0usize;
        while pos[i].chars().last() != Some('Z') {
            if line0[dir] == 'L' {
                pos[i] = lefts[&pos[i]].clone();
            } else {
                pos[i] = rights[&pos[i]].clone();
            }
            num[i] += 1;
            dir = (dir + 1) % line0.len();
        }
    }
    let mut lcm = 1usize;
    for n in num {
        lcm = num::integer::lcm(lcm, n);
    }
    println!("{}", lcm);
}
