fn main() {
    let input = include_str!("input.txt");
    let mut galaxies : Vec<Position> = vec![];
    let map : Vec<Vec<bool>> = input.split("\n")
        .map(str::trim)
        .map(|line| {
            line.chars().map(|c| {
                if c == '#' {
                    true
                } else {
                    false
                }
            })
            .collect()
        })
        .collect();

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] {
                galaxies.push(Position { x, y });
            }
        }
    }

    // Mark empty columns
    let mut empty_cols : Vec<usize> = vec![];

    for x in 0..map[0].len() {
        let mut empty = true;
        
        for y in 0..map.len() {
            if map[y][x] {
                empty = false;
                break;
            }
        }

        if empty {
            empty_cols.push(x);
        }
    }

    // Mark empty rows
    let mut empty_rows : Vec<usize> = vec![];

    for y in 0..map.len() {
        let mut empty = true;
        
        for x in 0..map[0].len() {
            if map[y][x] {
                empty = false;
                break;
            }
        }

        if empty {
            empty_rows.push(y);
        }
    }

    part1(&galaxies, &empty_cols, &empty_rows);
    part2(&galaxies, &empty_cols, &empty_rows);
}

fn part1(galaxies : &Vec<Position>, empty_cols : &Vec<usize>, empty_rows : &Vec<usize>) {
    let mut sum = 0;

    for i in 0..galaxies.len() {
        for j in i..galaxies.len() {
            let galaxy_a = galaxies[i];
            let galaxy_b = galaxies[j];

            let x_diff = diff(galaxy_a.x, galaxy_b.x); 
            let y_diff = diff(galaxy_a.y, galaxy_b.y);
            let mut x_added = 0;
            let mut y_added = 0;

            for col in empty_cols {
                if between(*col, galaxy_a.x, galaxy_b.x) {
                    x_added += 1;
                }
            }

            for row in empty_rows {
                if between(*row, galaxy_a.y, galaxy_b.y) {
                    y_added += 1;
                }
            }

            let distance = x_diff + y_diff + x_added + y_added;
            sum += distance;
        }
    }

    println!("PART 1: {}", sum);
}

fn part2(galaxies : &Vec<Position>, empty_cols : &Vec<usize>, empty_rows : &Vec<usize>) {
    let mut sum = 0;

    for i in 0..galaxies.len() {
        for j in i..galaxies.len() {
            let galaxy_a = galaxies[i];
            let galaxy_b = galaxies[j];

            let x_diff = diff(galaxy_a.x, galaxy_b.x); 
            let y_diff = diff(galaxy_a.y, galaxy_b.y);
            let mut x_added = 0;
            let mut y_added = 0;

            for col in empty_cols {
                if between(*col, galaxy_a.x, galaxy_b.x) {
                    x_added += 1;
                }
            }

            for row in empty_rows {
                if between(*row, galaxy_a.y, galaxy_b.y) {
                    y_added += 1;
                }
            }

            let distance = x_diff + y_diff + (x_added * (1_000_000 - 1)) + (y_added * (1_000_000 - 1));
            sum += distance;
        }
    }

    println!("PART 2: {}", sum);
}

fn between(num : usize, a : usize, b : usize) -> bool {
    if a > b {
        b < num && num < a
    } else {
        a < num && num < b
    }
}

fn diff(a : usize, b : usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}

#[derive(Debug, Clone, Copy)]
struct Position {
    x : usize,
    y : usize,
}
