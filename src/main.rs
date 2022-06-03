/// Keeps track of the last and the current value
struct History<T> {
    last: T,
    current: T,
}

impl<T> History<T> {
    fn new(last: T, current: T) -> Self {
        Self { last, current }
    }

    fn push(self, new: T) -> Self {
        Self {
            last: self.current,
            current: new,
        }
    }
}

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

    fn add_deps(&mut self, idx: usize, deps: Vec<usize>) {
        for dep in deps {
            self.dependencies[idx].push(dep);
        }
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

        // TODO allow even with no initial non-cycle node
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

        let mut remaining = History::new(usize::MAX, remaining_inbound.iter().sum());

        while remaining.current != 0 {
            if remaining.current == remaining.last {
                assert!(leafs.is_empty());
                eprintln!("we got no further, break up a cycle!");

                // find the highest one
                let rem_idx = remaining_inbound
                    .iter()
                    .enumerate()
                    .fold((None, 0), |(best_i, best_v), (i, v)| {
                        if *v > best_v {
                            (Some(i), *v)
                        } else {
                            (best_i, best_v)
                        }
                    })
                    .0
                    .unwrap();

                for (from, _) in self.out.iter().enumerate() {
                    for (i, to) in self.out[from].iter().enumerate() {
                        if *to != rem_idx {
                            continue;
                        }

                        if disabled[from][i] {
                            continue;
                        }

                        disabled[from][i] = true;
                        remaining_inbound[rem_idx] -= 1;
                    }
                }

                leafs.push_back(rem_idx);
            }

            while let Some(i) = leafs.pop_front() {
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

            remaining = remaining.push(remaining_inbound.iter().sum());
        }

        assert_eq!(remaining.current, 0);

        solution
    }
}

fn programs() -> Graph<&'static str> {
    let mut graph = GraphBuilder::default();

    let boot = graph.add_node("boot", vec![]);

    let xorg = graph.add_node("xorg", vec![boot]);
    let _dwm = graph.add_node("dwm", vec![xorg]);

    let net = graph.add_node("net", vec![boot]);

    let _firefox = graph.add_node("firefox", vec![net, xorg]);

    graph.build()
}

fn cycle() -> Graph<&'static str> {
    let mut graph = GraphBuilder::default();

    let a = graph.add_node("a", vec![]);
    let b = graph.add_node("b", vec![a]);
    let c = graph.add_node("c", vec![b]); // cant add [g, k] yet

    let d = graph.add_node("d", vec![c]);

    let e = graph.add_node("e", vec![d]);
    let f = graph.add_node("f", vec![e]);
    let g = graph.add_node("g", vec![f]);

    let i = graph.add_node("i", vec![d]);
    let j = graph.add_node("j", vec![i]);
    let k = graph.add_node("k", vec![j]);

    graph.add_deps(c, vec![g, k]);

    graph.build()
}

fn main() {
    for graph in [programs(), cycle()] {
        let solution = graph.solve();

        // get the values from the indecies
        let solution: Vec<_> = solution.into_iter().map(|i| graph.nodes[i]).collect();
        println!("The solution is {:?}", solution);
    }
}
