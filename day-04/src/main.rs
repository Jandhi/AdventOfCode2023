fn main() {
    let input = include_str!("input.txt");
    let lines = input.split("\n").map(str::trim).collect::<Vec<_>>();
    let cards = lines.iter()
        .map(|line| {
            ScratchCard::parse(line)
        })
        .collect::<Vec<_>>();
    part1(&cards);
    part2(&cards);
}

fn part1(cards : &Vec<ScratchCard>)
{
    

    let sum : u32 = cards.iter().map(|card| card.value()).sum();

    println!("PART 1: {}", sum);
}

fn part2(cards : &Vec<ScratchCard>)
{
    let mut counted_cards = cards.iter()
        .map(|card| ScratchCardCount{
            count: 1,
            card
        })
        .collect::<Vec<_>>();

    let mut counter = 0;
    while counter < counted_cards.len() {
        let value = counted_cards[counter].card.winning_count();
        let multiplier = counted_cards[counter].count;

        for offset in 1..=value as usize {
            if counter + offset >= counted_cards.len() {
                break;
            }

            counted_cards[counter + offset].count += multiplier;
        }
        
        counter += 1;
    }

    let sum : u32 = counted_cards.iter().map(|card| card.count).sum();

    println!("PART 2: {}", sum);
}

struct ScratchCardCount<'a> {
    count : u32,
    card : &'a ScratchCard
}

struct ScratchCard {
    winning_numbers : Vec<u32>,
    my_numbers : Vec<u32>,
}

impl ScratchCard {
    fn parse(input : &str) -> ScratchCard {
        let parts = input.split(":").collect::<Vec<_>>();
        let subparts = parts[1].split(" | ").collect::<Vec<_>>();
        ScratchCard { 
            winning_numbers: subparts[0].split(" ").map(|string| {
                    string.trim().parse::<u32>()
                }).filter(|result| result.is_ok())
                .map(|result: Result<u32, std::num::ParseIntError>| result.unwrap())
                .collect(), 
            my_numbers: subparts[1].split(" ").map(|string| {
                    string.trim().parse::<u32>()
                }).filter(|result| result.is_ok())
                .map(|result| result.unwrap())
                .collect()
        }
    }

    fn winning_count(&self) -> u32 {
        self.my_numbers.iter()
            .filter(|number| self.winning_numbers.contains(number))
            .count() as u32
    }

    fn value(&self) -> u32 {
        let count = self.winning_count();
        
        if count == 0 {
            0
        } else {
            let exp : u32 = 2;
            exp.pow(count - 1)
        }
    }
}