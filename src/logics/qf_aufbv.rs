
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
                      obool { }
                      );

define_logic!(QF_AUFBV,
              QF_AUFBV_Fn,
              QF_AUFBV_Sorts,
              map { QF_AUFBV_Sorts::BV(_) => bitvec::OpCodes::FreeVar,
                  QF_AUFBV_Sorts::ArrayEx(_) => array_ex::OpCodes::FreeVar
              }
              );
