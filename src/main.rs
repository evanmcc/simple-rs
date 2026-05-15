use anyhow::Result;

mod prog;
mod soup;

use clap::Parser;

use crate::prog::{Prog, assign, bin_op, int, name};
use crate::soup::Op::*;

#[derive(Parser)]
struct Cli {
    file_name: std::path::PathBuf,
}

fn main() -> Result<()> {
    let mut p = Prog::default();

    p.assign("a", int(1));
    p.assign("b", int(2));
    p.assign("c", int(0));
    p.block(vec![
        assign("b", int(3)),
        assign("c", bin_op(Add, name("a"), name("b"))),
    ]);

    p.ret(0, name("c"));

    p.print();
    Ok(())
}

#[cfg(test)]
mod tests {

    use crate::prog::{Prog, bin_op, int, neg};
    use crate::soup::Op::*;

    #[test]
    fn basic() {
        let mut p = Prog::default();

        // 0 is the default start, we're cheating here this will need to be
        // fixed when we add basic blocks
        p.ret(0, int(1));

        //not sure what to test here!
    }

    #[test]
    fn ops() {
        let mut p = Prog::default();
        // return 1 + 2 * 3 + -5;
        p.ret(
            0,
            bin_op(
                Add,
                int(1),
                bin_op(Add, int(1), bin_op(Mul, int(2), neg(int(5)))),
            ),
        );
    }
}
