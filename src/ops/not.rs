use crate::{Bdd, BddPointer, PeabodyInner};
use std::ops::Not;

impl PeabodyInner {
    pub(crate) fn not_rec(&mut self, bdd: BddPointer) -> BddPointer {
        if bdd.is_constant(true) {
            return BddPointer::constant(false);
        }
        if bdd.is_constant(false) {
            return BddPointer::constant(true);
        }
        let low = self.not_rec(self.low_of(bdd));
        let high = self.not_rec(self.high_of(bdd));
        self.new_node(self.var_of(bdd), low, high)
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
