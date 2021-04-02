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
            Exp::IfExp(ref e1, ref e2, ref e3) => {
                write!(fmt, "(if ({:?}) then ({:?}) else ({:?}))", e1, e2, e3)
            }
            Exp::Eval(ref e1, ref e2) => write!(fmt, "({:?} in {:?})", e1, e2),
            Exp::Assignment(ref e1, ref e2) => {
                write!(fmt, "(let {:?} = {:?})", e1, e2)
            }
            Exp::FunctionDef(ref f, ref args, ref e) => {
                write!(fmt, "({:?}  ({:?}) return ({:?}) )", f, args, e)
            }
            Exp::ExpressionSequence(ref e1, ref e2) => write!(fmt, "({:?} {:?})", e1, e2),
            Exp::Args(ref e1, ref e2) => write!(fmt, "({:?} {:?})", e1, e2),
            Exp::Elems(ref e1, ref e2) => write!(fmt, "({:?} {:?})", e1, e2),
            Exp::ArrayCreate(ref e1, ref e2) => {
                write!(fmt, "(arr.create length={:?}  init={:?})", e1, e2)
            }
            Exp::ArrayAssignment(ref e1, ref e2) => {
                write!(fmt, "({:?}={:?})", e1, e2)
            }
            Exp::Index(ref e1, ref e2) => {
                write!(fmt, "({:?}[{:?}])", e1, e2)
            }
            Exp::Bool(b) => write!(fmt, "({:?})", b),
            Exp::Identifier(ref s) => write!(fmt, "({})", s),
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
