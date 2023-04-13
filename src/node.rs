#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BddVariable(pub u16);

impl From<usize> for BddVariable {
    fn from(value: usize) -> Self {
        Self(value as _)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BddPointer(pub u32);

impl From<usize> for BddPointer {
    fn from(value: usize) -> Self {
        Self(value as _)
    }
}

impl BddPointer {
    pub fn as_bool(&self) -> Option<bool> {
        match self.0 {
            0 => Some(false),
            1 => Some(true),
            _ => None,
        }
    }

    pub fn from_bool(value: bool) -> BddPointer {
        if value {
            BddPointer::constant(true)
        } else {
            BddPointer::constant(false)
        }
    }

    pub fn constant(val: bool) -> BddPointer {
        BddPointer(val as _)
    }

    pub fn is_constant(&self, val: bool) -> bool {
        self.0 == val as _
    }

    pub fn flip_if_terminal(&mut self) {
        if self.0 < 2 {
            if self.0 == 0 {
                self.0 = 1
            } else if self.0 == 1 {
                self.0 = 0
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct BddNode {
    pub var: BddVariable,
    pub low_link: BddPointer,
    pub high_link: BddPointer,
}

impl BddNode {
    pub fn mk_constant(val: bool) -> Self {
        Self {
            var: BddVariable(u16::MAX),
            low_link: BddPointer::constant(val),
            high_link: BddPointer::constant(val),
        }
    }

    pub fn mk_node(var: BddVariable, low_link: BddPointer, high_link: BddPointer) -> Self {
        Self {
            var,
            low_link,
            high_link,
        }
    }
}
