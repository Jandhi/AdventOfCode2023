type Num = u64;

fn main() {
    let input = include_str!("input.txt");
    let lines = input.split("\n").map(|line| line.trim()).collect::<Vec<_>>();
    let seeds = parse_seeds(lines[0]);

    let maps = lines
        .split(|line| *line == "")
        .map(|lines| Map::parse(lines))
        .collect::<Vec<_>>();

    part1(&seeds, &maps);
    part2(&seeds, &maps);
}

fn part1(seeds : &Vec<Num>, maps : &Vec<Map>) {
    let lowest_location = seeds.iter()
        .map(|seed| maps.iter().fold(*seed, |acc, map| map.process(acc)))
        .min()
        .unwrap();

    println!("PART 1: {}", lowest_location);
}

fn part2(seeds : &Vec<Num>, maps : &Vec<Map>) {
    let ranges = seeds_p2(&seeds);
    let lowest_location = maps.iter()
        .fold(ranges, |acc, map| {
            map.process_ranges(acc)
        })
        .iter()
        .map(|range| range.begin)
        .min()
        .unwrap();

    println!("PART 2: {}", lowest_location);
}

fn parse_seeds(input : &str) -> Vec<Num> {
    input[6..]
        .split(" ")
        .map(|num| num.parse::<Num>())
        .filter(|result| result.is_ok())
        .map(|result| result.unwrap())
        .collect::<Vec<_>>()
}

fn seeds_p2(input : &Vec<Num>) -> Vec<Range> {
    let mut ranges = vec![];

    for i in 0..input.len() / 2 {
        ranges.push(Range { begin: input[i * 2], size: input[i * 2 + 1] });
    }

    ranges
}

#[derive(Clone, Copy, Debug)]
struct Range {
    begin : Num,
    size : Num
}

impl Range {
    fn last(&self) -> Num {
        self.begin + self.size - 1
    }
}

struct Map {
    name : String,
    mappings : Vec<Mapping>,
}

impl Map {
    fn parse(input : &[&str]) -> Map {
        let name = input[0].split(":").collect::<Vec<_>>()[0];
        let mappings = input[1..].iter()
            .map(|line| Mapping::parse(line))
            .collect::<Vec<_>>();

        Map { name: name.to_string(), mappings: mappings }
    }

    fn process(&self, input : Num) -> Num {
        for mapping in self.mappings.iter() {
            if let Some(output) = mapping.process(input) {
                return output;
            }
        }

        input
    }

    fn process_ranges(&self, input : Vec<Range>) -> Vec<Range> {
        let mut result = vec![];
        let mut ranges_to_process = input.clone();

        for mapping in self.mappings.iter() {
            let range_queue = ranges_to_process.clone();
            let mut removal_indices = vec![];

            for (index, range) in range_queue.iter().enumerate() {
                // Range is entirely contained
                if mapping.src_begin() <= range.begin && range.last() <= mapping.src_last() {
                    removal_indices.push(index);
                    result.push(Range { begin: mapping.process(range.begin).expect("Begin should be in range of mapping"), size: range.size });
                    continue;
                }
                // Range is not contained
                if range.last() < mapping.src_begin() || mapping.src_last() < range.begin {
                    continue;
                }

                // Range is split somehow
                removal_indices.push(index);

                // Add any range below
                if range.begin < mapping.src_begin() {
                    let below = Range {
                        begin: range.begin,
                        size: mapping.src_begin() - range.begin,
                    };
                    ranges_to_process.push(below);
                }
                
                // Add any range above
                if mapping.src_last() < range.last() {
                    let above = Range {
                        begin: mapping.src_last() + 1,
                        size: range.last() - mapping.src_last()
                    };
                    ranges_to_process.push(above);
                }

                // Mapped portion
                let begin = range.begin.max(mapping.src_begin());
                let size = range.last().min(mapping.src_last()) + 1 - begin;
                result.push(Range { begin: mapping.process(begin).expect("Mapping should contain begin"), size });
            }      

            let mut removed = 0;
            for index in removal_indices {
                ranges_to_process.remove(index - removed);
                removed += 1;
            }
        }   

        for leftover in ranges_to_process {
            result.push(leftover);
        }

        result
    }
}

#[derive(Debug)]
struct Mapping {
    destination_range_start : Num,
    source_range_start : Num,
    range_length : Num,
}

impl Mapping {
    fn parse(input : &str) -> Mapping {
        let parts = input
            .split(" ")
            .map(|num| num.parse::<Num>().unwrap())
            .collect::<Vec<_>>();

        Mapping { destination_range_start: parts[0], source_range_start: parts[1], range_length: parts[2] }
    }
    
    fn process(&self, input : Num) -> Option<Num> {
        if self.source_range_start <= input && input < self.source_range_start + self.range_length {
            return Some(self.destination_range_start + (input - self.source_range_start));
        }

        None
    }

    fn src_begin(&self) -> Num {
        self.source_range_start
    }

    fn src_last(&self) -> Num {
        self.source_range_start + self.range_length - 1
    }
}