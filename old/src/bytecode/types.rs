/// An instruction opcode.
pub(crate) type Opcode = u8;

/// A register index.
pub(crate) type Reg = u8;

/// An index.
pub(crate) type Idx = u16;

/// A signed jump offset, relative to the current instruction.
pub(crate) type Off = i16;

/// An array length.
pub(crate) type Len = u32;

/// An argument count.
pub(crate) type Argc = u8;