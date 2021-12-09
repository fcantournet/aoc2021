use std::{cmp::max, collections::HashMap, time::Instant};


fn main() {
    let input = include_str!("input.txt");
    println!("Result 1 : {:?}", solve_1(input));
    let start = Instant::now();
    println!("Result 2 : {:?}", solve_2(input));
    let timing = start.elapsed();
    println!("took : {}", timing.as_micros());
}


fn parse(input: &str) -> Vec<usize> {
    input.split(',').map(|x| x.parse::<usize>().unwrap()).collect()
}

fn solve_2(input: &str) -> usize {
    let mut initial_fishes = parse(input);
    let mut new_fishes = 0usize;

    let mut MEMO: HashMap<(isize, isize), usize> = HashMap::default();


    for f in initial_fishes.iter_mut() {
        new_fishes += total_descendants(256, *f as isize, &mut MEMO);
    }
    println!("{}", new_fishes);
    new_fishes
}


fn total_descendants(rem_days: isize, start_timer: isize, memo: &mut HashMap<(isize, isize), usize>) -> usize { 
    if rem_days <= start_timer {
        return 1;
    }
    if let Some(&total) = memo.get(&(rem_days, start_timer)) {
        return total;
    }

    let mut total = 1usize;
    let direct_children = 1 + (rem_days - start_timer - 1) / 7;
    for i in 0..direct_children {
        let cbd = rem_days - start_timer - 1 - (7*i);
        total += total_descendants(cbd, 8, memo);
    }
   
    memo.insert((rem_days, start_timer), total);
    total
}





fn solve_1(input: &str) -> usize {
    let mut fishes = parse(input);
    let mut new_fish = 0usize;
    for _ in 0..80 {
        for f in fishes.iter_mut() {
            if *f == 0 {
                new_fish += 1;
                *f = 6;
            } else {
                *f -= 1;
            }
        }
        for _ in 0..new_fish {
            fishes.push(8);
        }
        new_fish = 0;
    }
    fishes.len()
}


#[test]
fn test_sample1() {
    let input = include_str!("sample.txt");
    assert_eq!(solve_1(&input), 5934)
}

#[test]
fn test_sample2() {
    let input = include_str!("sample.txt");
    assert_eq!(solve_2(&input), 26984457539)
}