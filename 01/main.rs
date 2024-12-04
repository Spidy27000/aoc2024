use std::{
    fs::File,
    io::{self, Read},
};

fn main() -> io::Result<()> {
    let file_name = "input.txt";
    let mut right_arr: Vec<i32> = Vec::new();
    let mut left_arr: Vec<i32> = Vec::new();

    let mut f = File::open(file_name)?;
    let mut content = String::new();

    f.read_to_string(&mut content)?;

    let content: Vec<String> = content.split("\n").map(|x| x.to_string()).collect();

    for it in content {
        if it == "" {
            break;
        }

        let arr: Vec<i32> = it.split(' ').map(|x| x.parse::<i32>().unwrap()).collect();
        left_arr.push(arr[0]);
        right_arr.push(arr[1]);
    }
    left_arr.sort();
    right_arr.sort();

    // part one
    //let mut result = 0;
    //for i in 0..left_arr.len() {
    //    result += (left_arr[i] - right_arr[i]).abs();
    //}

    //part2
    let mut result = 0;
    for it in left_arr {
        let count = right_arr.iter().filter(|x| **x == it).count();
        result += it * count as i32;
    }

    println!("{result}");

    Ok(())
}
