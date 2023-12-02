fn main() {
    let input = include_str!("./input.txt");
    part1(input);
    part2(input);
}

fn part1(input : &str) {
    let games : Vec<Game> = input.split("\n")
        .map(str::trim)
        .map(|line| Game::parse(line))
        .collect();

    let sum : u32 = games.iter()
        .filter(|game| game.is_possible_p1())
        .map(|game| game.id)
        .sum();

    println!("PART 1: {}", sum);
}

fn part2(input : &str) {
    let games : Vec<Game> = input.split("\n")
        .map(str::trim)
        .map(|line| Game::parse(line))
        .collect();

    let sum : u32 = games.iter()
        .map(|game| game.minimum_set())
        .map(|set| set.power())
        .sum();

        println!("PART 2: {}", sum);
}

#[derive(Debug)]
pub struct Game {
    id : u32,
    draws : Vec<Set>
}

impl Game {
    fn parse(line : &str) -> Game {
        let parts : Vec<&str> = line.split(":").collect();
        let game_id_string = &parts[0][5..];
        let game_id = game_id_string.parse::<u32>().expect(&format!("{} could not be parsed", game_id_string));
        let draws : Vec<Set> = parts[1].split(";").map(Set::parse_draw).collect();
        Game { id: game_id, draws: draws }
    }

    fn is_possible_p1(&self) -> bool {
        self.draws.iter().all(Set::is_possible_p1)
    }

    fn minimum_set(&self) -> Set {
        self.draws.iter().fold(Set::new(), |acc, draw| Set{
            red: acc.red.max(draw.red),
            blue: acc.blue.max(draw.blue),
            green: acc.green.max(draw.green),
        })
    }
}

#[derive(Debug)]
pub struct Set {
    red : u32,
    blue : u32,
    green : u32,
} 

impl Set {
    fn new() -> Set {
        Set { red: 0, blue: 0, green: 0 }
    }

    fn parse_draw(string : &str) -> Set {
        let mut draw = Set::new();
        for bit in string.split(",") {
            let parts : Vec<&str> = bit.trim().split(" ").collect();
            let count : u32 = parts[0].parse().expect(&format!("{} is not a number", parts[0]));
            let color = parts[1];

            match color {
                "red" => { draw.red += count; }
                "blue" => { draw.blue += count; }
                "green" => { draw.green += count; }
                _ => {}
            }
        }

        draw
    }

    fn is_possible_p1(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}