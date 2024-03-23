use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, DataEnum, Fields, Type};

pub fn derive_command_for_enum(ast: &DeriveInput, enum_data: &DataEnum) -> TokenStream {
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
                        _ => panic!("#[derive(Command) operates on wrapped struct, such as CommandName(CommandStruct), but none was found")
                    }
                },
                _ => panic!("#[derive(Command) operates on wrapped struct, such as CommandName(CommandStruct), but none was found")
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

    (quote! {
        #[allow(unused_qualifications, unused_parens)]
        #[automatically_derived]
        impl #impl_generics ::base::domain::entity::command::Command for #tname #ty_generics #where_clause {
            fn to_string(&self) -> String {
                match self {
                    #(#event_matches)*
                }
            }
        }
    })
    .into()
}
