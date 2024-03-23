use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{Data, DataEnum, DataStruct, DeriveInput, Fields, Type};

use crate::helpers::{find_attribute, find_field_attributes};

pub fn derive_event_metadata_for_struct(
    ast: &DeriveInput,
    _struct_data: &DataStruct,
) -> TokenStream {
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let tname = &ast.ident;
    let evversion = find_attribute("event_version", &ast.attrs)
        .map(|attr| (&attr.tokens).into_token_stream())
        .unwrap_or_else(|| quote!("v0"));
    let evtype = find_attribute("event_type", &ast.attrs)
        .map(|attr| (&attr.tokens).into_token_stream())
        .unwrap_or_else(|| quote!(stringify!(#tname)));
    let evid_field = match &ast.data {
        Data::Struct(s) => find_field_attributes("event_id", &s.fields),
        _ => None,
    };
    let evid = evid_field
        .map(|f| (&f.ident).into_token_stream())
        .unwrap_or_else(|| quote!(event_id));

    (quote! {
        #[allow(unused_qualifications, unused_parens)]
        #[automatically_derived]
        impl #impl_generics ::base::domain::entity::event::EventMetadata for #tname #ty_generics #where_clause {
            fn event_version(&self) -> String {
                #evversion.into()
            }
            fn event_type(&self) -> String {
                #evtype.into()
            }
            fn event_id(&self) -> String {
                self.#evid.clone()
            }
        }
    })
    .into()
}

pub fn derive_event_for_enum(ast: &DeriveInput, enum_data: &DataEnum) -> TokenStream {
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let tname = &ast.ident;

    let event_matches = enum_data
        .variants
        .iter()
        .map(|variant| {
            let vname = match &variant.fields {
                Fields::Unnamed(f) => {
                    match &f.unnamed[0].ty {
                        Type::Path(p) => &p.path.segments[0].ident,
                        _ => panic!("#[derive(Event) operates on wrapped struct, such as EventName(EventStruct), but none was found")
                    }
                },
                _ => panic!("#[derive(Event) operates on wrapped struct, such as EventName(EventStruct), but none was found")
            };
            let ename = &variant.ident;
            let evtype = &format!("{}", vname);
            match variant.fields {
                Fields::Unit => quote! {
                    #tname::#ename => #evtype.into(),
                },
                Fields::Unnamed(ref fields) => {
                    let field_names = fields
                        .unnamed
                        .pairs()
                        .map(|p| p.value().ident.as_ref())
                        .collect::<Vec<_>>();
                    quote! {
                        #tname::#ename( #(_ #field_names,)* ) => #evtype.into(),
                    }
                }
                Fields::Named(ref fields) => {
                    let field_names = fields
                        .named
                        .pairs()
                        .map(|p| p.value().ident.as_ref())
                        .collect::<Vec<_>>();
                    quote! {
                        #tname::#ename { #(#field_names: _,)* } => #evtype.into(),
                    }
                }
            }
        })
        .collect::<Vec<_>>();

    let event_matches_event_id = enum_data
        .variants
        .iter()
        .map(|variant| {
            let vname = match &variant.fields {
                Fields::Unnamed(f) => {
                    match &f.unnamed[0].ty {
                        Type::Path(p) => &p.path.segments[0].ident,
                        _ => panic!("#[derive(Event) operates on wrapped struct, such as EventName(EventStruct), but none was found")
                    }
                },
                _ => panic!("#[derive(Event) operates on wrapped struct, such as EventName(EventStruct), but none was found")
            };
            let ename = &variant.ident;
            let evtype = &format!("{}", vname);
            match variant.fields {
                Fields::Unit => quote! {
                    #tname::#ename => #evtype.event_id(),
                },
                Fields::Unnamed(ref fields) => {
                    let field_names = fields
                        .unnamed
                        .pairs()
                        .map(|p| p.value().ident.as_ref())
                        .collect::<Vec<_>>();
                    quote! {
                        #tname::#ename( #(e #field_names,)* ) => e.event_id(),
                    }
                }
                Fields::Named(ref fields) => {
                    let field_names = fields
                        .named
                        .pairs()
                        .map(|p| p.value().ident.as_ref())
                        .collect::<Vec<_>>();
                    quote! {
                        #tname::#ename { #(#field_names: _,)* } => #evtype.event_id(),
                    }
                }
            }
        })
        .collect::<Vec<_>>();

    let event_matches_event_type = enum_data
        .variants
        .iter()
        .map(|variant| {
            let vname = match &variant.fields {
                Fields::Unnamed(f) => {
                    match &f.unnamed[0].ty {
                        Type::Path(p) => &p.path.segments[0].ident,
                        _ => panic!("#[derive(Event) operates on wrapped struct, such as EventName(EventStruct), but none was found")
                    }
                },
                _ => panic!("#[derive(Event) operates on wrapped struct, such as EventName(EventStruct), but none was found")
            };
            let ename = &variant.ident;
            let evtype = &format!("{}", vname);
            match variant.fields {
                Fields::Unit => quote! {
                    #tname::#ename => #evtype.event_type(),
                },
                Fields::Unnamed(ref fields) => {
                    let field_names = fields
                        .unnamed
                        .pairs()
                        .map(|p| p.value().ident.as_ref())
                        .collect::<Vec<_>>();
                    quote! {
                        #tname::#ename( #(e #field_names,)* ) => e.event_type(),
                    }
                }
                Fields::Named(ref fields) => {
                    let field_names = fields
                        .named
                        .pairs()
                        .map(|p| p.value().ident.as_ref())
                        .collect::<Vec<_>>();
                    quote! {
                        #tname::#ename { #(#field_names: _,)* } => #evtype.event_type(),
                    }
                }
            }
        })
        .collect::<Vec<_>>();

    let event_matches_event_version = enum_data
        .variants
        .iter()
        .map(|variant| {
            let vname = match &variant.fields {
                Fields::Unnamed(f) => {
                    match &f.unnamed[0].ty {
                        Type::Path(p) => &p.path.segments[0].ident,
                        _ => panic!("#[derive(Event) operates on wrapped struct, such as EventName(EventStruct), but none was found")
                    }
                },
                _ => panic!("#[derive(Event) operates on wrapped struct, such as EventName(EventStruct), but none was found")
            };
            let ename = &variant.ident;
            let evtype = &format!("{}", vname);
            match variant.fields {
                Fields::Unit => quote! {
                    #tname::#ename => #evtype.event_version(),
                },
                Fields::Unnamed(ref fields) => {
                    let field_names = fields
                        .unnamed
                        .pairs()
                        .map(|p| p.value().ident.as_ref())
                        .collect::<Vec<_>>();
                    quote! {
                        #tname::#ename( #(e #field_names,)* ) => e.event_version(),
                    }
                }
                Fields::Named(ref fields) => {
                    let field_names = fields
                        .named
                        .pairs()
                        .map(|p| p.value().ident.as_ref())
                        .collect::<Vec<_>>();
                    quote! {
                        #tname::#ename { #(#field_names: _,)* } => #evtype.event_version(),
                    }
                }
            }
        })
        .collect::<Vec<_>>();

    (quote! {
        #[allow(unused_qualifications, unused_parens)]
        #[automatically_derived]
        impl #impl_generics ::base::domain::entity::event::Event for #tname #ty_generics #where_clause {
            fn to_string(&self) -> String {
                match self {
                    #(#event_matches)*
                }
            }

            fn event_id(&self) -> String {
                match self {
                    #(#event_matches_event_id)*
                }
            }

            fn event_type(&self) -> String {
                match self {
                    #(#event_matches_event_type)*
                }
            }

            fn event_version(&self) -> String {
                match self {
                    #(#event_matches_event_version)*
                }
            }
        }
    })
    .into()
}
