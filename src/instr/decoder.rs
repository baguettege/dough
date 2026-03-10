use std::rc::Rc;
use crate::instr::Instr;

pub(crate) struct Decoder {
    code: Rc<Vec<u8>>,
    offset: usize
}

impl Decoder {
    pub(crate) fn new(code: Rc<Vec<u8>>) -> Self {
        Self { code, offset: 0 }
    }

    pub(crate) fn next_instr(&mut self) -> Instr {
        Instr::decode(self)
    }

    pub(crate) fn has_next(&self) -> bool {
        self.offset < self.code.len()
    }

    pub(crate) fn jump(&mut self, offset: i16) {
        let new_offset = self.offset as isize + offset as isize;
        assert!(new_offset >= 0, "jumped to negative offset");
        self.offset = new_offset as usize;
    }

    pub(super) fn next_u8(&mut self) -> u8 {
        let b = self.code[self.offset];
        self.offset += 1;
        b
    }

    pub(super) fn next_u16(&mut self) -> u16 {
        let hi = self.next_u8() as u16;
        let lo = self.next_u8() as u16;
        (hi << 8) | lo
    }

    pub(super) fn next_u32(&mut self) -> u32 {
        let hi = self.next_u16() as u32;
        let lo = self.next_u16() as u32;
        (hi << 16) | lo
    }

    pub(super) fn next_i16(&mut self) -> i16 {
        self.next_u16() as i16
    }
}

pub(super) trait Decode {
    fn decode(decoder: &mut Decoder) -> Self;
}

macro_rules! impl_decode {
    ($( $ty:ty => $method:ident ),* $(,)?) => {
        $(
            impl Decode for $ty {
                fn decode(decoder: &mut Decoder) -> Self {
                    decoder.$method()
                }
            }
        )*
    };
}

impl_decode! {
    u8 => next_u8,
    u16 => next_u16,
    u32 => next_u32,
    i16 => next_i16
}