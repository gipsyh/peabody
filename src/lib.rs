#![feature(assert_matches)]

mod node;
pub use node::*;

mod ops;

use fxhash::FxBuildHasher;
use std::{
    assert_matches::assert_matches,
    collections::HashMap,
    fmt::{self, Debug},
    sync::{Arc, Mutex},
};

pub(crate) struct PeabodyInner {
    nodes: Vec<BddNode>,
    existing: HashMap<BddNode, BddPointer, FxBuildHasher>,
}

impl PeabodyInner {
    #[inline]
    pub(crate) fn var_of(&self, pointer: BddPointer) -> u16 {
        self.nodes[pointer.0 as usize].var
    }

    #[inline]
    pub(crate) fn low_of(&self, node: BddPointer) -> BddPointer {
        self.nodes[node.0 as usize].low
    }

    #[inline]
    pub(crate) fn high_of(&self, node: BddPointer) -> BddPointer {
        self.nodes[node.0 as usize].high
    }

    #[inline]
    pub fn new_node(&mut self, var: u16, low: BddPointer, high: BddPointer) -> BddPointer {
        if low == high {
            return low;
        }
        let node = BddNode { var, high, low };
        if let Some(res) = self.existing.get(&node) {
            return *res;
        }
        self.nodes.push(node.clone());
        let point = BddPointer(self.nodes.len() as u32 - 1);
        assert_matches!(self.existing.insert(node, point), None);
        point
    }
}

impl PeabodyInner {
    pub fn new() -> Self {
        let nodes = vec![BddNode::constant(false), BddNode::constant(true)];
        let existing = HashMap::with_hasher(FxBuildHasher::default());
        Self { nodes, existing }
    }

    pub fn ith_var(&mut self, var: usize) -> BddPointer {
        self.new_node(
            var as _,
            BddPointer::constant(false),
            BddPointer::constant(true),
        )
    }
}

impl Debug for PeabodyInner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.nodes.fmt(f)
    }
}

#[derive(Clone)]
pub struct Peabody {
    inner: Arc<Mutex<PeabodyInner>>,
}

impl Peabody {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(PeabodyInner::new())),
        }
    }

    pub fn ith_var(&self, var: usize) -> Bdd {
        let pointer = self.inner.lock().unwrap().ith_var(var);
        Bdd::new(&self.inner, pointer)
    }
}

impl Debug for Peabody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.lock().unwrap().fmt(f)
    }
}
