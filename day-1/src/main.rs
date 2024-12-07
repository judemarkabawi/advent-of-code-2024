use std::{fs::File, io::Read, iter::zip};

fn main() {
    println!("{}", part_1());
    println!("{}", part_2());
}

fn get_lists(buf: &str) -> (Vec<i32>, Vec<i32>) {
    let result = buf.split_whitespace().fold(
        /* Alternate adding to each list */
        (Vec::new(), Vec::new(), true),
        |(mut list1, mut list2, add_to_first), element| {
            let to_add_to = if add_to_first { &mut list1 } else { &mut list2 };
            to_add_to.push(element.parse::<i32>().unwrap());
            (list1, list2, !add_to_first)
        },
    );
    (result.0, result.1)
}

fn part_1() -> i32 {
    let mut file = File::open("input_1.txt").expect("File not found");
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    let (mut list1, mut list2) = get_lists(&buf);
    list1.sort();
    list2.sort();

    zip(list1, list2)
        .map(|(num1, num2)| (num2 - num1).abs())
        .sum()
}

fn part_2() -> i32 {
    let mut file = File::open("input_2.txt").expect("File not found");
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    let (list1, list2) = get_lists(&buf);

    list1.iter().map(|num1| {
        num1 * list2.iter().filter(|num2| num1 == *num2).count() as i32
    }).sum()
}
