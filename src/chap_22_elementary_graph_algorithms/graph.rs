use std::collections::HashMap;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Node {
    label: &'static str,
}

impl Node {
    pub fn new(label: &'static str) -> Node {
        Node {
            label: label
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Edge {
    start: Node,
    end: Node,
    weight: i32
}

impl Edge {
    pub fn new(start: Node, end: Node, weight: i32) -> Edge {
        Edge {
            start: start,
            end: end, 
            weight: weight
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            nodes: vec![],
            edges: vec![]
        }
    }

    pub fn add_node(&mut self, label: &'static str) -> Result<Node, &'static str> {
        let node = Node::new(label);

        for n in &(self.nodes) {
            if *n == node {
                return Err("Node already exists!")
            }
        }

        self.nodes.push(node);

        Ok(node)
    }

    pub fn node(&mut self, label: &'static str) -> Node {
        for n in &(self.nodes) {
            if (*n).label == label {
                return *n;
            }
        }
        self.add_node(label).unwrap()
    }

    pub fn add_edge(&mut self, start: Node, end: Node, weight: i32) -> Result<Edge, &'static str> {
        let edge = Edge::new(start, end, weight);

        for e in &(self.edges) {
            if *e == edge {
                return Err("Edge already exists!")
            }
        }

        self.edges.push(edge);

        Ok(edge)
    }

    pub fn edge(&mut self, start: Node, end: Node) -> Edge {
        for e in &(self.edges) {
            if (*e).start == start && (*e).end == end {
                return *e;
            }
        }
        self.add_edge(start, end, 0i32).unwrap()
    }

    /* TODO: Make this work!
    pub fn set_weight(&mut self, edge: Edge, weight: i32) {
        for e in &(self.edges) {
            if *e == edge {
                (*e).weight = weight;
            }
        }
        panic!("Attempted to set weight on non-existent edge!");
    }
    */

    // These copies of nodes will prevent ownership issues when working
    pub fn get_adjacency_list(&self) -> HashMap<Node, Vec<Node>> {
        let mut adj_list = HashMap::new();
        adj_list
    }

    // These copies of nodes will prevent ownership issues when working
    pub fn get_adjacency_matrix(&self) -> HashMap<(Node, Node), i32> {
        let mut adj_mat = HashMap::new();
        adj_mat
    }
}