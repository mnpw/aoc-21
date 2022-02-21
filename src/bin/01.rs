use std::fs;

fn main() {
    let input_data = fs::read_to_string("input/01.1").unwrap();

    part1(&input_data);
    part2(&input_data);
}

fn part1 (input: &String) {
    println!("Part 1: {}", solver(input, 1));
}

fn part2 (input: &String) {
    println!("Part 2: {}", solver(input, 3));
}

fn solver (input: &String, window_size: usize) -> i32 {

    let input: Vec<i32> = input.lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let mut count = 0;

    for i in  0..input.len() {
        
        // break if window is too big
        if i + window_size >= input.len() {
            break;
        }

        if input[i] < input[i+window_size] {
            count+=1;
        }
    } 

    count
}
