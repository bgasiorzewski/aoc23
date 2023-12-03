// idea: scan character by character and assemble numbers manually, this will make the whole process easy
// idea: create a Vec of (parsed number, `valid` bit)
// each entry in 2d map is one of: index of a parsed number, symbol, or empty
// encountering a symbol means validating all adjacent parsed numbers
#[derive(Clone, Copy, Eq, PartialEq)]
enum Entry {
    Digit(usize),
    Symbol,
    Dot,
}

fn main() {
    let mut numbers: Vec<(i64, bool)> = Vec::new();
    let mut map: Vec<Vec<Entry>> = Vec::new();
    for rline in std::io::stdin().lines() {
        let mut row = Vec::new();
        let mut counting = false;
        let line = rline.unwrap();
        for ch in line.chars() {
            if ch.is_digit(10) {
                let digit: i64 = ch.to_digit(10).unwrap().into();
                if counting {
                    let last_number = numbers.last_mut().unwrap();
                    let number = last_number.0 * 10 + digit;
                    *last_number = (number, false);
                    row.push(*row.last().unwrap());
                } else {
                    counting = true;
                    numbers.push((digit, false));
                    row.push(Entry::Digit(numbers.len()-1));
                }
            } else if ch == '.' {
                counting = false;
                row.push(Entry::Dot);
            } else {
                counting = false;
                row.push(Entry::Symbol);
            }
        }
        map.push(row);
    }
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            if map[row][col] == Entry::Symbol {
                for drow in -1..2 {
                    for dcol in -1..2 {
                        if drow == 0 && dcol == 0 {
                            continue;
                        }
                        let nrow = (row as i64 + drow) as usize;
                        let ncol = (col as i64 + dcol) as usize;
                        if nrow >= map.len() || ncol >= map[row].len() {
                            continue;
                        }
                        let entry = map[nrow][ncol];
                        if let Entry::Digit(ix) = entry {
                            numbers[ix] = (numbers[ix].0, true);
                        }
                    }
                }
            }
        }
    }
    let mut total = 0;
    for (number, valid) in numbers {
        if valid {
            total += number;
        }
    }
    println!("{}", total);
}
