use crate::BddPointer;

mod apply;
mod image;
mod not;

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq)]
pub(crate) enum BddOps {
    Not(BddPointer),
    And(BddPointer, BddPointer),
    Or(BddPointer, BddPointer),
    Xor(BddPointer, BddPointer),
    OriginalState(BddPointer),
    NextState(BddPointer),
    PreImage(BddPointer, BddPointer),
    PostImage(BddPointer, BddPointer),
    AndAbstract(BddPointer, BddPointer, BddPointer),
}
