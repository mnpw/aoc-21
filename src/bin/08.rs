use std::{any::Any, collections::*};

fn main() {
    let input = std::fs::read_to_string("input/08").unwrap();
    let display_entries: Vec<DisplayEntry> = parse_input(input);
    part2(&display_entries);
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

fn part2(entries: &Vec<DisplayEntry>) {
    let mut sum = 0;

    for entry in entries {
        let mut num = 0;
        for op in &entry.op_display {
            num = (num * 10)
                + dbg!(Digit::ssd_segments_to_num(&SsdUtils::standardize(
                    op,
                    &entry.mapping
                )));
        }
        sum += dbg!(num);
    }

    println!("Part 2 sol: {}", sum);
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
                let mut col = x.chars().collect::<Vec<char>>();
                col.sort();
                col.iter().collect::<String>()
            })
            .collect();

        let display: Vec<String> = entry
            .last()
            .unwrap()
            .split(" ")
            .map(|x| {
                let mut col = x.chars().collect::<Vec<char>>();
                col.sort();
                col.iter().collect::<String>()
            })
            .collect();

        display_entries.push(DisplayEntry::new(signal_pattern, display));
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
            Digit::Eight => "abcdefg",
            Digit::Nine => "abcdfg",
        }
    }

    fn ssd_segments_to_num(segments: &str) -> i32 {
        if segments == "abcefg" {
            0
        } else if segments == "cf" {
            1
        } else if segments == "acdeg" {
            2
        } else if segments == "acdfg" {
            3
        } else if segments == "bcdf" {
            4
        } else if segments == "abdfg" {
            5
        } else if segments == "abdefg" {
            6
        } else if segments == "acf" {
            7
        } else if segments == "abcdefg" {
            8
        } else if segments == "abcdfg" {
            9
        } else {
            -1
        }
    }
}

impl DisplayEntry {
    fn new(ip_signal_pattern: Vec<String>, op_display: Vec<String>) -> DisplayEntry {
        let mut result = DisplayEntry {
            ip_signal_pattern,
            op_display,
            mapping: HashMap::new(),
        };

        result.deduce_mapping();

        result
    }
    fn deduce_mapping(&mut self) {
        self.deduce_mapping_digit()
    }

    fn filter_ip_signal_by_len(&self, len: i32) -> Vec<&String> {
        self.ip_signal_pattern
            .iter()
            .filter(|&signal| signal.len() as i32 == len)
            .collect::<Vec<&String>>()
    }

    fn deduce_mapping_digit(&mut self) {
        // find out 1, 4, 7 and 8
        let one = self
            .filter_ip_signal_by_len(Digit::One.ssd_segments_consumed())
            .first()
            .unwrap()
            .to_owned()
            .to_owned();

        let four = self
            .filter_ip_signal_by_len(Digit::Four.ssd_segments_consumed())
            .first()
            .unwrap()
            .to_owned()
            .to_owned();

        let seven = self
            .filter_ip_signal_by_len(Digit::Seven.ssd_segments_consumed())
            .first()
            .unwrap()
            .to_owned()
            .to_owned();

        let eight = self
            .filter_ip_signal_by_len(Digit::Eight.ssd_segments_consumed())
            .first()
            .unwrap()
            .to_owned()
            .to_owned();

        let mapping = &mut self.mapping;

        // 7 - 1 gives the segment equivalent to 'a'
        let maps_to_a = SsdUtils::minus(&seven, &one);
        mapping.insert(maps_to_a.chars().next().unwrap(), 'a');

        // 2 ∩ 3 ∩ 5 ∩ 4 gives the segment equivalent to 'd'
        let two_three_or_five = SsdUtils::intersection_list(self.filter_ip_signal_by_len(5));
        let maps_to_d = SsdUtils::intersection(&two_three_or_five, &four);
        self.mapping.insert(maps_to_d.chars().next().unwrap(), 'd');

        // (2 ∩ 3 ∩ 5) - a - d gives the segment equivalent to 'g'
        let maps_to_g =
            SsdUtils::minus(&SsdUtils::minus(&two_three_or_five, &maps_to_a), &maps_to_d);
        self.mapping.insert(maps_to_g.chars().next().unwrap(), 'g');

        // 4 - d - 1 gives the segment equivalent to 'b'
        let maps_to_b = SsdUtils::minus(&SsdUtils::minus(&four, &maps_to_d), &one);
        self.mapping.insert(maps_to_b.chars().next().unwrap(), 'b');

        // (0 ∩ 6 ∩ 9) - a - b - g gives the segment equivalent to 'f'
        let zero_six_or_nine = SsdUtils::intersection_list(self.filter_ip_signal_by_len(6));
        let maps_to_f = SsdUtils::minus(
            &SsdUtils::minus(&SsdUtils::minus(&zero_six_or_nine, &maps_to_a), &maps_to_b),
            &maps_to_g,
        );
        self.mapping.insert(maps_to_f.chars().next().unwrap(), 'f');

        // 1 - f gives segment equivalent to 'c'
        let maps_to_c = SsdUtils::minus(&one, &maps_to_f);
        self.mapping.insert(maps_to_c.chars().next().unwrap(), 'c');

        // 8 - a - b - c - d - f - g gives segment equivalent to 'e'
        let mut maps_to_e = SsdUtils::minus(&eight, &maps_to_a);
        maps_to_e = SsdUtils::minus(&maps_to_e, &maps_to_b);
        maps_to_e = SsdUtils::minus(&maps_to_e, &maps_to_c);
        maps_to_e = SsdUtils::minus(&maps_to_e, &maps_to_d);
        maps_to_e = SsdUtils::minus(&maps_to_e, &maps_to_f);
        maps_to_e = SsdUtils::minus(&maps_to_e, &maps_to_g);
        self.mapping.insert(maps_to_e.chars().next().unwrap(), 'e');
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

    fn intersection_list(vec: Vec<&String>) -> String {
        let mut vec_iter = vec.iter();
        let mut result = vec_iter.next().unwrap().clone().to_owned();
        for &item in vec_iter {
            result = SsdUtils::intersection(&result, &item);
        }

        result
    }

    fn standardize(digit: &str, map: &HashMap<char, char>) -> String {
        let mut result = String::new();

        for char in digit.chars() {
            result.push(*map.get(&char).unwrap());
        }

        let mut chars = result.chars().collect::<Vec<char>>();
        chars.sort();
        chars.iter().collect::<String>()
    }
}
