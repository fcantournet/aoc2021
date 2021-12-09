use std::cmp;

fn main() {
    let input = include_str!("input.txt");
    println!("Result 1 : {:?}", solve_1(input));
    println!("Result 2 : {:?}", solve_2(input));
}


fn parse(input: &str) -> Vec<usize> {
    input.split(',').map(|x| x.parse().unwrap()).collect()
}

fn fuel_cost(target: usize, pos: usize) -> usize {
    let n = abs_diff(target, pos);
    n*(n+1)/2  // sum of integer from 0..n
    //(0..=abs_diff(target, pos)).fold(0, |acc, x| acc + x)
}

#[inline(always)]
fn abs_diff(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}

fn solve_2(input: &str) -> usize {
    solve(input, fuel_cost)
}

fn solve_1(input: &str) -> usize {
    solve(input, abs_diff)
}

fn solve(input: &str, cost: fn (usize, usize) -> usize) -> usize {
    let points = parse(input);
    let mut min_dist = usize::MAX;
    for i in 0..*points.iter().max().unwrap() {
        let dist: usize = points.iter().map(|&x| cost(i, x)).sum();
        min_dist = cmp::min(dist, min_dist);
    }
    min_dist
}


#[test]
fn test_sample1() {
    let input = include_str!("sample.txt");
    assert_eq!(solve_1(&input), 37)
}

#[test]
fn test_sample2() {
    let input = include_str!("sample.txt");
    assert_eq!(solve_2(&input), 168)
}

#[test]
fn test_fuel_cost() {
    let inputs: [usize; 10] = [16,1,2,0,4,2,7,1,2,14];
    let results = [66,10,6,15,1,6,3,10,6,45];
    for (i, &v) in inputs.iter().enumerate() {
        assert_eq!(fuel_cost(5, v), results[i])
    }
}