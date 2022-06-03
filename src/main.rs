#[derive(Default)]
struct GraphBuilder<T> {
    nodes: Vec<T>,
    dependencies: Vec<Vec<usize>>,
}

impl<T> GraphBuilder<T> {
    fn add_node(&mut self, value: T, dependencies: Vec<usize>) -> usize {
        let idx = self.nodes.len();

        self.nodes.push(value);
        self.dependencies.push(dependencies);

        idx
    }

    fn build(self) -> Graph<T> {
        let mut outward = vec![vec![]; self.dependencies.len()];

        for (from, inbound_edges) in self.dependencies.iter().enumerate() {
            for to in inbound_edges {
                outward[*to].push(from);
            }
        }

        Graph {
            nodes: self.nodes,
            inb: self.dependencies,
            out: outward,
        }
    }
}

struct Graph<T> {
    nodes: Vec<T>,
    inb: Vec<Vec<usize>>,
    out: Vec<Vec<usize>>,
}

impl<T> Graph<T> {
    fn solve(&self) -> Vec<usize> {
        let mut leafs = std::collections::VecDeque::new();

        for (i, edges) in self.inb.iter().enumerate() {
            if edges.is_empty() {
                leafs.push_back(i);
            }
        }

        let mut disabled: Vec<Vec<bool>> = self
            .out
            .iter()
            .map(|e| e.iter().map(|_| false).collect())
            .collect();

        let mut remaining_inbound: Vec<_> = self.inb.iter().map(|e| e.len()).collect();

        let mut solution = Vec::new();

        loop {
            let i = leafs.pop_front();
            if i.is_none() {
                break;
            }
            let i = i.unwrap();

            solution.push(i);

            for (j, to) in self.out[i].iter().enumerate() {
                if disabled[i][j] {
                    continue;
                }
                disabled[i][j] = true;

                remaining_inbound[*to] -= 1;

                // if this was the last edge...
                if remaining_inbound[*to] == 0 {
                    leafs.push_back(*to);
                }
            }
        }

        let remaining_edges: usize = remaining_inbound.into_iter().sum();
        if remaining_edges != 0 {
            panic!("failed, probably due to a cycle!");
        }

        solution
    }
}

fn main() {
    let mut graph = GraphBuilder::default();

    {
        let boot = graph.add_node("boot", vec![]);

        let xorg = graph.add_node("xorg", vec![boot]);
        let _dwm = graph.add_node("dwm", vec![xorg]);

        let net = graph.add_node("net", vec![boot]);

        let _firefox = graph.add_node("firefox", vec![net, xorg]);
    }

    let graph = graph.build();
    let solution = graph.solve();

    // get the values from the indecies
    let solution: Vec<_> = solution.into_iter().map(|i| graph.nodes[i]).collect();
    println!("The solution is {:?}", solution);
}
