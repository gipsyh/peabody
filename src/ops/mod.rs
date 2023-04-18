mod apply;
pub use apply::*;

mod image;
pub use image::*;

mod op_function;

use crate::Bdd;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

impl Not for Bdd {
    type Output = Self;

    fn not(mut self) -> Self::Output {
        if self.is_constant(true) {
            Bdd::constant(false)
        } else if self.is_constant(false) {
            Bdd::constant(true)
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

impl<T: AsRef<Bdd>> BitAndAssign<T> for Bdd {
    fn bitand_assign(&mut self, rhs: T) {
        *self = self.as_ref() & rhs;
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

impl<T: AsRef<Bdd>> BitOrAssign<T> for Bdd {
    fn bitor_assign(&mut self, rhs: T) {
        *self = self.as_ref() | rhs;
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

impl<T: AsRef<Bdd>> BitXorAssign<T> for Bdd {
    fn bitxor_assign(&mut self, rhs: T) {
        *self = self.as_ref() ^ rhs;
    }
}

impl Bdd {
    pub fn if_then_else(&self, _then: &Bdd, _else: &Bdd) -> Bdd {
        (self & _then) | (!self & _else)
    }
}
