use super::BddOps;
use crate::{Bdd, BddPointer, PeabodyInner};
use std::{
    cmp::min,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign},
};

impl PeabodyInner {
    fn apply_rec<T: ApplyOp>(
        &mut self,
        mut left: BddPointer,
        mut right: BddPointer,
        apply_op: T,
    ) -> BddPointer {
        if let Some(res) = apply_op
            .terminal(left.as_bool(), right.as_bool())
            .map(BddPointer::from_bool)
        {
            return res;
        }
        if left > right {
            (left, right) = (right, left)
        }
        let bdd_op = apply_op.bdd_ops(left, right);
        if let Some(res) = self.ops_cache_get(bdd_op) {
            return res;
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
        let low = self.apply_rec(l_low, r_low, apply_op);
        let high = self.apply_rec(l_high, r_high, apply_op);
        let res = self.new_node(decision_var, low, high);
        self.ops_cache_set(bdd_op, res);
        res
    }
}

trait ApplyOp: Copy {
    fn terminal(&self, l: Option<bool>, r: Option<bool>) -> Option<bool>;

    fn bdd_ops(&self, left: BddPointer, right: BddPointer) -> BddOps;
}

#[derive(Clone, Copy)]
struct AndOp;

impl ApplyOp for AndOp {
    fn terminal(&self, l: Option<bool>, r: Option<bool>) -> Option<bool> {
        match (l, r) {
            (Some(true), Some(true)) => Some(true),
            (Some(false), _) => Some(false),
            (_, Some(false)) => Some(false),
            _ => None,
        }
    }

    fn bdd_ops(&self, left: BddPointer, right: BddPointer) -> BddOps {
        BddOps::And(left, right)
    }
}

#[derive(Clone, Copy)]
struct OrOp;

impl ApplyOp for OrOp {
    fn terminal(&self, l: Option<bool>, r: Option<bool>) -> Option<bool> {
        match (l, r) {
            (Some(false), Some(false)) => Some(false),
            (Some(true), _) => Some(true),
            (_, Some(true)) => Some(true),
            _ => None,
        }
    }

    fn bdd_ops(&self, left: BddPointer, right: BddPointer) -> BddOps {
        BddOps::Or(left, right)
    }
}

#[derive(Clone, Copy)]
struct XorOp;

impl ApplyOp for XorOp {
    fn terminal(&self, l: Option<bool>, r: Option<bool>) -> Option<bool> {
        match (l, r) {
            (Some(l), Some(r)) => Some(l ^ r),
            _ => None,
        }
    }

    fn bdd_ops(&self, left: BddPointer, right: BddPointer) -> BddOps {
        BddOps::Xor(left, right)
    }
}

// pub fn imp(l: Option<bool>, r: Option<bool>) -> Option<bool> {
//     match (l, r) {
//         (Some(true), Some(false)) => Some(false),
//         (Some(false), _) => Some(true),
//         (_, Some(true)) => Some(true),
//         _ => None,
//     }
// }

// pub fn iff(l: Option<bool>, r: Option<bool>) -> Option<bool> {
//     match (l, r) {
//         (Some(l), Some(r)) => Some(l == r),
//         _ => None,
//     }
// }

impl PeabodyInner {
    pub(crate) fn and(&mut self, left: BddPointer, right: BddPointer) -> BddPointer {
        self.apply_rec(left, right, AndOp)
    }

    pub(crate) fn or(&mut self, left: BddPointer, right: BddPointer) -> BddPointer {
        self.apply_rec(left, right, OrOp)
    }

    pub(crate) fn xor(&mut self, left: BddPointer, right: BddPointer) -> BddPointer {
        self.apply_rec(left, right, XorOp)
    }
}

impl<T: AsRef<Bdd>> BitAnd<T> for Bdd {
    type Output = Bdd;

    fn bitand(self, rhs: T) -> Self::Output {
        Bdd::new(
            &self.manager,
            self.manager
                .lock()
                .unwrap()
                .and(self.pointer, rhs.as_ref().pointer),
        )
    }
}

impl<T: AsRef<Bdd>> BitAnd<T> for &Bdd {
    type Output = Bdd;

    fn bitand(self, rhs: T) -> Self::Output {
        Bdd::new(
            &self.manager,
            self.manager
                .lock()
                .unwrap()
                .and(self.pointer, rhs.as_ref().pointer),
        )
    }
}

impl<T: AsRef<Bdd>> BitAndAssign<T> for Bdd {
    fn bitand_assign(&mut self, rhs: T) {
        *self = self.as_ref() & rhs;
    }
}

impl<T: AsRef<Bdd>> BitOr<T> for Bdd {
    type Output = Bdd;

    fn bitor(self, rhs: T) -> Self::Output {
        Bdd::new(
            &self.manager,
            self.manager
                .lock()
                .unwrap()
                .or(self.pointer, rhs.as_ref().pointer),
        )
    }
}

impl<T: AsRef<Bdd>> BitOr<T> for &Bdd {
    type Output = Bdd;

    fn bitor(self, rhs: T) -> Self::Output {
        Bdd::new(
            &self.manager,
            self.manager
                .lock()
                .unwrap()
                .or(self.pointer, rhs.as_ref().pointer),
        )
    }
}

impl<T: AsRef<Bdd>> BitOrAssign<T> for Bdd {
    fn bitor_assign(&mut self, rhs: T) {
        *self = self.as_ref() | rhs;
    }
}

impl<T: AsRef<Bdd>> BitXor<T> for Bdd {
    type Output = Bdd;

    fn bitxor(self, rhs: T) -> Self::Output {
        Bdd::new(
            &self.manager,
            self.manager
                .lock()
                .unwrap()
                .xor(self.pointer, rhs.as_ref().pointer),
        )
    }
}

impl<T: AsRef<Bdd>> BitXor<T> for &Bdd {
    type Output = Bdd;

    fn bitxor(self, rhs: T) -> Self::Output {
        Bdd::new(
            &self.manager,
            self.manager
                .lock()
                .unwrap()
                .xor(self.pointer, rhs.as_ref().pointer),
        )
    }
}

impl<T: AsRef<Bdd>> BitXorAssign<T> for Bdd {
    fn bitxor_assign(&mut self, rhs: T) {
        *self = self.as_ref() ^ rhs;
    }
}

// impl Bdd {
//     pub fn if_then_else(&self, _then: &Bdd, _else: &Bdd) -> Bdd {
//         (self & _then) | (!self & _else)
//     }
// }

#[cfg(test)]
mod tests {
    use crate::Peabody;

    #[test]
    fn test_not() {
        let peabody = Peabody::new();
        let a = peabody.ith_var(0);
        assert_eq!(a, !!&a);
    }

    #[test]
    fn test_and_or() {
        let peabody = Peabody::new();
        let a = peabody.ith_var(0);
        let b = peabody.ith_var(1);
        let and = &a & &b;
        let or = !a | !b;
        assert_eq!(and, !or);
    }
}
