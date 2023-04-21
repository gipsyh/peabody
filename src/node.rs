use crate::PeabodyInner;
use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

#[derive(Clone)]
pub struct Bdd {
    pub(crate) manager: Arc<Mutex<PeabodyInner>>,
    pub(crate) pointer: BddPointer,
}

impl AsRef<Bdd> for Bdd {
    fn as_ref(&self) -> &Bdd {
        self
    }
}

impl PartialEq for Bdd {
    fn eq(&self, other: &Self) -> bool {
        self.pointer == other.pointer
    }
}

impl Eq for Bdd {}

impl Debug for Bdd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.pointer.fmt(f)
    }
}

impl Bdd {
    pub(crate) fn new(manager: &Arc<Mutex<PeabodyInner>>, pointer: BddPointer) -> Self {
        Self {
            manager: manager.clone(),
            pointer,
        }
    }
}

impl Bdd {
    #[inline]
    pub fn is_constant(&self, val: bool) -> bool {
        self.pointer.is_constant(val)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub(crate) struct BddPointer(pub u32);

impl BddPointer {
    #[inline]
    pub fn constant(val: bool) -> Self {
        Self(val as _)
    }

    #[inline]
    pub fn is_constant(&self, val: bool) -> bool {
        *self == Self::constant(val)
    }

    #[inline]
    pub fn as_bool(&self) -> Option<bool> {
        match self.0 {
            0 => Some(false),
            1 => Some(true),
            _ => None,
        }
    }

    #[inline]
    pub fn from_bool(val: bool) -> Self {
        Self::constant(val)
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) struct BddNode {
    pub var: u16,
    pub high: BddPointer,
    pub low: BddPointer,
}

impl BddNode {
    pub fn constant(val: bool) -> Self {
        Self {
            var: u16::MAX,
            low: BddPointer::constant(val),
            high: BddPointer::constant(val),
        }
    }
}
