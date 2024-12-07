use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    println!("{}", part_1());
    println!("{}", part_2());
}

fn report_is_safe(report: &[i32]) -> bool {
    let monotonic = report.iter().is_sorted() || report.iter().rev().is_sorted();
    let differ_by_1_to_3 = report
        .windows(2)
        .map(|nums| match nums {
            &[num1, num2] => {
                let diff = (num2 - num1).abs();
                diff >= 1 && diff <= 3
            }
            _ => panic!("Invalid input"),
        })
        .all(|cond| cond);
    monotonic && differ_by_1_to_3
}

fn report_is_1_away_from_safe(report: &[i32]) -> bool {
    if report_is_safe(report) {
        return true;
    }

    // Try 1 element modifications
    for ignore_idx in 0..report.len() {
        let modified_report = &report
            .iter()
            .enumerate()
            .filter_map(|(i, elem)| match i != ignore_idx {
                true => Some(*elem),
                false => None,
            })
            .collect::<Vec<i32>>();
        if report_is_safe(&modified_report) {
            return true;
        }
    }
    false
}

fn part_1() -> usize {
    let file = File::open("input_1.txt").expect("File not found");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| {
            let report: Vec<i32> = line
                .unwrap()
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();
            report_is_safe(&report)
        })
        .filter(|cond| *cond)
        .count()
}

fn part_2() -> usize {
    let file = File::open("input_2.txt").expect("File not found");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| {
            let report: Vec<i32> = line
                .unwrap()
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();
            report_is_1_away_from_safe(&report)
        })
        .filter(|cond| *cond)
        .count()
}
