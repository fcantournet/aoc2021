use std::collections::HashMap;

use nom::{IResult, sequence::{separated_pair, tuple}, character::{self}, bytes::complete::take};
fn main() {
    let input = include_str!("input.txt");
    println!("Result 1 : {:?}", solve_1(input));
    println!("Result 2 : {:?}", solve_2(input));
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Point {
    x : i32,
    y: i32,
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    let (input, (x, y)) = separated_pair(character::complete::i32, character::complete::char(','), character::complete::i32)(input)?;
    Ok((input, Point {x, y}))
}

struct Segment {
    start: Point,
    end: Point,
}

impl Segment {

    // Only return points of strait segments
    fn straight_segments(self) -> Vec<Point> {
        let (max_x, min_x) = if self.start.x > self.end.x {
            (self.start.x, self.end.x)
        } else {
            (self.end.x, self.start.x)
        };
        let (max_y, min_y) = if self.start.y > self.end.y {
            (self.start.y, self.end.y)
        } else {
            (self.end.y, self.start.y)
        };


        match (min_x - max_x, min_y - max_y) {
            (0, _) => (min_y..=max_y).map(|y| Point{x: min_x, y}).collect(),
            (_, 0) => (min_x..=max_x).map(|x| Point{x, y: min_y}).collect(), // vertical
            _ => Vec::<Point>::default(),
        }
    }

    // return points whether segment is straight or not
    fn all_points(self) -> Vec<Point> {
        let vx: Vec<_> = if self.start.x > self.end.x {
            (self.end.x..=self.start.x).rev().collect()
        } else {
            (self.start.x..=self.end.x).collect()
        };
        let vy: Vec<_> = if self.start.y > self.end.y {
            (self.end.y..=self.start.y).rev().collect()
        } else {
            (self.start.y..=self.end.y).collect()
        };

        match (self.start.x - self.end.x, self.start.y - self.end.y) {
            (0, _) => vy.iter().map(|&y| Point{x: self.start.x, y}).collect(),
            (_, 0) => vx.iter().map(|&x| Point{x, y: self.start.y}).collect(), // vertical
            _ => {
                // always same sized vx, vy because segment is 45°
                vx.iter().zip(vy.iter()).map(|(&x, &y)| Point{x, y}).collect()
            }
        }
    }
}

fn parse_line(input: &str) -> IResult<&str, Segment> {
    let (input, (start, _,  end)) = tuple((parse_point, take(4usize), parse_point))(input)?;
    Ok((input, Segment {start, end}))
}

fn parse(input: &str) -> Vec<Segment> {
    input.lines().map(|x| {
        let (_, l) = parse_line(x).unwrap();
        l
    }).collect()
}

fn solve_2(input: &str) -> usize {
    solve(input, Segment::all_points)
}

fn solve_1(input: &str) -> usize {
    solve(input, Segment::straight_segments)
}

fn solve(input: &str, getpoints: fn(Segment) -> Vec<Point> ) -> usize {

    let segments = parse(input);
    let mut grid = HashMap::<Point, usize>::default();
    let mut danger = Vec::<Point>::default();

    for s in segments {
        for p in getpoints(s) {
            let level = grid.entry(p.clone()).or_insert(0);
            *level += 1;
            if *level == 2 {
                danger.push(p)
            }
        }
    }
    danger.len()
}

#[test]
fn test_sample1() {
    let input = include_str!("sample.txt");
    assert_eq!(solve_1(&input), 5)
}

#[test]
fn test_sample2() {
    let input = include_str!("sample.txt");
    assert_eq!(solve_2(&input), 12)
}