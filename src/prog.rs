use std::collections::{HashMap, VecDeque};

use crate::soup::{ControlNode, DataNode, Literal, Node, NodeT, Op, ScopeNode, Soup};

// I'm sure that some of you will look at this file and think, why not just a parser?
// and to that, I answer: I won't and you can't make me.
//
// More seriously, I started this project a few times with various parser generators and
// approaches before thinking: this is actually not what I am interested in working on,
// and decided to either write out ASTs by hand or to do something like this.  I reserve
// the right to change my mind and do the other stuff later, but for now the focus is on
// the various optimization approaches to Sea-of-Nodes/Continuation-Soup-style IRs.

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
pub struct Assign {
    name: String,
    val: Box<Expr>,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Literal),
    BinOp(BinOp),
    UnaryOp(UnaryOp),
    Name(String),
    Assign(Assign),
}

#[derive(Debug)]
pub struct Prog {
    soup: Soup,
    scopes: VecDeque<HashMap<String, usize>>,
}

impl Default for Prog {
    fn default() -> Self {
        // I don't love how we're handling current stack and current function context
        // but presumably we'll get some direction in a chapter or two and figure out
        // how this should actually look.
        let start = Node::new(NodeT::ControlNode(ControlNode::Start), vec![], vec![]);
        let mut scopes = VecDeque::new();
        let top: HashMap<String, usize> = HashMap::new();
        scopes.push_front(top);
        let mut r = Self {
            soup: Soup::new(),
            scopes,
        };
        r.soup.insert_node(start);
        r
    }
}

impl Prog {
    pub fn print(self) {
        for (i, n) in self.soup.nodes.iter().enumerate() {
            if n.dead() {
                continue;
            }
            println!("node: {i} {n:?}");
        }
    }

    pub fn ret(&mut self, st: usize, e: Expr) -> usize {
        // TODO: we also need to pop the scope here and wrap it up in a scope node
        // we have to do this translation before we pop the scope, otherwise we can
        // make the node inaccessible to the translation code.  whee oop :/
        let ret_idx = self.expr_to_node(&e);
        let scope = self.scopes.pop_front().expect("popping empty scope");
        let scope_node = NodeT::ScopeNode(ScopeNode {
            level: self.scopes.len(),
        });
        let inputs = scope.into_values().collect();
        self.add_scope_node(scope_node, inputs, vec![]);
        let r = self.add_node(NodeT::ControlNode(ControlNode::Return), vec![], vec![]);
        // this is a cheat since we don't really have functions yet, eventually a return
        // will need an input pointer to a start node
        self.soup.nodes[r].add_input(st);
        self.soup.nodes[r].add_input(ret_idx);
        r
    }
    pub fn assign(&mut self, s: &str, e: Expr) {
        let i = self.expr_to_node(&e);
        println!("storing {s}");
        self.scopes[0].insert(s.to_string(), i);
    }

    pub fn block(&mut self, instrs: Vec<Expr>) {
        for i in instrs {
            match i {
                Expr::Assign(Assign { name, val }) => {
                    let idx = self.expr_to_node(&val);
                    println!("storing {name}");
                    self.scopes[0].insert(name, idx);
                }

                _ => _ = self.expr_to_node(&i),
            }
        }
    }

    fn find_var(&self, name: &str) -> Option<usize> {
        for s in self.scopes.iter().rev() {
            match s.get(name) {
                Some(idx) => return Some(*idx),
                None => continue,
            }
        }
        None
    }

    fn add_node(&mut self, t: NodeT, ins: Vec<Expr>, outs: Vec<Expr>) -> usize {
        let inputs = ins.iter().map(|e| self.expr_to_node(e)).collect();
        let outputs = outs.iter().map(|e| self.expr_to_node(e)).collect();
        self.soup.insert_node(Node::new(t, inputs, outputs))
    }

    // TODO: ugh
    fn add_scope_node(&mut self, t: NodeT, ins: Vec<usize>, outs: Vec<usize>) -> usize {
        self.soup.insert_node(Node::new(t, ins, outs))
    }

    fn expr_to_node(&mut self, e: &Expr) -> usize {
        let n = match e {
            Expr::Literal(l) => Node::new(
                NodeT::DataNode(DataNode::Constant(l.clone())),
                vec![],
                vec![],
            ),
            Expr::UnaryOp(UnaryOp { op, val }) => {
                let v = self.expr_to_node(val);
                Node::new(NodeT::DataNode(DataNode::Op(op.clone())), vec![v], vec![])
            }
            Expr::BinOp(BinOp { op, lhs, rhs }) => {
                let l = self.expr_to_node(lhs);

                let r = self.expr_to_node(rhs);
                Node::new(
                    NodeT::DataNode(DataNode::Op(op.clone())),
                    vec![l, r],
                    vec![],
                )
            }
            Expr::Name(name) => {
                println!("finding {name}");
                let v = self.find_var(name).clone();
                match v {
                    Some(i) => return i.clone(),
                    None => unreachable!("didn't find a var"),
                }
            }
            _ => unimplemented!("unknown expr: {e:?}"),
        };
        self.soup.insert_node(n)
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

pub fn name(s: &str) -> Expr {
    Expr::Name(s.to_string())
}

pub fn assign(s: &str, e: Expr) -> Expr {
    Expr::Assign(Assign {
        name: s.to_string(),
        val: Box::new(e),
    })
}
