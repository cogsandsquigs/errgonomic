use crate::prelude::{CustomError, Error, ErrorKind, Result, State, Underlying};

pub fn prefix_cons_wrapper<
    I: Underlying,
    OOp,
    OExpr,
    E: CustomError,
    F: FnMut(OOp, OExpr) -> std::result::Result<OExpr, E>,
>(
    op: OOp,
    rhs: OExpr,
    mut f: F,
    state: State<I, E>,
) -> Result<I, OExpr, E> {
    f(op, rhs).map(|x| (state.fork(), x)).map_err(|e| {
        let location = state.as_input().fork();
        state.with_error(Error::new(ErrorKind::custom(e), location))
    })
}

pub fn infix_cons_wrapper<
    I: Underlying,
    OOp,
    OExpr,
    E: CustomError,
    F: FnMut(OExpr, OOp, OExpr) -> std::result::Result<OExpr, E>,
>(
    lhs: OExpr,
    op: OOp,
    rhs: OExpr,
    mut f: F,
    state: State<I, E>,
) -> Result<I, OExpr, E> {
    f(lhs, op, rhs).map(|x| (state.fork(), x)).map_err(|e| {
        let location = state.as_input().fork();
        state.with_error(Error::new(ErrorKind::custom(e), location))
    })
}

pub fn postfix_cons_wrapper<
    I: Underlying,
    OOp,
    OExpr,
    E: CustomError,
    F: FnMut(OExpr, OOp) -> std::result::Result<OExpr, E>,
>(
    lhs: OExpr,
    op: OOp,
    mut f: F,
    state: State<I, E>,
) -> Result<I, OExpr, E> {
    f(lhs, op).map(|x| (state.fork(), x)).map_err(|e| {
        let location = state.as_input().fork();
        state.with_error(Error::new(ErrorKind::custom(e), location))
    })
}
