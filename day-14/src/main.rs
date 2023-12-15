fn main() {
    let input = include_str!("input.txt");
    let board : Vec<Vec<Tile>> = input.split("\n")
        .map(str::trim)
        .map(|line| {
            line.chars()
                .map(|c| c.into())
                .collect()
        })
        .collect();

    part1(&board);
    part2(&board);
}

fn print_board(board : &Vec<Vec<Tile>>) {
    let mut str = "".to_string();

    for line in board.iter() {
        for tile in line.iter() {
            str.push(match tile {
                Tile::Empty => '.',
                Tile::Round => 'O',
                Tile::Square => '#',
            });
        }

        str.push('\n');
    }

    println!("{}", str);
}

fn part1(input_board : &Vec<Vec<Tile>>) {
    let mut board = input_board.clone();
    tilt(&mut board, Direction::North);
    println!("PART 1: {}", load(&board));
}

fn part2(input_board : &Vec<Vec<Tile>>) {
    let mut board = input_board.clone();

    let mut before_cycle = board.clone();

    
    
    loop {
        spin_cycle(&mut board);
        
        if board == before_cycle {
            break;
        } else {
            before_cycle = board.clone();
        }

        println!("LOAD: {}", load(&board));
    }

    println!("PART 2: {}", load(&board));
}

#[derive(PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn load(board : &Vec<Vec<Tile>>) -> usize {
    let height = board.len();
    board.iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(|(x, tile)| {
                    match tile {
                        Tile::Round => height - y,
                        _ => 0,
                    }
                })
                .sum::<usize>()
        })
        .sum()
}

fn spin_cycle(board : &mut Vec<Vec<Tile>>) {
    tilt(board, Direction::North);
    tilt(board, Direction::West);
    tilt(board, Direction::South);
    tilt(board, Direction::East);
}

#[derive(PartialEq, Eq, Copy, Clone)]
struct Pos {
    x : usize,
    y : usize
}

impl Pos {
    fn next(&self, direction : &Direction, board : &Vec<Vec<Tile>>) -> Option<Pos> {
        match direction {
            Direction::North => match self.y > 0 {
                true => Some(Pos { x: self.x, y: self.y - 1 }),
                false => None,
            },
            Direction::South => match self.y < board.len() - 1 {
                true => Some(Pos { x: self.x, y: self.y + 1}),
                false => None,
            }
            Direction::East => match self.x < board[0].len() - 1 {
                true => Some(Pos { x: self.x + 1, y: self.y }),
                false => None,
            },
            Direction::West => match self.x > 0 {
                true => Some(Pos { x: self.x - 1, y: self.y }),
                false => None,
            },
        }
    }

    fn get(&self, board : &Vec<Vec<Tile>>) -> Tile {
        board[self.y][self.x]
    }

    fn set(&self, board : &mut Vec<Vec<Tile>>, value : Tile) {
        board[self.y][self.x] = value;
    }

    fn start_positions(direction : &Direction, board : &Vec<Vec<Tile>>) -> Vec<Pos> {
        let mut positions = vec![];

        match direction {
            Direction::North => {
                for x in 0..board[0].len() {
                    positions.push(Pos { x, y: board.len() - 1 });
                }
            },
            Direction::South => {
                for x in 0..board[0].len() {
                    positions.push(Pos { x, y: 0 })
                }
            },
            Direction::East => {
                for y in 0..board.len() {
                    positions.push(Pos { x: 0, y })
                }
            },
            Direction::West => {
                for y in 0..board.len() {
                    positions.push(Pos { x: board[0].len() - 1, y });
                }
            },
        }

        positions
    }
}

fn tilt(board : &mut Vec<Vec<Tile>>, direction : Direction) {
    for start_pos in Pos::start_positions(&direction, board) {
        let mut run : Vec<Pos> = vec![];
        let mut round_count = 0;
        let mut curr_pos = start_pos;

        loop {
            if curr_pos.get(board) == Tile::Square {
                if run.len() > 0 {
                    for (index, pos) in run.iter().enumerate() {
                        if run.len() - index <= round_count {
                            pos.set(board, Tile::Round);
                        } else {
                            pos.set(board, Tile::Empty);
                        }
                    }

                    round_count = 0;
                    run.clear();
                }
            } else {
                run.push(curr_pos);

                if curr_pos.get(board) == Tile::Round {
                    round_count += 1;
                }
            }

            match curr_pos.next(&direction, board) {
                Some(pos) => curr_pos = pos,
                None => break,
            };
        }

        if run.len() > 0 {
            for (index, pos) in run.iter().enumerate() {
                if run.len() - index <= round_count {
                    pos.set(board, Tile::Round);
                } else {
                    pos.set(board, Tile::Empty);
                }
            }

            round_count = 0;
            run.clear();
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Tile {
    Empty,
    Round,
    Square,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            'O' => Self::Round,
            '#' => Self::Square,
            _ => Self::Empty,
        }
    }
}