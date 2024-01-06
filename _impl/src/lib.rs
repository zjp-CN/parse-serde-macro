use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, Parser},
    parse_macro_input,
    punctuated::Punctuated,
    Attribute, Data, DataStruct, DeriveInput, Fields, Ident, LitStr, Meta, Token,
};

#[proc_macro_derive(ExcelSerialize, attributes(rust_xlsxwriter, xlsxwriter))]
pub fn excel_serialize_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    if let Data::Struct(DataStruct {
        fields: Fields::Named(fields),
        ..
    }) = input.data
    {
        for field in fields.named {
            let op: Vec<_> = field.attrs.into_iter().map(parse_meta).collect();
            println!("field `{}` => {op:?}", field.ident.unwrap());
        }
    }
    quote! {}.into() // TODO: generate impl ExcelSerialize for Data
}

fn parse_meta(attr: Attribute) -> syn::Result<Vec<AttributeTypes>> {
    match attr.meta {
        Meta::Path(_) => (),
        Meta::List(list) => {
            let Some(ident) = list.path.get_ident() else {
                return Ok(Vec::new());
            };
            if ident == "rust_xlsxwriter" || ident == "xlsxwriter" || ident == "serde" {
                // e.g. #[ident(one)] or #[ident(one, two)] or #[ident(one, two, ...)]
                let parsed = Punctuated::<AttributeTypes, Token![,]>::parse_separated_nonempty
                    .parse2(list.tokens)?;
                return Ok(parsed.into_iter().collect());
            }
        }
        Meta::NameValue(_) => (),
    }
    Ok(Vec::new())
}

#[derive(Debug)]
enum AttributeTypes {
    Skip,
    Rename(LitStr),
    NumFormat(LitStr),
}

// Parse tokens in meta list, ignoring the meta path.
// e.g. for Skip variant, only peek it in #[serde(skip)] or #[rust_xlsxwriter(skip)] or
//      #[xlsxwriter(skip)] or #[whatever(skip)]
impl Parse for AttributeTypes {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<Ident>()?;
        if ident == "skip" {
            Ok(Self::Skip)
        } else if ident == "rename" {
            let _ = input.parse::<Token![=]>()?;
            Ok(Self::Rename(input.parse()?))
        } else if ident == "num_format" {
            let _ = input.parse::<Token![=]>()?;
            Ok(Self::NumFormat(input.parse()?))
        } else {
            Err(syn::Error::new(
                input.span(),
                "Not supported by ExcelSerialize derive macro",
            ))
        }
    }
}
