use regex::Regex;
use std::{fs::File, io::Read};

enum Command {
    Mul(u32, u32),
    Do,
    Dont,
}

fn main() {
    println!("{}", part_1());
    println!("{}", part_2());
}

fn parse_element(element: &str) -> Command {
    if element.contains("mul") {
        // m u l ( d ...  dn   )
        // 0 1 2 3 4 ...  -2  -1
        let split: Vec<&str> = element[4..element.len() - 1].split(",").collect();
        let u1: u32 = split[0].parse().unwrap();
        let u2: u32 = split[1].parse().unwrap();
        Command::Mul(u1, u2)
    } else if element == "do()" {
        Command::Do
    } else {
        Command::Dont
    }
}

fn part_1() -> u32 {
    let mut file = File::open("input_1.txt").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let mut total = 0;
    for mul in re.find_iter(&data) {
        if let Command::Mul(u1, u2) = parse_element(mul.as_str()) {
            total += u1 * u2;
        }
    }
    total
}

fn part_2() -> u32 {
    let mut file = File::open("input_2.txt").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)|don't\(\)|do\(\)").unwrap();
    let commands = re
        .find_iter(&data)
        .map(|match_element| parse_element(match_element.as_str()));

    let mut do_command = true;
    let mut total = 0;

    for command in commands {
        match command {
            Command::Mul(u1, u2) => {
                total += (do_command as u32) * u1 * u2;
            }
            Command::Do => {
                do_command = true;
            }
            Command::Dont => {
                do_command = false;
            }
        }
    }

    total
}
