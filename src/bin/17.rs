use std::ops::Range;

fn main() {
    let input = std::fs::read_to_string("input/17").unwrap();
    let range = parse_input(&input);

    part1(&range);
}

fn parse_input(input: &str) -> Target {
    let split = input.strip_prefix("target area: ").unwrap().split(", ");
    let target_ranges = split.collect::<Vec<&str>>();

    let x_range: Vec<&str> = target_ranges
        .first()
        .unwrap()
        .strip_prefix("x=")
        .unwrap()
        .split("..")
        .collect();

    let x: Range<i32> = Range {
        start: x_range.first().unwrap().parse().unwrap(),
        end: x_range.last().unwrap().parse().unwrap(),
    };

    let y_range: Vec<&str> = target_ranges
        .last()
        .unwrap()
        .strip_prefix("y=")
        .unwrap()
        .split("..")
        .collect();

    let y: Range<i32> = Range {
        start: y_range.first().unwrap().parse().unwrap(),
        end: y_range.last().unwrap().parse().unwrap(),
    };

    Target { x, y }
}

fn part1(target: &Target) {
    // initial vertical velocity
    let u_y = if target.y.start > 0 && target.y.end > 0 {
        // Range is above y = 0
        //
        //          ######
        //          ######
        //          ######
        //
        // y = 0 ---------

        target.y.end
    } else if target.y.start < 0 && target.y.end < 0 {
        // Range is below y = 0
        //
        // y = 0 ---------
        //
        //          ######
        //          ######
        //          ######

        -target.y.start - 1
    } else {
        // Range intersects y = 0
        //
        //          ######
        // y = 0 ---######
        //          ######

        let delta_up = target.y.end;
        let delta_down = -target.y.start - 1;

        std::cmp::max(delta_up, delta_down)
    };

    let y_max = (u_y * (u_y + 1)) / 2;

    println!("Part 1 sol: {}", y_max);
}

#[derive(Debug)]
struct Target {
    x: Range<i32>,
    y: Range<i32>,
}
