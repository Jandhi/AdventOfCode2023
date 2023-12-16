fn main() {
    let input = include_str!("input.txt");
    let map : Vec<Vec<Tile>> = input.split("\n")
        .map(str::trim)
        .map(|line| line
            .chars()
            .map(|c| c.into())
            .collect()
        )
        .collect();
   

    part1(&map);
    part2(&map);
}

fn part1(map : &Vec<Vec<Tile>>) {
    println!("PART 1: {}", find_energized_sum(map, Beam{
        x: 0,
        y: 0,
        direction: Direction::Right,
    }));
}

fn part2(map : &Vec<Vec<Tile>>) {
    let mut starting_beams : Vec<Beam> = vec![];

    for x in 0..map[0].len() {
        starting_beams.push(Beam { x: x, y: 0, direction: Direction::Down });
        starting_beams.push(Beam { x: x, y: map.len() - 1, direction: Direction::Up });
    }

    for y in 0..map.len() {
        starting_beams.push(Beam { x: 0, y: y, direction: Direction::Right });
        starting_beams.push(Beam { x: map[0].len() - 1, y: y, direction: Direction::Left });
    }

    let max_energized = starting_beams
        .iter()
        .map(|start| find_energized_sum(map, start.clone()))
        .max()
        .unwrap();

    println!("PART 2: {}", max_energized);
}

fn find_energized_sum(map : &Vec<Vec<Tile>>, start: Beam) -> usize
{
    let mut beams = vec![start];

    let mut energized : Vec<Vec<bool>> = map.iter()
    .map(|line| line.iter()
        .map(|_| false)
        .collect()
    ).collect();

    let mut visited : Vec<Vec<Vec<Direction>>> = energized
        .iter()
        .map(|line| line
            .iter()
            .map(|_| vec![])
            .collect()
        ).collect();
    
    while beams.len() > 0 {
        let beam = beams.pop().unwrap();

        energized[beam.y][beam.x] = true;

        if visited[beam.y][beam.x].contains(&beam.direction) {
            continue;
        } else {
            visited[beam.y][beam.x].push(beam.direction);
        }


        // Direction
        let mut next_directions : Vec<Direction> = vec![];

        match map[beam.y][beam.x] {
            Tile::Empty => {
                next_directions.push(beam.direction);
            },
            Tile::SlashMirror => {
                match beam.direction {
                    Direction::Up => {
                        next_directions.push(Direction::Right);
                    },
                    Direction::Right => {
                        next_directions.push(Direction::Up);
                    },
                    Direction::Down => {
                        next_directions.push(Direction::Left);
                    },
                    Direction::Left => {
                        next_directions.push(Direction::Down);
                    }
                }
            },
            Tile::BackslashMirror => {
                match beam.direction {
                    Direction::Up => {
                        next_directions.push(Direction::Left);
                    },
                    Direction::Right => {
                        next_directions.push(Direction::Down);
                    },
                    Direction::Down => {
                        next_directions.push(Direction::Right);
                    },
                    Direction::Left => {
                        next_directions.push(Direction::Up);
                    },
                }
            },
            Tile::VerticalSplitter => {
                match beam.direction {
                    Direction::Up | Direction::Down => {
                        next_directions.push(beam.direction);
                    }
                    _ => {
                        next_directions.push(Direction::Up);
                        next_directions.push(Direction::Down);
                    }
                }
            },
            Tile::HorizontalSplitter => {
                match beam.direction {
                    Direction::Right | Direction::Left => {
                        next_directions.push(beam.direction);
                    }
                    _ => {
                        next_directions.push(Direction::Right);
                        next_directions.push(Direction::Left);
                    }
                }
            },
        };

        for direction in next_directions {
            match direction {
                Direction::Up => {
                    if beam.y == 0 {
                        continue;
                    }

                    beams.push(Beam { x: beam.x, y: beam.y - 1, direction });
                },
                Direction::Right => {
                    if beam.x == map[0].len() - 1 {
                        continue;
                    }

                    beams.push(Beam { x: beam.x + 1, y: beam.y, direction });
                },
                Direction::Down => {
                    if beam.y == map.len() - 1 {
                        continue;
                    }

                    beams.push(Beam { x: beam.x, y: beam.y + 1, direction });
                },
                Direction::Left => {
                    if beam.x == 0 {
                        continue;
                    }

                    beams.push(Beam { x: beam.x - 1, y: beam.y, direction });
                },
            }
        }
    }

    energized
        .iter()
        .map(|line| line.iter()
            .filter(|is_energized| **is_energized)
            .count()
        ).sum()
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Tile {
    Empty,
    SlashMirror,
    BackslashMirror,
    VerticalSplitter,
    HorizontalSplitter,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '/'  => Self::SlashMirror,
            '\\' => Self::BackslashMirror,
            '|'  => Self::VerticalSplitter,
            '-'  => Self::HorizontalSplitter,
            _ => Self::Empty,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy)]
struct Beam {
    x : usize,
    y : usize,
    direction :Direction,
}

