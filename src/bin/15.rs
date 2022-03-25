use std::{cmp::Ordering, collections::HashMap};

use priority_queue::PriorityQueue;

fn main() {
    let input = std::fs::read_to_string("input/15").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let start_time = std::time::Instant::now();

    let input_vector = parse_input(input);
    let graph = Graph::from(&input_vector);

    let paths = graph.shortest_path();
    println!(
        "[{:.2?}] Part 1 sol: {:?}",
        start_time.elapsed(),
        paths[&graph.end]
    );
}

fn part2(input: &str) {
    let start_time = std::time::Instant::now();

    const EXPANSION_SCALE: usize = 5;
    let input_vector = parse_input(input);
    let height = input_vector.len();
    let width = input_vector.first().unwrap().len();

    // create expanded map
    let new_height = height * EXPANSION_SCALE;
    let new_width = width * EXPANSION_SCALE;
    let mut expanded_input_vector = vec![vec![0; new_width]; new_height];

    for i in 0..new_height {
        for j in 0..new_width {
            let base = input_vector[i % height][j % width];
            let delta = (i / height) + (j / width);
            let mut new_val = (base + delta as i32) % 9;

            if new_val == 0 {
                new_val = 9;
            }

            expanded_input_vector[i][j] = new_val;
        }
    }

    let graph = Graph::from(&expanded_input_vector);

    let paths = graph.shortest_path();
    println!(
        "[{:.2?}] Part 2 sol: {:?}",
        start_time.elapsed(),
        paths[&graph.end]
    );
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    let input_vector = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    input_vector
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct NodeDistance(i32);

impl Ord for NodeDistance {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.cmp(&self.0)
    }
}

impl PartialOrd for NodeDistance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

#[derive(Debug)]
struct Graph {
    start: i32,
    end: i32,
    nodes: Vec<i32>,
    edges: HashMap<i32, Vec<i32>>,
    weights: HashMap<(i32, i32), i32>,
}

impl Graph {
    fn from(input_vector: &Vec<Vec<i32>>) -> Graph {
        // create a graph from the input risk level map

        let grid_height = input_vector.len();
        let line_size = input_vector.first().unwrap().len();

        let mut nodes: Vec<i32> = Vec::new();
        let mut edges: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut weights: HashMap<(i32, i32), i32> = HashMap::new();

        for (i, line) in input_vector.iter().enumerate() {
            for (j, _) in line.iter().enumerate() {
                let node_id = (i * line_size + j) as i32;
                let mut neighbours: Vec<i32> = Vec::new();

                if i as i32 - 1 >= 0 {
                    let up_node_id = ((i - 1) * line_size + j) as i32;
                    neighbours.push(up_node_id);
                    weights.insert((node_id, up_node_id), input_vector[i - 1][j]);
                }

                if j as i32 - 1 >= 0 {
                    let left_node_id = (i * line_size + (j - 1)) as i32;
                    neighbours.push(left_node_id);
                    weights.insert((node_id, left_node_id), input_vector[i][j - 1]);
                }

                if i + 1 < grid_height {
                    let down_node_id = ((i + 1) * line_size + j) as i32;
                    neighbours.push(down_node_id);
                    weights.insert((node_id, down_node_id), input_vector[i + 1][j]);
                }

                if j + 1 < line_size {
                    let right_node_id = (i * line_size + (j + 1)) as i32;
                    neighbours.push(right_node_id);
                    weights.insert((node_id, right_node_id), input_vector[i][j + 1]);
                }

                nodes.push(node_id);
                edges.insert(node_id, neighbours);
            }
        }

        let start = *nodes.first().unwrap();
        let end = *nodes.last().unwrap();

        Graph {
            start,
            end,
            nodes,
            edges,
            weights,
        }
    }

    fn shortest_path(&self) -> HashMap<i32, i32> {
        // implementation of Dijkstra's algorithm

        let mut previous: HashMap<i32, i32> = HashMap::new();
        let mut visitable: HashMap<i32, bool> = HashMap::new();
        let mut distance: HashMap<i32, i32> = HashMap::new();
        let mut queue = PriorityQueue::new();

        distance.insert(self.start, 0);
        queue.push(self.start, NodeDistance(0));

        while !queue.is_empty() {
            let curr_node = queue.pop().unwrap();
            visitable.insert(curr_node.0, false);

            for e in self.edges.get(&curr_node.0).unwrap() {
                if !*visitable.entry(*e).or_insert(true) {
                    continue;
                }

                let alt = distance[&curr_node.0] + *self.weights.get(&(curr_node.0, *e)).unwrap();

                if alt < *distance.entry(*e).or_insert(i32::MAX) {
                    distance.insert(*e, alt);
                    previous.insert(*e, curr_node.0);
                    queue.push(*e, NodeDistance(alt));
                }
            }
        }

        distance
    }
}
