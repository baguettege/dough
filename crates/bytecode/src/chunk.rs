use crate::Instr;
use std::fmt::{Display, Formatter, Result};

#[derive(Clone)]
pub enum Constant {
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(String),
}

impl Display for Constant {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Constant::Int(v) => write!(f, "{}", v),
            Constant::Float(v) => write!(f, "{}", v),
            Constant::Bool(v) => write!(f, "{}", v),
            Constant::Str(v) => write!(f, "\"{}\"", v),
        }
    }
}

pub struct Chunk {
    code: Vec<Instr>,
    constants: Vec<Constant>,
}

impl Chunk {
    pub fn new(code: Vec<Instr>, constants: Vec<Constant>) -> Self {
        Self { code, constants }
    }

    pub fn code(&self) -> &[Instr] {
        &self.code
    }

    pub fn constants(&self) -> &[Constant] {
        &self.constants
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if self.constants.is_empty() {
            writeln!(f, "consts: (none)")?;
        } else {
            writeln!(f, "consts:")?;
            for (idx, constant) in self.constants.iter().enumerate() {
                writeln!(f, "  [{idx}] {constant}")?;
            }
        }

        for instr in self.code.iter() {
            writeln!(f, "  {instr}")?;
        }

        Ok(())
    }
}
