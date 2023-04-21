use crate::{Bdd, BddPointer, PeabodyInner};
use std::ops::Not;

use super::BddOps;

impl PeabodyInner {
    pub(crate) fn not_rec(&mut self, bdd: BddPointer) -> BddPointer {
        if bdd.is_constant(true) {
            return BddPointer::constant(false);
        }
        if bdd.is_constant(false) {
            return BddPointer::constant(true);
        }
        let bdd_op = BddOps::Not(bdd);
        if let Some(res) = self.ops_cache_get(bdd_op) {
            return res;
        }
        let low = self.not_rec(self.low_of(bdd));
        let high = self.not_rec(self.high_of(bdd));
        let res = self.new_node(self.var_of(bdd), low, high);
        self.ops_cache_set(bdd_op, res);
        res
    }
}

impl Not for Bdd {
    type Output = Bdd;

    fn not(self) -> Self::Output {
        let pointer = self.manager.lock().unwrap().not_rec(self.pointer);
        Bdd::new(&self.manager, pointer)
    }
}

impl Not for &Bdd {
    type Output = Bdd;

    fn not(self) -> Self::Output {
        let pointer = self.manager.lock().unwrap().not_rec(self.pointer);
        Bdd::new(&self.manager, pointer)
    }
}
