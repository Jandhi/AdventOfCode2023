fn main() {
    let input = include_str!("./input.txt");
    part1(input);
    part2(input);
}

fn get_num(line : &str) -> i32 {
    let digits : Vec<char> = line.chars().filter(|char| {
        match char {
            '0'..='9' => true,
            _ => false
        }
    }).collect();

    let mut number = "".to_string();

    if digits.len() == 0 {
        return 0
    }

    number.push(digits[0]);
    number.push(*digits.last().expect("No Digits"));
    let result = number.parse().unwrap();

    println!("{} -> {}", line, result);

    result
}

fn replace_digits(line : &str) -> String {
    let digits : Vec<(&str, &str)> = vec![
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];
    let mut buffer = line.to_string();
    let mut result = "".to_string();

    while buffer.len() > 0 {
        for (word, digit) in digits.iter() {
            if buffer.starts_with(word) {
                buffer.remove(0);
                result += digit;
            }
        }

        if buffer.len() == 0 {
            break;
        }

        result.push(buffer.remove(0));
    }

    println!("Replaced {} with {}", line, result);

    result
}

fn part1(input : &str) {
    let lines = input.split('\n');
    let sum : i32 = lines.map(|line| get_num(line)).sum();
    println!("PART 1: {}", sum)
}

fn part2(input : &str) {
    let lines = input.split('\n').map(|line| line.trim());
    let sum : i32 = lines.map(|line| get_num(&replace_digits(line))).sum();
    println!("PART 2: {}", sum)
}