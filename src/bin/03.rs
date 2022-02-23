fn main() {
    let input = std::fs::read_to_string("input/03.1").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(diagnostics: &String) {
    let diagnostic_len = diagnostics.lines().next().unwrap().len();

    let mut count = vec![(0, 0); diagnostic_len];
    let total_diagnostic_lines = diagnostics.lines().count();
    let mut gamma_rate = String::new();
    let mut epsilon_rate = String::new();

    for i in 0..diagnostic_len {
        let zero_freq_at_pos_i = diagnostics
            .lines()
            .filter(|x| x.chars().nth(i).unwrap() == '0')
            .count();

        count[i].0 = zero_freq_at_pos_i;
        count[i].1 = total_diagnostic_lines - zero_freq_at_pos_i;

        if count[i].0 > count[i].1 {
            gamma_rate.push('0');
            epsilon_rate.push('1');
        } else {
            gamma_rate.push('1');
            epsilon_rate.push('0');
        }
    }

    let gamma_inferred = isize::from_str_radix(&gamma_rate, 2).unwrap();
    let epsilon_inferred = isize::from_str_radix(&epsilon_rate, 2).unwrap();

    println!("Part 1: {}", gamma_inferred * epsilon_inferred);
}

fn part2(diagnostics: &String) {
    let diagnostic_len = diagnostics.lines().next().unwrap().len();
    let mut filtered_oxygen_diagnostic: Vec<&str> = diagnostics.lines().collect();
    let mut filtered_carbon_diagnostic: Vec<&str> = diagnostics.lines().collect();

    for i in 0..diagnostic_len {
        filtered_oxygen_diagnostic =
            filter_diagnostic(filtered_oxygen_diagnostic, i, "oxygen".to_string());
        filtered_carbon_diagnostic =
            filter_diagnostic(filtered_carbon_diagnostic, i, "carbon".to_string())
    }

    let oxygen_inferred = isize::from_str_radix(filtered_oxygen_diagnostic[0], 2).unwrap();
    let carbon_inferred = isize::from_str_radix(filtered_carbon_diagnostic[0], 2).unwrap();

    println!("Part 2: {:}", oxygen_inferred * carbon_inferred);
}

fn filter_diagnostic(mut diag: Vec<&str>, i: usize, diag_type: String) -> Vec<&str> {
    if diag.len() == 1 {
        return diag;
    }
    let zero_freq_at_pos_i = diag
        .iter()
        .filter(|x| x.chars().nth(i).unwrap() == '0')
        .count();

    let one_freq_at_pos_i = diag.len() - zero_freq_at_pos_i;

    if diag_type == "oxygen" {
        if zero_freq_at_pos_i > one_freq_at_pos_i {
            diag = diag
                .into_iter()
                .filter(|&x| x.chars().nth(i).unwrap() == '0')
                .collect();
        } else {
            diag = diag
                .into_iter()
                .filter(|x| x.chars().nth(i).unwrap() == '1')
                .collect();
        };
    } else {
        if zero_freq_at_pos_i > one_freq_at_pos_i {
            diag = diag
                .into_iter()
                .filter(|&x| x.chars().nth(i).unwrap() == '1')
                .collect();
        } else {
            diag = diag
                .into_iter()
                .filter(|x| x.chars().nth(i).unwrap() == '0')
                .collect();
        };
    }
    return diag;
}
