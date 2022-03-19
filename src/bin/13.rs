fn main() {
    let mut input = std::fs::read_to_string("input/13").unwrap();
    let (origami, instructions) = parse_input(&input);
    // part1(origami, instructions);
    part2(origami, instructions);
}

fn part1(mut origami: Origami, instructions: Vec<&str>) {
    for i in 0..1 {
        origami.execute_instruction(instructions[i]);
        origami.print_sheet();
    }

    println!("Part 1 sol: {}", origami.count_points());
}

fn part2(mut origami: Origami, instructions: Vec<&str>) {
    for i in instructions {
        origami.execute_instruction(i);

        println!(
            "\n[instruction] {}\n[height x width] {} x {}\n\n\n",
            i, origami.height, origami.width
        );
    }

    println!("Part 2 sol: ");
    origami.print_sheet();
}

fn parse_input(input: &str) -> (Origami, Vec<&str>) {
    let mut split = input.split("\n\n");
    let input_points = split.next().unwrap().split("\n").collect();
    let input_instructions = split.next().unwrap();

    let origami = Origami::from(input_points);
    let input_instructions: Vec<&str> = input_instructions.split("\n").collect();
    (origami, input_instructions)
}

struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn from(input: Vec<usize>) -> Point {
        let mut iter = input.into_iter();
        Point {
            x: iter.next().unwrap(),
            y: iter.next().unwrap(),
        }
    }
}

struct Origami {
    sheet: Vec<Vec<bool>>,
    height: usize,
    width: usize,
}

impl Origami {
    fn from(input: Vec<&str>) -> Origami {
        let mut height: usize = 0;
        let mut width: usize = 0;
        let mut points: Vec<Point> = Vec::new();

        for line in input {
            let point = Point::from(
                line.split(",")
                    .map(|chr| chr.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>(),
            );

            if point.x > width {
                width = point.x
            }

            if point.y > height {
                height = point.y
            }

            points.push(point);
        }

        width += 1;
        height += 1;

        let mut sheet = vec![vec![false; width]; height];
        for p in points {
            sheet[p.y][p.x] = true;
        }

        Origami {
            sheet,
            height,
            width,
        }
    }

    fn count_points(&self) -> i32 {
        let mut count = 0;
        for line in &self.sheet {
            for point in line {
                if *point {
                    count += 1;
                }
            }
        }

        count
    }

    fn print_sheet(&self) {
        for line in &self.sheet {
            let mut display_line = String::new();
            for point in line {
                if *point {
                    display_line += "#";
                } else {
                    display_line += " ";
                }
            }
            println!("{}", display_line);
        }

        println!("\n\n");
    }

    fn execute_instruction(&mut self, instruction: &str) {
        let fold = instruction.split("fold along ").last().unwrap();
        let cut: Vec<&str> = fold.split("=").collect();

        let axis = cut.first().unwrap().to_owned();
        let cut_const = cut.last().unwrap().parse().unwrap();

        dbg!(&axis, &cut_const);

        match axis {
            "x" => {
                self.fold_horizontal(cut_const);
            }
            "y" => {
                self.fold_vertical(cut_const);
            }
            _ => (),
        }
    }

    fn fold_vertical(&mut self, cut: usize) {
        // fold along line y = a
        // along ↑ direction

        let new_height = if cut > self.height - cut - 1 {
            cut
        } else {
            self.height - cut - 1
        };

        let mut new_sheet = vec![vec![false; self.width]; new_height];

        for row in (0..new_height).rev() {
            let delta = row + 1;

            for col in 0..self.width {
                let lower_half = if cut + delta < self.height {
                    self.sheet[cut + delta][col]
                } else {
                    false
                };

                let upper_half = if cut as i32 - delta as i32 >= 0 {
                    self.sheet[cut - delta][col]
                } else {
                    false
                };

                new_sheet[row][col] = upper_half || lower_half;
            }
        }

        self.sheet = new_sheet;
        self.height = new_height;
    }

    fn fold_horizontal(&mut self, cut: usize) {
        // fold along line x = a
        // along → direction

        let new_width = if cut > self.width - cut - 1 {
            cut
        } else {
            self.width - cut - 1
        };

        let mut new_sheet = vec![vec![false; new_width]; self.height];

        for col in (0..new_width).rev() {
            let delta = col + 1;

            for row in 0..self.height {
                let right_half = if cut + delta < self.width {
                    self.sheet[row][cut + delta]
                } else {
                    false
                };

                let left_half = if cut as i32 - delta as i32 >= 0 {
                    self.sheet[row][cut - delta]
                } else {
                    false
                };

                new_sheet[row][col] = right_half || left_half;
            }
        }

        self.sheet = new_sheet;
        self.width = new_width;
    }
}
