fn main() {
    let input = include_str!("input.txt");
    part1(input);
    part2(input);
}

fn part1(input : &str) {
    let sum : u32 = input.split(",")
        .map(str::trim)
        .map(hash)
        .sum();

    println!("PART 1: {}", sum);
}

fn part2(input : &str) {
    let mut boxes : Vec<Vec<Lens>> = vec![];

    for _ in 0..256 {
        boxes.push(vec![]);
    }

    for step in input.split(",").map(str::trim) {
        if step.ends_with("-") {
            let label = &step[..step.len() - 1];
            let box_index = hash(label) as usize;

            boxes[box_index].retain(|lens| lens.label != label);

        } else {
            let parts : Vec<&str> = step.split("=").collect();
            let label = parts[0];
            let focal_length = parts[1].parse::<usize>().unwrap();
            let box_index = hash(label) as usize;
            let same_label_position = boxes[box_index].iter().position(|lens| lens.label == label);

            if let Some(pos) = same_label_position {
                boxes[box_index][pos].focal_length = focal_length;
            } else  {
                boxes[box_index].push(Lens { label, focal_length })
            }
        }
    }

    let focusing_power : usize = boxes.iter()
        .enumerate()
        .map(|(index, bx)| {
            bx.iter()
                .enumerate()
                .map(|(slot, lens)| {
                    (index + 1) * (slot + 1) * lens.focal_length
                })
                .sum::<usize>()
        }).sum();

    println!("PART 2: {}", focusing_power);
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct Lens<'a> {
    label : &'a str,
    focal_length : usize,
}

fn hash(input : &str) -> u32 {
    let mut cur_val = 0;

    for c in input.chars() {
        let ascii = c as u32;
        cur_val += ascii;
        cur_val *= 17;
        cur_val = cur_val % 256;
    }

    cur_val
}