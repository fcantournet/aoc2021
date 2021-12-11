use nom::{IResult, bytes::complete::tag, character::complete::space0, character::complete::alpha1, Parser, multi::count};
use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    println!("Result 1 : {:?}", solve_1(input));
    println!("Result 2 : {:?}", solve_2(input));
}


type Signal = HashSet<char>;
struct Display {
    output: Vec<Signal>,
    patterns: Vec<Signal>,
}


fn remove_filter<T, F>(input: &mut Vec<T>, predicate: F) -> T
where
    T: Clone,
    F: Fn(&&T) -> bool + Copy,
{
    let mut found = input.iter().filter(predicate);
    let retval = found.next().expect("didn't find any value for predicate").clone();
    input.retain(|x| !predicate(&x));
    retval
}

impl Display {
    fn calculate(&self) -> usize {
        let lookup = Self::untangle(self.patterns.clone());
        self.output.iter().fold( 0, |acc, x| {
            acc * 10 + lookup.iter().enumerate().find(|(_, s)| *s == x).map(|(i, _)| i).unwrap()
        })
    }
    fn untangle(mut signals: Vec<Signal>) -> [Signal; 10] {
        let n1 = remove_filter(&mut signals, |&x| x.len() == 2);
        let n7 = remove_filter(&mut signals, |&x| x.len() == 3);
        let n4 = remove_filter(&mut signals, |&x| x.len() == 4);
        let n8 = remove_filter(&mut signals, |&x| x.len() == 7);

        let n2 = remove_filter(&mut signals, |&x| x.len() == 5 && (x & &n4).len() == 2);
        let n3 = remove_filter(&mut signals, |&x| x.len() == 5 && (x & &n1).len() == 2);
        let n5 = remove_filter(&mut signals, |&x| x.len() == 5);

        let n6 = remove_filter(&mut signals, |&x| x.len() == 6 && (x & &n1).len() == 1);
        let n9 = remove_filter(&mut signals, |&x| x.len() == 6 && (x & &n4).len() == 4);
        let n0 = remove_filter(&mut signals, |&x| x.len() == 6);

        [n0, n1, n2, n3, n4, n5, n6, n7, n8, n9]
    }
}

fn parse(input: &str) -> Vec<Display> {
    input.lines().map(parse_display).collect()
}

fn parse_display(input: &str) -> Display {
    let (_, ((patterns, _), output)) = count(parse_cell, 10).and(tag("| ")).and(count(parse_cell, 4)).parse(input).unwrap();
    Display {
        output,
        patterns,
    }
}

fn parse_cell(input: &str) -> IResult<&str, Signal> {
    let (input, (x, _)) = alpha1.and(space0).parse(input)?;
    Ok((input, x.chars().collect()))
}
  
fn solve_2(input: &str) -> usize {
    let displays = parse(input);
    displays.iter().map(Display::calculate).sum()
}

fn solve_1(input: &str) -> usize {
    let displays = parse(input);
    displays.iter().fold(0, |acc, d| acc + d.output.iter().fold(0, |acc, c| match c.len() {
        2|3|4|7 => acc + 1,
        _ => acc
    }))
}


#[test]
fn test_sample1() {
    let input = include_str!("sample.txt");
    assert_eq!(solve_1(&input), 26)
}

#[test]
fn test_sample2() {
    let input = include_str!("sample.txt");
    assert_eq!(solve_2(&input), 61229)
}

#[test]
fn test_parse_line() {
    let input = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
    let display = parse_display(input);

    assert_eq!(display.patterns.len(), 10);
    assert_eq!(display.output.len(), 4);
    assert_eq!(display.output[1], HashSet::<char>::from(['f','c','a','d','b']));
}