use std::iter::zip;

fn main() {
    let input = include_str!("input.txt");
    let lines = input.split("\n").map(str::trim).collect::<Vec<_>>();

    part1(&lines);
    part2(&lines);
}

fn part1(lines : &Vec<&str>) {
    let times = lines[0]
        .split(" ")
        .map(|part| part.parse::<u64>())
        .filter(|result| result.is_ok())
        .map(|result| result.unwrap())
        .collect::<Vec<_>>();

    let records = lines[1]
        .split(" ")
        .map(|part| part.parse::<u64>())
        .filter(|result| result.is_ok())
        .map(|result| result.unwrap())
        .collect::<Vec<_>>();

    let times_and_records = zip(times, records).collect::<Vec<_>>();

    let prod = times_and_records.iter()
        .map(|(time, record)| possible_times(*time, *record))
        .product::<u64>();

    println!("PART 1: {}", prod);
}

fn part2(lines : &Vec<&str>) {
    let time = lines[0][6..].chars()
        .filter(|c| *c != ' ')
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    let record = lines[1][10..].chars()
        .filter(|c| *c != ' ')
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    println!("PART 2: {}", possible_times(time, record));
}

fn possible_times(total_time : u64, record : u64) -> u64 {
    // h = (t +/- sqrt(t ^ 2 - 4r)) / 2
    let t = total_time as f64;
    let r = record as f64;

    let lower = (t - (t.powf(2.0) - 4.0 * r).powf(0.5)) / 2.0;
    let upper = (t + (t.powf(2.0) - 4.0 * r).powf(0.5)) / 2.0;
    
    let times = round_down(upper) - lower as u64;

    times
}

fn round_down(input : f64) -> u64 {
    let rounded = input as u64;

    if rounded as f64 == input {
        return rounded - 1
    }

    rounded
}