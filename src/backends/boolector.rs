//! Struct and methods to interact with the boolector solver.

use crate::backends::smtlib2::SMTProc;
use std::process::{Child, Command, Stdio};

#[derive(Default)]
pub struct Boolector {
    fd: Option<Child>,
}

// Add boolector binary path to $PATH
impl SMTProc for Boolector {
    fn init(&mut self) {
        let (cmd, args) = ("boolector", &["-m", "--smt2-model", "--smt2"]);
        let child = Command::new(cmd)
                        .args(args)
                        .stdout(Stdio::piped())
                        .stdin(Stdio::piped())
                        .stderr(Stdio::piped())
                        .spawn()
                        .expect("Failed to spawn process");
        self.fd = Some(child);
    }

    fn pipe<'a>(&'a mut self) -> &'a mut Child {
        if self.fd.is_none() {
            self.init();
        }
        self.fd.as_mut().unwrap()
    }
}

#[cfg(test)]
mod test {
    use crate::backends::backend::*;
    use crate::backends::smtlib2::*;
    use super::*;
    use crate::theories::bitvec;
    use crate::theories::{core};
    use crate::logics::{qf_bv};

    #[test]
    fn test_bl_bitvec() {
        let mut bl: Boolector = Default::default();
        let mut solver = SMTLib2::new(Some(qf_bv::QF_BV));
        let x = solver.new_var(Some("X"), bitvec::Sorts::BitVector(32));
        let c = solver.new_const(bitvec::OpCodes::Const(10, 32));
        let c8 = solver.new_const(bitvec::OpCodes::Const(8, 32));
        let y = solver.new_var(Some("Y"), bitvec::Sorts::BitVector(32));
        solver.assert(core::OpCodes::Cmp, &[x, c]);
        let x_xor_y = solver.assert(bitvec::OpCodes::BvXor, &[x, y]);
        solver.assert(core::OpCodes::Cmp, &[x_xor_y, c8]);
        let result = solver.solve(&mut bl, false).0.unwrap();
        assert_eq!(result[&x], 10);
        assert_eq!(result[&y], 2);
    }
}
