use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

#[derive(Clone, Copy)]
enum Operator {
    Plus,
    Times,
    Concat,
}

struct Equation(Vec<u64>);

impl Equation {
    fn target(&self) -> u64 {
        self.0[0]
    }

    fn operands(&self) -> &[u64] {
        &self.0[1..]
    }

    fn evaluate(&self, operators: &[Operator]) -> u64 {
        assert_eq!(self.operands().len() - 1, operators.len()); // target: i1 .. in needs n-1 operators

        assert!(self.operands().len() >= 2);
        let first = self.operands()[0];

        self.operands()[1..]
            .iter()
            .zip(operators)
            .fold(first, |result, (operand, operator)| match *operator {
                Operator::Plus => result + *operand,
                Operator::Times => result * *operand,
                Operator::Concat => {
                    let num_digits = operand.ilog10() + 1;
                    result * (10_u64.pow(num_digits)) + operand
                }
            })
    }
}

impl FromStr for Equation {
    type Err = ();

    // "target: num1 num2 num3" etc...
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let equation: Vec<u64> = s
            .split_whitespace()
            .enumerate()
            .map(|(i, section)| {
                match i {
                    0 => section[..section.len() - 1].parse::<u64>().unwrap(), // remove ":"
                    _ => section.parse::<u64>().unwrap(),
                }
            })
            .collect();

        assert!(equation.len() >= 3);

        Ok(Equation(equation))
    }
}

// For each equation, try all combinations of the operators
// to satisfy the equation's target. Sum the targets for only those equations that
// can be satisfied
fn sum_of_satisfiable_equations(
    equations: impl Iterator<Item = Equation>,
    operators: &[Operator],
) -> u64 {
    equations
        .filter_map(|equation| {
            let num_operators = equation.operands().len() - 1;
            let operator_combinations =
                itertools::repeat_n(operators.to_owned(), num_operators).multi_cartesian_product();
            for operators in operator_combinations {
                if equation.evaluate(&operators) == equation.target() {
                    return Some(equation.target());
                }
            }
            None
        })
        .sum()
}

fn part_1() -> u64 {
    let file = File::open("input_1.txt").expect("File not found");
    let reader = BufReader::new(file);

    let equations = reader
        .lines()
        .map(|line| line.unwrap().parse::<Equation>().unwrap());

    sum_of_satisfiable_equations(equations, &[Operator::Plus, Operator::Times])
}

fn part_2() -> u64 {
    let file = File::open("input_1.txt").expect("File not found");
    let reader = BufReader::new(file);

    let equations = reader
        .lines()
        .map(|line| line.unwrap().parse::<Equation>().unwrap());

    use std::time::Instant;
    let now = Instant::now();
    let result = sum_of_satisfiable_equations(
        equations,
        &[Operator::Plus, Operator::Times, Operator::Concat],
    );
    let elapsed = now.elapsed();
    println!("Elapsed: {:?}", elapsed);

    result
}

fn main() {
    println!("{}", part_1());
    println!("{}", part_2());
}
