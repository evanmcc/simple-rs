use std::collections::HashMap;

use crate::{ControlNode, DataNode, Node, NodeT};

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
}

#[derive(Debug, Clone)]
pub struct BinOp {
    op: Op,
    lhs: Box<Expr>,
    rhs: Box<Expr>,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Literal),
    BinOp(BinOp),
}

#[derive(Debug)]
pub struct Prog {
    pub nodes: Vec<Node>,
    pub hash_cons: HashMap<Node, usize>,
}

impl Default for Prog {
    fn default() -> Self {
        let start = Node {
            t: NodeT::ControlNode(ControlNode::Start),
            inputs: vec![],
            outputs: vec![],
        };
        Self {
            nodes: vec![start],
            hash_cons: HashMap::new(),
        }
    }
}

impl Prog {
    pub fn ret(&mut self, e: Expr) -> usize {
        let r = self.add_node(NodeT::ControlNode(ControlNode::Return), vec![e], vec![]);
        // this is a cheat since we don't really have functions yet, eventually a return
        // will need an input pointer to a start node
        self.nodes[r].inputs.insert(0, 0);
        r
    }

    fn add_node(&mut self, t: NodeT, ins: Vec<Expr>, outs: Vec<Expr>) -> usize {
        let inputs = ins
            .iter()
            .map(|e| {
                let n = expr_to_node(e);
                // memoized insertion to get the index
                let node_idx = self.insert_node(n);
                node_idx
            })
            .collect();
        let outputs = outs
            .iter()
            .map(|e| {
                let n = expr_to_node(e);
                // memoized insertion to get the index
                let node_idx = self.insert_node(n);
                node_idx
            })
            .collect();
        self.insert_node(Node { t, inputs, outputs })
    }

    fn insert_node(&mut self, n: Node) -> usize {
        self.nodes.push(n);
        return self.nodes.len() - 1;
    }
}

fn expr_to_node(e: &Expr) -> Node {
    match e {
        Expr::Literal(l) => Node {
            t: NodeT::DataNode(DataNode::Constant(l.clone())),
            inputs: vec![],
            outputs: vec![],
        },
        _ => unimplemented!("unknown expr"),
    }
}

// some easy constructors to make writing psuedocode a bit easier.

pub fn int(i: isize) -> Expr {
    Expr::Literal(Literal::Integer(i))
}

pub fn bin_op(o: Op, lhs: &Expr, rhs: &Expr) -> Expr {
    Expr::BinOp(BinOp {
        op: o,
        lhs: Box::new(lhs.clone()),
        rhs: Box::new(rhs.clone()),
    })
}
