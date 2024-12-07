use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;
const SIZE: usize = 140;

fn get_directions(x: usize, y: usize) -> Vec<[i32; 2]> {
    let mut res: Vec<[i32; 2]> = vec![];
    if x < SIZE - 3 {
        res.push([1, 0]);
    }
    if y < SIZE - 3 {
        res.push([0, 1]);
    }
    if x < SIZE - 3 && y < SIZE - 3 {
        res.push([1, 1]);
    }
    if x > 2 && y < SIZE - 3 {
        res.push([-1, 1]);
    }
    res
}

fn solve_part1(data: &[[char; SIZE]; SIZE]) -> i32 {
    let mut ans = 0;
    for j in 0..SIZE {
        for i in 0..SIZE {
            let dirs = get_directions(i, j);
            for dir in dirs.iter() {
                let mut chars: [char; 4] = [' '; 4];
                for k in 0..4 {
                    let x = ((k * dir[0]) + i as i32) as usize;
                    let y = ((k * dir[1]) + j as i32) as usize;
                    chars[k as usize] = data[y][x];
                }

                let string = String::from_iter(chars.iter());
                if &string == "XMAS" || &string == "SAMX" {
                    ans += 1;
                }
            }
        }
    }
    ans
}

fn main() -> std::io::Result<()> {
    let file_name = "input.txt";
    let f = File::open(file_name)?;
    let f = BufReader::new(f);

    let mut data: [[char; SIZE]; SIZE] = [[' '; SIZE]; SIZE];

    let mut x: usize;
    let mut y: usize = 0;
    for line in f.lines() {
        let line = line.unwrap();
        x = 0;
        for char in line.chars() {
            data[y][x] = char;
            x += 1;
        }
        y += 1;
    }

    let sol1 = solve_part1(&data);
    //let sol2 = solve_part1(&data);
    println!("{sol1}");
    Ok(())
}
