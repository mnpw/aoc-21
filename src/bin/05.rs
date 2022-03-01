const BOARD_SIZE: usize = 1000;

fn main() {
    let input = std::fs::read_to_string("input/05").unwrap();
    part1(input);
}

fn part1(input: String) {
    let board = parsed_input(input);
    let mut overlapping_count = 0;
    for row in &board.state {
        for val in row {
            if val > &1 {
                overlapping_count += 1;
            }
        }
    }

    println!("Part 2 sol: {}", overlapping_count);
}

fn parsed_input(mut input: String) -> Board {
    input.retain(|c| c != ' ');
    let modified: Vec<Vec<String>> = input
        .lines()
        .map(|line| line.split("->").map(|word| String::from(word)).collect())
        .into_iter()
        .collect();

    let mut board = Board {
        state: vec![vec![0; BOARD_SIZE]; BOARD_SIZE],
        x_size: BOARD_SIZE,
        y_size: BOARD_SIZE,
    };

    for line in modified {
        let start_point = Point::from(&line[0]);
        let end_point = Point::from(&line[1]);
        let line = Line::from(start_point, end_point);
        // println!("{:#?}", line);
        board.fill_with(line);
    }
    // println!("{:?}", board);
    return board;
}

#[derive(Debug)]
struct Board {
    state: Vec<Vec<usize>>,
    x_size: usize,
    y_size: usize,
}

impl Board {
    fn fill_with(&mut self, line: Line) {
        match line.line_type {
            LineType::Horizontal | LineType::Vertical | LineType::Diagonal => {
                // println!(
                //     "Line from p1: {:?}, p2: {:?}\n range: {:?}",
                //     line.start, line.end, line.line_range
                // );
                for point in line.line_range {
                    self.state[point.y as usize][point.x as usize] += 1;
                }
                // println!("{:?}", self.state);
            }
            _ => (),
        }
    }
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
    line_type: LineType,
    line_range: Vec<Point>,
}

#[derive(Debug)]
enum LineType {
    Vertical,
    Horizontal,
    Diagonal,
    Else,
}

impl Line {
    fn from(p1: Point, p2: Point) -> Line {
        let line_type = if p1.y == p2.y && p1.x != p2.x {
            LineType::Horizontal
        } else if p1.x == p2.x && p1.y != p2.y {
            LineType::Vertical
        } else if isize::abs((p1.x - p2.x) / (p1.y - p2.y)) == 1 {
            LineType::Diagonal
        } else {
            LineType::Else
        };

        let mut points: Vec<Point> = Vec::new();
        let line_range = match &line_type {
            LineType::Horizontal | LineType::Vertical | LineType::Diagonal => {
                let mut curr_point = Point { x: p1.x, y: p1.y };
                while curr_point.x != p2.x || curr_point.y != p2.y {
                    points.push(Point {
                        x: curr_point.x,
                        y: curr_point.y,
                    });
                    if curr_point.x < p2.x {
                        curr_point.x += 1
                    } else if curr_point.x > p2.x {
                        curr_point.x -= 1
                    }

                    if curr_point.y < p2.y {
                        curr_point.y += 1
                    } else if curr_point.y > p2.y {
                        curr_point.y -= 1
                    }
                }
                points.push(Point {
                    x: curr_point.x,
                    y: curr_point.y,
                });
                points
            }
            _ => points,
        };

        Line {
            start: p1,
            end: p2,
            line_type,
            line_range,
        }
    }
}

#[derive(Debug)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn from(s: &str) -> Point {
        // s is of type "point_1,point_2"
        let tmp = s
            .split(",")
            .map(|point| point.parse().unwrap())
            .collect::<Vec<isize>>();
        Point {
            x: tmp[0],
            y: tmp[1],
        }
    }
}
