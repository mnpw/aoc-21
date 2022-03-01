use std::vec;

fn main() {
    let input = std::fs::read_to_string("input/06").unwrap();
    let lanternfish_vector: Vec<i64> = input
        .trim()
        .split(",")
        .map(|num| num.parse().unwrap())
        .collect();
    solve(&lanternfish_vector, 80, "1");
    solve(&lanternfish_vector, 256, "2");
}

fn solve(vector: &Vec<i64>, days: i64, part: &str) {
    const MAX_TIMER_VALUE: i64 = 10;
    let mut timer_vs_days_map = vec![vec![-1; days as usize]; MAX_TIMER_VALUE as usize];

    let mut count = 0;

    for timer in vector {
        count += get_from_map_else_update(*timer, days - 1, &mut timer_vs_days_map);
    }

    println!("Part {} sol: {}", part, count);
}

fn get_from_map_else_update(timer: i64, days: i64, timer_vs_days_map: &mut Vec<Vec<i64>>) -> i64 {
    if days < 0 {
        return 1;
    }

    if timer_vs_days_map[timer as usize][days as usize] == -1 {
        let count = if timer == 0 {
            get_from_map_else_update(6, days - 1, timer_vs_days_map)
                + get_from_map_else_update(8, days - 1, timer_vs_days_map)
        } else {
            get_from_map_else_update(timer - 1, days - 1, timer_vs_days_map)
        };
        timer_vs_days_map[timer as usize][days as usize] = count;
    }

    return timer_vs_days_map[timer as usize][days as usize];
}
