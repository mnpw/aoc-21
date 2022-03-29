use std::ops::RangeInclusive;

fn main() {
    let input = std::fs::read_to_string("input/17").unwrap();
    let range = parse_input(&input);

    part1(&range);
    part2(&range);
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

    let x: RangeInclusive<i32> = RangeInclusive::new(
        x_range.first().unwrap().parse().unwrap(),
        x_range.last().unwrap().parse().unwrap(),
    );

    let y_range: Vec<&str> = target_ranges
        .last()
        .unwrap()
        .strip_prefix("y=")
        .unwrap()
        .split("..")
        .collect();

    let y: RangeInclusive<i32> = RangeInclusive::new(
        y_range.first().unwrap().parse().unwrap(),
        y_range.last().unwrap().parse().unwrap(),
    );

    Target { x, y }
}

fn part1(target: &Target) {
    // initial vertical velocity
    let u_y = get_u_y_max(target);

    let y_max = (u_y * (u_y + 1)) / 2;

    println!("Part 1 sol: {}", y_max);
}

fn part2(target: &Target) {
    let u_x_range = get_u_x_range(target);
    let u_y_range = get_u_y_range(target);

    let mut valid_initial_velocities = Vec::new();

    for u_x in u_x_range.clone() {
        for u_y in u_y_range.clone() {
            if shot_in_target(u_x, u_y, target) {
                valid_initial_velocities.push((u_x, u_y));
            }
        }
    }

    println!("Part 2 sol: {}", valid_initial_velocities.len(),);
}

fn get_u_x_range(target: &Target) -> RangeInclusive<i32> {
    if *target.x.start() >= 0 && *target.x.end() >= 0 {
        // Target is above x = 0
        //  x = 0
        //    |     ######
        //    |     ######
        //    |     ######
        //    |

        return 0..=*target.x.end();
    } else if *target.x.start() <= 0 && *target.x.end() <= 0 {
        // Target is below x = 0
        //            x = 0
        //  ######      |
        //  ######      |
        //  ######      |
        //              |

        return *target.x.start()..=0;
    } else {
        // Target cuts x = 0
        //   x = 0
        //  ###|##
        //  ###|##
        //  ###|##
        //     |

        return *target.x.start()..=*target.x.end();
    }
}

fn get_u_y_range(target: &Target) -> RangeInclusive<i32> {
    let u_y_max = get_u_y_max(target);
    if u_y_max < 0 {
        u_y_max..=-u_y_max
    } else {
        -u_y_max..=u_y_max
    }
}

fn get_u_y_max(target: &Target) -> i32 {
    if *target.y.start() >= 0 && *target.y.end() >= 0 {
        // Target is above y = 0
        //
        //          ######
        //          ######
        //          ######
        //
        // y = 0 ---------

        *target.y.end()
    } else if *target.y.start() <= 0 && *target.y.end() <= 0 {
        // Target is below y = 0
        //
        // y = 0 ---------
        //
        //          ######
        //          ######
        //          ######

        *target.y.start()
    } else {
        // Target intersects y = 0
        //
        //          ######
        // y = 0 ---######
        //          ######

        let delta_up = *target.y.end();
        let delta_down = *target.y.start();

        std::cmp::max(delta_up.abs(), delta_down.abs())
    }
}

#[derive(Debug)]
struct Target {
    x: RangeInclusive<i32>,
    y: RangeInclusive<i32>,
}

fn shot_in_target(u_x: i32, u_y: i32, target: &Target) -> bool {
    let mut result = false;

    let mut i = 0;

    let mut v_x = u_x;
    let mut v_y = u_y;

    let mut dist_x = 0;
    let mut dist_y = 0;

    loop {
        i += 1;

        dist_x += v_x;
        dist_y += v_y;
        if target.x.contains(&dist_x) && target.y.contains(&dist_y) {
            result = true;
            break;
        }

        if dist_x > *target.x.end() || dist_y < *target.y.start() {
            break;
        }
        v_x = if u_x - i > 0 { u_x - i } else { 0 };
        v_y = u_y - i;
    }

    result
}
