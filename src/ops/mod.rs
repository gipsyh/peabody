mod apply;
pub use apply::*;

use crate::Bdd;
use std::ops::{BitAnd, BitOr, BitXor, Not};

impl Not for Bdd {
    type Output = Self;

    fn not(mut self) -> Self::Output {
        if self.is_constant(true) {
            Bdd::constant(true)
        } else if self.is_constant(false) {
            Bdd::constant(false)
        } else {
            for node in self.0.iter_mut().skip(2) {
                node.high_link.flip_if_terminal();
                node.low_link.flip_if_terminal();
            }
            self
        }
    }
}

impl Not for &Bdd {
    type Output = Bdd;

    fn not(self) -> Self::Output {
        !self.clone()
    }
}

impl<T: AsRef<Bdd>> BitAnd<T> for Bdd {
    type Output = Bdd;

    fn bitand(self, rhs: T) -> Self::Output {
        self.and(rhs.as_ref())
    }
}

impl<T: AsRef<Bdd>> BitAnd<T> for &Bdd {
    type Output = Bdd;

    fn bitand(self, rhs: T) -> Self::Output {
        self.and(rhs.as_ref())
    }
}

impl<T: AsRef<Bdd>> BitOr<T> for Bdd {
    type Output = Bdd;

    fn bitor(self, rhs: T) -> Self::Output {
        self.or(rhs.as_ref())
    }
}

impl<T: AsRef<Bdd>> BitOr<T> for &Bdd {
    type Output = Bdd;

    fn bitor(self, rhs: T) -> Self::Output {
        self.or(rhs.as_ref())
    }
}

impl<T: AsRef<Bdd>> BitXor<T> for Bdd {
    type Output = Bdd;

    fn bitxor(self, rhs: T) -> Self::Output {
        self.xor(rhs.as_ref())
    }
}

impl<T: AsRef<Bdd>> BitXor<T> for &Bdd {
    type Output = Bdd;

    fn bitxor(self, rhs: T) -> Self::Output {
        self.xor(rhs.as_ref())
    }
}
