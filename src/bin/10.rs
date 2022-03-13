use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input/10").unwrap();
    solve(&input);
}

fn solve(input: &str) {
    // read a line
    // put that in a data structure -> make a bracket stack
    // push -> can be valid or not
    // pop can be valid or not
    // in the end we check size
    // if size 0 then clean
    // otherwise unclean
    // define pairings
    let mut error_points = 0;
    let mut closing_costs: Vec<i64> = Vec::new();

    for line in input.lines() {
        let mut br_stack = BracketStack::new();
        let mut br_stack_error_score = 0;
        for char in line.chars() {
            match br_stack.add_bracket(char) {
                Ok(_) => {}
                Err(error_score) => {
                    br_stack_error_score = error_score;
                    break;
                }
            }
        }
        error_points += br_stack_error_score;
        if br_stack_error_score == 0 && br_stack.size() != 0 {
            closing_costs.push(br_stack.close_score());
        }
    }

    closing_costs.sort();

    println!("Part 1 sol: {}", error_points);
    println!("Part 2 sol: {}", closing_costs[closing_costs.len() / 2]);
}

struct BracketStack {
    inner: Vec<char>,
    // `pairings` should be static but not sure how to
    // do that in rust without a external crate
    close_open_pairing: HashMap<char, char>,
    open_close_pairing: HashMap<char, char>,
}

impl BracketStack {
    fn new() -> BracketStack {
        let close_open_pairing = HashMap::from([(']', '['), (')', '('), ('}', '{'), ('>', '<')]);
        let open_close_pairing = HashMap::from([('[', ']'), ('(', ')'), ('{', '}'), ('<', '>')]);
        BracketStack {
            inner: Vec::new(),
            close_open_pairing,
            open_close_pairing,
        }
    }

    fn add_bracket(&mut self, chr: char) -> Result<char, i32> {
        if self.close_open_pairing.contains_key(&chr) {
            // bracket to be added is closing type
            if self.inner.last().unwrap() == self.close_open_pairing.get(&chr).unwrap() {
                // closing bracket is valid to add
                self.inner.pop();
            } else {
                return Err(BracketStack::chr_error_score(chr));
            }
        } else {
            self.inner.push(chr);
        }
        return Ok(chr);
    }

    fn close_score(&mut self) -> i64 {
        let mut score: i64 = 0;
        while self.size() != 0 {
            score = score * 5
                + match self.inner.pop() {
                    None => 0,
                    Some(chr) => match chr {
                        '(' => 1,
                        '[' => 2,
                        '{' => 3,
                        '<' => 4,
                        _ => 0,
                    },
                }
        }

        score
    }

    fn chr_error_score(chr: char) -> i32 {
        match chr {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => 0,
        }
    }

    fn size(&self) -> usize {
        self.inner.len()
    }
}
