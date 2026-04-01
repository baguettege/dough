crate::util::record! {
    Field {
        ident: syn::Ident,
        ty: syn::Type,
        is_copy: bool,
    }
}

impl TryFrom<syn::Field> for Field {
    type Error = syn::Error;

    fn try_from(field: syn::Field) -> Result<Self, Self::Error> {
        let ident = field.ident
            .ok_or_else(|| syn::Error::new_spanned(
            &field.ty, "expected named field"))?;
        let ty = field.ty;
        let is_copy = field.attrs.iter().any(|a| a.path().is_ident("copy"));

        Ok(Field::new(ident, ty, is_copy))
    }
}
