mod ops;
use std::fmt::Debug;

pub use ops::*;

mod node;

use node::{BddNode, BddPointer, BddVariable};

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct Bdd(Vec<BddNode>);

impl Bdd {
    pub(crate) fn root_pointer(&self) -> BddPointer {
        BddPointer::from(self.0.len() - 1)
    }

    pub(crate) fn push_node(&mut self, node: BddNode) {
        self.0.push(node);
    }

    pub(crate) fn var_of(&self, node: BddPointer) -> BddVariable {
        self.0[node.0 as usize].var
    }

    pub(crate) fn low_link_of(&self, node: BddPointer) -> BddPointer {
        self.0[node.0 as usize].low_link
    }

    pub(crate) fn high_link_of(&self, node: BddPointer) -> BddPointer {
        self.0[node.0 as usize].high_link
    }
}

impl Bdd {
    pub fn num_node(&self) -> usize {
        self.0.len()
    }

    pub fn ith_var(var: usize) -> Self {
        let mut bdd = Self::constant(true);
        bdd.push_node(BddNode::mk_node(
            var.into(),
            BddPointer::constant(false),
            BddPointer::constant(true),
        ));
        bdd
    }

    pub fn constant(val: bool) -> Self {
        if val {
            Bdd(vec![
                BddNode::mk_constant(false),
                BddNode::mk_constant(true),
            ])
        } else {
            Bdd(vec![BddNode::mk_constant(false)])
        }
    }

    pub fn is_constant(&self, val: bool) -> bool {
        if val {
            self.0.len() == 2
        } else {
            self.0.len() == 1
        }
    }
}

impl AsRef<Bdd> for Bdd {
    fn as_ref(&self) -> &Bdd {
        self
    }
}

impl Debug for Bdd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.sat_cubes())
    }
}

impl Bdd {
    pub fn sat_cubes_rec(
        &self,
        node: BddPointer,
        cube: &mut Vec<isize>,
        res: &mut Vec<Vec<isize>>,
    ) {
        if node.is_constant(false) {
            return;
        }
        if node.is_constant(true) {
            res.push(cube.to_vec());
            return;
        }
        let var = self.var_of(node).0 as isize;
        cube.push(var);
        self.sat_cubes_rec(self.high_link_of(node), cube, res);
        cube.pop().unwrap();
        cube.push(-var);
        self.sat_cubes_rec(self.low_link_of(node), cube, res);
        cube.pop().unwrap();
    }

    pub fn sat_cubes(&self) -> Vec<Vec<isize>> {
        let mut res = Vec::new();
        let mut cube = Vec::new();
        self.sat_cubes_rec(self.root_pointer(), &mut cube, &mut res);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a = Bdd::ith_var(0);
        let b = Bdd::ith_var(1);
        let and = &a & &b;
        let or = !a | !b;
        assert_eq!(and, !or);
    }
}
