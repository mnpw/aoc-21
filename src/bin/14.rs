use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input/14").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    println!("Part 1 sol: {}", solve(input, 10));
}

fn part2(input: &str) {
    println!("Part 2 sol: {}", solve(input, 40));
}

fn solve(input: &str, steps: i32) -> i64 {
    let mut polymer = Polymer::from(input);
    let mut freq_map_vals: Vec<i64> = polymer
        .get_char_freq_map_fast(steps)
        .into_values()
        .collect();
    freq_map_vals.sort();

    freq_map_vals.last().unwrap() - freq_map_vals.first().unwrap()
}

struct Polymer {
    base: String,
    substitution_map: HashMap<String, char>,
    state: Vec<char>,
    steps: i32,
}

impl Polymer {
    fn from(input: &str) -> Polymer {
        let mut iter = input.split("\n\n");
        let base = iter.next().unwrap().to_string();

        let mut substitution_map: HashMap<String, char> = HashMap::new();
        for sub in iter.next().unwrap().lines() {
            let mut it = sub.split(" -> ");
            let key = it.next().unwrap().to_string();
            let val = it.next().unwrap().chars().next().unwrap();
            substitution_map.insert(key, val);
        }

        let state = base.chars().collect::<Vec<char>>();

        Polymer {
            base,
            substitution_map,
            state,
            steps: 0,
        }
    }

    fn step_up_to(&mut self, count: i32) {
        // idempotent(?)

        for _ in self.steps..count {
            self.step_chain();
            println!("[{}]", &self.steps);
        }
    }

    fn step_chain(&mut self) {
        // execute pair insertion process on the current
        // state of polymer chain

        let mut new_state = vec![' '; 2 * self.state.len() - 1];

        // fill new_state with old_state data
        for (i, val) in new_state.iter_mut().enumerate() {
            if i % 2 == 0 {
                *val = self.state[i / 2];
            }
        }

        // fill 'holes' in new_state by using
        // substitution map
        for i in 0..new_state.len() {
            if i % 2 != 0 {
                let key = new_state[i - 1].to_string() + &new_state[i + 1].to_string();

                new_state[i] = self.substitution_map[&key];
            }
        }

        self.steps += 1;
        self.state = new_state;
    }

    fn get_freq_map(&self) -> HashMap<char, i32> {
        let mut map: HashMap<char, i32> = HashMap::new();

        for char in &self.state {
            *map.entry(*char).or_insert(0) += 1;
        }

        map
    }

    fn get_char_freq_map_fast(&mut self, count: i32) -> HashMap<char, i64> {
        // character frequency map from polymer chain
        // without creating the chain structure

        let mut pair_freq_map: HashMap<String, i64> = HashMap::new();
        let mut char_freq_map: HashMap<char, i64> = HashMap::new();

        let base_state = self.base.chars().collect::<Vec<char>>();
        // init pair frequency map
        //
        // with base as NNCB,  it should look like
        // { NN -> 1, NC -> 1, CB -> 1}
        for i in 0..(base_state.len() - 1) {
            let key = base_state[i].to_string() + &base_state[i + 1].to_string();
            *pair_freq_map.entry(key).or_insert(0) += 1;
        }
        // init char freq map
        //
        // with base as NNCB, it should look like
        // {N -> 2, C -> 1, B -> 1}
        for char in base_state {
            *char_freq_map.entry(char).or_insert(0) += 1;
        }

        for _ in 0..count {
            let mut new_map = HashMap::new();
            for key in self.substitution_map.keys() {
                let curr_key_count = *pair_freq_map.entry(key.to_string()).or_insert(0);

                let insertion_char = self.substitution_map.get(key).unwrap();
                let mut iter = key.chars().into_iter();

                let first_char = iter.next().unwrap();
                let second_char = *insertion_char;
                let third_char = iter.next_back().unwrap();

                let insertion_child_first = first_char.to_string() + &second_char.to_string();
                let insertion_child_second = second_char.to_string() + &third_char.to_string();
                *new_map.entry(insertion_child_first).or_insert(0) += curr_key_count;
                *new_map.entry(insertion_child_second).or_insert(0) += curr_key_count;

                *char_freq_map.entry(second_char).or_insert(0) += curr_key_count;
            }
            pair_freq_map = new_map;
        }

        char_freq_map
    }
}
