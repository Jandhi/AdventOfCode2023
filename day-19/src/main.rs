use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");

    let lines : Vec<&str> = input.split("\n")
        .map(str::trim)
        .collect();

    let mut second_section = false;
    let mut workflows : HashMap<String, Workflow> = HashMap::new();
    let mut parts : Vec<Part> = vec![];

    for line in lines {
        if line == "" {
            second_section = true;
            continue;
        }

        if !second_section {
            let workflow = Workflow::from(line);
            workflows.insert(workflow.name.clone(), workflow);
        } else {
            parts.push(Part::from(line));
        }
    }

    part1(&parts, &workflows);
    part2(&workflows);

}

fn part1(parts : &Vec<Part>, workflows : &HashMap<String ,Workflow>) {
    let rating_sum : usize = parts.iter()
        .filter(|part| passes_inspection(part, workflows))
        .map(|part| part.rating())
        .sum();

    println!("PART 1: {}", rating_sum);
}

fn part2(workflows : &HashMap<String ,Workflow>) {
    let mut ranges : Vec<(Range, &str)> = vec![(Range::new(), "in")];

    let mut sum : usize = 0;

    while ranges.len() > 0 {
        let (range, destination) = ranges.pop().unwrap();

        if destination == "A" {
            sum += range.size();
            continue;
        }
        
        if destination == "R" {
            continue;
        }

        let workflow = &workflows[destination];
        let mut next_ranges = workflow.route(range);
        ranges.append(&mut next_ranges);
    }

    println!("PART 2: {}", sum);
}

fn passes_inspection(part : &Part, workflows : &HashMap<String ,Workflow>) -> bool {
    let mut key = "in";

    loop {
        key = workflows[key].next(part);
    
        if key == "A" {
            return true;
        } 

        if key == "R" {
            return false;
        }
    }
}


#[derive(Debug)]
struct Part {
    x : usize,
    m : usize,
    a : usize,
    s : usize
}

impl Part {
    fn rating(&self) -> usize {
        return self.x + self.m + self.a + self.s
    }

    fn get(&self, field : &Field) -> usize {
        match field {
            Field::X => self.x,
            Field::M => self.m,
            Field::A => self.a,
            Field::S => self.s,
        }
    }
}

impl From<&str> for Part {
    fn from(value: &str) -> Self {
        let parts : Vec<&str> = value[1..value.len()-1].split(",").collect();

        Self {
            x: parts[0][2..].parse::<usize>().unwrap(), 
            m: parts[1][2..].parse::<usize>().unwrap(), 
            a: parts[2][2..].parse::<usize>().unwrap(), 
            s: parts[3][2..].parse::<usize>().unwrap() 
        }
    }
}

#[derive(Debug)]
struct Workflow {
    name : String,
    rules : Vec<(Rule, String)>,
    default : String,
}

impl Workflow {
    fn next(&self, part : &Part) -> &str {
        for (rule, result) in self.rules.iter() {
            if rule.eval(part) {
                return result;
            }
        }

        &self.default
    }
}

impl Workflow {
    fn route(&self, mut range : Range) -> Vec<(Range, &str)> {
        let mut ranges : Vec<(Range, &str)> = vec![];

        for (rule, destination) in self.rules.iter() {
            let (passes, fails) = rule.split(range);

            range = fails;

            if passes.exists() {
                ranges.push((passes, destination))
            }
        }

        if range.exists() {
            ranges.push((range, &self.default))
        }

        ranges
    }
}

impl From<&str> for Workflow {
    fn from(value: &str) -> Self {
        let parts : Vec<&str> = value.split("{").collect();
        let name = parts[0].to_string();
        let rule_parts : Vec<&str> = parts[1].split(",").collect();

        let rules : Vec<(Rule, String)> = rule_parts[0..rule_parts.len()-1]
            .iter()
            .map(|input| {
                let parts : Vec<&str> = input.split(":").collect();
                let rule = parts[0].into();
                let name = parts[1].to_string();
                (rule, name)
            })
            .collect();

        let default = rule_parts.last().unwrap().replace("}", "").to_string();

        Workflow { name, rules: rules, default: default }
    }
}

#[derive(Debug, Clone, Copy)]
struct Range {
    x : Span,
    m : Span,
    a : Span,
    s : Span,
}

impl Range {
    fn new() -> Self {
        Self { x: Span::new(), m: Span::new(), a: Span::new(), s: Span::new() }
    }

    fn size(&self) -> usize {
        self.x.size() * self.m.size() * self.a.size() * self.s.size()
    }

    fn set_min(&mut self, field : Field, value : usize) {
        match field {
            Field::X => &mut self.x,
            Field::M => &mut self.m,
            Field::A => &mut self.a,
            Field::S => &mut self.s,
        }.min = value;
    }

    fn set_max(&mut self, field : Field, value : usize) {
        match field {
            Field::X => &mut self.x,
            Field::M => &mut self.m,
            Field::A => &mut self.a,
            Field::S => &mut self.s,
        }.max = value;
    }

    fn split_at(&self, min_point : usize, field : Field) -> (Range, Range) {
        let mut lower = self.clone();
        let mut upper = self.clone();

        lower.set_max(field, min_point - 1);
        upper.set_min(field, min_point);

        return (lower, upper)
    }

    fn exists(&self) -> bool {
        self.x.exists() && self.m.exists() && self.a.exists() && self.s.exists()
    }
}

#[derive(Debug, Clone, Copy)]
struct Span {
    min : usize,
    max : usize // Inclusive
}

impl Span {
    fn new() -> Span {
        Self { min: 1, max: 4000 }
    }

    fn exists(&self) -> bool {
        1 <= self.min && self.min <= self.max && self.max <= 4000
    }

    fn size(&self) -> usize {
        self.max + 1 - self.min
    }
}

#[derive(Debug)]
struct Rule {
    field : Field,
    condition : Condition,
    value : usize,
}

impl Rule {
    fn eval(&self, part : &Part) -> bool {
        match self.condition {
            Condition::LessThan => part.get(&self.field) < self.value,
            Condition::GreaterThan => part.get(&self.field) > self.value,
        }
    }

    fn split(&self, range : Range) -> (Range, Range) {
        match self.condition {
            Condition::LessThan => {
                let (lower, upper) = range.split_at(self.value, self.field);
                (lower, upper)
            },
            Condition::GreaterThan => {
                let (lower, upper) = range.split_at(self.value + 1, self.field);
                (upper, lower)
            },
        }
    }
}

impl From<&str> for Rule {
    fn from(value: &str) -> Self {
        Rule { field: value[0..1].into(), condition: value[1..2].into(), value: value[2..].parse::<usize>().unwrap() }
    }
}

#[derive(Debug)]
enum Condition {
    LessThan,
    GreaterThan
}

impl From<&str> for Condition {
    fn from(value: &str) -> Self {
        match value {
            ">" => Self::GreaterThan,
            "<" => Self::LessThan,
            _ => panic!("Unknown value: {}", value)
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Field {
    X,
    M,
    A,
    S,
}

impl From<&str> for Field {
    fn from(value: &str) -> Self {
        match value {
            "x" => Self::X,
            "m" => Self::M,
            "a" => Self::A,
            "s" => Self::S,
            _ => panic!("Unknown value: {}", value)
        }
    }
}
