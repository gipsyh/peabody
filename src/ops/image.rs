use crate::{
    apply_with_flip,
    node::{BddNode, BddPointer, BddVariable},
    Bdd,
};
use super::op_function;

impl Bdd {
    pub fn and_rec(left_bdd: &Bdd, left: BddPointer, right_bdd: &Bdd, right: BddPointer) {}
}

impl Bdd {
    pub fn fused_binary_flip_op<T>(
        left: (&Bdd, Option<BddVariable>),
        right: (&Bdd, Option<BddVariable>),
        flip_output: Option<BddVariable>,
        op_function: T,
    ) -> Bdd
    where
        T: Fn(Option<bool>, Option<bool>) -> Option<bool>,
    {
        apply_with_flip(left.0, right.0, left.1, right.1, flip_output, op_function)
    }

    pub fn var_project(&self, variable: BddVariable) -> Bdd {
        Bdd::fused_binary_flip_op((self, None), (self, Some(variable)), None, op_function::or)
    }

    pub fn project(&self, variables: &[BddVariable]) -> Bdd {
        sorted(variables)
            .into_iter()
            .rev()
            .fold(self.clone(), |result, v| result.var_project(v))
    }
}

fn sorted(variables: &[BddVariable]) -> Vec<BddVariable> {
    let mut variables: Vec<BddVariable> = variables.to_vec();
    variables.sort();
    variables
}

impl BddNode {
    #[inline]
    pub fn is_original(&self) -> bool {
        self.var.0 & 1 == 0
    }

    #[inline]
    pub fn is_next(&self) -> bool {
        self.var.0 & 1 == 1
    }

    #[inline]
    pub fn to_original(&mut self) {
        self.var.0 &= !1;
    }

    #[inline]
    pub fn to_next(&mut self) {
        self.var.0 |= 1;
    }
}

impl Bdd {
    #[inline]
    pub fn original_state(mut self) -> Self {
        for node in self.0.iter_mut().skip(2) {
            assert!(node.is_next());
            node.to_original();
        }
        self
    }

    #[inline]
    pub fn next_state(mut self) -> Self {
        for node in self.0.iter_mut().skip(2) {
            assert!(node.is_original());
            node.to_next();
        }
        self
    }

    #[inline]
    pub fn pre_image(self, trans: &Bdd) -> Self {
        let post_state = self.next_state();
        let prod = post_state & trans;
        let vars: Vec<BddVariable> = prod
            .0
            .iter()
            .filter(|node| node.is_next())
            .map(|x| x.var)
            .collect();
        prod.project(&vars)
    }

    #[inline]
    pub fn post_image(self, trans: &Bdd) -> Self {
        let prod = self & trans;
        let vars: Vec<BddVariable> = prod
            .0
            .iter()
            .filter(|node| node.is_original())
            .map(|x| x.var)
            .collect();
        prod.project(&vars).original_state()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state() {
        let a = Bdd::ith_var(0);
        let ap = Bdd::ith_var(1);
        let b = Bdd::ith_var(2);
        let bp = Bdd::ith_var(3);
        let state = a & b;
        let next_state = ap & bp;
        assert_eq!(state.clone().next_state(), next_state);
        assert_eq!(state, next_state.original_state());
    }

    #[test]
    fn test_image() {
        let a = Bdd::ith_var(0);
        let ap = Bdd::ith_var(1);
        let b = Bdd::ith_var(2);
        let bp = Bdd::ith_var(3);
        let trans = &a & &b & !&ap & !&bp;
        let state = &a & &b;
        let next_state = !&a & !&b;
        assert_eq!(state.clone().post_image(&trans), next_state);
        assert_eq!(state, next_state.pre_image(&trans));
    }
}
