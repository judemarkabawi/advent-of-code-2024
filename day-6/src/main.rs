use std::{collections::HashSet, fs::File, io::Read};

enum BoardTile {
    Empty,
    Blocked,
}

struct BoardPosition {
    visited: bool,
    tile: BoardTile,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

struct Board {
    data: Vec<BoardPosition>,
    width: usize,
    height: usize,
    player_start_position: (i64, i64),
    player_position: (i64, i64),
    player_direction: Direction,
}

impl Board {
    fn from_str(source: &str) -> Board {
        let lines: Vec<_> = source.split_whitespace().collect();
        let width = lines[0].len();
        let height = lines.len();

        let chars: Vec<char> = lines.iter().flat_map(|line| line.chars()).collect();
        let mut data: Vec<BoardPosition> = chars
            .iter()
            .map(|c| match c {
                '.' => BoardPosition {
                    visited: false,
                    tile: BoardTile::Empty,
                },
                '^' => BoardPosition {
                    visited: false,
                    tile: BoardTile::Empty,
                },
                _ => BoardPosition {
                    visited: false,
                    tile: BoardTile::Blocked,
                },
            })
            .collect();
        let start_index = chars
            .iter()
            .position(|board_position| *board_position == '^')
            .unwrap() as i64;
        data[start_index as usize].visited = true;

        let start_position = (start_index / width as i64, start_index % width as i64);
        Board {
            data,
            width,
            height,
            player_start_position: start_position,
            player_position: start_position,
            player_direction: Direction::Up,
        }
    }

    fn get_mut(&mut self, i: i64, j: i64) -> Option<&mut BoardPosition> {
        if i < 0 || j < 0 {
            return None;
        }

        let i = i as usize;
        let j = j as usize;
        if i >= self.height || j >= self.width {
            return None;
        }

        Some(&mut self.data[i * self.width + j])
    }

    fn player_on_board(&self) -> bool {
        self.player_position.0 >= 0
            && self.player_position.0 < self.height as i64
            && self.player_position.1 >= 0
            && self.player_position.1 < self.width as i64
    }

    fn right_of(dir: Direction) -> Direction {
        match dir {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn step(&mut self) {
        let i_step = match self.player_direction {
            Direction::Up => -1,
            Direction::Down => 1,
            _ => 0,
        };
        let j_step = match self.player_direction {
            Direction::Right => 1,
            Direction::Left => -1,
            _ => 0,
        };

        let next_i = self.player_position.0 + i_step;
        let next_j = self.player_position.1 + j_step;

        if let Some(next_position) = self.get_mut(next_i, next_j) {
            match next_position.tile {
                BoardTile::Blocked => {
                    // Turn and try again
                    self.player_direction = Board::right_of(self.player_direction);
                    self.step(); // recurse max 2 times with these boards
                }
                BoardTile::Empty => {
                    // Move forward
                    next_position.visited = true;
                    self.player_position = (next_i, next_j);
                }
            }
        } else {
            // Off board
            self.player_position = (next_i, next_j);
        }
    }

    fn reset(&mut self) {
        for board_position in &mut self.data {
            board_position.visited = false;
        }
        self.player_position = self.player_start_position;
        self.player_direction = Direction::Up;
    }
}

fn is_stuck_in_loop(board: &mut Board) -> bool {
    let mut visited: HashSet<(i64, i64, Direction)> = HashSet::new();

    while board.player_on_board() {
        let position_info = (
            board.player_position.0,
            board.player_position.1,
            board.player_direction,
        );
        if visited.contains(&position_info) {
            return true;
        }

        visited.insert(position_info);
        board.step();
    }
    false
}

fn main() {
    println!("{}", part_1());
    println!("{}", part_2());
}

fn part_1() -> usize {
    let mut file = File::open("input_1.txt").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let mut board = Board::from_str(&data);

    // Run through board until off it
    while board.player_on_board() {
        board.step();
    }

    board
        .data
        .iter()
        .filter(|board_position| board_position.visited)
        .count()
}

fn part_2() -> usize {
    let mut file = File::open("input_1.txt").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let mut board = Board::from_str(&data);

    // Map out the path and find visited indexes since only putting an obstacle
    // on one of those would affect our path and cause a loop
    while board.player_on_board() {
        board.step();
    }
    let visited_idxs: Vec<usize> = board
        .data
        .iter()
        .enumerate()
        .filter(|(_, board_position)| board_position.visited)
        .map(|(i, _)| i)
        .collect();

    board.reset();

    // Try blocking each tile along their original path, checking for making a loop
    let mut num_can_be_made_loop = 0;
    for i in visited_idxs {
        board.data[i].tile = BoardTile::Blocked;

        num_can_be_made_loop += is_stuck_in_loop(&mut board) as usize;

        // Undo and reset board
        board.reset();
        board.data[i].tile = BoardTile::Empty;
    }

    num_can_be_made_loop
}
