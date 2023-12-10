use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");

    let map = input.split("\n").map(str::trim)
        .map(|line| {
            line.chars()
                .map(Tile::get)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let start_pos = map.iter()
        .enumerate()
        .map(|(y, line)| {
            (y, line.iter()
                .enumerate()
                .find(|(x, tile)| {
                    **tile == Tile::S
                }))
        })
        .filter(|(y, opt)| opt.is_some())
        .map(|(y, opt)| {
            Position {
                x: opt.unwrap().0,
                y: y,
            }
        })
        .last().unwrap();

    let mut visited : HashMap<Position, usize> = HashMap::new();
    visited.insert(start_pos, 0);
    let mut visit_stack : Vec<Position> = vec![start_pos];
    let directions = Direction::all();

    while visit_stack.len() > 0 {
        let curr_pos = visit_stack.remove(0);
        let curr_tile = &map[curr_pos.y][curr_pos.x];
        let distance = visited[&curr_pos];

        for direction in directions.iter() {
            if !curr_tile.connects(direction) {
                continue;
            }

            if let Some(next_pos) = curr_pos.towards(*direction, &map) {
                let next_tile = &map[next_pos.y][next_pos.x];

                if visited.contains_key(&next_pos) {
                    continue;
                }

                if !next_tile.connects(&direction.inverse()) {
                    continue;
                }

                // We know we want to go here
                visited.insert(next_pos, distance + 1);
                visit_stack.push(next_pos);
            }
        }
    }

    part1(&visited);
    part2(&map, &visited);
        
}

fn part1(visited : &HashMap<Position, usize>) {
    let max = visited.iter()
        .map(|(_, distance)| distance)
        .max()
        .unwrap();

    println!("PART 1: {}", max);
}

fn part2(map : &Vec<Vec<Tile>>, visited : &HashMap<Position, usize>) {
    let mut enclosed = 0;
    
    for y in 0..map.len() {
        let mut is_outside = true;
        let mut coming_from_north = true;

        for x in 0..map[0].len() {
            let mut tile = map[y][x];
            let pos = Position { x, y };

            // Fix S
            if tile == Tile::S {
                let connects_north = map[y - 1][x].connects(&Direction::South);
                let connects_south = map[y + 1][x].connects(&Direction::North);
                let connects_west = map[y][x - 1].connects(&Direction::East);
                let connects_east = map[y][x + 1].connects(&Direction::West);

                if connects_north && connects_south {
                    tile = Tile::NorthSouth;
                } else if connects_north && connects_west {
                    tile = Tile::NorthWest;
                } else if connects_north && connects_east {
                    tile = Tile::NorthEast;
                } else if connects_east && connects_west {
                    tile = Tile::EastWest;
                } else if connects_east && connects_south {
                    tile = Tile::SouthEast;
                } else {
                    tile = Tile::SouthWest;
                }
            }

            if visited.contains_key(&pos) {
                if tile == Tile::NorthSouth {
                    is_outside = !is_outside;
                }

                if tile == Tile::NorthEast {
                    coming_from_north = true;
                }

                if tile == Tile::SouthEast {
                    coming_from_north = false;
                }

                if tile == Tile::NorthWest && !coming_from_north {
                    is_outside = !is_outside;
                }

                if tile == Tile::SouthWest && coming_from_north {
                    is_outside = !is_outside;
                }
            } else {
                if !is_outside {
                    enclosed += 1;
                }
            }
        }
    }

    println!("PART 2: {}", enclosed);
}

#[derive(Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West
}

impl Direction {
    fn all() -> Vec<Direction> {
        vec![Self::North, Self::South, Self::East, Self::West]
    }

    fn inverse(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position { 
    x : usize,
    y : usize
}

impl Position {
    fn towards(&self, direction : Direction, map : &Vec<Vec<Tile>>) -> Option<Position> {
        match direction {
            Direction::North => {
                if self.y == 0 {
                    None
                } else {
                    Some(Position { x: self.x, y: self.y - 1 })
                }
            },
            Direction::South => {
                if self.y == map.len() - 1 {
                    None
                } else {
                    Some(Position { x: self.x, y: self.y + 1 })
                }
            },
            Direction::East => {
                if self.x == map[0].len() - 1 {
                    None
                } else {
                    Some(Position { x: self.x + 1, y: self.y })
                }
            },
            Direction::West => {
                if self.x == 0 {
                    None
                } else {
                    Some(Position { x: self.x - 1, y: self.y })
                }
            }
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum Tile {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    S,
}

impl Tile {
    pub fn get(c : char) -> Tile {
        match c {
            '|' => Self::NorthSouth,
            '-' => Self::EastWest,
            'L' => Self::NorthEast,
            'J' => Self::NorthWest,
            '7' => Self::SouthWest,
            'F' => Self::SouthEast,
            'S' => Self::S,
            _ => Self::Ground,
        }
    }

    pub fn connects(&self, direction : &Direction) -> bool {
        match direction {
            Direction::North => match self {
                Self::NorthSouth | Self::NorthEast | Self::NorthWest | Self::S => true,
                _ => false,
            },
            Direction::South => match self {
                Self::NorthSouth | Self::SouthWest | Self::SouthEast | Self::S => true, 
                _ => false,
            }
            Direction::East => match self {
                Self::NorthEast | Self::EastWest | Self::SouthEast | Self::S => true,
                _ => false,
            }
            Direction::West => match self {
                Self::NorthWest | Self::EastWest | Self::SouthWest | Self::S => true,
                _ => false,
            }
        }
    }
}