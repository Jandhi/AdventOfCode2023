use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");

    let mut start : Pos = Pos { x: 0, y: 0 };

    let map : Vec<Vec<Tile>> = input.split("\n")
        .map(str::trim)
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' {
                        start = Pos { x, y };
                    }
                    c.into()
                })
                .collect()
        })
        .collect();

    
    let visited = visit(&map, start);

    let distances : Vec<Vec<usize>> = map.iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(|(x, tile)| {
                    let pos = Pos { x, y };
                    if visited.contains_key(&pos) {
                        visited[&pos]
                    } else {
                        usize::MAX
                    }
                })
                .collect()
        })
        .collect();

    part1(&visited);

    println!("Visited {}", visited[&Pos{ x: 3, y: 0 }]);
    println!("w {} h {}", map[0].len(), map.len());

    let width = map[0].len();
    let height = map.len();

    let start_to_top = visited[&Pos { x: start.x, y : 0}];
    let start_to_bottom = visited[&Pos { x: start.x, y : height - 1}];
    let start_to_right = visited[&Pos { x: width - 1, y : start.y}];
    let start_to_left = visited[&Pos { x: 0, y : start.y}];

    println!("TOP {} BOTTOM {} RIGHT {} LEFT {}", start_to_top, start_to_bottom, start_to_right, start_to_left);

    let visited_from_bottom = visit(&map, start_pos);
}

fn part1(visited : &HashMap<Pos, usize>) {
    let sum = visited.iter()
        .filter(|(_, steps)| **steps <= 64 && **steps % 2 == 0)
        .count();

    println!("PART 1: {}", sum);
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Pos {
    x : usize,
    y : usize
}

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Garden,
    Rocks
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            'S' | '.' => Self::Garden,
            '#' => Self::Rocks,
            _ => panic!("Unexpected value: {}", value)
        }
    }
}

fn visit(map : &Vec<Vec<Tile>>, start_pos : Pos) -> HashMap<Pos, usize> {
    let mut visited : HashMap<Pos, usize> = HashMap::new();
    let mut stack : Vec<(Pos, usize)> = vec![
        (start_pos, 0)
    ];

    while stack.len() > 0 {
        let (pos, steps) = stack.remove(0);

        if visited.contains_key(&pos) {
            continue;
        } else {
            visited.insert(pos, steps);
        }

        if pos.y > 0 && map[pos.y - 1][pos.x] != Tile::Rocks {
            stack.push((Pos { x: pos.x, y : pos.y - 1}, steps + 1))
        }
        if pos.y < map.len() - 1 && map[pos.y + 1][pos.x] != Tile::Rocks {
            stack.push((Pos { x: pos.x, y : pos.y + 1}, steps + 1))
        }
        if pos.x > 0 && map[pos.y][pos.x - 1] != Tile::Rocks {
            stack.push((Pos { x: pos.x - 1, y : pos.y}, steps + 1))
        }
        if pos.x < map[0].len() - 1 && map[pos.y][pos.x + 1] != Tile::Rocks {
            stack.push((Pos { x: pos.x + 1, y : pos.y}, steps + 1))
        }
    }

    visited
}