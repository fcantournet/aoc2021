fn main() {
    let input: Vec<_> = include_str!("input.txt").lines().map(|x| x.parse::<i32>().unwrap()).collect();
    println!("Result 1 : {:?}", solve_1(&input));
    println!("Result 2 : {:?}", solve_2(&input));
}

fn solve_1(input: &[i32]) -> i32 {
    (&input[1..]).iter().zip(&input[0..input.len()]).map(|(x, y)| if x > y {1} else {0}).sum()  
}

fn solve_2(input: &[i32]) -> i32 {
    let windows: Vec<_> = input.windows(3).map(|w| w[0] + w[1] + w[2]).collect();
    solve_1(&windows)
}

#[test]
fn test_sample1() {
    let input: Vec<_> = include_str!("sample.txt").lines().map(|x| x.parse::<i32>().unwrap()).collect();
    assert_eq!(solve_1(&input), 7)
}

#[test]
fn test_sample2() {
    let input: Vec<_> = include_str!("sample.txt").lines().map(|x| x.parse::<i32>().unwrap()).collect();
    assert_eq!(solve_2(&input), 5)  
}