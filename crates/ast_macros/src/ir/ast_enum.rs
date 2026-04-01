use crate::ir::variant::Variant;

crate::util::record! {
    AstEnum {
        ident: syn::Ident,
        variants: Vec<Variant>,
    }
}

impl TryFrom<syn::ItemEnum> for AstEnum {
    type Error = syn::Error;

    fn try_from(item: syn::ItemEnum) -> Result<Self, Self::Error> {
        let ident = item.ident;
        let variants = item.variants
            .into_iter()
            .map(Variant::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(AstEnum::new(ident, variants))
    }
}
