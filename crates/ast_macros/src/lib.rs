mod util;
mod ir;
mod expand;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn ast(attr: TokenStream, item: TokenStream) -> TokenStream {
    ast_inner(attr, item).unwrap_or_else(
        |err| err.to_compile_error().into())
}

fn ast_inner(attr: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
    let attr = proc_macro2::TokenStream::from(attr);
    if !attr.is_empty() {
        return Err(syn::Error::new_spanned(
            attr,
            "this attribute does not take arguments"
        ));
    }

    let item = proc_macro2::TokenStream::from(item);
    let item = syn::parse2::<syn::ItemEnum>(item)?;
    let item = ir::AstEnum::try_from(item)?;

    Ok(expand::expand(&item).into())
}


