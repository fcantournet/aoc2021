//use bit_vec::BitVec;

fn main() {
    let input = include_str!("input.txt");
    println!("Result 1 : {:?}", solve_1(input));
    println!("Result 2 : {:?}", solve_2(input));
}

struct BinaryBlob {
    lines: usize,
    bitsize: usize,
    // # of 1 for that bit column
    ones: Vec<usize>,
    // numbers as parsed lines by lines
    numbers: Vec<usize>
}


fn parse(input: &str) -> BinaryBlob {
    let bitsize = input.lines().nth(0).unwrap().len();
    let lines = input.lines().count();
    //let mut n = 0usize;
    let mut ones = vec![0; bitsize];
    let mut numbers = vec![0; lines];
    for (n, line) in input.lines().enumerate() {
        for (i, c) in line.chars().rev().enumerate() {
            match c {
                '1' => {
                    ones[i] += 1;
                    numbers[n] += 1 << i;
                }
                _ => {},
            }
        }
    }
    BinaryBlob {
        lines: lines,
        bitsize: bitsize,
        ones: ones,
        numbers: numbers,
    }
}

fn solve_2(input: &str) -> usize {
    let blob = parse(input);


    let (ones, zeroes) : (Vec<usize>, Vec<usize>) = blob.numbers.iter().partition(|&&x| x & (1 << blob.bitsize-1) != 0 ); 
    let (mut oxy, mut co2) = if ones.len() >= zeroes.len() {
        (ones, zeroes)
    } else {
        (zeroes, ones)
    };

    for i in (0..blob.bitsize-1).rev() {
        // let ones_oxy = oxy.iter().fold(0, |acc, &x| if x & (1 << i) != 0 {acc + 1} else { acc });
        let (ones, zeroes) : (Vec<usize>, Vec<usize> )= oxy.iter().partition(|&&x| x & (1 << i) != 0 );
        if ones.len() >= zeroes.len() {
            oxy = ones;
        } else {
            oxy = zeroes;
        }
        if oxy.len() == 1 {
            break
        }
    }

    for i in (0..blob.bitsize-1).rev() {
        // let ones_co2 = co2.iter().fold(0, |acc, &x| if x & (1 << i) != 0 {acc + 1} else { acc });
        let (ones, zeroes) : (Vec<usize>, Vec<usize> )= co2.iter().partition(|&&x| x & (1 << i) != 0 );
        if ones.len() < zeroes.len() {
            co2 = ones;
        } else {
            co2 = zeroes;
        }
        if co2.len() == 1 {
            break
        }
    }
    co2[0] * oxy[0]
}

fn solve_1(input: &str) -> u32 {
    let blob = parse(input);
    let mut gamma = 0u32;

    let mut bitmask = 0u32;

    for (i, b) in blob.ones.iter().enumerate() {
        bitmask += 1 << i;
        println!("b = {}", b);
        if *b > (blob.lines / 2) {
            gamma += 1 << i
        }
        println!("gamma = {} {:b}", gamma, gamma);
    }
    println!("bitmask = {} {:b}", bitmask, bitmask);
    let epsi = !gamma & bitmask;
    println!("gamma = {} {:b}", gamma, gamma);
    println!("epsi = {} {:b}", epsi, epsi);
    gamma * epsi
}

#[test]
fn test_sample1() {
    let input = include_str!("sample.txt");
    assert_eq!(solve_1(&input), 198)
}

#[test]
fn test_sample2() {
    let input = include_str!("sample.txt");
    assert_eq!(solve_2(&input), 230)
}