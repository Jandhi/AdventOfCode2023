use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    let mut bricks : Vec<Brick> = input.split("\n")
        .map(str::trim)
        .map(|line| {
            let points : Vec<Vec<usize>> = line.split("~").map(|values| {
                values.split(",")
                    .map(|val| val.parse::<usize>().unwrap())
                    .collect()
            }).collect();
            Brick {
                min: Pos { 
                    x: points[0][0].min(points[1][0]), 
                    y: points[0][1].min(points[1][1]), 
                    z: points[0][2].min(points[1][2]), 
                },
                max: Pos { 
                    x: points[0][0].max(points[1][0]), 
                    y: points[0][1].max(points[1][1]), 
                    z: points[0][2].max(points[1][2]),
                },
            }
        })
        .collect();

    fall(&mut bricks);
    let (supports, supported_by) = calculate_supports(&bricks);
    
    let fall_counts : Vec<usize> = bricks.iter()
        .enumerate()
        .map(|(index, _)| fall_count(index, &supports, &supported_by))
        .collect();

    let destintagrate_count = fall_counts.iter()
        .filter(|count| **count == 0)
        .count();

    println!("PART 1: {}", destintagrate_count);

    let total_falls : usize = fall_counts.iter()
        .sum();

    println!("PART 2: {}", total_falls);
}

fn fall_count(index : usize, supports : &Vec<Vec<usize>>, supported_by: &Vec<Vec<usize>>) -> usize {
    let mut fallers : HashSet<usize> = HashSet::new();
    fallers.insert(index);
    let mut queue : Vec<usize> = supports[index].iter().map(|i| *i).collect();

    while queue.len() > 0 {
        let next = queue.remove(0);

        if fallers.contains(&next) {
            continue;
        }

        if supported_by[next].iter().all(|supporter| fallers.contains(supporter)) {
            fallers.insert(next);
            for supportee in supports[next].iter() {
                queue.push(*supportee);
            }
        }
    }

    fallers.iter().count() - 1
}

fn calculate_supports(bricks : &Vec<Brick>) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let mut supports : Vec<Vec<usize>> = bricks.iter().map(|_| vec![]).collect();
    let mut supported_by : Vec<Vec<usize>> = bricks.iter().map(|_| vec![]).collect();

    for i in 0..bricks.len() {
        let above = bricks[i].points_above();

        for point in above {
            match brick_id(&point, bricks) {
                Some(id) => {
                    supports[i].push(id);
                    supported_by[id].push(i);
                },
                None => {},
            }
        }
    }

    (supports, supported_by)
}

fn fall(bricks : &mut Vec<Brick>) {
    let mut moved = true;

    while moved {
        moved = false;

        for i in 0..bricks.len() {
            if bricks[i].min.z == 1 {
                continue;;
            }

            let below = bricks[i].points_below();

            if below.iter().all(|pos| !is_brick(pos, bricks)) {
                moved = true;
                bricks[i].min.z -= 1;
                bricks[i].max.z -= 1;
            }
        }
    }
}

fn is_brick(pos : &Pos, bricks : &Vec<Brick>) -> bool {
    bricks.iter()
        .any(|brick| brick.contains(pos))
}

fn brick_id(pos : &Pos, bricks : &Vec<Brick>) -> Option<usize> {
    for (index, brick) in bricks.iter().enumerate() {
        if brick.contains(pos) {
            return Some(index);
        }
    }

    None
}

#[derive(Debug, Clone, Copy)]
struct Pos {
    x : usize,
    y : usize,
    z : usize,
}

#[derive(Debug)]
struct Brick {
    min : Pos,
    max : Pos,
}

impl Brick {
    fn contains(&self, pos : &Pos) -> bool {
        self.min.x <= pos.x && pos.x <= self.max.x
        && self.min.y <= pos.y && pos.y <= self.max.y
        && self.min.z <= pos.z && pos.z <= self.max.z
    }

    fn points_below(&self) -> Vec<Pos> {
        let mut points = vec![];

        for x in self.min.x..=self.max.x {
            for y in self.min.y..=self.max.y {
                points.push(Pos { x, y, z: self.min.z - 1 })
            }
        }

        points
    }

    fn points_above(&self) -> Vec<Pos> {
        let mut points = vec![];

        for x in self.min.x..=self.max.x {
            for y in self.min.y..=self.max.y {
                points.push(Pos { x, y, z: self.max.z + 1})
            }
        }

        points
    }
}