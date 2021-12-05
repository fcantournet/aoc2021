use ndarray::Array2;
use std::{str, default};

fn main() {
    let input = include_str!("input.txt");
    println!("Result 1 : {:?}", solve_1(input));
    println!("Result 2 : {:?}", solve_2(input));
}

#[derive(Debug, Clone)]
struct Board {
    numbers: Array2::<usize>,
    // use usize as bool so we can sum over columns and rows
    marked: Array2::<usize>,
    bingo: bool,
}


fn parse(input: &str) -> (Vec<usize>, Vec<Board>) {
    let mut it = input.split("\n\n");
    let tirage: Vec<usize> = it.next().unwrap().split(",").map(|x| x.parse::<usize>().unwrap()).collect();

    let mut boards = Vec::<Board>::default();

    for b in it {
        let mut numbers = Array2::<usize>::zeros((5,5));
        let mut marked = Array2::<usize>::ones((5,5));
        for (x, line) in b.lines().enumerate() {
            for (y, c) in line.split_whitespace().enumerate() {
                numbers[[x, y]] = c.parse().unwrap();
            }
        }
        boards.push(Board {numbers, marked, bingo: false})
    }
    (tirage, boards)
}

fn solve_1(input: &str) -> usize {
    let (tirage, mut boards) = parse(input);

    for n in tirage {
        for b in boards.iter_mut() {
            if mark(b, n) {
                return n * sum_unmarked(b.clone());
            }
        }
    }
    0
}

fn solve_2(input: &str) -> usize {
    let (tirage, mut boards) = parse(input);

    let mut to_bingo = boards.len();

    for n in tirage {
        for b in boards.iter_mut() {
            if !b.bingo & mark(b, n) {
                b.bingo = true;
                to_bingo -= 1;

                if to_bingo == 0 {
                    return n * sum_unmarked(b.clone())
                }
            }
        }        
    }

    0
}

fn sum_unmarked(b: Board) -> usize {
    (b.marked * b.numbers).sum()
}

fn mark(b: &mut Board, n: usize) -> bool {
    let mut bingo = false;
    for ((x, y), k) in b.numbers.indexed_iter_mut() {
        if *k == n {
            b.marked[[x,y]] = 0;
            // let's check this row and column for bingo
            bingo = b.marked.row(x).sum() == 0 || b.marked.column(y).sum() == 0
        }
    }
    bingo
}

#[test]
fn test_sample1() {
    let input = include_str!("sample.txt");
    assert_eq!(solve_1(&input), 4512)
}

#[test]
fn test_sample2() {
    let input = include_str!("sample.txt");
    assert_eq!(solve_2(&input), 1924)
}