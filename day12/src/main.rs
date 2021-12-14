use std::collections::{HashMap, HashSet};



#[derive(Debug, PartialEq, Clone, Eq, Hash)]
enum Cave {
    Big(String),
    Small(String),
}

type World = HashMap<Cave, HashSet<Cave>>;

fn main() {
    let input = include_str!("input.txt");
    println!("Result 1 : {:?}", solve_1(input));
    println!("Result 2 : {:?}", solve_2(input));
}


fn parse(input: &str) -> World {
    let edges: Vec<_> = input.lines().map(|s| {
        let mut v = s.split('-').map(|ss|{
            if ss.chars().next().unwrap().is_lowercase() {
                Cave::Small(ss.into())
            }else {
                Cave::Big(ss.into())
            }
        });
        (v.next().unwrap(),v.next().unwrap())
    }).collect();

    let mut world = World::default();
    for (a,b) in edges {
        world.entry(a.clone()).or_insert(HashSet::default()).insert(b.clone());
        world.entry(b).or_insert(HashSet::default()).insert(a);
    }

    world
}


fn solve_2(input: &str) -> usize {
    let world = parse(input);
    count_paths(&Cave::Small("start".into()), &world, HashSet::default(), true)
}

fn solve_1(input: &str) -> usize {
    let world = parse(input);
    count_paths(&Cave::Small("start".into()), &world, HashSet::default(), false)
}


fn count_paths(current: &Cave, world: &World, mut visited: HashSet<Cave>, mut can_visit_twice: bool) -> usize {

    if *current == Cave::Small("end".into()) {
        return 1;
    }

    if visited.contains(&current) {
        if !can_visit_twice || *current == Cave::Small("start".into()) {
            return 0;
        }
        can_visit_twice = false;
    }

    if let Cave::Small(_)  = current {
        visited.insert(current.clone());
    }

    let mut paths = 0usize;
    for n in world[&current].iter() {
        paths += count_paths(&n, world, visited.clone(), can_visit_twice)
    }
    paths
}



#[test]
fn test_sample1() {
    let input = include_str!("sample.txt");
    assert_eq!(solve_1(&input), 226)
}

#[test]
fn test_sample2() {
    let input = include_str!("sample.txt");
    assert_eq!(solve_2(&input), 3509)
}