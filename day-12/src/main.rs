fn main() {
    println!("Hello, world!");
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Spring {
    Intact,
    Damaged,
    Unknown,
}

struct Line {
    runs : Vec<usize>,
    springs : Vec<Spring>,   
}

impl Line {
    fn min_pos(&self, index : usize) -> usize {

    }

    fn max_pos(&self, index : usize) -> usize {
        let after = &self.runs[index+1..];
        let length = after.iter().sum::<usize>() + after.len();
        return self.springs.len() - length - self.runs[index];
    }
}