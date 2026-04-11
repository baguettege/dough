use bytecode::{Instr, Off};
use crate::{Error, Result};

pub(super) struct Cursor<'a> {
    code: &'a [Instr],
    ip: usize,
}

impl<'a> Cursor<'a> {
    pub(super) fn new(code: &'a [Instr]) -> Self {
        Self { code, ip: 0 }
    }

    pub(super) fn next(&mut self) -> Result<Instr> {
        let instr = self.code
            .get(self.ip)
            .copied()
            .ok_or(Error::UnexpectedEof)?;
        self.ip = self.ip
            .checked_add(1)
            .ok_or(Error::IpOverflow)?;
        Ok(instr)
    }

    pub(super) fn jump(&mut self, off: Off) -> Result<()> {
        if off >= 0 {
            self.ip = self.ip
                .checked_add(off as usize)
                .ok_or(Error::JumpOutOfRange)?;
        } else {
            let abs = off.unsigned_abs() as usize;
            self.ip = self.ip
                .checked_sub(abs)
                .ok_or(Error::JumpOutOfRange)?;
        }

        Ok(())
    }
}
