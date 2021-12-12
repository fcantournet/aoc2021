fn main() {
    let input = include_str!("input.txt");
    println!("Result 1 : {:?}", solve_1(input));
    println!("Result 2 : {:?}", solve_2(input));


    let mut x = 0usize;
    let xx = -1isize;

    let y = (x as isize + xx) as usize;
    println!("{}", y)


}


fn parse(input: &str) -> usize {
    0
}
fn solve_2(input: &str) -> usize {
    0
}

fn solve_1(input: &str) -> usize {
    0
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