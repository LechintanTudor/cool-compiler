mod constraint;

pub use self::constraint::*;
use crate::resolve::BindingId;
use crate::ty::{tys, TyId};
use cool_collections::{id_newtype, IdIndexedVec};
use rustc_hash::FxHashMap;
use std::collections::VecDeque;

id_newtype!(TyVarId);
id_newtype!(ExprId);

#[derive(Default, Debug)]
pub struct ExprTyTable {
    expr_tys: IdIndexedVec<ExprId, TyId>,
    binding_tys: FxHashMap<BindingId, TyId>,
}

impl ExprTyTable {
    #[inline]
    pub fn add_expr(&mut self) -> ExprId {
        self.expr_tys.push(tys::INFERRED_INT)
    }

    #[inline]
    pub fn get_expr_ty(&self, expr_id: ExprId) -> TyId {
        self.expr_tys[expr_id]
    }

    #[inline]
    pub fn get_binding_ty(&self, binding_id: BindingId) -> Option<TyId> {
        self.binding_tys.get(&binding_id).copied()
    }

    pub fn unifier(&mut self) -> ExprTyUnifier {
        ExprTyUnifier {
            expr_ty_table: self,
            ty_vars: Default::default(),
            substitutions: Default::default(),
            constraints: Default::default(),
        }
    }
}

#[derive(Debug)]
pub struct ExprTyUnifier<'a> {
    expr_ty_table: &'a mut ExprTyTable,
    ty_vars: IdIndexedVec<TyVarId, TyId>,
    substitutions: VecDeque<(Constraint, Constraint)>,
    constraints: VecDeque<(Constraint, Constraint)>,
}

impl<'a> ExprTyUnifier<'a> {
    #[inline]
    pub fn add_ty_var(&mut self) -> TyVarId {
        self.ty_vars.push(tys::INFERRED_INT)
    }

    pub fn add_constraint<C1, C2>(&mut self, lhs: C1, rhs: C2)
    where
        C1: Into<Constraint>,
        C2: Into<Constraint>,
    {
        self.constraints.push_back((lhs.into(), rhs.into()));
    }

    pub fn solve_constraints(&mut self) {
        while let Some((lhs, rhs)) = self.constraints.pop_front() {
            if lhs == rhs {
                continue;
            } else if !lhs.is_ty() {
                for (slhs, srhs) in self.substitutions.iter_mut() {
                    slhs.replace_if_eq(&lhs, &rhs);
                    srhs.replace_if_eq(&lhs, &rhs);
                }

                for (clhs, crhs) in self.constraints.iter_mut() {
                    clhs.replace_if_eq(&lhs, &rhs);
                    crhs.replace_if_eq(&lhs, &rhs);
                }

                self.substitutions.push_back((lhs, rhs));
            } else if !rhs.is_ty() {
                for (slhs, srhs) in self.substitutions.iter_mut() {
                    slhs.replace_if_eq(&rhs, &lhs);
                    srhs.replace_if_eq(&rhs, &lhs);
                }

                for (clhs, crhs) in self.constraints.iter_mut() {
                    clhs.replace_if_eq(&rhs, &lhs);
                    crhs.replace_if_eq(&rhs, &lhs);
                }

                self.substitutions.push_back((rhs, lhs));
            } else {
                panic!("Type error");
            }

            let mut substitutions = VecDeque::new();

            while let Some(substitution) = self.substitutions.pop_front() {
                match substitution {
                    (Constraint::Ty(a), Constraint::Ty(b)) => {
                        if a != b {
                            panic!("Type error");
                        }
                    }
                    (Constraint::Binding(binding_id), Constraint::Ty(ty_id)) => {
                        self.expr_ty_table.binding_tys.insert(binding_id, ty_id);
                    }
                    (Constraint::Expr(expr_id), Constraint::Ty(ty_id)) => {
                        self.expr_ty_table.expr_tys[expr_id] = ty_id;
                    }
                    _ => substitutions.push_back(substitution),
                }
            }

            self.substitutions = substitutions;
        }
    }

    pub fn finish(&mut self) {
        self.solve_constraints();
        // TODO: Check the all types are resovled
    }

    #[inline]
    pub fn expr_ty_table(&self) -> &ExprTyTable {
        self.expr_ty_table
    }
}
