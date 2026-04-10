use std::fmt;
use crate::{Encoder, Idx, Instr, Off, Opcode};

#[derive(Clone)]
pub enum Constant {
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(String),
}

impl fmt::Display for Constant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Constant::Int(v) => write!(f, "{}", v),
            Constant::Float(v) => write!(f, "{}", v),
            Constant::Bool(v) => write!(f, "{}", v),
            Constant::Str(v) => write!(f, "{}", v),
        }
    }
}

pub struct Chunk {
    code: Vec<u8>,
    constants: Vec<Constant>,
}

impl Chunk {
    pub fn new(code: Vec<u8>, constants: Vec<Constant>) -> Self {
        Self { code, constants }
    }

    pub fn code(&self) -> &[u8] {
        &self.code
    }

    pub fn constants(&self) -> &[Constant] {
        &self.constants
    }
}

pub enum JumpKind {
    Jmp,
    Jf,
}

pub struct Offset(usize);

pub struct PatchSite {
    // the byte offset of the instr
    op: Offset,
    // the byte offset after the instr
    next: Offset,
}

impl PatchSite {
    fn new(op: Offset, next: Offset) -> Self {
        Self { op, next }
    }
}

fn jump_offset(from: usize, to: usize) -> Off {
    let delta = to as isize - from as isize;
    delta.try_into().expect("jump out of range")
}

#[derive(Default)]
pub struct Builder {
    encoder: Encoder,
    constants: Vec<Constant>,
}

impl Builder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn offset(&self) -> Offset {
        Offset(self.encoder.len())
    }

    pub fn emit(&mut self, instr: Instr) {
        self.encoder.encode(&instr);
    }

    pub fn emit_jump(&mut self, kind: JumpKind, to: Offset) {
        // account for the vm decoding this jump instr being emitted
        let from = self.offset().0 + size_of::<Opcode>() + size_of::<Off>();
        // offset from current vm ip to `to`
        let offset = jump_offset(from, to.0);

        let instr = match kind {
            JumpKind::Jmp => Instr::Jmp(offset),
            JumpKind::Jf => Instr::Jf(offset),
        };

        self.emit(instr);
    }

    pub fn emit_patch(&mut self, kind: JumpKind) -> PatchSite {
        let instr = match kind {
            JumpKind::Jmp => Instr::Jmp(0),
            JumpKind::Jf => Instr::Jf(0),
        };

        let op = self.offset();
        self.emit(instr);
        let next = self.offset();

        PatchSite::new(op, next)
    }

    pub fn patch(&mut self, site: PatchSite) {
        // offset from `site` to `self.offset()`
        let offset = jump_offset(site.next.0, self.offset().0);

        // `Instr::Jmp/Jf` format: [opcode][off]
        let operand = site.op.0 + size_of::<Opcode>();
        self.encoder.patch(operand, &offset.to_be_bytes());
    }

    pub fn constant(&mut self, constant: Constant) -> Idx {
        let idx: Idx = self.constants.len()
            .try_into()
            .expect("constant pool limit exceeded");
        self.constants.push(constant);
        idx
    }

    pub fn build(self) -> Chunk {
        let code = self.encoder.into_code();
        Chunk::new(code, self.constants)
    }
}
