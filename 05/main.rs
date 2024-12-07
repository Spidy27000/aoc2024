use std::fs::File;
use std::io::Read;

#[derive(Debug)]
struct Rule {
    first: i32,
    second: i32,
}
#[derive(Debug)]
struct Rules {
    rules: Vec<Rule>,
}

impl Rule {
    fn new(first: i32, second: i32) -> Self {
        Rule { first, second }
    }
    fn equals(&self, other: &Rule) -> bool {
        (self.first == other.first) && (self.second == other.second)
    }
}
impl Rules {
    fn new(rules: &str) -> Self {
        Rules {
            rules: rules
                .split('\n')
                .map(|x| {
                    x.split("|")
                        .map(|x| x.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>()
                })
                .collect::<Vec<Vec<i32>>>()
                .into_iter()
                .map(|x| Rule::new(x[0], x[1]))
                .collect(),
        }
    }
    fn rule_exists(&self, rule: &Rule) -> bool {
        self.rules.iter().filter(|x| x.equals(rule)).count() > 0
    }
}

fn solve_part1(rules: &Rules, input: &str) -> i32 {
    let inputs = input.split("\n").collect::<Vec<&str>>();
    let mut result = 0;
    for input in inputs.iter() {
        if *input == "" {
            break;
        }
        let data = input
            .split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let mut is_valid: bool = true;
        for i in 0..data.len() {
            for j in i + 1..data.len() {
                let rule = Rule::new(data[j], data[i]);
                if rules.rule_exists(&rule) {
                    is_valid = false;
                    break;
                }
            }
            if !is_valid {
                break;
            }
        }
        if is_valid {
            let len = data.len();
            println!("{input}");
            result += data[(len / 2) as usize];
        }
    }
    result
}

fn solve_part2(rules: &Rules, input: &str) -> i32 {
    let inputs = input.split("\n").collect::<Vec<&str>>();
    let mut result = 0;
    for input in inputs.iter() {
        if *input == "" {
            break;
        }
        let mut data = input
            .split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let mut is_valid: bool = true;
        for i in 0..data.len() {
            for j in i + 1..data.len() {
                let rule = Rule::new(data[j], data[i]);
                if rules.rule_exists(&rule) {
                    let temp = data[j];
                    data[j] = data[i];
                    data[i] = temp;
                    is_valid = false;
                    continue;
                }
            }
        }
        if !is_valid {
            println!("{input}");
            println!("{:?}", data);
            let len = data.len();
            result += data[(len / 2) as usize];
        }
    }
    result
}

fn main() -> std::io::Result<()> {
    let file_name = "input.txt";
    let mut file = File::open(file_name)?;
    let mut content = String::new();

    file.read_to_string(&mut content)?;
    let (rules, input) = content.split_once("\n\n").unwrap();
    let rules = Rules::new(rules);

    let sol1 = solve_part1(&rules, input);
    let sol2 = solve_part2(&rules, input);
    println!("{sol1}\n{sol2}");

    Ok(())
}
