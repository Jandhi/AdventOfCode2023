fn main() {
    let input = include_str!("input.txt");
    let grids = process_grids(input);
    part1(&grids);
    part2(&grids);
}

fn part1(grids : &Vec<Grid>) {
    let sum : usize = grids.iter()
        .map(|grid| grid.find_symmetry(false))
        .map(|sym| match sym.is_horizontal {
            true => sym.value * 100,
            false => sym.value,
        })
        .sum();

    println!("PART 1: {}", sum);
}

fn part2(grids : &Vec<Grid>) {
    let sum : usize = grids.iter()
        .map(|grid| grid.find_symmetry(true))
        .map(|sym| match sym.is_horizontal {
            true => sym.value * 100,
            false => sym.value,
        })
        .sum();

    println!("PART 1: {}", sum);
}

type Grid = Vec<Vec<Tile>>;

fn process_grids(input : &str) -> Vec<Grid> {
    let mut grids : Vec<Grid> = vec![];
    let mut curr_grid : Grid = vec![];

    for line in input.split("\n").map(str::trim) {
        if line == "" {
            grids.push(curr_grid);
            curr_grid = vec![];
        } else {
            curr_grid.push(line
                .chars()
                .map(|c| c.into())
                .collect()
            );
        }
    }

    if curr_grid.len() > 0 {
        grids.push(curr_grid);
    }

    grids
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Tile {
    Ash,
    Rocks
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Ash,
            '#' => Self::Rocks,
            _ => panic!("Invalid char: {}", value),
        }
    }
}

#[derive(Debug)]
struct Symmetry {
    value : usize,
    is_horizontal : bool,
}

trait Mirrorable {
    fn has_vertical_symmetry(&self, cols_left : usize, smudged : bool) -> bool;
    fn has_horizontal_symmetry(&self, rows_above : usize, smudged : bool) -> bool;
    fn find_symmetry(&self, smudged : bool) -> Symmetry;
}

impl Mirrorable for Grid {
    fn has_vertical_symmetry(&self, cols_left : usize, smudged : bool) -> bool {
        let mut used_smudge = false;

        for x in 0..cols_left {
            let x_mirror = cols_left * 2 - 1 - x;

            if x_mirror >= self[0].len() {
                continue;
            }

            for y in 0..self.len() {
                if self[y][x] != self[y][x_mirror] {
                    if !smudged {
                        return false
                    } else if used_smudge {
                        return false
                    } else {
                        used_smudge = true;
                    }
                }
            }
        }

        !smudged || used_smudge
    }

    fn has_horizontal_symmetry(&self, rows_above : usize, smudged : bool) -> bool {
        let mut used_smudge = false;

        for y in 0..rows_above {
            let y_mirror = rows_above * 2 - 1 - y;

            if y_mirror >= self.len() {
                continue;
            }

            for x in 0..self[0].len() {
                if self[y][x] != self[y_mirror][x] {
                    if !smudged {
                        return false
                    } else if used_smudge {
                        return false
                    } else {
                        used_smudge = true;
                    }
                }
            }
        }

        !smudged || used_smudge
    }

    fn find_symmetry(&self, smudged : bool) -> Symmetry {
        for x in 1..self[0].len() {
            if self.has_vertical_symmetry(x, smudged) {
                return Symmetry {
                    value: x,
                    is_horizontal: false,
                }
            }
        }

        for y in 1..self.len() {
            if self.has_horizontal_symmetry(y, smudged) {
                return Symmetry {
                    value: y,
                    is_horizontal: true
                }
            }
        }

        panic!("No symmetry found!");
    }
}