fn main() {
    let input = include_str!("input.txt");

    let map : Vec<Vec<Tile>> = input.split("\n")
        .map(str::trim)
        .map(|line| {
            line.chars()
                .map(|c| c.into())
                .collect()
        })
        .collect();

    

    println!("PART 1: {}", longest_path(&map, true));
    println!("PART 2: {}", longest_path(&map, false));
}

struct Graph {
    nodes : Vec<Pos>,
    connections : Vec<Edge>,
}

struct Edge {
    junction_a : usize,
    junction_b : usize,
    length : usize,
}

fn graphify(map : &Vec<Vec<Tile>>) -> Graph {
    
}

fn longest_path(map : &Vec<Vec<Tile>>, slopes : bool) -> usize {
    let begin_path = vec![Pos { x: 1, y: 0 }];
    let mut paths : Vec<Vec<Pos>> = vec![begin_path];

    let mut lengths : Vec<usize> = vec![];
        
    while paths.len() > 0 {
        let curr_path = paths.pop().unwrap();

        let last = curr_path.last().unwrap();

        if last.y == map.len() - 1 && last.x == map[0].len() - 2 {
            lengths.push(curr_path.len() - 1);
            continue;
        }

        for pos in last.next_pos(&map, slopes)
            .iter()
            .filter(|pos| !curr_path.contains(pos)) {
            let mut new_path = curr_path.clone();
            new_path.push(*pos);
            paths.push(new_path);
        }
    }

    *lengths.iter().max().unwrap()
}

fn is_passable(pos : &Pos, map : &Vec<Vec<Tile>>) -> bool {
    match map[pos.y][pos.x] {
        Tile::Wall => false,
        _ => true,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pos {
    x : usize,
    y : usize,
}

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Open,
    Wall,
    Junction,
    Slope(Direction),
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Open,
            '#' => Self::Wall,
            '>' => Self::Slope(Direction::Right),
            '<' => Self::Slope(Direction::Left),
            '^' => Self::Slope(Direction::Up),
            'v' => Self::Slope(Direction::Down),
            _ => panic!()
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Right,
    Left
}

impl Pos {
    fn next_pos(self, map : &Vec<Vec<Tile>>, slopes : bool) -> Vec<Pos> {
        let mut next = vec![];

        if slopes {
            match &map[self.y][self.x] {
                Tile::Open | Tile::Junction => {
                    if self.y != 0 {
                        next.push(Pos { x: self.x, y: self.y - 1 })
                    }
                    if self.y != map.len() - 1 {
                        next.push(Pos { x: self.x, y: self.y + 1 })
                    }
                    if self.x != map[0].len() - 1 {
                        next.push(Pos { x: self.x + 1, y: self.y })
                    }
                    if self.x != 0 {
                        next.push(Pos { x: self.x - 1, y: self.y })
                    }
                },
                Tile::Wall => {
                    // Nothing
                },
                Tile::Slope(dir) => {
                    match dir {
                        Direction::Up => {
                            if self.y != 0 {
                                next.push(Pos { x: self.x, y: self.y - 1 })
                            }
                        },
                        Direction::Down => {
                            if self.y != map.len() - 1 {
                                next.push(Pos { x: self.x, y: self.y + 1 })
                            }
                        },
                        Direction::Right => {
                            if self.x != map[0].len() - 1 {
                                next.push(Pos { x: self.x + 1, y: self.y })
                            }
                        },
                        Direction::Left => {
                            if self.x != 0 {
                                next.push(Pos { x: self.x - 1, y: self.y })
                            }
                        },
                    }
                },
            }
        } else {
            if self.y != 0 {
                next.push(Pos { x: self.x, y: self.y - 1 })
            }
            if self.y != map.len() - 1 {
                next.push(Pos { x: self.x, y: self.y + 1 })
            }
            if self.x != map[0].len() - 1 {
                next.push(Pos { x: self.x + 1, y: self.y })
            }
            if self.x != 0 {
                next.push(Pos { x: self.x - 1, y: self.y })
            }
        }

        next.iter().filter(|pos| is_passable(pos, map)).map(|pos| *pos).collect()
    }
}