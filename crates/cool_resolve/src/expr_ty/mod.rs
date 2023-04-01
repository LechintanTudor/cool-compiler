use crate::resolve::BindingId;
use crate::ty::TyId;
use cool_collections::id_newtype;

id_newtype!(TyVarId);
id_newtype!(ExprId);

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Constraint {
    TyVar(TyVarId),
    Binding(BindingId),
    Expr(ExprId),
    Ty(TyId),
}

impl Constraint {
    #[inline]
    pub fn is_ty(&self) -> bool {
        matches!(self, Self::Ty(_))
    }

    #[inline]
    pub fn replace_if_eq(&mut self, compared: &Self, replacement: &Self) {
        if self == compared {
            *self = replacement.clone();
        }
    }
}

#[derive(Default, Debug)]
pub struct UnificationTable {
    last_ty_var_id: u32,
    last_expr_id: u32,
    constraints: Vec<(Constraint, Constraint)>,
}

impl UnificationTable {
    #[inline]
    pub fn add_constraint(&mut self, lhs: Constraint, rhs: Constraint) {
        self.constraints.push((lhs, rhs));
    }

    #[inline]
    pub fn gen_ty_var(&mut self) -> TyVarId {
        self.last_ty_var_id += 1;
        TyVarId::new_unwrap(self.last_ty_var_id)
    }

    #[inline]
    pub fn gen_expr(&mut self) -> ExprId {
        self.last_expr_id += 1;
        ExprId::new_unwrap(self.last_expr_id)
    }

    pub fn unify(&mut self) -> Vec<(Constraint, Constraint)> {
        let mut substitutions = Vec::<(Constraint, Constraint)>::new();

        while let Some((lhs, rhs)) = self.constraints.pop() {
            if lhs == rhs {
                continue;
            } else if !lhs.is_ty() {
                for (slhs, srhs) in substitutions.iter_mut() {
                    slhs.replace_if_eq(&lhs, &rhs);
                    srhs.replace_if_eq(&lhs, &rhs);
                }

                for (clhs, crhs) in self.constraints.iter_mut() {
                    clhs.replace_if_eq(&lhs, &rhs);
                    crhs.replace_if_eq(&lhs, &rhs);
                }

                substitutions.push((lhs, rhs));
            } else if !rhs.is_ty() {
                for (slhs, srhs) in substitutions.iter_mut() {
                    slhs.replace_if_eq(&rhs, &lhs);
                    srhs.replace_if_eq(&rhs, &lhs);
                }

                for (clhs, crhs) in self.constraints.iter_mut() {
                    clhs.replace_if_eq(&rhs, &lhs);
                    crhs.replace_if_eq(&rhs, &lhs);
                }

                substitutions.push((rhs, lhs));
            } else {
                panic!("Type error");
            }
        }

        for (lhs, rhs) in substitutions.iter() {
            match (lhs, rhs) {
                (Constraint::Ty(a), Constraint::Ty(b)) => {
                    if a != b {
                        panic!("type error");
                    }
                }
                _ => (),
            }
        }

        self.last_ty_var_id = 0;
        self.constraints.clear();
        substitutions
    }
}

/*
    a := 1_i32 (1);
    b := (a (2) + 2 (3)) (4);

    a = ?A
    a = (1)
    (1) = i32

    b = ?B
    b = (4)
    (4) = (2)
    (4) = (3)
    (2) = a
    (3) = 2
    (2) = (3)
    a = ?C
    2 = number

    ====================================

    subst = [
        a -> ?A
    ]

    ?A = (1)
    (1) = i32

    b = ?B
    b = (4)
    (4) = (2)
    (4) = (3)
    (2) = ?A
    (3) = 2
    (2) = (3)
    ?A = ?C
    2 = number

    ====================================

    subst = [
        a -> ?A
    ]

    ?A = (1)
    (1) = i32

    b = ?B
    b = (4)
    (4) = (2)
    (4) = (3)
    (2) = ?A
    (3) = 2
    (2) = (3)
    ?A = ?C
    2 = number

    ====================================

    subst = [
        a -> (1)
        ?A -> (1)
    ]

    (1) = i32

    b = ?B
    b = (4)
    (4) = (2)
    (4) = (3)
    (2) = (1)
    (3) = 2
    (2) = (3)
    (1) = ?C
    2 = number

    ====================================

    subst = [
        a -> i32
        ?A -> i32
        (1) = i32
    ]

    b = ?B
    b = (4)
    (4) = (2)
    (4) = (3)
    (2) = i32
    (3) = 2
    (2) = (3)
    i32 = ?C
    2 = number

    ====================================

    subst = [
        a -> i32
        ?A -> i32
        (1) = i32
        b = ?B
    ]

    ?B = (4)
    (4) = (2)
    (4) = (3)
    (2) = i32
    (3) = 2
    (2) = (3)
    i32 = ?C
    2 = number

    ====================================

    subst = [
        a -> i32
        ?A -> i32
        (1) = i32
        b = (4)
        ?B = (4)
    ]

    (4) = (2)
    (4) = (3)
    (2) = i32
    (3) = 2
    (2) = (3)
    i32 = ?C
    2 = number

    ====================================

    subst = [
        a -> i32
        ?A -> i32
        (1) = i32
        b = (2)
        ?B = (2)
        (4) = (2)
    ]

    (4) = (2)
    (2) = i32
    (3) = 2
    (2) = (3)
    i32 = ?C
    2 = number

    ====================================

    subst = [
        a -> i32
        ?A -> i32
        (1) = i32
        b = (2)
        ?B = (2)
        (2) = (2)
        (4) = (2)
    ]

    (2) = i32
    (3) = 2
    (2) = (3)
    i32 = ?C
    2 = number

    ====================================

    subst = [
        a -> i32
        ?A -> i32
        (1) = i32
        b = i32
        ?B = i32
        i32 = i32
        (4) = i32
        (2) = i32
    ]

    (3) = 2
    (i32) = (3)
    i32 = ?C
    2 = number

    ====================================

    subst = [
        a -> i32
        ?A -> i32
        (1) = i32
        b = i32
        ?B = i32
        i32 = i32
        (4) = i32
        (2) = i32
        (3) = 2
    ]

    (i32) = 2
    i32 = ?C
    2 = number

    ====================================

    subst = [
        a -> i32
        ?A -> i32
        (1) = i32
        b = i32
        ?B = i32
        i32 = i32
        (4) = i32
        (2) = i32
        (3) = i32
        2 = i32
    ]

    i32 = ?C
    2 = number

    ====================================

    subst = [
        a -> i32
        ?A -> i32
        (1) = i32
        b = i32
        ?B = i32
        i32 = i32
        (4) = i32
        (2) = i32
        (3) = i32
        2 = i32
        ?C = i32
    ]

    2 = number
*/
