use core::num;
use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");

    let parts : Vec<Vec<char>> = input
        .split("\n")
        .map(str::trim)
        .map(|line| {
            line.chars().collect::<Vec<char>>()
        })
        .collect();
    let parts_map = read_map(&parts);
    
    part1(&parts_map);
    part2(&parts_map);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    x : usize,
    y : usize
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Number {
    pos : Position,
    length : usize,
    value : u32,
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Number(Number),
    Gear,
    Symbol,
    Nothing
}

fn read_map(parts : &Vec<Vec<char>>) -> Vec<Vec<Tile>> {
    let mut parts_map = vec![];
    for y in 0..parts.len() {
        let mut row = vec![];

        for x in 0..parts[0].len() {
            row.push(Tile::Nothing);
        }

        parts_map.push(row);
    }

    for y in 0..parts.len() {
        let mut x = 0;
        let mut number_str = "".to_string();

        while x < parts[0].len() {
            let curr_char = parts[y][x];
            
            match curr_char {
                '0'..='9' => {
                    number_str.push(curr_char);
                },
                _  => {
                    if number_str.len() > 0 {
                        let number = Number {
                            pos: Position { x: x - number_str.len(), y },
                            length: number_str.len(),
                            value: number_str.parse().expect(&format!("Couldn't parse {}", number_str)),
                        };

                        for x_diff in 0..number_str.len() {
                            parts_map[y][x - 1 - x_diff] = Tile::Number(number);
                        }

                        number_str.clear();
                    }

                    match curr_char {
                        '.' => {
                            // We do nothing
                        },
                        '*' => {
                            parts_map[y][x] = Tile::Gear;
                        },
                        _ => {
                            parts_map[y][x] = Tile::Symbol;
                        },
                    }
                }
            };
            x += 1;
        }

        if number_str.len() > 0 {
            let number = Number {
                pos: Position { x: x - number_str.len(), y },
                length: number_str.len(),
                value: number_str.parse().unwrap(),
            };

            for x_diff in 0..number_str.len() {
                parts_map[y][x - 1 - x_diff] = Tile::Number(number);
            }

            number_str.clear();
        }
    }

    parts_map
}

fn part1(part_map : &Vec<Vec<Tile>>) {
    let mut sum = 0;

    for y in 0..part_map.len() {
        let mut x = 0;

        while x < part_map[0].len() {
            if let Tile::Number(num) = part_map[y][x] {
                if has_surrounding_symbol(num, part_map) {
                    sum += num.value;
                }

                x += num.length;
                continue;
            }

            x += 1;
        }
    }

    println!("PART 1: {sum}")
}

fn has_surrounding_symbol(number : Number, part_map : &Vec<Vec<Tile>>) -> bool {
    for x in (number.pos.x as i32)-1..=(number.pos.x+number.length) as i32{
        for y in (number.pos.y as i32)-1..=(number.pos.y+1) as i32 {
            if let Some(tile) = get(x, y, part_map) {
                match tile {
                    Tile::Gear | Tile::Symbol => {
                        return true;
                    }
                    _ => {}
                }
            }
        }
    }

    false
}

fn get(x: i32, y : i32, part_map : &Vec<Vec<Tile>>) -> Option<Tile> {
    if x < 0 || y < 0 || y as usize >= part_map.len() || x as usize >= part_map[0].len() {
        None
    } else {
        Some(part_map[y as usize][x as usize])
    }
}

fn part2(part_map : &Vec<Vec<Tile>>) {
    let mut sum = 0;

    for y in 0..part_map.len() {
        for x in 0..part_map[0].len() {
            if let Tile::Gear = part_map[y][x] {
                let mut numbers : Vec<Number> = vec![];

                for x_diff in -1..=1 {
                    for y_diff in -1..=1 {
                        if let Some(tile) = get(x as i32 + x_diff, y as i32 + y_diff, part_map) {

                            if let Tile::Number(num) = tile {
                                if !numbers.iter().any(|number| *number == num) {
                                    numbers.push(num);
                                }
                            }
                        }

                        
                    }
                }

                if numbers.len() == 2 {
                    sum += numbers[0].value * numbers[1].value;
                }
            }
        }
    }

    println!("PART 2: {sum}")
}