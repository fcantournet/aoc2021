
use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    println!("Result 1 : {:?}", solve_1(input));
    println!("Result 2 : {:?}", solve_2(input));
}

struct Grid {
    heights: Vec<u32>,
    row_size: usize,
    rows: usize,
}

impl Grid {
    fn get(&self, x: isize, y: isize) -> u32 {
        if (x as usize) < self.row_size && (y as usize) < self.rows && x >= 0 && y >= 0 {
            self.heights[x as usize + y as usize*self.row_size]
        } else {
            9
        }
    }
    fn set(&mut self, x: isize, y: isize, val: u32) {
        if (x as usize) < self.row_size && (y as usize) < self.rows && x >= 0 && y >= 0 {
            self.heights[x as usize + y as usize*self.row_size] = val; 
        }
    }

    fn is_low_point(&self, x: isize, y: isize) -> bool {
        [(x+1, y), (x-1, y), (x, y+1), (x, y-1)].iter()
        .fold(true, |acc, &(xi, yi)| acc && self.get(xi, yi) > self.get(x, y))
    }
}

fn parse(input: &str) -> Grid {
    let row_size = input.lines().next().unwrap().len();
    let heights: Vec<_>= input.lines().flat_map(|s| s.chars()).map(|i| i.to_digit(10).unwrap()).collect();
    let rows = heights.len() / row_size;
    Grid{
        heights,
        row_size,
        rows,
    }
}

fn solve_2(input: &str) -> usize {
    let mut grid = parse(input);
    let low_points: Vec<_> = (0..grid.row_size as isize).cartesian_product(0..grid.rows as isize)
    .filter(|&(x, y)| grid.is_low_point(x, y)).collect();

    //println!("{:#?}", low_points);

    let mut floodfill_stack = Vec::new();
    let mut bassin_sizes = Vec::new();
    for (x, y) in low_points {


        floodfill_stack.push((x, y));
        grid.set(x, y, 9);
        let mut bassin_size = 0;

        while floodfill_stack.len() > 0 {
            let (x, y) = floodfill_stack.pop().unwrap();
            bassin_size += 1;

            for (xi, yi) in [(x+1, y), (x-1, y), (x, y+1), (x, y-1)]{
                if grid.get(xi, yi) != 9 {
                    floodfill_stack.push((xi, yi));
                    grid.set(xi, yi, 9);
                }
            }
        }
        bassin_sizes.push(bassin_size);
    }

    bassin_sizes.sort();
    bassin_sizes.iter().rev().take(3).fold(1, |acc, curr| acc * curr)
}

fn solve_1(input: &str) -> u32 {
    let grid = parse(input);

    let mut total = 0u32;
    for x in 0..grid.row_size as isize {
        for y in 0..grid.rows as isize {
            if grid.is_low_point(x, y) {
                total += grid.get(x, y) + 1;
            }
        }
    }
    total
}


#[test]
fn test_sample1() {
    let input = include_str!("sample.txt");
    assert_eq!(solve_1(&input), 15)
}

#[test]
fn test_sample2() {
    let input = include_str!("sample.txt");
    assert_eq!(solve_2(&input), 12)
}