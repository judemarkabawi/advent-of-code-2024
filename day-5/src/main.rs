use std::{cmp::Ordering, collections::HashMap, fs::File, io::Read, str::FromStr};

fn main() {
    println!("{}", part_1());
    println!("{}", part_2());
}

struct PagesInfo {
    ordering_rules: Vec<(i32, i32)>,
    update_pages: Vec<Vec<i32>>,
}

impl FromStr for PagesInfo {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ordering_rules: Vec<(i32, i32)> = Vec::new();
        let mut update_pages: Vec<Vec<i32>> = Vec::new();
        s.split_whitespace().for_each(|line| {
            if line.contains("|") {
                // Ordering rule
                let rule_orders: Vec<i32> = line
                    .split("|")
                    .map(|num| num.parse::<i32>().unwrap())
                    .collect();
                assert_eq!(rule_orders.len(), 2);
                ordering_rules.push((rule_orders[0], rule_orders[1]));
            } else {
                let updates: Vec<i32> = line
                    .split(",")
                    .map(|num| num.parse::<i32>().unwrap())
                    .collect();
                update_pages.push(updates);
            }
        });
        Ok(PagesInfo {
            ordering_rules,
            update_pages,
        })
    }
}

fn update_page_to_index_map(update_page: &[i32]) -> HashMap<i32, usize> {
    let mut result = HashMap::with_capacity(update_page.len());
    update_page.iter().enumerate().for_each(|(index, value)| {
        result.insert(*value, index);
    });
    result
}

fn is_correctly_ordered_update(pages_info: &PagesInfo, update_page: &[i32]) -> bool {
    let index_map = update_page_to_index_map(update_page);
    for (before, after) in &pages_info.ordering_rules {
        if let Some(before_idx) = index_map.get(before) {
            if let Some(after_idx) = index_map.get(after) {
                // Check if ee broke a rule
                if before_idx >= after_idx {
                    return false;
                }
            }
        }
    }
    true
}

fn part_1() -> i32 {
    let mut file = File::open("input_1.txt").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let pages_info: PagesInfo = data.parse().unwrap();
    pages_info
        .update_pages
        .iter()
        // Filter out any pages that don't follow the ordering rules
        .filter(|update_page| is_correctly_ordered_update(&pages_info, update_page))
        // Get the middle page number
        .map(|update_page| update_page[update_page.len() / 2])
        .sum()
}

fn part_2() -> i32 {
    let mut file = File::open("input_1.txt").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let pages_info: PagesInfo = data.parse().unwrap();
    pages_info
        .update_pages
        .iter()
        // Filter out any pages that don't follow the ordering rules
        .filter(|update_page| !is_correctly_ordered_update(&pages_info, update_page))
        // Correct them to be properly ordered
        .map(|update_page| {
            let mut update_page = update_page.clone();
            update_page.sort_by(|a, b| {
                // Sort by ordering rules
                let exists_rule_a_before_b = pages_info
                    .ordering_rules
                    .iter()
                    .any(|(before, after)| (before, after) == (a, b));
                let exists_rule_b_before_a = pages_info
                    .ordering_rules
                    .iter()
                    .any(|(before, after)| (before, after) == (b, a));
                if exists_rule_a_before_b {
                    Ordering::Less
                } else if exists_rule_b_before_a {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            });
            update_page
        })
        // Get the middle page number
        .map(|update_page| update_page[update_page.len() / 2])
        .sum()
}
