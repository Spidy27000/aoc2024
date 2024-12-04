use std::fs::File;
use std::io::{self, Read};

fn solve_part2(input: String) -> i32 {
    let mut idx: usize = 0;
    let mut x = 0;
    let mut y = 0;
    let mut res = 0;
    let mut enable: bool = true;
    loop {
        if idx > input.len() - 7 {
            break;
        }
        if &input[idx..idx + 4] == "do()" {
            enable = true;
        }
        if &input[idx..idx + 7] == "don't()" {
            enable = false;
        }

        if !enable {
            idx += 1;
            continue;
        }

        // mul(
        if !(&input[idx..idx + 4] == "mul(") {
            idx += 1;
            continue;
        }
        idx += 4;

        // x

        if !((&input[idx..idx + 1]).parse::<i32>().is_ok()) {
            idx += 1;
            continue;
        }
        let mut inc = 0;
        while (&input[idx + inc..idx + 1 + inc]).parse::<i32>().is_ok() {
            inc += 1;
        }
        x = (&input[idx..idx + inc]).parse::<i32>().unwrap();
        idx += inc;
        // ,

        if !(&input[idx..idx + 1] == ",") {
            idx += 1;
            x = 0;
            continue;
        }
        idx += 1;
        // y

        if !((&input[idx..idx + 1]).parse::<i32>().is_ok()) {
            idx += 1;
            x = 0;
            continue;
        }

        let mut inc = 0;

        while (&input[idx + inc..idx + 1 + inc]).parse::<i32>().is_ok() {
            inc += 1;
        }

        y = (&input[idx..idx + inc]).parse::<i32>().ok().unwrap();
        idx += inc;
        // )

        if !(&input[idx..idx + 1] == ")") {
            x = 0;
            y = 0;
            idx += 1;
            continue;
        }

        idx += 1;
        res += x * y;
    }
    res
}

fn solve_part1(input: String) -> i32 {
    let mut idx: usize = 0;
    let mut x = 0;
    let mut y = 0;
    let mut res = 0;
    loop {
        if idx > input.len() - 7 {
            break;
        }
        // mul(
        if !(&input[idx..idx + 4] == "mul(") {
            idx += 1;
            continue;
        }
        idx += 4;

        // x

        if !((&input[idx..idx + 1]).parse::<i32>().is_ok()) {
            idx += 1;
            continue;
        }
        let mut inc = 0;
        while (&input[idx + inc..idx + 1 + inc]).parse::<i32>().is_ok() {
            inc += 1;
        }
        x = (&input[idx..idx + inc]).parse::<i32>().unwrap();
        idx += inc;
        // ,

        if !(&input[idx..idx + 1] == ",") {
            idx += 1;
            x = 0;
            continue;
        }
        idx += 1;
        // y

        if !((&input[idx..idx + 1]).parse::<i32>().is_ok()) {
            idx += 1;
            x = 0;
            continue;
        }

        let mut inc = 0;

        while (&input[idx + inc..idx + 1 + inc]).parse::<i32>().is_ok() {
            inc += 1;
        }

        y = (&input[idx..idx + inc]).parse::<i32>().ok().unwrap();
        idx += inc;
        // )

        if !(&input[idx..idx + 1] == ")") {
            x = 0;
            y = 0;
            idx += 1;
            continue;
        }

        idx += 1;
        res += x * y;
    }
    res
}

fn main() -> io::Result<()> {
    let mut f = File::open("input.txt")?;
    let mut buff = String::new();
    f.read_to_string(&mut buff)?;
    let x = solve_part1(buff.clone());
    let y = solve_part2(buff);
    print!("{x}, {y}");
    Ok(())
}
