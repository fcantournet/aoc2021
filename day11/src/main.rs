use ndarray::{Array2};

fn main() {
    let input = include_str!("input.txt");
    println!("Result 1 : {:?}", solve_1(input));
    println!("Result 2 : {:?}", solve_2(input));
}


#[derive(Debug, Clone, PartialEq)]
struct Cell {
    level: u8,
    flashed: bool,
}

struct World {
    cells: Array2<Cell>,

    rows: isize,
    cols: isize,
}

impl World {

    // Return list of up to 8 neighbourgs clipped to the grid rows * cols
    fn neighbourgs(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let x = x as isize;
        let y = y as isize;
        [(x-1, y-1), (x-1, y), (x-1, y+1), (x, y-1), (x, y+1), (x+1, y-1), (x+1, y), (x+1, y+1)].iter()
        .filter(|&&(xi, yi)| xi >= 0 && xi < self.rows && yi >= 0 && yi < self.cols)
        .map(|&(xi, yi)| (xi as usize, yi as usize)).collect()
    }

    fn step(&mut self) -> usize {

        self.cells.map_inplace(|c| c.level += 1);

        // We use a stack of octopuses to flash
        let mut to_flash: Vec<_> = self.cells.indexed_iter().filter_map(|((x, y), c)| if c.level == 10 { Some((x, y)) } else { None } ).collect();

        while to_flash.len() != 0 {
            let (x, y) = to_flash.pop().unwrap().clone();
            let mut n : Vec<_> = self.neighbourgs(x, y);
            n.retain(|&(xi, yi)| !self.cells[[xi, yi]].flashed);
            for (xi, yi) in n {
                let c = self.cells.get_mut((xi, yi)).unwrap();
                c.level += 1;
                if c.level == 10 {
                   to_flash.push((xi, yi))
                }                
            }
            self.cells[[x, y]].flashed = true;
        }

        self.cells.iter_mut().fold(0, |acc, c| if c.flashed { c.flashed = false; c.level = 0; acc + 1} else {acc})
    }
}

fn parse(input: &str) -> World {
    let cells: Vec<_>= input.lines().flat_map(|s| s.chars()).map(|i| Cell{ level: i.to_digit(10).unwrap() as u8, flashed: false }).collect();


    World { cells: Array2::from_shape_vec((10,10),cells).unwrap(), rows: 10, cols: 10 }
}

fn solve_2(input: &str) -> usize {
    let mut world = parse(input);
    let mut steps = 0usize;
    while steps < 1000 {
        steps += 1;
        if world.step() == 100 {
            return steps
        }
    }
    panic!("unlikely it would take more than 1000 steps")
}

fn solve_1(input: &str) -> usize {
    let mut world = parse(input);
    let mut flashes = 0usize;
    for _ in 0..100 {
        flashes += world.step();
    }
    flashes
}


#[test]
fn test_sample1() {
    let input = include_str!("sample.txt");
    assert_eq!(solve_1(&input), 1656)
}

#[test]
fn test_sample2() {
    let input = include_str!("sample.txt");
    assert_eq!(solve_2(&input), 195)
}