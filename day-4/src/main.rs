use std::{fs::File, io::Read};

fn main() {
    println!("{}", part_1());
    println!("{}", part_2());
}

struct Board {
    data: Vec<char>,
    width: usize,
    height: usize,
}

struct Direction(i64, i64);

impl Board {
    fn from_str(source: &str) -> Board {
        let lines: Vec<_> = source.split_whitespace().collect();
        let width = lines[0].len();
        let height = lines.len();
        Board {
            data: lines.iter().flat_map(|line| line.chars()).collect(),
            width,
            height,
        }
    }

    fn get(&self, i: i64, j: i64) -> Option<char> {
        if i < 0 || j < 0 {
            return None;
        }

        let i = i as usize;
        let j = j as usize;
        if i >= self.height || j >= self.width {
            return None;
        }

        Some(self.data[i * self.width + j])
    }
}

fn is_xmas_from(board: &Board, i: usize, j: usize, dir: Direction) -> bool {
    let i = i as i64;
    let j = j as i64;

    ['X', 'M', 'A', 'S']
        .iter()
        .enumerate()
        .map(|(dir_step, target_letter)| {
            let dir_step = dir_step as i64;
            board
                .get(i + dir_step * dir.0, j + dir_step * dir.1)
                .map_or(false, |letter| letter == *target_letter)
        })
        .all(|equals| equals)
}

fn is_x_shape_mas_from(board: &Board, i: usize, j: usize) -> bool {
    fn inner(board: &Board, i: i64, j: i64) -> Option<()> {
        let start = board.get(i, j)?;
        let top_left = board.get(i - 1, j - 1)?;
        let top_right = board.get(i - 1, j + 1)?;
        let bottom_left = board.get(i + 1, j - 1)?;
        let bottom_right = board.get(i + 1, j + 1)?;

        if start == 'A'
            && ((top_left == 'M' && bottom_right == 'S')
                || (top_left == 'S' && bottom_right == 'M'))
            && ((bottom_left == 'M' && top_right == 'S')
                || (bottom_left == 'S' && top_right == 'M'))
        {
            Some(())
        } else {
            None
        }
    }

    inner(board, i as i64, j as i64).is_some()
}

fn part_1() -> u32 {
    let mut file = File::open("input_1.txt").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let board = Board::from_str(&data);
    let mut num_xmas_found = 0;
    for i in 0..board.height {
        for j in 0..board.width {
            for step_x in [-1, 0, 1] {
                for step_y in [-1, 0, 1] {
                    num_xmas_found += is_xmas_from(&board, i, j, Direction(step_x, step_y)) as u32
                }
            }
        }
    }
    num_xmas_found
}

fn part_2() -> u32 {
    let mut file = File::open("input_2.txt").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let board = Board::from_str(&data);
    let mut num_xmas_found = 0;
    for i in 0..board.height {
        for j in 0..board.width {
            num_xmas_found += is_x_shape_mas_from(&board, i, j) as u32;
        }
    }
    num_xmas_found
}
