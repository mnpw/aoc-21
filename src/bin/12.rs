use std::collections::*;

fn main() {
    let input = std::fs::read_to_string("input/12").unwrap();
    let graph = parse_input(&input);
    part1(graph);
}

fn parse_input(input: &str) -> Graph {
    let input = input.lines().collect::<Vec<&str>>();
    Graph::from(input)
}

fn part1(mut graph: Graph) {
    graph.find_paths(VisitLogic::type_A);
    dbg!(&graph.nodes, &graph.edges, &graph.paths);
    println!("Part 1 sol: {}", &graph.paths.len());
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

    fn find_paths(&mut self, update_visit_status: fn() -> bool) {
        // pick node
        // mark it visited if applicable
        // then pick all visitable nodes connected to curr node
        // if no node or reached end node then return back
        let visited = HashMap::new();

        self.run_dfs(&self.start.to_owned(), visited, "", update_visit_status);
    }

    fn run_dfs(
        &mut self,
        node: &str,
        mut visited: HashMap<String, i32>,
        path_so_far: &str,
        update_visit_status: fn() -> bool,
    ) {
        // return if we reached end node
        // this is base condition
        if node == self.end {
            self.paths
                .insert(path_so_far.to_owned() + &self.end.to_owned());
            return;
        }

        if node.chars().all(|c| c.is_lowercase()) {
            let entry = visited.entry(node.to_string()).or_insert(0);
            *entry = 1;
        }

        // find all visitable neighbours and run dfs on each node
        let neighbours = self.edges.get(node).unwrap().to_owned();
        for neighbour in neighbours {
            if *visited.entry(neighbour.clone()).or_insert(0) == 0 {
                self.run_dfs(
                    &neighbour,
                    visited.clone(),
                    &(path_so_far.to_owned() + node + &",".to_owned()),
                    update_visit_status,
                );
            }
        }
    }
}

#[derive(Clone)]
struct VisitStatus {
    freq: i32,
    is_visitable: bool,
}

struct VisitLogic {}

impl VisitLogic {
    fn type_A() -> bool {
        false
    }

    fn type_B() -> bool {
        false
    }
}
