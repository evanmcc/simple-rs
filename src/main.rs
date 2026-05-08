use anyhow::Result;

mod prog;

use clap::Parser;

use crate::prog::{Literal, Op, Op::*, Prog, bin_op, int, neg};

#[derive(Debug)]
enum DataNode {
    Constant(Literal),
    Op(Op),
}

#[derive(Debug)]
enum ControlNode {
    Start,
    Return,
}

#[derive(Debug)]
enum NodeT {
    DataNode(DataNode),
    ControlNode(ControlNode),
    DeadNode,
}

#[derive(Debug)]
struct Node {
    t: NodeT,
    inputs: Vec<usize>,
    outputs: Vec<usize>,
}

struct SoN {
    nodes: Vec<Node>,
}

impl SoN {
    fn new() -> Self {
        Self { nodes: Vec::new() }
    }
}

#[derive(Parser)]
struct Cli {
    file_name: std::path::PathBuf,
}

fn main() -> Result<()> {
    let mut p = Prog::default();

    // return 1 + 2 * 3 + -5;
    p.ret(bin_op(
        Add,
        int(1),
        bin_op(Add, int(1), bin_op(Mul, int(2), neg(int(5)))),
    ));

    for (i, n) in p.nodes.iter().enumerate() {
        if let NodeT::DeadNode = n.t {
            continue;
        }
        println!("node: {i} {n:?}");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::prog::{Prog, int};

    #[test]
    fn basic() {
        let mut p = Prog::default();

        p.ret(int(1));

        //not sure what to test here!
    }
}
