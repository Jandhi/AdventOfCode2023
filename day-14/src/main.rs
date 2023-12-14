fn main() {
    let input = include_str!("test.txt");
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
        break; // temp
        
        if board == before_cycle {
            break;
        } else {
            before_cycle = board.clone();
        }
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
    print_board(&board);
    tilt(board, Direction::West);
    print_board(&board);
    tilt(board, Direction::South);
    print_board(&board);
    tilt(board, Direction::East);
    print_board(&board);
}

fn tilt(board : &mut Vec<Vec<Tile>>, direction : Direction) {
    // TODO more elegant tilt
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