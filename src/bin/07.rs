fn main() {
    let input = std::fs::read_to_string("input/07").unwrap();
    let mut crab_horizontal_positions: Vec<i32> = input
        .trim()
        .split(",")
        .map(|num| num.parse().unwrap())
        .collect();
    crab_horizontal_positions.sort();
    part1(&mut crab_horizontal_positions);
    part2(&mut crab_horizontal_positions);
}

fn part1(hor_pos: &mut Vec<i32>) {
    // Note: What I've written in $..$ is mostly wrong.
    //
    // $
    // need to find x such that Σ|hor_pos[i] - x| is minimum
    // would be equivalent to find x such that Σ|hor_pos[i] - x|^2 in min
    //
    // we can find such x by differentiating Σ|hor_pos[i] - x|^2 wrt x
    // and equating to 0
    //
    // this results in x = Σ(hor_pos[i])/n
    // where n is length of hor_pos array
    //
    // this can give us a float value of x so we can test for
    // floor(x) and ceil(x) to test which one works
    // $
    //
    // see: https://en.wikipedia.org/wiki/Geometric_median#Properties
    // proof: https://math.stackexchange.com/questions/318381/on-a-1-d-line-the-point-that-minimizes-the-sum-of-the-distances-is-the-median
    // I'm not really in a state to understand the proof right now

    let _x = easy_way(hor_pos);
    let x = hard_way(hor_pos);

    println!("Part 1 sol: {}", x);
}

fn part2(hor_pos: &mut Vec<i32>) {
    let mut fuel_consumption_for_pos = vec![-1; *hor_pos.last().unwrap() as usize];

    for i in 0..fuel_consumption_for_pos.len() {
        fuel_consumption_for_pos[i] = hor_pos
            .iter()
            .map(|&num| {
                let n = i32::abs(num - i as i32);
                n * (n + 1) / 2
            })
            .sum();
    }

    println!("{:?}", fuel_consumption_for_pos);

    let ans = fuel_consumption_for_pos
        .into_iter()
        .reduce(i32::min)
        .unwrap();

    println!("Part 2 sol: {}", ans);
}

fn easy_way(hor_pos: &Vec<i32>) -> i32 {
    // compute median basically
    let len = hor_pos.len();

    if len % 2 == 0 {
        (hor_pos[len / 2 - 1] + hor_pos[len / 2]) / 2
    } else {
        hor_pos[len / 2]
    }
}

fn hard_way(hor_pos: &Vec<i32>) -> i32 {
    // i hate this code this is so bad
    let mut fuel_consumption_for_pos = vec![-1; *hor_pos.last().unwrap() as usize];

    for i in 0..fuel_consumption_for_pos.len() {
        fuel_consumption_for_pos[i] = hor_pos.iter().map(|&num| i32::abs(num - i as i32)).sum();
    }

    fuel_consumption_for_pos.into_iter().min().unwrap()
}
