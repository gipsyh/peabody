use super::op_function;
use crate::*;
use fxhash::FxBuildHasher;
use std::{cmp::min, collections::HashMap};

pub fn apply_with_flip<T>(
    left: &Bdd,
    right: &Bdd,
    flip_left_if: Option<BddVariable>,
    flip_right_if: Option<BddVariable>,
    flip_out_if: Option<BddVariable>,
    terminal_lookup: T,
) -> Bdd
where
    T: Fn(Option<bool>, Option<bool>) -> Option<bool>,
{
    let mut result: Bdd = Bdd::constant(true);
    let mut is_not_empty = false;

    let mut existing: HashMap<BddNode, BddPointer, FxBuildHasher> =
        HashMap::with_capacity_and_hasher(
            left.num_node().max(right.num_node()),
            FxBuildHasher::default(),
        );
    existing.insert(BddNode::mk_constant(false), BddPointer::constant(false));
    existing.insert(BddNode::mk_constant(true), BddPointer::constant(true));

    #[derive(Eq, PartialEq, Hash, Copy, Clone)]
    struct Task {
        left: BddPointer,
        right: BddPointer,
    }

    let mut stack: Vec<Task> = Vec::with_capacity(left.num_node().max(right.num_node()));
    stack.push(Task {
        left: left.root_pointer(),
        right: right.root_pointer(),
    });

    let mut finished: HashMap<Task, BddPointer, FxBuildHasher> = HashMap::with_capacity_and_hasher(
        left.num_node().max(right.num_node()),
        FxBuildHasher::default(),
    );

    while let Some(on_stack) = stack.last() {
        if finished.contains_key(on_stack) {
            stack.pop();
        } else {
            let (l, r) = (on_stack.left, on_stack.right);
            let (l_v, r_v) = (left.var_of(l), right.var_of(r));
            let decision_var = min(l_v, r_v);
            let (l_low, l_high) = if l_v != decision_var {
                (l, l)
            } else if Some(l_v) == flip_left_if {
                (left.high_link_of(l), left.low_link_of(l))
            } else {
                (left.low_link_of(l), left.high_link_of(l))
            };
            let (r_low, r_high) = if r_v != decision_var {
                (r, r)
            } else if Some(r_v) == flip_right_if {
                (right.high_link_of(r), right.low_link_of(r))
            } else {
                (right.low_link_of(r), right.high_link_of(r))
            };
            let comp_low = Task {
                left: l_low,
                right: r_low,
            };
            let comp_high = Task {
                left: l_high,
                right: r_high,
            };
            let new_low = terminal_lookup(l_low.as_bool(), r_low.as_bool())
                .map(BddPointer::from_bool)
                .or_else(|| finished.get(&comp_low).cloned());
            let new_high = terminal_lookup(l_high.as_bool(), r_high.as_bool())
                .map(BddPointer::from_bool)
                .or_else(|| finished.get(&comp_high).cloned());
            if let (Some(new_low), Some(new_high)) = (new_low, new_high) {
                if new_low.is_constant(true) || new_high.is_constant(true) {
                    is_not_empty = true
                }

                if new_low == new_high {
                    finished.insert(*on_stack, new_low);
                } else {
                    let node = if flip_out_if == Some(decision_var) {
                        BddNode::mk_node(decision_var, new_high, new_low)
                    } else {
                        BddNode::mk_node(decision_var, new_low, new_high)
                    };
                    if let Some(index) = existing.get(&node) {
                        finished.insert(*on_stack, *index);
                    } else {
                        result.push_node(node);
                        existing.insert(node, result.root_pointer());
                        finished.insert(*on_stack, result.root_pointer());
                    }
                }
                stack.pop();
            } else {
                if flip_out_if == Some(decision_var) {
                    if new_high.is_none() {
                        stack.push(comp_high);
                    }
                    if new_low.is_none() {
                        stack.push(comp_low);
                    }
                } else {
                    if new_low.is_none() {
                        stack.push(comp_low);
                    }
                    if new_high.is_none() {
                        stack.push(comp_high);
                    }
                }
            }
        }
    }

    if is_not_empty {
        result
    } else {
        Bdd::constant(false)
    }
}

fn apply<T>(left: &Bdd, right: &Bdd, terminal_lookup: T) -> Bdd
where
    T: Fn(Option<bool>, Option<bool>) -> Option<bool>,
{
    apply_with_flip(left, right, None, None, None, terminal_lookup)
}

impl Bdd {
    pub fn and(&self, right: &Bdd) -> Bdd {
        apply(self, right, op_function::and)
    }

    pub fn or(&self, right: &Bdd) -> Bdd {
        apply(self, right, op_function::or)
    }

    pub fn imp(&self, right: &Bdd) -> Bdd {
        apply(self, right, op_function::imp)
    }

    pub fn iff(&self, right: &Bdd) -> Bdd {
        apply(self, right, op_function::iff)
    }

    pub fn xor(&self, right: &Bdd) -> Bdd {
        apply(self, right, op_function::xor)
    }

    pub fn and_not(&self, right: &Bdd) -> Bdd {
        apply(self, right, op_function::and_not)
    }
}

pub struct ApplyContext<'a, T> {
    left: &'a Bdd,
    right: &'a Bdd,
    op: T,
    result: Bdd,
    existing: HashMap<BddNode, BddPointer, FxBuildHasher>,
}

impl<'a, T> ApplyContext<'a, T> {
    pub fn new(left: &'a Bdd, right: &'a Bdd, op: T) -> Self {
        let mut result: Bdd = Bdd::constant(true);
        let mut is_not_empty = false;
        let mut existing: HashMap<BddNode, BddPointer, FxBuildHasher> =
            HashMap::with_capacity_and_hasher(
                left.num_node().max(right.num_node()),
                FxBuildHasher::default(),
            );
        existing.insert(BddNode::mk_constant(false), BddPointer::constant(false));
        existing.insert(BddNode::mk_constant(true), BddPointer::constant(true));
        Self {
            left,
            right,
            op,
            result,
        }
    }

    pub fn apply_rec(&mut self, left: &BddPointer, right: &BddPointer)
    where
        T: Fn(Option<bool>, Option<bool>) -> Option<bool>,
    {
    }
}
