macro_rules! node {
    (
        $node:ident {
            $(
                $variant:ident {
                    $(
                     $field:ident: $ty:ty
                    ),* $(,)?
                }
            ),* $(,)?
        }
    ) => {
        #[derive(Debug)]
        pub enum $node {
            $( $variant($variant) ),*
        }

        $(
            #[derive(Debug)]
            pub struct $variant {
                id: $crate::NodeId,
                $( $field: $ty ),*
            }

            impl $variant {
                pub fn new( id: $crate::NodeId, $( $field: $ty ),* ) -> Self {
                    Self { id, $( $field ),* }
                }

                $(
                    pub fn $field(&self) -> &$ty {
                        &self.$field
                    }
                )*
            }

            impl $crate::Node for $variant {
                fn id(&self) -> $crate::NodeId {
                    self.id
                }
            }

            impl From<$variant> for $node {
                fn from(node: $variant) -> Self {
                    Self::$variant(node)
                }
            }
        )*
    };
}

pub mod types;
pub mod typed;
pub mod untyped;

pub type NodeId = u32;

pub trait Node {
    fn id(&self) -> NodeId;
}
