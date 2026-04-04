use proc_macro2::TokenStream;
use syn::Ident;
use quote::quote;
use crate::ir::{AstEnum, Field, Variant};

pub(crate) fn expand(item: &AstEnum) -> TokenStream {
    let ident = item.ident();
    let ast_enum = expand_into_enum(item);
    let structs = item.variants()
        .iter()
        .map(|v| expand_into_struct(v, ident));

    quote! {
        #ast_enum
        #( #structs )*
    }
}

fn expand_into_enum(item: &AstEnum) -> TokenStream {
    let ident = item.ident();
    let variants = item.variants()
        .iter()
        .map(expand_into_variant);

    quote! {
        #[derive(Debug)]
        pub enum #ident {
            #( #variants ),*
        }
    }
}

fn expand_into_variant(variant: &Variant) -> TokenStream {
    let ident = variant.ident();
    quote! { #ident(#ident) }
}

fn expand_into_struct(variant: &Variant, enum_ident: &Ident) -> TokenStream {
    let ident = variant.ident();
    let fields = variant.fields();

    let new_fn = expand_new_fn(fields);
    let getters = fields
        .iter()
        .map(expand_getter);
    let fields = fields
        .iter()
        .map(expand_field);

    let from_impl = expand_from_impl(ident, enum_ident);

    quote! {
        #[derive(Debug)]
        pub struct #ident {
            id: dough::node::NodeId,
            #( #fields ),*
        }

        impl #ident {
            #new_fn
            #( #getters )*
        }

        impl dough::node::Node for #ident {
            fn id(&self) -> dough::node::NodeId {
                self.id
            }
        }

        #from_impl
    }
}

fn expand_from_impl(ident: &Ident, enum_ident: &Ident) -> TokenStream {
    quote! {
        impl From<#ident> for #enum_ident {
            fn from(value: #ident) -> Self {
                #enum_ident::#ident(value)
            }
        }
    }
}

fn expand_field(field: &Field) -> TokenStream {
    let ident = field.ident();
    let ty = field.ty();
    quote! { #ident: #ty }
}

fn expand_new_fn(fields: &[Field]) -> TokenStream {
    let (params, idents): (Vec<_>, Vec<_>) = fields
        .iter()
        .map(|f| (expand_field(f), f.ident()))
        .unzip();

    quote! {
        pub fn new( id: dough::node::NodeId, #( #params ),* ) -> Self {
            Self { id, #( #idents ),* }
        }
    }
}

fn expand_getter(field: &Field) -> TokenStream {
    let ident = field.ident();
    let ty = field.ty();

    let (ret_ty, body) = if *field.is_copy() {
        (quote! { #ty }, quote! { self.#ident })
    } else {
        (quote! { &#ty }, quote! { &self.#ident })
    };

    quote! {
        pub fn #ident(&self) -> #ret_ty {
            #body
        }
    }
}
