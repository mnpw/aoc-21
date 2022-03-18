use std::collections::*;

fn main() {
    let input = std::fs::read_to_string("input/12").unwrap();
    let graph = parse_input(&input);
    part2(graph);
}

fn parse_input(input: &str) -> Graph {
    let input = input.lines().collect::<Vec<&str>>();
    Graph::from(input)
}

fn part1(mut graph: Graph) {
    graph.find_paths();
    // dbg!(&graph.nodes, &graph.edges, &graph.paths);
    println!("Part 1 sol: {}", &graph.paths.len());
}

fn part2(mut graph: Graph) {
    graph.find_paths_part_two();

    let mut paths = graph.paths.into_iter().collect::<Vec<String>>();
    paths.sort();
    // dbg!(&graph.nodes, &graph.edges, &paths);
    println!("Part 2 sol: {}", &paths.len());
}

struct Graph {
    start: String,
    end: String,
    nodes: HashSet<String>,
    edges: HashMap<String, Vec<String>>,
    paths: HashSet<String>,
}

impl Graph {
    fn from(input: Vec<&str>) -> Graph {
        let start = String::from("start");
        let end = String::from("end");
        let mut nodes: HashSet<String> = HashSet::new();
        let mut edges: HashMap<String, Vec<String>> = HashMap::new();
        let paths: HashSet<String> = HashSet::new();

        for line in input {
            let mut iter = line.split("-");
            let first_node = iter.next().unwrap();
            let second_node = iter.next().unwrap();

            nodes.insert(first_node.to_string());
            nodes.insert(second_node.to_string());

            edges
                .entry(first_node.to_string())
                .or_insert(Vec::new())
                .push(second_node.to_string());

            edges
                .entry(second_node.to_string())
                .or_insert(Vec::new())
                .push(first_node.to_string());
        }

        Graph {
            start,
            end,
            nodes,
            edges,
            paths,
        }
    }

    fn find_paths(&mut self) {
        // run dfs that can take variable strategy to
        // mark a node visitable or not

        let visit_map = self.init_visited_part_one();
        let path_so_far = "";

        self.run_dfs(&self.start.to_owned(), visit_map, path_so_far);
    }

    fn find_paths_part_two(&mut self) {
        let visit_maps = self.init_visisted_part_two();

        for visit_map in visit_maps {
            let path_so_far = "";
            self.run_dfs(&self.start.to_owned(), visit_map, path_so_far);
        }
    }

    fn init_visited_part_one(&self) -> HashMap<String, VisitStatus> {
        let mut map: HashMap<String, VisitStatus> = HashMap::new();

        for node in &self.nodes {
            let mut visit_status = VisitStatus::default();

            if Util::is_lowercase_word(&node) {
                visit_status.infinitly_visitable = false;
                visit_status.remaining_visits = 1;
            }
            map.insert(node.to_string(), visit_status);
        }

        map
    }

    fn init_visisted_part_two(&self) -> Vec<HashMap<String, VisitStatus>> {
        let mut map_list: Vec<HashMap<String, VisitStatus>> = Vec::new();

        for node in &self.nodes {
            if *node == self.start || *node == self.end {
                continue;
            }
            let mut map = self.init_visited_part_one();

            if Util::is_lowercase_word(&node) {
                map.get_mut(node).unwrap().remaining_visits = 2;
            }

            map_list.push(map);
        }

        map_list
    }

    fn run_dfs(
        &mut self,
        node: &str,
        mut visited: HashMap<String, VisitStatus>,
        path_so_far: &str,
    ) {
        Graph::visit_logic(node, &mut visited);

        // return if we reached end node
        // this is base condition
        if node == self.end {
            self.paths
                .insert(path_so_far.to_owned() + &self.end.to_owned());
            return;
        }
        // find all visitable neighbours and run dfs on each node
        let neighbours = self.edges.get(node).unwrap().to_owned();
        for neighbour in neighbours {
            if visited
                .entry(neighbour.clone())
                .or_insert(VisitStatus::default())
                .is_visitable
            {
                self.run_dfs(
                    &neighbour,
                    visited.clone(),
                    &(path_so_far.to_owned() + node + &",".to_owned()),
                );
            }
        }
    }

    fn visit_logic(node: &str, visit_map: &mut HashMap<String, VisitStatus>) {
        let mut entry = visit_map
            .entry(node.to_owned())
            .or_insert(VisitStatus::default())
            .clone();

        if entry.infinitly_visitable {
            return;
        }

        entry.remaining_visits -= 1;
        if entry.remaining_visits == 0 {
            entry.is_visitable = false;
        }

        visit_map.insert(node.to_string(), entry.clone());
    }
}

#[derive(Debug, Clone)]
struct VisitStatus {
    remaining_visits: i32,
    is_visitable: bool,
    infinitly_visitable: bool,
}
impl VisitStatus {
    fn default() -> VisitStatus {
        VisitStatus {
            remaining_visits: 0,
            is_visitable: true,
            infinitly_visitable: true,
        }
    }
}

struct Util;
impl Util {
    fn is_lowercase_word(word: &str) -> bool {
        word.chars().all(|char| char.is_lowercase())
    }
}
