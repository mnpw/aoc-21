fn main() {
    let input = std::fs::read_to_string("input/02.1").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(course: &String) {
    let mut fwd = 0;
    let mut down = 0;
    let mut up = 0;

    for instruction in course.lines() {
        let i: Vec<&str> = instruction.split(" ").collect();
        let direction = i[0];
        let delta = i[1].parse::<i32>().unwrap();
        if direction == "forward" {
            fwd += delta;
        } else if direction == "down" {
            down += delta;
        } else if direction == "up" {
            up += delta;
        }
    }

    println!("Part 1: {}", fwd * (down - up))
}

fn part2(course: &String) {
    let mut fwd = 0;
    let mut depth = 0;
    let mut aim = 0;
    
    for instruction in course.lines() {
        let i: Vec<&str> = instruction.split(" ").collect();
        let direction = i[0];
        let delta = i[1].parse::<i32>().unwrap();

        if direction == "forward" {
            fwd += delta;
            depth += aim*delta;
        } else if direction == "down" {
            aim += delta;
        } else if direction == "up" {
            aim -= delta;
        }
    }

    println!("Part 2: {}", fwd * depth)
}