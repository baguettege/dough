use crate::{Result, Error};

pub struct Decoder<'a> {
    code: &'a [u8],
}

impl<'a> Decoder<'a> {
    pub fn new(code: &'a [u8]) -> Self {
        Self { code }
    }
    
    pub fn is_empty(&self) -> bool {
        self.code.is_empty()
    }

    pub fn read<const N: usize>(&mut self) -> Result<[u8; N]> {
        match self.code.split_at_checked(N) {
            None => Err(Error::UnexpectedEof),
            Some((head, tail)) => {
                self.code = tail;
                // unwrap won't panic due to `split_at_checked` already
                // ensuring `head.len() == N`
                Ok(head.try_into().unwrap())
            },
        }
    }

    pub fn decode<T: Decode>(&mut self) -> Result<T> {
        T::decode(self)
    }
}

pub trait Decode: Sized {
    fn decode(decoder: &mut Decoder) -> Result<Self>;
}

macro_rules! impl_decode {
    ($( $ty:ty ),* $(,)?) => {
        $(
            impl Decode for $ty {
                fn decode(decoder: &mut Decoder) -> Result<Self> {
                    const SIZE: usize = size_of::<$ty>();
                    let bytes = decoder.read::<SIZE>()?;
                    Ok(<$ty>::from_be_bytes(bytes))
                }
            }
        )*
    };
}

impl_decode! {
    u8, u16, u32, u64, u128,
    i8, i16, i32, i64, i128,
}
