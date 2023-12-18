use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    let map : Vec<Vec<usize>> = input.split("\n")
        .map(str::trim)
        .map(|line| line.chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect()
        ).collect();

    let start_states : Vec<State> = vec![
        State { x: 0, y: 0, direction: Direction::Right, moves: 0 },
        State { x: 0, y: 0, direction: Direction::Down, moves: 0 },
    ];

    part1(&map, &start_states);
    part2(&map, &start_states);
}

fn part1(map : &Vec<Vec<usize>>, start_states : &Vec<State>) {
    let path = best_path(&map, &start_states, false);
    let heat_loss : usize = path.iter()
        .map(|state| state.heat_loss(&map))
        .sum::<usize>() - path[0].heat_loss(&map);

    println!("PART 1: {}", heat_loss);
}

fn part2(map : &Vec<Vec<usize>>, start_states : &Vec<State>) {
    let path = best_path(&map, &start_states, true);
    let heat_loss : usize = path.iter()
        .map(|state| state.heat_loss(&map))
        .sum::<usize>() - path[0].heat_loss(&map);

    println!("PART 2: {}", heat_loss);
}

fn best_path(map : &Vec<Vec<usize>>, start_states : &Vec<State>, ultra_crucible : bool) -> Vec<State> {
    let mut heap : Vec<(Vec<State>, usize)> = start_states.iter()
        .map(|s| (vec![*s], s.heat_loss(map)))
        .collect();

    let mut visited : HashSet<State> = HashSet::new();

    while heap.len() > 0 {
        let (path, cost) = heap.remove(0);
        let last_state = path.last().unwrap();

        if visited.contains(last_state)
        {
            continue;
        }

        visited.insert(*last_state);

        for state  in last_state.next(&map, ultra_crucible) {
            let mut new_path = path.clone();
            new_path.push(state);

            if state.x == map[0].len() - 1 && state.y == map.len() - 1 {
                return new_path
            } else {
                heap_insert(&mut heap, (new_path.clone(), State::cost(&map, &new_path, cost)))
            }
        }
    }

    vec![]
}

fn heap_insert(heap : &mut Vec<(Vec<State>, usize)>, element : (Vec<State>, usize)) {
    let mut index = 0;

    while index < heap.len() && heap[index].1 < element.1 {
        index += 1;
    }

    heap.insert(index, element);
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
struct State {
    x : usize,
    y : usize,
    direction : Direction,
    moves : usize,
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    Up,
    Down,
    Right,
    Left
}

impl Direction {
    fn neighbouring(&self) -> Vec<Direction> {
        match self {
            Direction::Up | Direction::Down => vec![Direction::Left, Direction::Right],
            _ => vec![Direction::Up, Direction::Down]
        }
    }
}

impl State { 
    fn next(&self, map : &Vec<Vec<usize>>, ultra_crucible : bool) -> Vec<State> {
        let mut next_directions : Vec<Direction> = vec![];

        if !ultra_crucible {
            next_directions.append(&mut self.direction.neighbouring());

            if self.moves < 2 {
                next_directions.push(self.direction);
            }
        } else {
            if self.moves >= 3 {
                next_directions.append(&mut self.direction.neighbouring());
            }

            if self.moves < 9 {
                next_directions.push(self.direction);
            }
        }        

        let mut next_states = vec![];

        for dir in next_directions {
            let (x, y) = match dir {
                Direction::Up => {
                    if self.y == 0 {
                        continue;
                    }

                    (self.x, self.y - 1)
                },
                Direction::Down => {
                    if self.y == map.len() - 1 {
                        continue;
                    }

                    (self.x, self.y + 1)
                },
                Direction::Right => {
                    if self.x == map[0].len() - 1 {
                        continue;
                    }

                    (self.x + 1, self.y)
                },
                Direction::Left => {
                    if self.x == 0 {
                        continue;
                    }

                    (self.x - 1, self.y)
                },
            };

            next_states.push(Self { x, y, direction: dir, moves: match self.direction == dir {
                true => self.moves + 1,
                false => 0,
            }});
        }

        next_states
    }

    fn distance(&self, map : &Vec<Vec<usize>>) -> usize {
        map[0].len() - 1 - self.x + map.len() - 1 - self.y
    }

    fn heat_loss(&self, map : &Vec<Vec<usize>>) -> usize {
        map[self.y][self.x]
    }

    fn cost(map : &Vec<Vec<usize>>, path : &Vec<State>, prev_cost : usize) -> usize {
        let last_state = path[path.len() - 1];
        let second_last_state = path[path.len() - 2];
        prev_cost + last_state.heat_loss(map) + last_state.distance(map) - second_last_state.distance(map)
    }
}