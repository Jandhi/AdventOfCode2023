fn main() {
    // Day 1 2015

    let mut floor = 0;
    let mut position = 0;
    let mut first_basement_position : Option<i32> = None;
    let input = include_str!("./input.txt");
    
    for char in input.chars() {
        position += 1;
        match char {
            '(' => {
                floor += 1;
            }
            _ => {
                floor -= 1;
            }
        }

        if first_basement_position == None && floor == -1 {
            first_basement_position = Some(position);
        }
    }

    println!("Part 1: Floor is {}", floor);
    println!("Part 2: Position is {}", first_basement_position.unwrap());
}
