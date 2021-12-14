use std::{cmp::max, io::BufRead};
use std::ops::{Deref, DerefMut};
use std::fmt;

fn main() {
    let input = include_str!("input.txt");
    println!("Result 1 : {:?}", solve_1(input));
    println!("Result 2 :\n {}", solve_2(input));
}


struct Game {
    paper: Paper,
    xmax: usize,
    ymax: usize
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        for y in 0..self.ymax {
            for &v in self.paper[y].iter() {
                if v {
                    write!(f, "#")?;
                } else {
                    write!(f, " ")?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(()) 
    }
}
enum Instruction {
    Up(usize),
    Left(usize)
}

struct Paper(Vec<Vec<bool>>);

impl Paper {
    fn new(xmax: usize, ymax: usize) -> Self {
        let mut paper = Vec::<Vec::<bool>>::new();
        for _ in 0..ymax {
            paper.push(vec![false; xmax]);
        }
        Paper(paper)
    }
}

impl Deref for Paper {
    type Target = Vec<Vec<bool>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Paper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}



fn parse(input: &str) -> (Game, Vec<Instruction>) {
    let mut parts = input.split("\n\n");
    let marks: Vec<(usize, usize)> = parts.next().unwrap().lines().map(|l|{
        let (x, y) = l.split_once(",").unwrap();
        (x.parse().unwrap(), y.parse().unwrap())
    }).collect();

    let (xmax, ymax) = marks.iter().fold((0,0), |(xmax, ymax), &(x, y)| (max(xmax, x), max(ymax, y)));

    let mut paper = Paper::new(xmax+1, ymax+1);
    for (x, y) in marks {
        paper[y][x] = true;
    }


    let instructions = parts.next().unwrap().lines().map(|l| {
        let (dir, line) = l.split_whitespace().nth(2).unwrap().split_once("=").unwrap();
        match dir {
            "y" => Instruction::Up(line.parse().unwrap()),
            "x" => Instruction::Left(line.parse().unwrap()),
            _ => unreachable!()
        }
    }).collect();

    (Game{ paper, xmax: xmax+1, ymax: ymax+1 }, instructions)
}

fn solve_2(input: &str) -> Game {
    let (mut game, instructions) = parse(input);
    for instruction in instructions.iter() {
        fold(instruction, &mut game);
    }

    game
}

fn solve_1(input: &str) -> usize {
    let (mut game, instructions) = parse(input);

    fold(&instructions[0], &mut game);

    game.paper.iter().flat_map(|v| v.iter().filter(|&&x| x)).count()
}

fn fold(instruction: &Instruction, game: &mut Game) {
    match instruction {
        Instruction::Up(n) => {
           let mut bottom = game.paper.split_off(*n);
           game.ymax = *n;
           bottom.reverse();
           bottom.pop();
           for y in 0..game.ymax {
               for x in 0..game.xmax {
                   game.paper[y][x] |= bottom[y][x];
               }
           }
        }
        Instruction::Left(n) => {
            game.xmax = *n;
            for y in 0..game.ymax {
                let mut right = game.paper[y].split_off(*n);
                right.reverse();
                right.pop();
                for x in 0..game.xmax {
                    game.paper[y][x] |= right[x];
                }
            }
        }
    }
}


#[test]
fn test_sample1() {
    let input = include_str!("sample.txt");
    assert_eq!(solve_1(&input), 17)
}

#[test]
fn test_sample2() {
    let input = include_str!("sample.txt");
    assert_eq!(solve_2(&input), 12)
}