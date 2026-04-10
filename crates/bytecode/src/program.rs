use std::fmt;
use crate::Chunk;

pub struct Program {
    entry: Chunk,
    funcs: Vec<Chunk>,
}

impl Program {
    pub fn new(entry: Chunk, funcs: Vec<Chunk>) -> Self {
        Self { entry, funcs }
    }

    pub fn entry(&self) -> &Chunk {
        &self.entry
    }

    pub fn funcs(&self) -> &[Chunk] {
        &self.funcs
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "=== entry ===")?;
        write!(f, "{}", self.entry)?;

        for (idx, func) in self.funcs.iter().enumerate() {
            writeln!(f, "\n=== func {idx} ===")?;
            write!(f, "{}", func)?;
        }

        Ok(())
    }
}
