use crate::{Bdd, BddPointer, PeabodyInner};
use std::{cmp::min, collections::HashSet};

#[inline]
pub fn state_is(var: u16, next: bool) -> bool {
    (var & 1 == 0) ^ next
}

#[inline]
pub fn state_to(var: u16, next: bool) -> u16 {
    if next {
        var | 1
    } else {
        var & !1
    }
}

impl PeabodyInner {
    pub fn state_transform_rec(&mut self, bdd: BddPointer, next: bool) -> BddPointer {
        if bdd.is_constant(true) || bdd.is_constant(false) {
            return bdd;
        }
        let var = self.var_of(bdd);
        assert!(state_is(var, !next));
        let low = self.low_of(bdd);
        let high = self.high_of(bdd);
        let low = self.state_transform_rec(low, next);
        let high = self.state_transform_rec(high, next);
        self.new_node(state_to(var, next), low, high)
    }

    pub fn and_exist_rec(
        &mut self,
        left: BddPointer,
        right: BddPointer,
        exist_vars: &HashSet<u16>,
    ) -> BddPointer {
        if left.is_constant(false) || right.is_constant(false) {
            return BddPointer::constant(false);
        }
        if left.is_constant(true) && right.is_constant(true) {
            return BddPointer::constant(true);
        }
        let lv = self.var_of(left);
        let rv = self.var_of(right);
        let decision_var = min(lv, rv);
        let (l_low, l_high) = if lv <= rv {
            (self.low_of(left), self.high_of(left))
        } else {
            (left, left)
        };
        let (r_low, r_high) = if rv <= lv {
            (self.low_of(right), self.high_of(right))
        } else {
            (right, right)
        };
        let low = self.and_exist_rec(l_low, r_low, exist_vars);
        let high = self.and_exist_rec(l_high, r_high, exist_vars);
        if exist_vars.contains(&decision_var) {
            self.or(low, high)
        } else {
            self.new_node(decision_var, low, high)
        }
    }
}

impl Bdd {
    pub fn and_exist<I: Iterator<Item = usize>>(&self, and: &Bdd, exist_vars: I) -> Bdd {
        let exist_vars = HashSet::from_iter(exist_vars.map(|x| x as u16));
        let res =
            self.manager
                .lock()
                .unwrap()
                .and_exist_rec(self.pointer, and.pointer, &exist_vars);
        Bdd::new(&self.manager, res)
    }

    pub fn original_state(&self) -> Self {
        let res = self
            .manager
            .lock()
            .unwrap()
            .state_transform_rec(self.pointer, false);
        Bdd::new(&self.manager, res)
    }

    pub fn next_state(&self) -> Self {
        let res = self
            .manager
            .lock()
            .unwrap()
            .state_transform_rec(self.pointer, true);
        Bdd::new(&self.manager, res)
    }

    pub fn pre_image(&self, trans: &Bdd) -> Self {
        let state = self.next_state();
        state.and_exist(trans, (0..200).filter(|x| x % 2 == 1))
    }

    pub fn post_image(&self, trans: &Bdd) -> Self {
        self.and_exist(trans, (0..200).filter(|x| x % 2 == 0))
            .original_state()
    }
}

#[cfg(test)]
mod tests {
    use crate::Peabody;

    #[test]
    fn test_and_exist() {
        let peabody = Peabody::new();
        let a = peabody.ith_var(0);
        let b = peabody.ith_var(1);
        let and_exist_a = a.and_exist(&b, [0].into_iter());
        assert_eq!(and_exist_a, b);
        let and_exist_b = a.and_exist(&b, [1].into_iter());
        assert_eq!(and_exist_b, a);
    }

    #[test]
    fn test_state() {
        let peabody = Peabody::new();
        let a = peabody.ith_var(0);
        let ap = peabody.ith_var(1);
        let b = peabody.ith_var(2);
        let bp = peabody.ith_var(3);
        let state = a & b;
        let next_state = ap & bp;
        assert_eq!(state.next_state(), next_state);
        assert_eq!(state, next_state.original_state());
    }

    #[test]
    fn test_image() {
        let peabody = Peabody::new();
        let a = peabody.ith_var(0);
        let ap = peabody.ith_var(1);
        let b = peabody.ith_var(2);
        let bp = peabody.ith_var(3);
        let trans = &a & &b & !&ap & !&bp;
        let state = &a & &b;
        let next_state = !&a & !&b;
        assert_eq!(state.clone().post_image(&trans), next_state);
        assert_eq!(state, next_state.pre_image(&trans));
    }
}
