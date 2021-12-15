use std::collections::HashMap;
use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    println!("Result 1 : {:?}", solve_1(input));
    println!("Result 2 : {:?}", solve_2(input));
}


type Rules = HashMap<(char, char), char>;
type CharCount = HashMap<char, u64>;
type PairCount = HashMap<(char, char), u64>;

fn parse(input: &str) -> (&str, Rules) {
    let mut parts = input.split("\n\n");
    let base = parts.next().unwrap().trim_end();

    let rules = parts.next().unwrap().lines().map(|l|{
        let (pattern, insert) = l.split_once(" -> ").unwrap();
        let mut pc = pattern.chars();
        ((pc.next().unwrap(), pc.next().unwrap()), insert.chars().next().unwrap())
    }).collect();

    (base, rules)
}

fn solve_1(input: &str) -> u64 {
    solve(input, 10)
}

fn solve_2(input: &str) -> u64 {
    solve(input, 40)
}


fn step(pair_counts: PairCount, char_count: &mut CharCount, rules: &Rules) -> PairCount {
    let mut next = PairCount::new();
    for ((a,b), i) in pair_counts {
        if let Some(c) = rules.get(&(a, b)) {
            *char_count.entry(*c).or_default() += i;
            *next.entry((a, *c)).or_default() += i;
            *next.entry((*c, b)).or_default() += i;
        }
    }
    next
}

fn solve(input: &str, steps: u64) -> u64 {
    let (poly, rules) = parse(input);

    let mut pair_counts: PairCount = HashMap::new();
    let mut char_counts: CharCount = HashMap::new();
    for (l,r) in poly.chars().tuple_windows() {
        *pair_counts.entry((l,r)).or_insert(0) += 1;
    }
    for ch in poly.chars() {
        *char_counts.entry(ch).or_insert(0) += 1;
    }

    for _ in 0..steps {
        pair_counts = step(pair_counts, &mut char_counts, &rules);
    }

    let mut max = 0u64;
    let mut min = u64::MAX;
    for &v in char_counts.values() {
        if v > max {
            max = v;
        }
        if v < min {
            min = v;
        }
    }
    max - min
}

#[test]
fn test_sample1() {
    let input = include_str!("sample.txt");
    assert_eq!(solve_1(&input), 1588)
}

#[test]
fn test_sample2() {
    let input = include_str!("sample.txt");
    assert_eq!(solve_2(&input), 2188189693529)
}