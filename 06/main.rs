use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};
// input : 130
// sample : 10
const SIZE: usize = 10;

#[derive(Copy, Clone, PartialEq, Debug)]
enum CellType {
    DOT,
    WALL,
    NONE,
}

struct Player {
    x: i16,
    y: i16,
    dir: i8,
}

fn from_pos(x: i16, y: i16) -> i32 {
    let x = x as i32;
    let y = y as i32;
    x << 16 | y
}

fn get_dir(dir: i8) -> [i32; 2] {
    match dir {
        0 => [0, -1],
        1 => [1, 0],
        2 => [0, 1],
        3 => [-1, 0],
        _ => [0, 0],
    }
}

fn solve_part1(map: &[[CellType; SIZE]; SIZE], player: &Player) -> i32 {
    let mut curr_x = player.x;
    let mut curr_y = player.y;
    let mut curr_dir = player.dir;
    let mut pos = from_pos(curr_x, curr_y);
    let mut dir = get_dir(curr_dir);

    let mut visited: HashSet<i32> = HashSet::new();
    visited.insert(pos);
    loop {
        if curr_x <= 0 || curr_x >= (SIZE - 1) as i16 || curr_y < 0 || curr_y >= (SIZE - 1) as i16 {
            break;
        }
        println!(
            "{curr_dir}: {:?}, {:?}",
            dir,
            //map[(curr_y + (dir[1] as i16)) as usize][(curr_x + (dir[0] as i16)) as usize]
            map[curr_y as usize][curr_x as usize]
        );

        if map[(curr_y + (dir[1] as i16)) as usize][(curr_x + (dir[0] as i16)) as usize]
            == CellType::WALL
        {
            curr_dir = (curr_dir + 1) % 4;
            dir = get_dir(curr_dir);
        }
        curr_x += dir[0] as i16;
        curr_y += dir[1] as i16;
        pos = from_pos(curr_x, curr_y);
        visited.insert(pos);
        println!("{:?}", &visited)
    }
    let ans = visited.len();

    ans as i32
}

fn solve_part2(map: &[[CellType; SIZE]; SIZE], player: &Player) -> i32 {
    let ans = 0;
    ans
}

fn main() -> std::io::Result<()> {
    let file_name = "sample.txt";
    let file = File::open(file_name)?;
    let file = BufReader::new(file);

    let mut player = Player { x: 0, y: 0, dir: 0 };
    let mut map = [[CellType::NONE; SIZE]; SIZE];
    for (j, line) in file.lines().enumerate() {
        let line = line.unwrap();
        for (i, char) in line.chars().enumerate() {
            if char == '^' {
                player.x = i as i16;
                player.y = j as i16;
            }
            map[j][i] = match char {
                '#' => CellType::WALL,
                '.' => CellType::DOT,
                _ => CellType::DOT,
            }
        }
    }

    let ans1 = solve_part1(&map, &player);
    let ans2 = solve_part2(&map, &player);

    println!("{ans1}\n{ans2}");
    Ok(())
}
