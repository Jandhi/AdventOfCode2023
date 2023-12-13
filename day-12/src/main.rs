fn main() {
    let input = include_str!("input.txt");
    let lines : Vec<Line> = input.split("\n")
        .map(str::trim)
        .map(Line::parse)
        .collect();

    part1(&lines);
}

fn part1(lines : &Vec<Line>) {
    let sum : usize = lines.iter()
        .map(Line::variations)
        .sum();

    println!("PART 1: {}", sum);
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Spring {
    Intact,
    Damaged,
    Unknown,
}

impl From<char> for Spring {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Damaged,
            '.' => Self::Intact,
            _ => Self::Unknown
        }
    }
}

struct Line {
    runs : Vec<usize>,
    springs : Vec<Spring>,   
}

impl Line {
    fn parse(input : &str) -> Line {
        let parts: Vec<&str> = input.split(" ").collect();
        let springs : Vec<Spring> = parts[0].chars()
            .map(|c| c.into())
            .collect();
        let runs : Vec<usize> = parts[1].split(",")
            .map(|num| num.parse().unwrap())
            .collect();

        Line { runs, springs }
    }

    fn min_pos(&self, index : usize) -> usize {
        let before = &self.runs[..index];
        let length = before.iter().sum::<usize>() + before.len();
        return length
    }

    fn max_pos(&self, index : usize) -> usize {
        let after = &self.runs[index+1..];
        let length = after.iter().sum::<usize>() + after.len();
        return self.springs.len() - length - self.runs[index];
    }

    fn variations(&self) -> usize {
        self.find_positions(&[])
    }

    fn find_positions(&self, placed : &[usize]) -> usize {
        if placed.len() == self.runs.len() {
            return match self.is_correct(placed) {
                true => {
                    println!("Position correct {:?}", placed);
                    return 1
                },
                false => 0,
            }
        } else {
            let mut sum = 0;
            let index = placed.len();
            let new_min = match placed.len() > 0 {
                true => placed.last().unwrap() + self.runs[index - 1] + 1,
                false => 0,
            };

            for i in self.min_pos(index).max(new_min)..=self.max_pos(index) {
                let mut arr = Vec::from(placed);
                arr.push(i);
                sum += self.find_positions(&arr);
            }

            sum
        }
    }

    fn is_correct(&self, positions : &[usize]) -> bool {
        for i in 0..self.springs.len() {
            if self.should_be_damaged(i, positions) {
                if self.springs[i] == Spring::Intact {
                    return false;
                }
            } else {
                if self.springs[i] == Spring::Damaged {
                    return false;
                }
            }
        }

        true
    }

    fn should_be_damaged(&self, index : usize, positions : &[usize]) -> bool {
        positions.iter()
            .enumerate()
            .any(|(run_index, position)| {
                index >= *position && index < position + self.runs[run_index]
            })
    }
}