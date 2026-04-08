use bytecode::{Chunk, Constant, Encoder, Idx, Instr, Off, Reg};
use crate::{Error, Result};

pub(super) enum JumpPatch {
    Jmp,
    Jf(Reg),
}

pub(super) struct PatchSite {
    // the byte offset of the instruction
    start: usize,
    // the byte offset after the instruction
    base: usize,
    jump: JumpPatch,
}

impl PatchSite {
    fn new(start: usize, base: usize, jump: JumpPatch) -> Self {
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

    pub(super) fn emit(&mut self, instr: Instr) {
        self.encoder.encode(&instr);
    }

    pub(super) fn emit_patch(&mut self, jump: JumpPatch) -> PatchSite {
        let instr = match jump {
            JumpPatch::Jmp => Instr::Jmp { off: 0 }, // use `0` as a temp value
            JumpPatch::Jf(dst) => Instr::Jf { dst, off: 0 },
        };

        let start = self.encoder.len();
        self.emit(instr);
        let base = self.encoder.len();

        // use the offset of the 1st byte AFTER the jump instr was emitted (`base`)
        // as this is where the vm's IP will be at after decoding the instr
        PatchSite::new(start, base, jump)
    }

    pub(super) fn patch(&mut self, site: PatchSite) -> Result<()> {
        let target = self.encoder.len();
        // delta (instr offset) + ip = target
        let delta = target as isize - site.base as isize;
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

        let start = site.start;
        let end = site.start + encoded.len();
        assert_eq!(site.base - start, encoded.len(), "rebuilt instr len != placeholder instr");

        let code = self.encoder.code_mut();
        code[start..end].copy_from_slice(&encoded);

        Ok(())
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
