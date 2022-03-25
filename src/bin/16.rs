use std::collections::VecDeque;

fn main() {
    let input = std::fs::read_to_string("input/16").unwrap();
    part1(&input);
}

fn part1(input: &str) {
    let binary_data = input
        .chars()
        .map(|ch| format!("{:04b}", ch.to_digit(16).unwrap()))
        .collect::<String>();

    let packet = Packet::parse(&binary_data);
    println!("Part 1 sol {}", packet.get_versions().iter().sum::<i64>());
}

#[derive(Debug)]
struct Packet {
    version: i64,
    type_id: i64,
    literal_value: Option<i64>,
    length_type_id: Option<i64>,
    subpackets: Option<Vec<Packet>>,
    size: usize,
}

impl Packet {
    fn parse(transmission: &str) -> Packet {
        let mut curr_idx: usize = 0;

        let version = Util::to_decimal(&transmission[curr_idx..curr_idx + 3]);
        curr_idx += 3;

        let type_id = Util::to_decimal(&transmission[curr_idx..curr_idx + 3]);
        curr_idx += 3;

        let mut literal_value = None;
        let mut length_type_id = None;
        let mut subpackets = None;

        if type_id == 4 {
            // literal type
            let (val, sz) = Packet::parse_literal_value(&transmission[curr_idx..]);
            curr_idx += sz;

            literal_value = Some(val);
        } else {
            // operator type
            length_type_id = Some(Util::to_decimal(&transmission[curr_idx..curr_idx + 1]));
            curr_idx += 1;

            let mut subpacket_list: Vec<Packet> = Vec::new();

            if length_type_id == Some(0) {
                // next 15 bits tell length of sub-packets
                let mut subpacket_length = Util::to_decimal(&transmission[curr_idx..curr_idx + 15]);
                curr_idx += 15;

                while subpacket_length > 0 {
                    let packet = Packet::parse(&transmission[curr_idx..]);
                    curr_idx += packet.size;
                    subpacket_length -= packet.size as i64;

                    subpacket_list.push(packet);
                }
            } else if length_type_id == Some(1) {
                // next 11 bits tell count of sub-packets
                let num_subpackets = Util::to_decimal(&transmission[curr_idx..curr_idx + 11]);
                curr_idx += 11;

                for _ in 0..num_subpackets {
                    let packet = Packet::parse(&transmission[curr_idx..]);
                    curr_idx += packet.size;

                    subpacket_list.push(packet);
                }
            }

            subpackets = Some(subpacket_list);
        }

        Packet {
            version,
            type_id,
            literal_value,
            length_type_id,
            subpackets,
            size: curr_idx,
        }
    }

    fn parse_literal_value(literal: &str) -> (i64, usize) {
        let mut literal_value = String::new();
        let mut literal_value_len: usize = 0;

        let vec_of_strings = literal
            .chars()
            .map(|c| c.to_string())
            .collect::<Vec<String>>();

        for chunk in vec_of_strings.chunks_exact(5) {
            literal_value_len += chunk.len();

            let literal_subpart = chunk[1..].join("");
            literal_value.push_str(&literal_subpart);

            if chunk[0] == "0" {
                break;
            }
        }

        (Util::to_decimal(&literal_value), literal_value_len)
    }

    fn get_versions(&self) -> Vec<i64> {
        let mut version_list = Vec::new();
        let mut queue = VecDeque::new();

        queue.push_back(self);
        while !queue.is_empty() {
            let curr_packet = queue.pop_front().unwrap();
            version_list.push(curr_packet.version);

            if curr_packet.subpackets.is_some() {
                for sp in curr_packet.subpackets.as_ref().unwrap() {
                    queue.push_back(sp);
                }
            }
        }

        version_list
    }
}

struct Util;
impl Util {
    fn to_decimal(string: &str) -> i64 {
        i64::from_str_radix(string, 2).unwrap()
    }
}
