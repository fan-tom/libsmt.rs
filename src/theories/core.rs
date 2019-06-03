//! Defines basic operation defined under Core theory in SMTLIB2.

use std::fmt;


use crate::backends::backend::SMTNode;

#[derive(Clone, Debug)]
pub enum OpCodes {
    True,
    False,
    Not,
    Imply,
    And,
    Or,
    Xor,
    Cmp,
    Distinct,
    ITE,
    FreeVar(String)
}

impl fmt::Display for OpCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            OpCodes::Not => "not".to_owned(),
            OpCodes::Imply => "=>".to_owned(),
            OpCodes::And => "and".to_owned(),
            OpCodes::Or => "or".to_owned(),
            OpCodes::Xor => "xor".to_owned(),
            OpCodes::Cmp => "=".to_owned(),
            OpCodes::Distinct => "distinct".to_owned(),
            OpCodes::ITE => "ite".to_owned(),
            OpCodes::True => "true".to_owned(),
            OpCodes::False => "false".to_owned(),
            OpCodes::FreeVar(ref s) => s.clone(),
        };
        write!(f, "{}", s)
    }
}

impl_smt_node!(OpCodes, define vars [OpCodes::FreeVar(_)], define consts []);

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Sorts {
    Bool
}

impl fmt::Display for Sorts {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "bool")
    }
}
