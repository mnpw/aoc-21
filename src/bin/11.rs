fn main() {
    let input = std::fs::read_to_string("input/11").unwrap();
    let mut octopus_grid = parse_input(&input);
    part1(&mut octopus_grid);
    part2(&mut octopus_grid);
}

fn parse_input(input: &str) -> OctopusGrid {
    let lines = input.lines().collect();
    OctopusGrid::from(lines)
}

fn part1(octopus_grid: &mut OctopusGrid) {
    // run 100 steps
    // return flashes
    octopus_grid.run_steps(100);

    println!("Part 1 sol: {}", octopus_grid.flashes);
}

fn part2(octopus_grid: &mut OctopusGrid) {
    // run as many steps as required to get full flash

    while octopus_grid.full_flash_steps.len() == 0 {
        octopus_grid.step();
    }

    println!(
        "Part 2 sol: {}",
        octopus_grid.full_flash_steps.first().unwrap()
    );
}

struct OctopusGrid {
    inner: Vec<Vec<i32>>,
    height: usize,
    width: usize,
    flashes: i32,
    steps: i32,
    full_flash_steps: Vec<i32>, // steps at which all cells flash
}

impl OctopusGrid {
    const ENERGY_MAX: i32 = 10;
    fn from(string_input: Vec<&str>) -> OctopusGrid {
        let height = string_input.len();
        let width = string_input.first().unwrap().len();
        let flashes = 0;
        let steps = 0;
        let mut inner = vec![vec![0; width]; height];

        for (i, line) in string_input.iter().enumerate() {
            for (j, char) in line.chars().enumerate() {
                inner[i][j] = char.to_digit(10).unwrap() as i32;
            }
        }

        OctopusGrid {
            inner,
            height,
            width,
            flashes,
            steps,
            full_flash_steps: Vec::new(),
        }
    }

    fn run_steps(&mut self, count: i32) {
        for _ in 0..count {
            self.step();
        }
    }

    fn step(&mut self) {
        // go to each cell
        // find out how many increments are to be done for each cell
        // - 1 unit will be done for each cell
        // - look at 8 adjacent cells
        // - if any cell is going to
        // increase step by 1

        // increase energy of all cells by 1
        for i in 0..self.height {
            for j in 0..self.width {
                self.inner[i][j] += 1;
            }
        }

        // maintain state of flash status of a cell
        // 0 -> has not flashed
        // 1 -> has flashed
        let mut flash_state = vec![vec![0; self.width]; self.height];

        // flash cells with energy >= ENERGY_MAX
        for i in 0..self.height {
            for j in 0..self.width {
                if self.inner[i][j] >= OctopusGrid::ENERGY_MAX {
                    self.flash_cell(i, j, &mut flash_state);
                }
            }
        }

        // count flashed cells and reset them to 0
        let mut flashed_count = 0;
        let mut cell_sum = 0;
        for i in 0..self.height {
            for j in 0..self.width {
                if self.inner[i][j] >= OctopusGrid::ENERGY_MAX {
                    flashed_count += 1;
                    self.inner[i][j] = 0;
                }
                cell_sum += self.inner[i][j];
            }
        }

        self.steps += 1;
        self.flashes += flashed_count;

        // check if the whole grid has flashed
        if cell_sum == 0 {
            self.full_flash_steps.push(self.steps)
        }
    }

    fn flash_cell(&mut self, i: usize, j: usize, flash_state: &mut Vec<Vec<i32>>) {
        // don't flash cell
        // if already flashed before OR
        // energy of cell is less than ENERGY_MAX
        if flash_state[i][j] == 1 || self.inner[i][j] < OctopusGrid::ENERGY_MAX {
            return;
        }

        // mark current cell flashed
        flash_state[i][j] = 1;

        // increase energy of adjacent cells and call flash on them
        let coordinate_enumeration = self.get_coordinate_enumeration(i as i32, j as i32);

        for (i_new, j_new) in coordinate_enumeration {
            self.inner[i_new][j_new] += 1;
            self.flash_cell(i_new, j_new, flash_state);
        }
    }

    fn get_coordinate_enumeration(&self, i: i32, j: i32) -> Vec<(usize, usize)> {
        let mut coordinate_enumeration = vec![];
        let delta = vec![
            (-1, -1), // ↖
            (-1, 0),  // ↑
            (-1, 1),  // ↗
            (0, 1),   // →
            (1, 1),   // ↘
            (1, 0),   // ↓
            (1, -1),  // ↙
            (0, -1),  // ←
        ];

        for d in delta {
            if self.valid_coordinate(i + d.0, j + d.1) {
                coordinate_enumeration.push(((i + d.0) as usize, (j + d.1) as usize));
            }
        }

        coordinate_enumeration
    }

    fn valid_coordinate(&self, i: i32, j: i32) -> bool {
        i >= 0 && j >= 0 && i < self.height as i32 && j < self.width as i32
    }
}
