use anyhow::Result;

mod prog;
use prog::{Literal, Prog, int};

use clap::Parser;

#[derive(Debug)]
enum DataNode {
    Constant(Literal),
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

    p.ret(int(1));

    for (i, n) in p.nodes.iter().enumerate() {
        if let NodeT::DeadNode = n.t {
            continue;
        }
        println!("node: {i} {n:?}");
    }
    Ok(())
}
