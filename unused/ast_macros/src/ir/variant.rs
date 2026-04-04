use crate::ir::Field;

crate::util::record! {
    Variant {
        ident: syn::Ident,
        fields: Vec<Field>,
    }
}

impl TryFrom<syn::Variant> for Variant {
    type Error = syn::Error;
    
    fn try_from(variant: syn::Variant) -> Result<Self, Self::Error> {
        let ident = variant.ident;
        let fields = variant.fields
            .into_iter()
            .map(Field::try_from)
            .collect::<Result<Vec<_>, _>>()?;
        
        Ok(Variant::new(ident, fields))
    }
}
