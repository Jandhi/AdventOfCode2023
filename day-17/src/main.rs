fn main() {
    let input = include_str!("test.txt");
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

    let mut heap : Vec<(Vec<State>, usize)> = start_states.iter()
        .map(|s| (vec![*s], s.heat_loss(&map)))
        .collect();

    while heap.len() > 0 {
        let (states, cost) = heap.remove(0);
        let last_state = states.last().unwrap();

        for state  in last_state.next(&map) {
            
        }
    }
}

fn heap_insert(heap : &mut Vec<(State, usize)>, element : (State, usize)) {
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
    fn next(&self, map : &Vec<Vec<usize>>) -> Vec<State> {
        vec![]
    }

    fn distance(&self, map : &Vec<Vec<usize>>) -> usize {
        map[0].len() - 1 - self.x + map.len() - 1 - self.y
    }

    fn heat_loss(&self, map : &Vec<Vec<usize>>) -> usize {
        map[self.y][self.x]
    }

    fn cost(map : &Vec<Vec<usize>>, path : Vec<State>, prev_cost : usize) -> usize {
        let last_state = path[path.len() - 1];
        let second_last_state = path[path.len() - 2];
        prev_cost + last_state.heat_loss(map) + last_state.distance(map) - second_last_state.distance(map)
    }
}