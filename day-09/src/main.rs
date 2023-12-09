fn main() {
    let input = include_str!("input.txt");
    part1(input);
    part2(input);
}   

fn part1(input : &str) {
    let sequences = input.split("\n")
        .map(str::trim)
        .map(|line| {
            line.split(" ")
                .map(|num| num.parse::<i32>().expect(&format!("{} should be parseable", num)))
                .collect::<Vec<_>>()
        }).collect::<Vec<_>>();
    
    let sum : i32 = sequences.iter()
        .map(|sequence| extrapolate(sequence, true))
        .sum();

    println!("PART 1: {}", sum);
}

fn part2(input : &str) {
    let sequences = input.split("\n")
        .map(str::trim)
        .map(|line| {
            line.split(" ")
                .map(|num| num.parse::<i32>().expect(&format!("{} should be parseable", num)))
                .collect::<Vec<_>>()
        }).collect::<Vec<_>>();
    
    let sum : i32 = sequences.iter()
        .map(|sequence| extrapolate(sequence, false))
        .sum();

    println!("PART 2: {}", sum);
}

fn extrapolate(sequence : &Vec<i32>, is_forward : bool) -> i32 {
    let mut sequences = vec![sequence.clone()];

    // Construct sequences
    loop {
        let mut new_seq = vec![];

        {
            let last = sequences.last().unwrap();

            for i in 0..last.len() - 1 {
                new_seq.push(last[i + 1] - last[i]);
            }
        }

        let all_zeroes = new_seq.iter().all(|num| *num == 0);
        sequences.push(new_seq);

        if all_zeroes {
            break;
        }
    }

    if is_forward {
        extrapolate_forward(&mut sequences)
    } else {
        extrapolate_backward(&mut sequences)
    }
}

fn extrapolate_forward(sequences : &mut Vec<Vec<i32>>) -> i32 {
    let max_index = sequences.len() - 1;
    let mut last_pushed = 0;

    for i in (0..=max_index).rev() {
        let sq = &mut sequences[i];
        let value;

        if i == max_index {
            value = 0;
        } else {
            value = last_pushed + sq.last().unwrap();
        }

        last_pushed = value;
        sq.push(value);
    }

    *sequences[0].last().unwrap()
}

fn extrapolate_backward(sequences : &mut Vec<Vec<i32>>) -> i32 {
    let max_index = sequences.len() - 1;
    let mut last_inserted = 0;

    for i in (0..=max_index).rev() {
        let sq = &mut sequences[i];
        let value;

        if i == max_index {
            value = 0;
        } else {
            value = sq[0] - last_inserted;
        }

        last_inserted = value;
        sq.insert(0, value);
    }

    sequences[0][0]
}