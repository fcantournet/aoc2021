
fn main() {
    let input = include_str!("input.txt");
    println!("Result 1 : {:?}", solve_1(input));
    println!("Result 2 : {:?}", solve_2(input));
}


fn solve_2(input: &str) -> usize {
    let mut scores: Vec<_> = input.lines().filter_map(|l|{
        if let Damage::Missing(missing) = find_premature_char(l) {
            Some(missing.iter().fold(0, |acc, c|{
                acc * 5 + match c {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => panic!("wrong char : {}", c)
                }
            }))
        } else {
            None
        }
    }).collect();

    scores.sort();
    println!("scores: {:#?}", scores);
    scores[scores.len() / 2]
}

fn solve_1(input: &str) -> usize {
    input.lines().map(|l| {
        if let Damage::Corrupted(c) = find_premature_char(l) {
            match c {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => panic!("unexpected wrong char {}", c)
            }
        } else {
            0
        }
    }).sum()
}

#[derive(Debug, PartialEq)]
enum Damage {
    Corrupted(char),
    Missing(Vec<char>)
}


fn find_premature_char(input: &str) -> Damage {
    let mut stack_opened = Vec::<char>::new();

    for c in input.chars() {
        match c {
            '<'|'('|'['|'{' => stack_opened.push(c),
            c => {
                if matching_pair(&c) != stack_opened.pop().unwrap() {
                    return Damage::Corrupted(c);
                }
            } 
        }
    }
    Damage::Missing(stack_opened.iter().rev().map(matching_pair).collect()) 
}

#[inline(always)]
fn matching_pair(c: &char) -> char {
    match c {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("Not a closing char")
    }
}

#[test]
fn test_1line() {
    let line = "{([(<{}[<>[]}>{[]{[(<()>";

    assert_eq!(find_premature_char(line), Damage::Corrupted('}'));
}

#[test]
fn test_sample1() {
    let input = include_str!("sample.txt");
    assert_eq!(solve_1(&input), 26397)
}

#[test]
fn test_sample2() {
    let input = include_str!("sample.txt");
    assert_eq!(solve_2(&input), 288957)
}