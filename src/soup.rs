use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone)]
pub enum Literal {
    Integer(isize),
}

#[derive(Debug, Clone)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Neg,
}

#[derive(Debug)]
pub enum DataNode {
    Constant(Literal),
    Op(Op),
}

#[derive(Debug)]
pub enum ControlNode {
    Start,
    Return,
}

#[derive(Debug)]
pub struct ScopeNode {
    // pub vars: HashMap<String, usize>,
    pub level: usize,
}

#[derive(Debug)]
pub enum NodeT {
    DataNode(DataNode),
    ControlNode(ControlNode),
    ScopeNode(ScopeNode),
    DeadNode,
}

#[derive(Debug)]
pub struct Node {
    t: NodeT,
    inputs: Vec<usize>,
    outputs: Vec<usize>,
}

impl Node {
    pub fn dead(&self) -> bool {
        match self.t {
            NodeT::DeadNode => true,
            _ => false,
        }
    }

    pub fn new(t: NodeT, inputs: Vec<usize>, outputs: Vec<usize>) -> Self {
        Node { t, inputs, outputs }
    }

    pub fn add_input(&mut self, idx: usize) {
        self.inputs.push(idx);
    }

    pub fn add_output(&mut self, idx: usize) {
        self.outputs.push(idx);
    }
}

#[derive(Debug)]
pub struct Soup {
    pub nodes: Vec<Node>,
    hash_cons: HashMap<Node, usize>,
}

impl Soup {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            hash_cons: HashMap::new(),
        }
    }

    pub fn insert_node(&mut self, n: Node) -> usize {
        self.nodes.push(n);
        return self.nodes.len() - 1;
    }
}
