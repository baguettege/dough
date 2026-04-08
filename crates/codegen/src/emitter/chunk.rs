use bytecode::{Chunk, Constant, Encoder, Idx, Instr, Off, Reg};
use crate::{Error, Result};

#[derive(Copy, Clone)]
pub(super) struct Offset(usize);

pub(super) enum JumpPatch {
    Jmp,
    Jf(Reg),
}

pub(super) struct PatchSite {
    // the byte offset of the instruction
    start: Offset,
    // the byte offset after the instruction
    base: Offset,
    jump: JumpPatch,
}

impl PatchSite {
    fn new(start: Offset, base: Offset, jump: JumpPatch) -> Self {
        Self { start, base, jump }
    }
}

#[derive(Default)]
pub(super) struct Builder {
    encoder: Encoder,
    constants: Vec<Constant>,
}

impl Builder {
    pub(super) fn new() -> Self {
        Self::default()
    }

    pub(super) fn offset(&self) -> Offset {
        Offset(self.encoder.len())
    }

    pub(super) fn emit(&mut self, instr: Instr) {
        self.encoder.encode(&instr);
    }

    pub(super) fn emit_jump_to(&mut self, jump: JumpPatch, target: Offset) -> Result<()> {
        // emits a jump whose target is already known
        let site = self.emit_patch(jump);
        self.patch_to(site, target)
    }

    pub(super) fn emit_patch(&mut self, jump: JumpPatch) -> PatchSite {
        // emits a placeholder jump and returns the data needed to rewrite it later

        let instr = match jump {
            JumpPatch::Jmp => Instr::Jmp { off: 0 }, // use `0` as a temp value
            JumpPatch::Jf(dst) => Instr::Jf { dst, off: 0 },
        };

        let start = self.offset();
        self.emit(instr);
        let base = self.offset();

        // use the offset of the 1st byte AFTER the jump instr was emitted (`base`)
        // as this is where the vm's IP will be at after decoding the instr
        PatchSite::new(start, base, jump)
    }

    pub(super) fn patch(&mut self, site: PatchSite) -> Result<()> {
        // rewrites a placeholder jump so it lands at the current offset,
        // which would end up being the next instr written
        self.patch_to(site, self.offset())
    }

    pub(super) fn constant(&mut self, constant: Constant) -> Result<Idx> {
        let idx: Idx = self.constants
            .len()
            .try_into()
            .map_err(|_| Error::OutOfIndices)?;
        self.constants.push(constant);
        Ok(idx)
    }

    pub(super) fn build(self, local_count: usize) -> Chunk {
        let code = self.encoder.into_code();
        Chunk::new(code, self.constants, local_count)
    }
}

impl Builder {
    fn patch_to(&mut self, site: PatchSite, target: Offset) -> Result<()> {
        // rewrites a previously emitted placeholder jump so it lands at `target`

        // delta (instr offset) + ip = target
        let delta = target.0 as isize - site.base.0 as isize;
        let off: Off = delta
            .try_into()
            .map_err(|_| Error::JumpOutOfRange)?;
        let instr = match site.jump {
            JumpPatch::Jmp => Instr::Jmp { off },
            JumpPatch::Jf(dst) => Instr::Jf { dst, off },
        };

        let mut encoder = Encoder::new();
        encoder.encode(&instr);
        let encoded = encoder.into_code();

        let start = site.start.0;
        let end = start + encoded.len();
        assert_eq!(site.base.0 - start, encoded.len(), "rebuilt instr len != placeholder instr len");

        let code = self.encoder.code_mut();
        code[start..end].copy_from_slice(&encoded);

        Ok(())
    }
}
