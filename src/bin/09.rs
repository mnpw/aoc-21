fn main() {
    let input = std::fs::read_to_string("input/09").unwrap();
    let height_map = parse_input(input);
    part1(&height_map);
    part2(&height_map);
}

fn part1(height_map: &HeightMap) {
    println!("Part 1 sol: {}", height_map.get_risk_level());
}

fn part2(height_map: &HeightMap) {
    let mut basins = height_map.get_basins();
    basins.sort_by(|a, b| b.cmp(a));

    // dbg!(&basins);
    let mut top_three_product = 1;

    for i in 0..3 {
        top_three_product *= basins[i];
    }

    println!("Part 2 sol: {}", top_three_product);
}

fn parse_input(input: String) -> HeightMap {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().chars().count();

    let mut height_map = vec![vec![0i32; width]; height];

    for (i, line) in input.lines().enumerate() {
        for (j, height) in line.chars().enumerate() {
            height_map[i][j] = height.to_digit(10).unwrap() as i32;
        }
    }

    HeightMap {
        inner: height_map,
        height,
        width,
    }
}

#[derive(Debug)]
struct HeightMap {
    inner: Vec<Vec<i32>>,
    height: usize,
    width: usize,
}

impl HeightMap {
    fn get_risk_level(&self) -> i32 {
        let mut risk_level = 0;
        for (i, row) in self.inner.iter().enumerate() {
            for (j, val) in row.iter().enumerate() {
                if self.is_minima(i, j) {
                    risk_level += val + 1;
                }
            }
        }

        risk_level
    }

    fn get_basins(&self) -> Vec<i32> {
        let mut list = Vec::new();
        let mut basin_map = vec![vec![0; self.width]; self.height];
        let mut status_map = vec![vec![1; self.width]; self.height];
        for (i, row) in self.inner.iter().enumerate() {
            for (j, val) in row.iter().enumerate() {
                if self.is_minima(i, j) {
                    list.push(self.get_basin_size(i, j, &mut basin_map, &mut status_map));
                    // println!("______________");
                }
            }
            HeightMap::print_vector(&self.inner, "map");
            HeightMap::print_vector(&status_map, "status");
            HeightMap::print_basin_edges(&self.inner, &basin_map);
        }

        list
    }

    fn is_minima(&self, x: usize, y: usize) -> bool {
        let mut minima = true;
        if x as isize - 1 >= 0 {
            minima = minima && self.inner[x][y] < self.inner[x - 1][y];
        }
        if y as isize - 1 >= 0 {
            minima = minima && self.inner[x][y] < self.inner[x][y - 1];
        }
        if x as isize + 1 < self.height as isize {
            minima = minima && self.inner[x][y] < self.inner[x + 1][y];
        }
        if y as isize + 1 < self.width as isize {
            minima = minima && self.inner[x][y] < self.inner[x][y + 1];
        }
        minima
    }

    fn get_basin_size(
        &self,
        x: usize,
        y: usize,
        mut basin_map: &mut Vec<Vec<i32>>,
        mut status_map: &mut Vec<Vec<i32>>,
    ) -> i32 {
        // dbg!((x, y));

        let mut count_valid_path = 0;

        // check if can go up
        let up = if x as isize - 1 >= 0
            && self.inner[x][y] < self.inner[x - 1][y]
            && status_map[x - 1][y] == 1
        {
            count_valid_path += 1;
            self.get_basin_size(x - 1, y, &mut basin_map, &mut status_map)
        } else {
            0
        };

        // check if can go right
        let right = if y + 1 < self.width
            && self.inner[x][y] < self.inner[x][y + 1]
            && status_map[x][y + 1] == 1
        {
            count_valid_path += 1;
            self.get_basin_size(x, y + 1, &mut basin_map, &mut status_map)
        } else {
            0
        };

        // check if can go down
        let down = if x + 1 < self.height
            && self.inner[x][y] < self.inner[x + 1][y]
            && status_map[x + 1][y] == 1
        {
            count_valid_path += 1;
            self.get_basin_size(x + 1, y, &mut basin_map, &mut status_map)
        } else {
            0
        };

        // check if can go left
        let left = if y as isize - 1 >= 0
            && self.inner[x][y] < self.inner[x][y - 1]
            && status_map[x][y - 1] == 1
        {
            count_valid_path += 1;
            self.get_basin_size(x, y - 1, &mut basin_map, &mut status_map)
        } else {
            0
        };

        let sum = up + right + down + left;
        if self.inner[x][y] == 9 {
            basin_map[x][y] = 0;
            return 0;
        }
        status_map[x][y] -= 1;
        // println!("At {}, {} with sum: {:?}", x, y, (up, right, down, left));
        basin_map[x][y] = sum + 1;
        return sum + 1;
    }

    fn print_vector(vec: &Vec<Vec<i32>>, file_name: &str) {
        let mut output_data = String::new();
        for line in vec {
            let mut map_line = String::new();
            for (j, num) in line.iter().enumerate() {
                map_line.push_str(&format!("{: >3} ", num));
            }
            output_data = output_data + "\n" + &format!("{:?}", map_line);
            // println!("{:?}", line);
        }
        std::fs::write("output/09/".to_owned() + file_name, output_data)
            .expect("Cannot write to the file.");
    }

    fn print_basin_edges(vec: &Vec<Vec<i32>>, basin_map: &Vec<Vec<i32>>) {
        let mut output_data = String::new();
        for (i, line) in vec.iter().enumerate() {
            let mut map_line = String::new();
            for (j, num) in line.iter().enumerate() {
                if num == &9 {
                    if basin_map[i][j] != 0 {
                        map_line.push_str("  X ");
                    } else {
                        map_line.push_str("  ■ ");
                    }
                } else {
                    map_line.push_str(&format!("{: >3} ", basin_map[i][j].to_string()));
                }
            }
            output_data = output_data + "\n" + &map_line;
            // println!("{}", map_line);
        }

        std::fs::write("output/09/basin", output_data).expect("Cannot write to the file.");
    }
}
