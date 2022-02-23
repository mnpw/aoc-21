fn main() {
    let input = std::fs::read_to_string("input/04.1").unwrap();
    solve(&input);
}

fn process_input(input: &String) -> (Vec<Board>, Vec<i32>) {
    const BOARD_SIZE: usize = 5;
    let boards: Vec<Board>;
    let drawn_numbers: Vec<i32>;

    let mut lines = input.lines();

    drawn_numbers = lines
        .next()
        .unwrap()
        .split(",")
        .map(|num| num.parse().unwrap())
        .collect();
    lines.next();

    boards = lines
        .collect::<Vec<&str>>()
        .chunks(BOARD_SIZE + 1)
        .map(|board_data| Board::from(board_data[0..BOARD_SIZE].to_vec(), BOARD_SIZE))
        .collect();

    (boards, drawn_numbers)
}

fn solve(input: &String) {
    let (mut boards, drawn_numbers) = process_input(input);
    let mut score_list: Vec<i32> = Vec::new();

    // for every drawn number
    for i in 0..drawn_numbers.len() {
        // mark that number inactive in all boards
        for board in &mut boards {
            mark_inactive(drawn_numbers[i], board);
            if board.playing && check_if_bingo(board) {
                // println!(
                //     "bingo happened! with drawn: {} and active sum: {} for\n {:?}",
                //     drawn_numbers[i],
                //     active_cells_sum(board),
                //     board
                // );
                board.playing = false;
                score_list.push(active_cells_sum(board) * drawn_numbers[i]);
            }
        }
    }

    println!("Part 1: {}", score_list.first().unwrap());
    println!("Part 2: {}", score_list.last().unwrap());
}

fn active_cells_sum(board: &Board) -> i32 {
    let mut sum = 0;
    for i in 0..board.values.len() {
        for j in 0..board.values[i].len() {
            if board.active[i][j] == 1 {
                sum += board.values[i][j];
            }
        }
    }
    sum
}

#[derive(Debug)]
struct Board {
    values: Vec<Vec<i32>>,
    active: Vec<Vec<u8>>,
    playing: bool,
    won: bool,
}

impl Board {
    fn new(size: usize) -> Board {
        let values = vec![vec![0i32; size]; size];
        let active = vec![vec![1u8; size]; size];
        Board {
            values,
            active,
            playing: true,
            won: false,
        }
    }

    fn from(board_input: Vec<&str>, size: usize) -> Board {
        let mut values = vec![vec![0i32; size]; size];
        let active = vec![vec![1u8; size]; size];

        for (i, row) in board_input.iter().enumerate() {
            for (j, val) in row.split_whitespace().enumerate() {
                values[i][j] = val.parse::<i32>().unwrap();
            }
        }

        Board {
            values,
            active,
            playing: true,
            won: false,
        }
    }
}

fn mark_inactive(drawn_number: i32, board: &mut Board) {
    for i in 0..board.values.len() {
        for j in 0..board.values[i].len() {
            if board.values[i][j] == drawn_number {
                board.active[i][j] = 0;
            }
        }
    }
}

fn check_if_bingo(board: &mut Board) -> bool {
    // check if any row is marked
    for i in 0..board.active.len() {
        if board.active[i].iter().all(|&num| num != 1) {
            board.won = true;
        }
    }

    // check if any column is marked
    for i in 0..board.active.len() {
        if board.active.iter().map(|row| row[i]).all(|num| num != 1) {
            board.won = true;
        }
    }
    board.won
}
