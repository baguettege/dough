pub(super) trait Encode {
    fn encode(self, buf: &mut Vec<u8>);
}

macro_rules! impl_encode {
    ( $( $ty:ty ),* $(,)? ) => {
        $(
            impl Encode for $ty {
                fn encode(self, buf: &mut Vec<u8>) {
                    buf.extend_from_slice(&self.to_be_bytes());
                }
            }
        )*
    };
}

impl_encode! {
    u8,
    u16,
    u32,
    i16
}