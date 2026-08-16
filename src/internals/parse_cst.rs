pub enum ExprOperator {
    Plus, Minus, Times, Divide,
    IsEqual, IsLesser, IsGreater, IsLessEq, IsGreatEq
}

pub struct Equation {
    number: isize,
    operator: Option<ExprOperator>,
}

pub enum Expression {

}

pub enum Statement {
    If{
        is_else: bool,
        lhs: Vec<Equation>,
		operator: ExprOperator,
        rhs: Vec<Equation>,
    },

    FuncDecl{
        // ...
    },
}
