#![recursion_limit = "128"]

use aggregate::derive_aggregate_for_struct;
use command::derive_command_for_enum;
use event::{derive_event_for_enum, derive_event_metadata_for_struct};
use proc_macro::TokenStream;
use query::derive_query_for_enum;
use syn::{parse_macro_input, Data, DeriveInput};

mod aggregate;
mod command;
mod event;
mod helpers;
mod query;

#[proc_macro_derive(EventMetadata, attributes(event_type, event_version, event_id))]
pub fn derive_event_metadata(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    // eprintln!("{:#?}", ast);

    match ast.data {
        Data::Struct(ref struct_data) => derive_event_metadata_for_struct(&ast, struct_data),
        _ => panic!("#[derive(EventMetadata)] is only defined for struct types, but not enum or union types")
    }
}

#[proc_macro_derive(Event)]
pub fn derive_event(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    // eprintln!("{:#?}", ast);

    match ast.data {
        Data::Enum(ref enum_data) => derive_event_for_enum(&ast, enum_data),
        _ => {
            panic!("#[derive(Event)] is only defined for enum types, but not struct or union types")
        }
    }
}

#[proc_macro_derive(Command)]
pub fn derive_command(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    // eprintln!("{:#?}", ast);

    match ast.data {
        Data::Enum(ref enum_data) => derive_command_for_enum(&ast, enum_data),
        _ => {
            panic!(
                "#[derive(Command)] is only defined for enum types, but not struct or union types"
            )
        }
    }
}

#[proc_macro_derive(Query)]
pub fn derive_query(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    // eprintln!("{:#?}", ast);

    match ast.data {
        Data::Enum(ref enum_data) => derive_query_for_enum(&ast, enum_data),
        _ => {
            panic!("#[derive(Query)] is only defined for enum types, but not struct or union types")
        }
    }
}

#[proc_macro_derive(Aggregate, attributes(aggregate_type, aggregate_id, event_type))]
pub fn derive_aggregate_metadata(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    // eprintln!("{:#?}", ast);

    match ast.data {
        Data::Struct(ref struct_data) => derive_aggregate_for_struct(&ast, struct_data),
        _ => panic!(
            "#[derive(Aggregate)] is only defined for struct types, but not enum or union types"
        ),
    }
}
