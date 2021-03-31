use std::fmt::{Debug, Error, Formatter};

#[derive(Copy, Clone)]
pub enum Operator {
    // Calculation operators.
    MulDot,
    DivDot,
    Plus,
    PlusDot,
    Minus,
    MinusDot,

    // Comparison operators.
    Greater,
    GreaterEqual,
    LesserGreater,
    Lesser,
    LesserEqual,
    // Looks like mincaml only has structural equality.
    Equal,

    Not,
}

pub enum Exp {
    UnaryExp(Operator, Box<Exp>),
    BinaryExp(Operator, Box<Exp>, Box<Exp>),
    SimpleExp,
    IfExp(Box<Exp>, Box<Exp>, Box<Exp>),
    Eval(Box<Exp>, Box<Exp>),
    Assignment(Vec<String>, Box<Exp>),
    FunctionDef(String, Vec<String>, Box<Exp>),
    ExpressionSequence(Box<Exp>, Box<Exp>),
    Args(Box<Exp>, Box<Exp>),
    Elems(Box<Exp>, Box<Exp>),
    ArrayCreate(Box<Exp>, Box<Exp>),
    ArrayAssignment(Box<Exp>, Box<Exp>),
    Index(Box<Exp>, Box<Exp>),
    NumberExp(Number),
    Bool(bool),
    Identifier(String),

    Unit,
    Unimplemented,
}

#[derive(Debug)]
pub enum SimpleExp {
    // Exp(),
// Identifier(),
// ArrayAccess(),
}

pub enum Number {
    Floating(f32),
    Integer(i32),
}

impl Debug for Exp {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match *self {
            Exp::UnaryExp(op, ref l) => write!(fmt, "({:?} {:?})", op, l),
            Exp::BinaryExp(op, ref l, ref r) => write!(fmt, "({:?} {:?} {:?})", l, op, r),
            Exp::NumberExp(ref n) => write!(fmt, "{:?}", n),
            _ => write!(fmt, "not implemented yet"),
        }
    }
}

impl Debug for Number {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match *self {
            Number::Floating(f) => write!(fmt, "{:?}", f),
            Number::Integer(n) => write!(fmt, "{:?}", n),
        }
    }
}

impl Debug for Operator {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Operator::*;
        match *self {
            MulDot => write!(fmt, "*."),
            DivDot => write!(fmt, "/."),
            Plus => write!(fmt, "+"),
            PlusDot => write!(fmt, "+."),
            Minus => write!(fmt, "-"),
            MinusDot => write!(fmt, "-."),
            Greater => write!(fmt, ">"),
            GreaterEqual => write!(fmt, ">="),
            LesserGreater => write!(fmt, "<>"),
            Lesser => write!(fmt, "<"),
            LesserEqual => write!(fmt, "<="),
            Equal => write!(fmt, "="),
            Not => write!(fmt, "not"),
        }
    }
}
