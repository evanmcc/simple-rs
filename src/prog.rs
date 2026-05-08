use std::collections::HashMap;

use crate::{ControlNode, DataNode, Node, NodeT};

// I'm sure that some of you will look at this file and think, why not just a parser?
// and to that, I answer: I won't and you can't make me.
//
// More seriously, I started this project a few times with various parser generators and
// approaches before thinking: this is actually not what I am interested in working on,
// and decided to either write out ASTs by hand or to do something like this.  I reserve
// the right to change my mind and do the other stuff later, but for now the focus is on
// the various optimization approaches to Sea-of-Nodes/Continuation-Soup-style IRs.

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

#[derive(Debug, Clone)]
pub struct BinOp {
    op: Op,
    lhs: Box<Expr>,
    rhs: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct UnaryOp {
    op: Op,
    val: Box<Expr>,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Literal),
    BinOp(BinOp),
    UnaryOp(UnaryOp),
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
                let n = self.expr_to_node(e);
                // memoized insertion to get the index
                let node_idx = self.insert_node(n);
                node_idx
            })
            .collect();
        let outputs = outs
            .iter()
            .map(|e| {
                let n = self.expr_to_node(e);
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

    fn expr_to_node(&mut self, e: &Expr) -> Node {
        match e {
            Expr::Literal(l) => Node {
                t: NodeT::DataNode(DataNode::Constant(l.clone())),
                inputs: vec![],
                outputs: vec![],
            },
            Expr::UnaryOp(UnaryOp { op, val }) => {
                let vn = self.expr_to_node(val);
                let v = self.insert_node(vn);
                Node {
                    t: NodeT::DataNode(DataNode::Op(op.clone())),
                    inputs: vec![v],
                    outputs: vec![],
                }
            }
            Expr::BinOp(BinOp { op, lhs, rhs }) => {
                let ln = self.expr_to_node(lhs);
                let l = self.insert_node(ln);

                let rn = self.expr_to_node(rhs);
                let r = self.insert_node(rn);
                Node {
                    t: NodeT::DataNode(DataNode::Op(op.clone())),
                    inputs: vec![l, r],
                    outputs: vec![],
                }
            }
            _ => unimplemented!("unknown expr: {e:?}"),
        }
    }
}

// some easy constructors to make writing psuedocode a bit easier.

pub fn int(i: isize) -> Expr {
    Expr::Literal(Literal::Integer(i))
}

pub fn neg(e: Expr) -> Expr {
    Expr::UnaryOp(UnaryOp {
        op: Op::Neg,
        val: Box::new(e),
    })
}

pub fn bin_op(o: Op, lhs: Expr, rhs: Expr) -> Expr {
    Expr::BinOp(BinOp {
        op: o,
        lhs: Box::new(lhs.clone()),
        rhs: Box::new(rhs.clone()),
    })
}
