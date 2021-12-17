use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::cmp::Reverse;

fn main() {
    let input = include_str!("input.txt");
    println!("Result 1 : {:?}", solve_1(input));
    println!("Result 2 : {:?}", solve_2(input));
}

type CostFn = fn(&Grid, isize, isize) -> Option<usize>;

struct Grid {
    map: HashMap<(isize, isize), usize>,
    rows: usize,
    cols: usize,
}
impl Grid {
    fn get(&self, x: isize, y: isize) -> Option<usize> {
        self.map.get(&(x, y)).and_then(|x| Some(*x))
    }

    fn tiled_cost(&self, x: isize, y: isize) -> Option<usize> {
        let R = x / self.cols as isize;
        let D = y / self.rows as isize;
        if R > 4 || D > 4 {
            return None;
        }

        let cost = *self.map.get(&(x % self.rows as isize , y % self.cols as isize))?;
        let n = (R + D) as usize;
        Some((cost + n -1) % 9 + 1)
    }
}


fn parse(input: &str) -> Grid {
    let mut map = HashMap::new();
    let mut x_max = 0usize;
    let mut y_max = 0usize;
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map.insert((x as isize, y as isize), c.to_digit(10).unwrap() as usize);
            x_max = x;
            y_max = y;
        }
    }
    Grid{map, cols: x_max + 1, rows: y_max +1}
}

fn solve_2(input: &str) -> usize {
    let grid = parse(input);
    let mut visited = HashSet::<(isize, isize)>::default();
    shortest_path(5, &mut visited, &grid, Grid::tiled_cost )
}

fn solve_1(input: &str) -> usize {
    let grid = parse(input);
    let mut visited = HashSet::<(isize, isize)>::default();
    shortest_path(1, &mut visited, &grid, Grid::get )
}



fn shortest_path(tiling: usize, visited: &mut HashSet<(isize,isize)>, grid: &Grid, cost_fn: CostFn) -> usize {

    let mut pq = BinaryHeap::new();
    pq.push((Reverse(0usize), (0isize, 0isize)));

    let x_max = (grid.cols * tiling - 1) as isize;
    let y_max = (grid.rows * tiling - 1) as isize;

    while let Some((cost, (x, y))) = pq.pop() {
        println!("Visiting ({}, {} at cost {}", x, y, cost.0);
        if x == x_max && y == y_max {
            return cost.0;
        }
        if visited.contains(&(x, y)) {
            continue;
        }


        for (dx, dy) in [(-1, 0), (1,0), (0, -1), (0, 1)] {
            if let Some(next_cost) = cost_fn(&grid, x+dx, y+dy) {
                pq.push((Reverse(next_cost+cost.0), (x+dx, y+dy)));
            }
        }

        visited.insert((x, y));
    }
    0
}


#[test]
fn test_sample1() {
    let input = include_str!("sample.txt");
    assert_eq!(solve_1(&input), 40)
}

#[test]
fn test_sample2() {
    let input = include_str!("sample.txt");
    let grid = parse(input);
    // assert_eq!(grid.tiled_cost(12, 10) , Some(8)); 
    assert_eq!(grid.tiled_cost(49, 49) , Some(1));
    assert_eq!(solve_2(&input), 315)
}