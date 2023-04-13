use crate::*;
use fxhash::FxBuildHasher;
use std::{cmp::min, collections::HashMap};

fn apply_with_flip<T>(
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

            // Determine which variable we are conditioning on, moving from smallest to largest.
            let (l_v, r_v) = (left.var_of(l), right.var_of(r));
            let decision_var = min(l_v, r_v);

            // If the variable is the same as in the left/right decision node,
            // advance the exploration there. Otherwise, keep the pointers the same.
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

            // Two tasks which correspond to the two recursive sub-problems we need to solve.
            let comp_low = Task {
                left: l_low,
                right: r_low,
            };
            let comp_high = Task {
                left: l_high,
                right: r_high,
            };

            // Try to solve the tasks using terminal lookup table or from cache.
            let new_low = terminal_lookup(l_low.as_bool(), r_low.as_bool())
                .map(BddPointer::from_bool)
                .or_else(|| finished.get(&comp_low).cloned());
            let new_high = terminal_lookup(l_high.as_bool(), r_high.as_bool())
                .map(BddPointer::from_bool)
                .or_else(|| finished.get(&comp_high).cloned());

            // If both values are computed, mark this task as resolved.
            if let (Some(new_low), Some(new_high)) = (new_low, new_high) {
                if new_low.is_constant(true) || new_high.is_constant(true) {
                    is_not_empty = true
                }

                if new_low == new_high {
                    // There is no decision, just skip this node and point to either child.
                    finished.insert(*on_stack, new_low);
                } else {
                    // There is a decision here.
                    let node = if flip_out_if == Some(decision_var) {
                        BddNode::mk_node(decision_var, new_high, new_low)
                    } else {
                        BddNode::mk_node(decision_var, new_low, new_high)
                    };
                    if let Some(index) = existing.get(&node) {
                        // Node already exists, just make it a result of this computation.
                        finished.insert(*on_stack, *index);
                    } else {
                        // Node does not exist, it needs to be pushed to result.
                        result.push_node(node);
                        existing.insert(node, result.root_pointer());
                        finished.insert(*on_stack, result.root_pointer());
                    }
                }
                stack.pop(); // Mark as resolved.
            } else {
                // Otherwise, if either value is unknown, push it to the stack.
                if flip_out_if == Some(decision_var) {
                    // If we are flipping output, we have to compute subtasks in the right order.
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

mod op_function {
    pub fn and(l: Option<bool>, r: Option<bool>) -> Option<bool> {
        match (l, r) {
            (Some(true), Some(true)) => Some(true),
            (Some(false), _) => Some(false),
            (_, Some(false)) => Some(false),
            _ => None,
        }
    }

    pub fn or(l: Option<bool>, r: Option<bool>) -> Option<bool> {
        match (l, r) {
            (Some(false), Some(false)) => Some(false),
            (Some(true), _) => Some(true),
            (_, Some(true)) => Some(true),
            _ => None,
        }
    }

    pub fn imp(l: Option<bool>, r: Option<bool>) -> Option<bool> {
        match (l, r) {
            (Some(true), Some(false)) => Some(false),
            (Some(false), _) => Some(true),
            (_, Some(true)) => Some(true),
            _ => None,
        }
    }

    pub fn iff(l: Option<bool>, r: Option<bool>) -> Option<bool> {
        match (l, r) {
            (Some(l), Some(r)) => Some(l == r),
            _ => None,
        }
    }

    pub fn xor(l: Option<bool>, r: Option<bool>) -> Option<bool> {
        match (l, r) {
            (Some(l), Some(r)) => Some(l ^ r),
            _ => None,
        }
    }

    pub fn and_not(l: Option<bool>, r: Option<bool>) -> Option<bool> {
        match (l, r) {
            (Some(false), _) => Some(false),
            (_, Some(true)) => Some(false),
            (Some(true), Some(false)) => Some(true),
            _ => None,
        }
    }
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
