macro_rules! ast {
    (
        $ident:ident {
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
        pub enum $ident {
            $( $variant($variant) ),*
        }

        $(
            #[derive(Debug)]
            pub struct $variant {
                id: dough::node::NodeId,
                $( $field: $ty ),*
            }

            impl $variant {
                pub fn new( id: dough::node::NodeId, $( $field: $ty ),* ) -> Self {
                    Self { id, $( $field ),* }
                }

                $(
                    pub fn $field(&self) -> &$ty {
                        &self.$field
                    }
                )*
            }

            impl dough::node::Node for $variant {
                fn id(&self) -> dough::node::NodeId {
                    self.id
                }
            }

            impl From<$variant> for $ident {
                fn from(value: $variant) -> Self {
                    $ident::$variant(value)
                }
            }
        )*
    };
}

pub(crate) use ast;
