use std::fs::File;
use std::io::{BufRead, BufReader};

fn check_condition(vec: &Vec<i32>) -> bool {
    if vec.windows(2).all(|w| w[0] > w[1]) {
        if vec
            .windows(2)
            .all(|w| 0 < (w[0] - w[1]).abs() && (w[0] - w[1]).abs() <= 3)
        {
            return true;
        }
    } else if vec.windows(2).all(|w| w[0] < w[1]) {
        if vec
            .windows(2)
            .all(|w| 0 < (w[0] - w[1]).abs() && (w[0] - w[1]).abs() <= 3)
        {
            return true;
        }
    }
    let mut vec = vec.clone();
    for i in 0..vec.len() {
        let value = vec.remove(i);

        if vec.windows(2).all(|w| w[0] > w[1]) {
            if vec
                .windows(2)
                .all(|w| 0 < (w[0] - w[1]).abs() && (w[0] - w[1]).abs() <= 3)
            {
                return true;
            }
        } else if vec.windows(2).all(|w| w[0] < w[1]) {
            if vec
                .windows(2)
                .all(|w| 0 < (w[0] - w[1]).abs() && (w[0] - w[1]).abs() <= 3)
            {
                return true;
            }
        }
        vec.insert(i, value);
    }
    false
}

fn main() -> std::io::Result<()> {
    let file_name = "input.txt";
    let f = File::open(file_name)?;
    let f = BufReader::new(f);

    let mut res = 0;
    for line in f.lines() {
        let data: Vec<i32> = line
            .unwrap()
            .split(' ')
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        res += if check_condition(&data) {
            print!("{:?}\n", data);
            1
        } else {
            0
        };
    }
    print!("{res}");

    Ok(())
}
