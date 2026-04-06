#[derive(Default)]
pub struct Encoder {
    code: Vec<u8>,
}

impl Encoder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn write<const N: usize>(&mut self, bytes: &[u8; N]) {
        let bytes = bytes.as_slice();
        self.code.extend_from_slice(bytes);
    }

    pub fn encode<T: Encode>(&mut self, value: &T) {
        value.encode(self);
    }

    pub fn into_code(self) -> Vec<u8> {
        self.code
    }
}

pub trait Encode {
    fn encode(&self, encoder: &mut Encoder);
}

macro_rules! impl_encode {
    ($( $ty:ty ),* $(,)?) => {
        $(
            impl Encode for $ty {
                fn encode(&self, encoder: &mut Encoder) {
                    let bytes = self.to_be_bytes();
                    encoder.write(&bytes);
                }
            }
        )*
    };
}

impl_encode! {
    u8, u16, u32, u64, u128,
    i8, i16, i32, i64, i128,
}
