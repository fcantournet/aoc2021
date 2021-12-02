

fn main() {
    let input: Vec<_> = include_str!("input.txt").lines().map(parse).collect();
    println!("Result 1 : {:?}", solve_1(&input));
    println!("Result 2 : {:?}", solve_2(&input));
}

enum Command {
    Forward(i32),
    Down(i32),
    Up(i32)
}

fn parse(line: &str) -> Command {
    let mut elems = line.split_whitespace();
    let command = elems.next().unwrap();
    let value = elems.next().unwrap().parse::<i32>().unwrap();
    match command {
        "forward" => Command::Forward(value),
        "up" => Command::Up(value),
        "down" => Command::Down(value),
        somethingelse => panic!("got {}", somethingelse) 
    }
}

fn solve_2(commands: &[Command]) -> i32 {
    let mut depth = 0;
    let mut pos = 0;
    let mut aim = 0;
    for command in commands {
        match command {
            Command::Forward(val) => {pos += val; depth += val * aim}
            Command::Up(val) => aim -= val,
            Command::Down(val) => aim += val,
        }
    }
    depth * pos
}


fn solve_1(commands: &[Command]) -> i32 {
    let mut depth = 0;
    let mut pos = 0;
    for command in commands {
        match command {
            Command::Forward(val) => pos += val,
            Command::Up(val) => depth -= val,
            Command::Down(val) => depth += val,
        }
    }
    depth * pos
}


#[test]
fn test_sample1() {
    let input: Vec<_> = include_str!("sample.txt").lines().map(parse).collect();
    assert_eq!(solve_1(&input), 150)
}
#[test]
fn test_sample2() {
    let input: Vec<_> = include_str!("sample.txt").lines().map(parse).collect();
    assert_eq!(solve_2(&input), 900)
}
