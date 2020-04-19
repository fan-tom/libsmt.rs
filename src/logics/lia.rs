//! Module that describes LIA logic.
//!
//! Note that the functions and structs that are defined.

use std::fmt;

use crate::theories::{integer, core};
use crate::backends::backend::{Logic, SMTNode};

define_sorts_for_logic!(LIA_Sorts,
                        Int -> integer::Sorts,
                        Core -> core::Sorts
                        );

define_fns_for_logic!(LIA_Fn,
                      map { IntOps -> integer::OpCodes,
                      CoreOps -> core::OpCodes
                      },
                      obool { LIA_Fn::CoreOps(core::OpCodes::Const(_)) => true,
                      LIA_Fn::CoreOps(core::OpCodes::Not) => true,
                      LIA_Fn::CoreOps(core::OpCodes::Imply) => true,
                      LIA_Fn::CoreOps(core::OpCodes::And) => true,
                      LIA_Fn::CoreOps(core::OpCodes::Or) => true,
                      LIA_Fn::CoreOps(core::OpCodes::Xor) => true,
                      LIA_Fn::CoreOps(core::OpCodes::Cmp) => true,
                      LIA_Fn::CoreOps(core::OpCodes::Distinct) => true,
                      LIA_Fn::IntOps(integer::OpCodes::Lt) => true,
                      LIA_Fn::IntOps(integer::OpCodes::Gt) => true,
                      LIA_Fn::IntOps(integer::OpCodes::Lte) => true,
                      LIA_Fn::IntOps(integer::OpCodes::Gte) => true
                      }
                      );

#[derive(Clone, Copy, Debug)]
pub struct LIA;

impl fmt::Display for LIA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "LIA")
    }
}

impl Logic for LIA {
    type Fns = LIA_Fn;
    type Sorts = LIA_Sorts;

    fn free_var<T: AsRef<str>>(name: T, ty: LIA_Sorts) -> Self::Fns {
        let fv = match ty {
            LIA_Sorts::Int(_) => integer::OpCodes::FreeVar(name.as_ref().to_owned()),
            LIA_Sorts::Core(_) => unreachable!(),
        };
        LIA_Fn::IntOps(fv)
    }
}
