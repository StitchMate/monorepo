use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{DataStruct, DeriveInput, Data};

use crate::helpers::{find_field_attributes, find_attribute};

pub fn derive_aggregate_for_struct(
    ast: &DeriveInput,
    _struct_data: &DataStruct,
) -> TokenStream {
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let tname = &ast.ident;
    let aggtype = find_attribute("aggregate_type", &ast.attrs)
        .map(|attr| attr.tokens.clone())
        .unwrap_or_else(|| quote!(stringify!(#tname)));
    let aggevttype = find_attribute("event_type", &ast.attrs)
        .map(|attr| attr.tokens.clone())
        .unwrap_or_else(|| panic!("#[derive(Aggregate) must define event_type which implements trait Event"));
    let aggid_field = match &ast.data {
        Data::Struct(s) => find_field_attributes("aggregate_id", &s.fields),
        _ => None,
    };
    let aggid = aggid_field
        .map(|f| (&f.ident).into_token_stream())
        .unwrap_or_else(|| quote!(aggregate_id));

    (quote! {
        #[allow(unused_qualifications, unused_parens)]
        #[automatically_derived]
        impl #impl_generics ::base::domain::entity::aggregate::Aggregate for #tname #ty_generics #where_clause {
            type Event = #aggevttype;

            fn aggregate_type(&self) -> String {
                #aggtype.into()
            }

            fn aggregate_id(&self) -> Option<String> {
                self.#aggid.clone()
            }
        }
    })
    .into()
}
