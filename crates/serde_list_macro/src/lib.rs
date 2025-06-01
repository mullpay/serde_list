#[proc_macro_derive(AsList, attributes(tag))]
pub fn derive_serialize_with_field(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let ident = &input.ident;
    let mut tag = "tag".to_string();

    for attr in input.attrs {
        if !attr.path().is_ident("tag") {
            continue;
        }
        let syn::Meta::NameValue(value) = attr.meta else {
            panic!("Only strings are supported");
        };
        let syn::Expr::Lit(lit) = value.value else {
            panic!("Only strings are supported");
        };
        let syn::Lit::Str(str) = lit.lit else {
            panic!("Only strings are supported");
        };

        tag = str.value();
    }

    let fields = match &input.data {
        syn::Data::Struct(data) => match &data.fields {
            syn::Fields::Named(fields) => &fields.named,
            _ => panic!("Only named fields are supported"),
        },
        _ => panic!("Only structs are supported"),
    };

    let mut elements = quote::quote! {};

    for field in fields {
        let name = field.ident.as_ref().unwrap();

        elements = quote::quote! {
            #elements

            if let Ok(serde_list::serde_json::Value::Object(mut map)) = serde_list::serde_json::to_value(&self. #name){
                map.insert(
                    #tag.to_string(),
                    serde_list::serde_json::Value::String(stringify!(#name).to_string()),
                );
                seq.serialize_element(&map)?;
            }
        };
    }

    let expanded = quote::quote! {
        impl serde_list::serde::Serialize for #ident {
            fn serialize<S: serde_list::serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                use serde_list::serde::ser::SerializeSeq;
                use serde_list::serde::ser::Serializer;

                let mut seq = serializer.serialize_seq(None)?;

                #elements

                seq.end()
            }
        }
    };

    expanded.into()
}
