use std::fmt::{Display, Formatter};
use crate::{Chunk, Constant, Decoder, Instr, Result};

pub struct Disasm {
    instrs: Vec<Instr>,
    constants: Vec<Constant>,
}

impl Disasm {
    fn new(instrs: Vec<Instr>, constants: Vec<Constant>) -> Self {
        Self { instrs, constants }
    }

    pub fn instrs(&self) -> &[Instr] {
        &self.instrs
    }

    pub fn constants(&self) -> &[Constant] {
        &self.constants
    }
}

impl Display for Disasm {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.constants.is_empty() {
            writeln!(f, "consts: (none)")?;
        } else {
            writeln!(f, "consts:")?;
            for (idx, constant) in self.constants.iter().enumerate() {
                writeln!(f, "  [{idx}] {constant}")?;
            }
        }

        let mut offset = 0usize;
        for instr in self.instrs.iter() {
            writeln!(f, "  {:04x} {instr}", offset)?;
            offset += instr.size();
        }

        Ok(())
    }
}

pub fn disasm(chunk: &Chunk) -> Result<Disasm> {
    let mut decoder = Decoder::new(chunk.code());

    let mut instrs = Vec::new();
    let constants = chunk.constants().to_vec();

    while !decoder.is_empty() {
        let instr = decoder.decode::<Instr>()?;
        instrs.push(instr);
    }

    Ok(Disasm::new(instrs, constants))
}

pub struct Program {
    entry: Disasm,
    funcs: Vec<Disasm>,
}

impl Program {
    fn new(entry: Disasm, funcs: Vec<Disasm>) -> Self {
        Self { entry, funcs }
    }

    pub fn entry(&self) -> &Disasm {
        &self.entry
    }

    pub fn funcs(&self) -> &[Disasm] {
        &self.funcs
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "=== entry ===")?;
        write!(f, "{}", self.entry)?;

        for (idx, func) in self.funcs.iter().enumerate() {
            writeln!(f, "\n=== func {idx} ===")?;
            write!(f, "{}", func)?;
        }

        Ok(())
    }
}

pub fn disasm_program(program: &crate::Program) -> Result<Program> {
    let entry = disasm(program.entry())?;
    let funcs = program.funcs()
        .iter()
        .map(|func| disasm(func))
        .collect::<Result<Vec<_>>>()?;

    Ok(Program::new(entry, funcs))
}
