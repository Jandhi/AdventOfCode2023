use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let lines = input.split("\n").map(str::trim).collect::<Vec<_>>();
    let instructions = lines[0].chars().collect::<Vec<_>>();
    let graph = Graph::parse(&lines[2..]);

    part1(&graph, &instructions);
    part2(&graph, &instructions);
}

fn part1(graph : &Graph, instructions : &Vec<char>) {
    let mut curr_node = "AAA".to_string();
    let mut counter = 0;

    while curr_node != "ZZZ" {
        let next_node = graph.follow_instruction(&curr_node, instructions[counter % instructions.len()]).to_string();
        curr_node = next_node;
        counter += 1;
    }
    
    println!("PART 1: {}", counter);
}

fn part2(graph : &Graph, instructions : &Vec<char>) {
    let start_nodes = graph.nodes.keys()
        .filter(|key| key.ends_with("A"))
        .map(|str| str.clone())
        .collect::<Vec<_>>();

    let mut cycles : Vec<Cycle> = vec![];

    for (index, start_node) in start_nodes.iter().enumerate() {
        let mut counter = 0;
        let mut curr_node = start_node.clone();
        let mut found_end = false;
        let mut first_end = 0;
        let second_end;

        loop {
            curr_node = graph.follow_instruction(&curr_node, instructions[counter % instructions.len()]).to_string();
            counter += 1;

            if curr_node.ends_with("Z") {
                if found_end {
                    second_end = counter;
                    break;
                } else {
                    first_end = counter;
                    found_end = true;
                }
            }
        }

        cycles.push(Cycle { offset: first_end as u64, size: second_end as u64 - first_end  as u64});
    }


    let mut offset = find_join(&cycles[0],&cycles[1]);
    let mut size = num::integer::lcm(cycles[0].size, cycles[1].size);

    for i in 2..cycles.len() {
        offset = find_join(&Cycle { offset, size }, &cycles[i]);
        size = num::integer::lcm(size, cycles[i].size);
    }

    println!("PART 2: {}", offset);
}

fn find_join(c1 : &Cycle, c2 : &Cycle) -> u64 {
    if c2.offset < c1.offset {
        return find_join(c2, c1);
    }

    let mut l2 : u64 = 0;
    let offset : u64 = c2.offset - c1.offset as u64;

    while (offset + l2 * c2.size) % c1.size != 0 {
        l2 += 1;
    }

    let l1 = (offset + l2 * c2.size) / c1.size;

    assert!(c1.offset + l1 * c1.size == c2.offset + l2 * c2.size);

    c1.offset + l1 * c1.size
}

struct Cycle {
    offset : u64,
    size : u64,
}

#[derive(Debug)]
struct Graph {
    nodes : HashMap<String, Vec<String>>
}

impl Graph {
    fn parse(lines : &[&str]) -> Graph {
        Graph { 
            nodes: lines.iter().map(|line| {
                let name = line[0..3].to_string();
                let left = line[7..10].to_string();
                let right = line[12..15].to_string();

                (name, vec![left, right])
            }).collect::<HashMap<_, _>>()
        }
    }

    fn follow_instruction(&self, curr_node : &str, instruction: char) -> String {
        self.nodes.get(&curr_node.to_string()).unwrap()[match instruction {
            'L' => 0,
            'R' => 1,
            _ => panic!("This shouldn't happen"),
        }].to_string()
    }
}