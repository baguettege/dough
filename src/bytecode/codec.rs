/// Decodes values from a bytecode buffer.
pub(crate) struct Decoder<'a> {
    buf: &'a [u8],
    offset: usize,
}

impl<'a> Decoder<'a> {
    pub(crate) fn new(buf: &'a [u8]) -> Self {
        Self { buf, offset: 0 }
    }

    pub(crate) fn decode<T: Decode>(&mut self) -> Option<T> {
        T::decode(self)
    }

    fn read(&mut self, n: usize) -> Option<&[u8]> {
        let bytes = self.buf.get(self.offset..self.offset + n)?;
        self.offset += n;
        Some(bytes)
    }
}

/// A type that can be decoded from a bytecode buffer.
pub(crate) trait Decode: Sized {
    fn decode(decoder: &mut Decoder) -> Option<Self>;
}

/// Encodes values into a bytecode buffer.
#[derive(Default)]
pub(crate) struct Encoder {
    buf: Vec<u8>,
}

impl Encoder {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn encode<T: Encode>(&mut self, value: &T) {
        value.encode(self);
    }

    fn write(&mut self, bytes: &[u8]) {
        self.buf.extend_from_slice(bytes);
    }

    pub(crate) fn finish(self) -> Vec<u8> {
        self.buf
    }
}

/// A type that can be encoded into a bytecode buffer.
pub(crate) trait Encode: Sized {
    fn encode(&self, encoder: &mut Encoder);
}

macro_rules! impl_codecs {
    ($( $type:ty ),* $(,)?) => {
        $(
            impl Decode for $type {
                fn decode(decoder: &mut Decoder) -> Option<Self> {
                    let bytes = decoder
                        .read(size_of::<$type>())?
                        .try_into()
                        .ok()?;
                    Some(<$type>::from_be_bytes(bytes))
                }
            }

            impl Encode for $type {
                fn encode(&self, encoder: &mut Encoder) {
                    let bytes = self.to_be_bytes();
                    encoder.write(&bytes);
                }
            }
        )*
    };
}

impl_codecs! {
    u8, u16, u32, u64, u128,
    i8, i16, i32, i64, i128,
}
