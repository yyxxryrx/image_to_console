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

enum DefaultType {
    DefaultValue,
    Call(syn::Path),
    None,
}

fn get_default(attrs: &Vec<syn::Attribute>) -> DefaultType {
    let mut ty = DefaultType::None;
    for attr in attrs.iter() {
        if attr.meta.path().is_ident("serde") {
            if let syn::Meta::List(list) = &attr.meta {
                _ = list.parse_nested_meta(|meta| {
                    if meta.path.is_ident("default") {
                        match meta.value() {
                            Ok(val) => {
                                ty = DefaultType::Call(syn::parse_str::<syn::Path>(
                                    &val.parse::<syn::LitStr>()?.value(),
                                )?)
                            }
                            Err(..) => ty = DefaultType::DefaultValue,
                        }
                    }
                    Ok(())
                });
            }
        }
    }
    ty
}

#[derive(Default)]
struct Args {
    no_default: bool,
    required: bool,
    minimum: Option<syn::Lit>,
    maximum: Option<syn::Lit>,
}

fn parse_args(attrs: &Vec<syn::Attribute>) -> Args {
    let mut args = Args::default();
    for attr in attrs.iter() {
        if attr.meta.path().is_ident("schema") {
            if let syn::Meta::List(list) = &attr.meta {
                list.parse_nested_meta(|meta| {
                    match &meta.path {
                        path if path.is_ident("required") => args.required = true,
                        path if path.is_ident("no_default") => args.no_default = true,
                        path if path.is_ident("minimum") => {
                            args.minimum = Some(meta.value()?.parse::<syn::Lit>()?);
                        }
                        path if path.is_ident("maximum") => {
                            args.maximum = Some(meta.value()?.parse::<syn::Lit>()?);
                        }
                        _ => {
                            return Err(meta.error(&format!(
                                "Unknown args: `{}`",
                                meta.path
                                    .get_ident()
                                    .map(|ident| ident.to_string())
                                    .unwrap_or_default()
                            )));
                        }
                    }
                    Ok(())
                })
                .unwrap();
            }
        }
    }
    args
}

fn schema_struct(input: &syn::DeriveInput, data: &syn::DataStruct, style: Style) -> TokenStream {
    let name = &input.ident;
    let doc = get_docs(&input.attrs);
    let (names, keys, tys, docs, defaults, others, required) = data.fields.iter().fold(
        (vec![], vec![], vec![], vec![], vec![], vec![], vec![]),
        |(mut names, mut keys, mut tys, mut docs, mut defaults, mut others, mut required), f| {
            if let Some(ident) = &f.ident {
                let ty = f.ty.clone();
                let args = parse_args(&f.attrs);
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
                let default = if args.no_default {
                    quote::quote! {}
                } else {
                    match get_default(&f.attrs) {
                        DefaultType::DefaultValue => quote::quote! {
                            "default": #ty::default(),
                        },
                        DefaultType::Call(path) => quote::quote! {
                            "default": #path(),
                        },
                        DefaultType::None => quote::quote! {},
                    }
                };
                if args.required {
                    required.push(key.clone());
                }
                let min = if let Some(min) =  args.minimum {
                    quote::quote! {
                        "minimum": #min,
                    }
                } else {
                    Default::default()
                };
                let max = if let Some(max) =  args.maximum {
                    quote::quote! {
                        "maximum": #max,
                    }
                } else {
                    Default::default()
                };
                tys.push(ty);
                docs.push(doc);
                keys.push(key);
                names.push(ident);
                defaults.push(default);
                others.push(quote::quote! {
                    #min
                    #max
                });
            }
            (names, keys, tys, docs, defaults, others, required)
        },
    );

    let required = if required.is_empty() {
        quote::quote! {}
    } else {
        quote::quote! {
            "required": [ #(#required),* ]
        }
    };

    quote::quote! {
        impl ::summon_schema::ToSchema for #name {
            fn schema() -> ::serde_json::Map<std::string::String, ::serde_json::Value> {
                #(
                    let mut #names = #tys::schema();
                    #names.extend(::summon_schema::map! {
                        "type": #tys::schema_type(),
                        "description": #docs,
                        #defaults
                        #others
                    });
                )*
                ::summon_schema::map! {
                    "description": #doc,
                    "properties": {
                        #(
                            #keys: #names,
                        )*
                    },
                    #required
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name_converter() {
        assert_eq!(Style::Lowercase.convert("Hello_World".to_string()), "hello_world");
        assert_eq!(Style::Uppercase.convert("Hello_World".to_string()), "HELLO_WORLD");
        assert_eq!(Style::KebabCase.convert("Hello_World".to_string()), "hello-world");
    }
}