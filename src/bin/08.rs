use std::{any::Any, collections::*};

fn main() {
    let input = std::fs::read_to_string("input/08").unwrap();
    let display_entries: Vec<DisplayEntry> = parse_input(input);
    part1(&display_entries);
}

fn part1(entries: &Vec<DisplayEntry>) {
    let mut simple_digits_count = 0;
    for entry in entries {
        simple_digits_count += entry
            .filter_ip_signal_by_len(Digit::One.ssd_segments_consumed())
            .len() as i32;

        simple_digits_count += entry
            .filter_ip_signal_by_len(Digit::Four.ssd_segments_consumed())
            .len() as i32;

        simple_digits_count += entry
            .filter_ip_signal_by_len(Digit::Seven.ssd_segments_consumed())
            .len() as i32;

        simple_digits_count += entry
            .filter_ip_signal_by_len(Digit::Eight.ssd_segments_consumed())
            .len() as i32;
    }

    println!("Part 1 sol: {}", simple_digits_count);
}

fn parse_input(input: String) -> Vec<DisplayEntry> {
    let mut display_entries = Vec::<DisplayEntry>::new();

    for line in input.lines() {
        let entry: Vec<&str> = line.split(" | ").collect();
        let signal_pattern: Vec<String> = entry
            .first()
            .unwrap()
            .split(" ")
            .map(|x| {
                x.chars().collect::<Vec<char>>().sort();
                x.to_owned()
            })
            .collect();

        let display: Vec<String> = entry
            .last()
            .unwrap()
            .split(" ")
            .map(|x| {
                x.chars().collect::<Vec<char>>().sort();
                x.to_owned()
            })
            .collect();

        // display_entries.push(DisplayEntry {
        //     ip_signal_pattern: signal_pattern,
        //     op_display: display,
        // });
    }

    display_entries
}

struct DisplayEntry {
    ip_signal_pattern: Vec<String>,
    op_display: Vec<String>,
    mapping: HashMap<char, char>,
}

enum Digit {
    // Digit on seven segment display
    //
    //  aaaa
    // b    c
    // b    c
    //  dddd
    // e    f
    // e    f
    //  gggg
    //
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl Digit {
    fn ssd_segments_consumed(&self) -> i32 {
        self.ssd_mapping().len() as i32
    }

    fn ssd_mapping(&self) -> &'static str {
        match self {
            Digit::Zero => "abcefg",
            Digit::One => "cf",
            Digit::Two => "acdeg",
            Digit::Three => "acdfg",
            Digit::Four => "bcdf",
            Digit::Five => "abdfg",
            Digit::Six => "abdefg",
            Digit::Seven => "acf",
            Digit::Eight => "abcdef",
            Digit::Nine => "abcdf",
        }
    }
}

impl DisplayEntry {
    fn deduce_mapping(&mut self) {
        self.deduce_mapping_digit()
    }

    fn filter_ip_signal_by_len(&mut self, len: i32) -> Vec<&String> {
        self.ip_signal_pattern
            .iter()
            .filter(|&signal| signal.len() as i32 == Digit::One.ssd_segments_consumed())
            .collect::<Vec<&String>>()
    }

    fn deduce_mapping_digit(&mut self) {
        // find out 1, 4, 7 and 8
        //
        let one = self
            .filter_ip_signal_by_len(Digit::One.ssd_segments_consumed())
            .first()
            .unwrap()
            .to_owned();

        let four = self
            .filter_ip_signal_by_len(Digit::Four.ssd_segments_consumed())
            .first()
            .unwrap()
            .to_owned();

        let seven = self
            .filter_ip_signal_by_len(Digit::Seven.ssd_segments_consumed())
            .first()
            .unwrap()
            .to_owned();

        let eight = self
            .filter_ip_signal_by_len(Digit::Eight.ssd_segments_consumed())
            .first()
            .unwrap()
            .to_owned();

        let mapping = &mut self.mapping;

        // 7 - 1 gives the character equivalent to 'a'
        let maps_to_a = SsdUtils::minus(seven, one);
        mapping.insert(maps_to_a.chars().next().unwrap(), 'a');

        let two_three_or_five = self.filter_ip_signal_by_len(5);
        let mut two_three_or_five = two_three_or_five.iter();
        let mut result = two_three_or_five.next().unwrap().clone().to_owned();
        for &num in two_three_or_five {
            let val = SsdUtils::intersection(&result, &num);
            result = val;
        }

        let maps_to_d = SsdUtils::intersection(&result, &four);
        self.mapping.insert(maps_to_a.chars().next().unwrap(), 'd');
    }
}

struct SsdUtils;

impl SsdUtils {
    fn minus(first: &str, second: &str) -> String {
        let mut first_set: HashSet<char> = HashSet::new();
        let mut second_set: HashSet<char> = HashSet::new();

        for char in first.chars() {
            first_set.insert(char);
        }

        for char in second.chars() {
            second_set.insert(char);
        }

        (&first_set - &second_set).into_iter().collect::<String>()
    }

    fn intersection(first: &str, second: &str) -> String {
        let mut first_set: HashSet<char> = HashSet::new();
        let mut second_set: HashSet<char> = HashSet::new();

        for char in first.chars() {
            first_set.insert(char);
        }

        for char in second.chars() {
            second_set.insert(char);
        }
        first_set
            .intersection(&second_set)
            .into_iter()
            .collect::<String>()
    }
}
