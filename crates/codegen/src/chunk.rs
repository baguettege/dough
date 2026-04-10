use bytecode::{Chunk, Constant, Idx, Instr, Off};

pub enum JumpKind {
    Jmp,
    Jf,
}

#[derive(Copy, Clone)]
pub struct Offset(usize);

pub struct PatchSite {
    offset: Offset,
    kind: JumpKind,
}

impl PatchSite {
    fn new(offset: Offset, kind: JumpKind) -> Self {
        Self { offset, kind }
    }
}

fn off(from: Offset, to: Offset) -> Off {
    (to.0 as isize - from.0 as isize)
        .try_into()
        .expect("jump offset out of range")
}

#[derive(Default)]
pub struct Builder {
    code: Vec<Instr>,
    constants: Vec<Constant>,
}

impl Builder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn offset(&self) -> Offset {
        Offset(self.code.len())
    }

    pub fn emit(&mut self, instr: Instr) {
        self.code.push(instr);
    }

    pub fn emit_jump(&mut self, kind: JumpKind, to: Offset) {
        let from = Offset(self.offset().0 + 1);
        let off = off(from, to);

        let instr = match kind {
            JumpKind::Jmp => Instr::Jmp(off),
            JumpKind::Jf => Instr::Jf(off),
        };

        self.emit(instr);
    }

    pub fn emit_patch(&mut self, kind: JumpKind) -> PatchSite {
        let instr = match kind {
            JumpKind::Jmp => Instr::Jmp(0),
            JumpKind::Jf => Instr::Jf(0),
        };

        let offset = self.offset();
        self.emit(instr);

        PatchSite::new(offset, kind)
    }

    pub fn patch(&mut self, site: PatchSite) {
        let from = Offset(site.offset.0 + 1);
        let to = self.offset();
        let off = off(from, to);

        let instr = match site.kind {
            JumpKind::Jmp => Instr::Jmp(off),
            JumpKind::Jf => Instr::Jf(off),
        };

        self.code[site.offset.0] = instr;
    }

    pub fn constant(&mut self, constant: Constant) -> Idx {
        let idx: Idx = self.constants.len()
            .try_into()
            .expect("constant pool limit exceeded");
        self.constants.push(constant);
        idx
    }

    pub fn build(self) -> Chunk {
        Chunk::new(self.code, self.constants)
    }
}
