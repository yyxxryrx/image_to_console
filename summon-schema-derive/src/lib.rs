use proc_macro::TokenStream;

fn get_docs(attrs: &Vec<syn::Attribute>) -> String {
    attrs
        .iter()
        .filter_map(|attr| {
            if attr.path().is_ident("doc") {
                if let syn::Meta::NameValue(meta) = &attr.meta {
                    if let syn::Expr::Lit(expr_lit) = &meta.value {
                        if let syn::Lit::Str(lit_str) = &expr_lit.lit {
                            return Some(lit_str.value().trim().to_string());
                        }
                    }
                }
            }
            None
        })
        .collect::<Vec<String>>()
        .join("\n")
}

fn to_kebab_case(name: &str) -> String {
    let mut segments = vec![];
    let mut s = String::new();
    for c in name.chars() {
        if c == '_' {
            if !s.is_empty() {
                segments.push(s.clone());
                s.clear();
                continue;
            }
        }
        if c.is_uppercase() {
            if !s.is_empty() {
                segments.push(s.clone());
                s.clear();
            }
            s.push(c.to_ascii_lowercase());
            continue;
        }
        s.push(c);
    }
    if !s.is_empty() {
        segments.push(s);
    }
    segments.join("-")
}

enum Style {
    Lowercase,
    Uppercase,
    KebabCase,
    None,
}

impl Style {
    fn convert(&self, name: String) -> String {
        let name = name.strip_prefix("r#").unwrap_or(&name);
        match self {
            Self::Lowercase => name.to_lowercase(),
            Self::Uppercase => name.to_uppercase(),
            Self::KebabCase => to_kebab_case(name),
            Self::None => name.to_string(),
        }
    }
}

impl From<String> for Style {
    fn from(value: String) -> Self {
        match value.as_str() {
            "lowercase" => Self::Lowercase,
            "uppercase" => Self::Uppercase,
            "kebab-case" => Self::KebabCase,
            _ => panic!("Unsupported style"),
        }
    }
}

fn get_style(attrs: &Vec<syn::Attribute>) -> Style {
    let mut style = Style::None;
    for attr in attrs.iter() {
        if attr.meta.path().is_ident("serde") {
            if let syn::Meta::List(list) = &attr.meta {
                list.parse_nested_meta(|meta| {
                    if meta.path.is_ident("rename_all") {
                        style = meta.value()?.parse::<syn::LitStr>()?.value().into();
                    }
                    Ok(())
                })
                .unwrap()
            }
        }
    }
    style
}

fn schema_struct(input: &syn::DeriveInput, data: &syn::DataStruct, style: Style) -> TokenStream {
    let name = &input.ident;
    let doc = get_docs(&input.attrs);
    let (names, keys, tys, docs) = data.fields.iter().fold(
        (vec![], vec![], vec![], vec![]),
        |(mut names, mut keys, mut tys, mut docs), f| {
            if let Some(ident) = &f.ident {
                let ty = f.ty.clone();
                let ty = if let syn::Type::Path(mut path) = ty {
                    if let Some(s) = path.path.segments.last_mut() {
                        if let syn::PathArguments::AngleBracketed(args) = &mut s.arguments {
                            if let None = args.colon2_token {
                                args.colon2_token =
                                    Some(syn::Token![::](proc_macro2::Span::call_site()));
                            }
                        }
                    }
                    syn::Type::Path(path)
                } else {
                    ty
                };
                let doc = get_docs(&f.attrs);
                let key = syn::LitStr::new(
                    &style.convert(ident.to_string()),
                    proc_macro2::Span::call_site(),
                );
                tys.push(ty);
                docs.push(doc);
                keys.push(key);
                names.push(ident);
            }
            (names, keys, tys, docs)
        },
    );

    quote::quote! {
        impl ::summon_schema::ToSchema for #name {
            fn schema() -> ::serde_json::Map<std::string::String, ::serde_json::Value> {
                #(
                    let mut #names = #tys::schema();
                    #names.extend(::summon_schema::map! {
                        "type": #tys::schema_type(),
                        "description": #docs,
                    });
                )*
                ::summon_schema::map! {
                    "description": #doc,
                    "properties": {
                        #(
                            #keys: #names,
                        )*
                    },
                }
            }

            fn schema_type() -> ::serde_json::Value {
                ::serde_json::json!("object")
            }
        }
    }
    .into()
}

fn schema_enum(input: &syn::DeriveInput, data: &syn::DataEnum, style: Style) -> TokenStream {
    let name = &input.ident;
    let doc = get_docs(&input.attrs);
    let idents = data
        .variants
        .iter()
        .map(|v| style.convert(v.ident.to_string()))
        .collect::<Vec<String>>();
    quote::quote! {
        impl ::summon_schema::ToSchema for #name {
            fn schema() -> ::serde_json::Map<std::string::String, ::serde_json::Value> {
                ::summon_schema::map! {
                    "enum": [ #(#idents),* ],
                    "description": #doc,
                }
            }

            fn schema_type() -> ::serde_json::Value {
                ::serde_json::json!("string")
            }
        }
    }
    .into()
}

#[proc_macro_derive(Schema, attributes(schema, serde))]
pub fn schema(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let style = get_style(&input.attrs);
    match &input.data {
        syn::Data::Struct(data) => schema_struct(&input, data, style),
        syn::Data::Enum(data) => schema_enum(&input, data, style),
        _ => panic!(),
    }
}
