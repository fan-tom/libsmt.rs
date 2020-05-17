use std::fmt;

use crate::theories::{array_ex, bitvec, core};
use crate::backends::backend::{Logic, SMTNode};

define_sorts_for_logic!(QF_AUFBV_Sorts,
                        BV -> bitvec::Sorts,
                        Core -> core::Sorts,
                        ArrayEx -> array_ex::Sorts<QF_AUFBV_Sorts, QF_AUFBV_Sorts>
                        );

define_fns_for_logic!(QF_AUFBV_Fn,
                      map { BVOps -> bitvec::OpCodes,
                      CoreOps -> core::OpCodes,
                      ArrayOps -> array_ex::OpCodes<QF_AUFBV_Sorts, QF_AUFBV_Sorts, QF_AUFBV_Fn>
                      },
                      obool {
                          QF_AUFBV_Fn::CoreOps(core::OpCodes::Const(_)) => true,
                          QF_AUFBV_Fn::CoreOps(core::OpCodes::Fun(_)) => true,
                          QF_AUFBV_Fn::CoreOps(core::OpCodes::Not) => true,
                          QF_AUFBV_Fn::CoreOps(core::OpCodes::Imply) => true,
                          QF_AUFBV_Fn::CoreOps(core::OpCodes::And) => true,
                          QF_AUFBV_Fn::CoreOps(core::OpCodes::Or) => true,
                          QF_AUFBV_Fn::CoreOps(core::OpCodes::Xor) => true,
                          QF_AUFBV_Fn::CoreOps(core::OpCodes::Cmp) => true,
                          QF_AUFBV_Fn::CoreOps(core::OpCodes::Distinct) => true,
                          QF_AUFBV_Fn::BVOps(bitvec::OpCodes::BvULt) => true,
                          QF_AUFBV_Fn::BVOps(bitvec::OpCodes::BvULe) => true,
                          QF_AUFBV_Fn::BVOps(bitvec::OpCodes::BvSLt) => true,
                          QF_AUFBV_Fn::BVOps(bitvec::OpCodes::BvSLe) => true,
                          QF_AUFBV_Fn::BVOps(bitvec::OpCodes::BvUGt) => true,
                          QF_AUFBV_Fn::BVOps(bitvec::OpCodes::BvUGe) => true,
                          QF_AUFBV_Fn::BVOps(bitvec::OpCodes::BvSGt) => true,
                          QF_AUFBV_Fn::BVOps(bitvec::OpCodes::BvSGe) => true
                      }
                      );

define_logic!(QF_AUFBV,
              QF_AUFBV_Fn,
              QF_AUFBV_Sorts,
              map { QF_AUFBV_Sorts::BV(_) => bitvec::OpCodes::FreeVar,
                  QF_AUFBV_Sorts::ArrayEx(_) => array_ex::OpCodes::FreeVar,
                  QF_AUFBV_Sorts::Core(_) => core::OpCodes::FreeVar
              }
              );
