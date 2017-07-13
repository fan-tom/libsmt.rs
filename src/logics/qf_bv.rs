//! Module that describes QF_BV (closed quatifier-free formulas built over
//! FixedSizeBitVector) logic.
//!
//! Note that the functions and structs that are defined.

use std::fmt;

use theories::{bitvec, core};
use backends::backend::{Logic, SMTNode};

define_sorts_for_logic!(QF_BV_Sorts,
                  BV -> bitvec::Sorts,
                  Core -> core::Sorts
                 );

define_fns_for_logic!(QF_BV_Fn,
                      map { BVOps -> bitvec::OpCodes,
                      CoreOps -> core::OpCodes
                      },
                      obool { QF_BV_Fn::CoreOps(core::OpCodes::True) => true,
                      QF_BV_Fn::CoreOps(core::OpCodes::False) => true,
                      QF_BV_Fn::CoreOps(core::OpCodes::Not) => true,
                      QF_BV_Fn::CoreOps(core::OpCodes::Imply) => true,
                      QF_BV_Fn::CoreOps(core::OpCodes::And) => true,
                      QF_BV_Fn::CoreOps(core::OpCodes::Or) => true,
                      QF_BV_Fn::CoreOps(core::OpCodes::Xor) => true,
                      QF_BV_Fn::CoreOps(core::OpCodes::Cmp) => true,
                      QF_BV_Fn::CoreOps(core::OpCodes::Distinct) => true,
                      QF_BV_Fn::BVOps(bitvec::OpCodes::BvULt) => true,
                      QF_BV_Fn::BVOps(bitvec::OpCodes::BvULe) => true,
                      QF_BV_Fn::BVOps(bitvec::OpCodes::BvSLt) => true,
                      QF_BV_Fn::BVOps(bitvec::OpCodes::BvSLe) => true,
                      QF_BV_Fn::BVOps(bitvec::OpCodes::BvUGt) => true,
                      QF_BV_Fn::BVOps(bitvec::OpCodes::BvUGe) => true,
                      QF_BV_Fn::BVOps(bitvec::OpCodes::BvSGt) => true,
                      QF_BV_Fn::BVOps(bitvec::OpCodes::BvSGe) => true
                      }
                      );

define_logic!(QF_BV,
              QF_BV_Fn,
              QF_BV_Sorts,
              map {
                  QF_BV_Sorts::BV(_) => bitvec::OpCodes::FreeVar
              }
             );
