use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    let moves : Vec<Move> = input.split("\n")
        .map(str::trim)
        .map(|str| str.into())
        .collect();
    
    let (start_x, start_y, mut grid) = starting_coords_and_grid(&moves);
    dig(start_x, start_y, &moves, &mut grid);
    let count = flood_fill_count(&grid);

    println!("PART 1: {}", count);

    let count2 = shoelace(&points_p2(&moves));

    println!("PART 2: {}", count2);
}

fn points_p1(moves : &Vec<Move>) -> Vec<(i64, i64)> {
    let (mut x, mut y) = (0, 0);

    let mut points : Vec<(i64, i64)> = vec![(x, y)];

    for mv in moves.iter() {
        match mv.direction {
            Direction::Up => y -= mv.amount as i64,
            Direction::Left => x -= mv.amount as i64,
            Direction::Right => x += mv.amount as i64,
            Direction::Down => y += mv.amount as i64,
        }
        println!("POINT ({}, {})", x, y);
        points.push((x, y));
    }

    points
}

fn points_p2(moves : &Vec<Move>) -> Vec<(i64, i64)> {
    let (mut x, mut y) = (0, 0);

    let mut points : Vec<(i64, i64)> = vec![(x, y)];

    for mv in moves.iter() {
        match mv.hex_dir {
            Direction::Up => y -= mv.hex_amount,
            Direction::Left => x -= mv.hex_amount,
            Direction::Right => x += mv.hex_amount,
            Direction::Down => y += mv.hex_amount,
        }
        points.push((x, y));
    }

    points
}

fn shoelace(points : &Vec<(i64, i64)>) -> i64 {
    let mut sum = 0;
    let mut perimeter = 0;

    for i in 0..points.len() {
        let (p1, p2) = (points[i], match i == points.len() - 1 {
            false => points[i + 1],
            true => points[0]
        });

        perimeter += (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs();

        let det = p1.0 * p2.1 - p1.1 * p2.0;


        sum += det;
    }

    (sum + perimeter) / 2 + 1
}

fn flood_fill_count(grid : &Vec<Vec<bool>>) -> usize {
    let count = 0;
    let mut not_dug_count = 0;

    let mut queue : Vec<(usize, usize)> = vec![];

    for x in 0..grid[0].len() {
        queue.push((x, 0));
        queue.push((x, grid.len() - 1));
    }

    for y in 0..grid.len() {
        queue.push((0, y));
        queue.push((grid[0].len() - 1, y));
    }

    let mut visited : HashSet<(usize, usize)> = HashSet::new();

    while queue.len() > 0 {
        let (x, y) = queue.remove(0);
        
        if visited.contains(&(x, y)) {
            continue;
        }

        visited.insert((x, y));
        
        if grid[y][x] {
            continue;
        }

        not_dug_count += 1;

        if x != 0 {
            queue.push((x - 1, y))
        }
        if x != grid[0].len() - 1 {
            queue.push((x + 1, y))
        }
        if y != 0 {
            queue.push((x, y - 1))
        }
        if y != grid.len() - 1 {
            queue.push((x, y + 1))
        }
    }

    (grid.len() * grid[0].len()) - not_dug_count
}

fn starting_coords_and_grid(moves : &Vec<Move>) -> (usize, usize, Vec<Vec<bool>>) 
{
    let (mut min_x, mut min_y, mut max_x, mut max_y) = (0, 0, 0, 0);

    {
        let (mut x, mut y) = (0, 0);
        
        for mv in moves.iter() {
            match mv.direction {
                Direction::Up => y -= mv.amount,
                Direction::Left => x -= mv.amount,
                Direction::Right => x += mv.amount,
                Direction::Down => y += mv.amount,
            }

            min_x = min_x.min(x);
            min_y = min_y.min(y);
            max_x = max_x.max(x);
            max_y = max_y.max(y);
        }
    }

    let start_x = -1 * min_x;
    let start_y = -1 * min_y;

    let length = max_x - min_x + 1;
    let height = max_y - min_y + 1;

    let mut grid : Vec<Vec<bool>> = vec![];

    for y in 0..height {
        grid.push(vec![]);

        for _ in 0..length {
            grid[y as usize].push(false);
        }
    }

    (start_x as usize, start_y as usize, grid)
}

fn dig(start_x : usize, start_y : usize, moves : &Vec<Move>, grid : &mut Vec<Vec<bool>>) {
    grid[start_y][start_x] = true;

    let (mut x, mut y) = (start_x, start_y);

    for mv in moves {
        for _ in 0..mv.amount {
            match mv.direction {
                Direction::Up => y -= 1,
                Direction::Left => x -= 1,
                Direction::Right => x += 1,
                Direction::Down => y += 1,
            };

            grid[y][x] = true;
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Left,
    Right,
    Down,
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "U" => Self::Up,
            "R" => Self::Right,
            "D" => Self::Down,
            "L" => Self::Left,
            _ => panic!("Invalid input {}", value),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Move {
    direction : Direction,
    amount : i32,
    hex_dir : Direction,
    hex_amount : i64,
}

impl From<&str> for Move {
    fn from(value: &str) -> Self {
        let parts : Vec<&str> = value.split(" ").collect();
        let color = &parts[2][2..parts[2].len() - 1];
        Self { 
            direction: parts[0].into(), 
            amount: parts[1].parse::<i32>().unwrap(), 
            hex_dir: match &color[5..] {
                "0" => Direction::Right,
                "1" => Direction::Down,
                "2" => Direction::Left,
                "3" => Direction::Up,
                _ => panic!()
            },
            hex_amount: i64::from_str_radix(&color[0..5], 16).unwrap()
        }
    }
}