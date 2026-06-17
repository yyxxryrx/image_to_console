//! Derive macros for summon-schema
//!
//! This crate provides procedural macros for automatically generating JSON Schema
//! implementations for custom structs and enums.
//!
//! # Features
//!
//! - Automatic schema generation for structs with named fields
//! - Support for enum variants as string enums
//! - Field-level documentation extraction
//! - Serde attribute compatibility (rename_all, default)
//! - Custom constraints via `#[schema]` attribute
//!
//! # Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! summon-schema = "version"
//! serde_json = "1.0"
//! ```
//!
//! Then use the derive macro:
//!
//! ```ignore
//! use summon_schema::Schema;
//!
//! #[derive(Schema)]
//! struct Person {
//!     name: String,
//!     age: u32,
//! }
//! ```
#![allow(clippy::collapsible_if)]
use proc_macro::TokenStream;
/// Extract documentation from attributes
///
/// Collects all `#[doc = "..."]` attributes and joins them into a single string.
/// Used to preserve field and struct documentation in the generated schema.
///
/// # Arguments
///
/// * `attrs` - Vector of syn::Attribute to extract documentation from
///
/// # Returns
///
/// A string containing all documentation comments joined by newlines
fn get_docs(attrs: &[syn::Attribute]) -> String {
    attrs
        .iter()
        .filter_map(|attr| {
            if attr.path().is_ident("doc")
                && let syn::Meta::NameValue(meta) = &attr.meta
            {
                if let syn::Expr::Lit(expr_lit) = &meta.value {
                    if let syn::Lit::Str(lit_str) = &expr_lit.lit {
                        return Some(lit_str.value().trim().to_string());
                    }
                }
            }

            None
        })
        .collect::<Vec<String>>()
        .join("\n")
}

/// Convert a string to kebab-case
///
/// Transforms identifiers like `hello_world` or `HelloWorld` into `hello-world`.
/// Handles both snake_case and camelCase inputs.
///
/// # Arguments
///
/// * `name` - The identifier name to convert
///
/// # Examples
///
/// ```ignore
/// assert_eq!(to_kebab_case("hello_world"), "hello-world");
/// assert_eq!(to_kebab_case("HelloWorld"), "hello-world");
/// ```
fn to_kebab_case(name: &str) -> String {
    let mut segments = vec![];
    let mut s = String::new();
    for c in name.chars() {
        if c == '_' && !s.is_empty() {
            segments.push(s.clone());
            s.clear();
            continue;
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

/// Naming convention styles for field renaming
///
/// Controls how Rust field names are transformed when generating schema keys.
/// Supports common naming conventions used in serialization formats.
enum Style {
    Lowercase,
    Uppercase,
    KebabCase,
    None,
}

impl Style {
    /// Convert a name according to the style
    ///
    /// Strips raw identifier prefixes (`r#`) before applying transformation.
    ///
    /// # Arguments
    ///
    /// * `name` - The name to convert
    ///
    /// # Returns
    ///
    /// The converted name as a String
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
    /// Create a Style from a string
    ///
    /// # Panics
    ///
    /// Panics if the string doesn't match any supported style.
    /// Supported values: "lowercase", "uppercase", "kebab-case"
    fn from(value: String) -> Self {
        match value.as_str() {
            "lowercase" => Self::Lowercase,
            "uppercase" => Self::Uppercase,
            "kebab-case" => Self::KebabCase,
            _ => panic!("Unsupported style"),
        }
    }
}

/// Extract the naming style from serde attributes
///
/// Parses `#[serde(rename_all = "...")]` attributes to determine the naming convention.
/// Defaults to `Style::None` if no rename attribute is present.
///
/// # Arguments
///
/// * `attrs` - Vector of attributes to search through
///
/// # Returns
///
/// The determined Style variant
fn get_style(attrs: &[syn::Attribute]) -> Style {
    let mut style = Style::None;
    for attr in attrs.iter() {
        if attr.meta.path().is_ident("serde")
            && let syn::Meta::List(list) = &attr.meta
        {
            list.parse_nested_meta(|meta| {
                if meta.path.is_ident("rename_all") {
                    style = meta.value()?.parse::<syn::LitStr>()?.value().into();
                }
                Ok(())
            })
            .unwrap()
        }
    }
    style
}

/// Represents the type of default value handling
///
/// Distinguishes between using the type's Default implementation,
/// calling a custom function, or having no default value.
enum DefaultType {
    DefaultValue,
    Call(syn::Path),
    None,
}

/// Extract default value configuration from serde attributes
///
/// Parses `#[serde(default)]` and `#[serde(default = "path")]` attributes.
///
/// # Arguments
///
/// * `attrs` - Vector of attributes to search through
///
/// # Returns
///
/// A DefaultType indicating how defaults should be handled
fn get_default(attrs: &[syn::Attribute]) -> DefaultType {
    let mut ty = DefaultType::None;
    for attr in attrs.iter() {
        if attr.meta.path().is_ident("serde")
            && let syn::Meta::List(list) = &attr.meta
        {
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
    ty
}

/// Configuration arguments for schema generation
///
/// Controls various aspects of schema generation for individual fields.
#[derive(Default)]
struct Args {
    no_default: bool,
    required: bool,
    minimum: Option<syn::Lit>,
    maximum: Option<syn::Lit>,
}

/// Parse schema-related attributes from field metadata
///
/// Extracts `#[schema(...)]` attributes to configure schema generation:
/// - `required`: Mark field as required
/// - `no_default`: Don't include default value in schema
/// - `minimum`: Set minimum value constraint
/// - `maximum`: Set maximum value constraint
///
/// # Arguments
///
/// * `attrs` - Vector of attributes to parse
///
/// # Returns
///
/// An Args struct containing the parsed configuration
fn parse_args(attrs: &[syn::Attribute]) -> Args {
    let mut args = Args::default();
    for attr in attrs.iter() {
        if attr.meta.path().is_ident("schema")
            && let syn::Meta::List(list) = &attr.meta
        {
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
                        return Err(meta.error(format!(
                            "Unknown attribute: `{}`",
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
    args
}

/// Generate ToSchema implementation for a struct
///
/// Creates a schema representation where each field becomes a property with:
/// - Type information from the field's type
/// - Documentation from doc comments
/// - Optional default values
/// - Optional constraints (min/max)
///
/// # Arguments
///
/// * `input` - The struct definition
/// * `data` - The struct's field data
/// * `style` - The naming convention to apply
///
/// # Returns
///
/// TokenStream containing the generated ToSchema implementation
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
                    if let Some(s) = path.path.segments.last_mut()
                        && let syn::PathArguments::AngleBracketed(args) = &mut s.arguments
                    {
                        if args.colon2_token.is_none() {
                            args.colon2_token =
                                Some(syn::Token![::](proc_macro2::Span::call_site()));
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
                let min = if let Some(min) = args.minimum {
                    quote::quote! {
                        "minimum": #min,
                    }
                } else {
                    Default::default()
                };
                let max = if let Some(max) = args.maximum {
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

/// Generate ToSchema implementation for an enum
///
/// Creates a string enum schema where each variant becomes a possible value.
/// All variants are treated as string options in the generated schema.
///
/// # Arguments
///
/// * `input` - The enum definition
/// * `data` - The enum's variant data
/// * `style` - The naming convention to apply to variant names
///
/// # Returns
///
/// TokenStream containing the generated ToSchema implementation
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

/// Derive macro for generating JSON Schema implementations
///
/// Automatically implements the `ToSchema` trait for structs and enums.
///
/// # Attributes
///
/// ## Struct/Enum level
///
/// - `#[serde(rename_all = "...")]` - Apply naming convention to all fields/variants
///
/// ## Field level (structs only)
///
/// - `#[schema(required)]` - Mark field as required
/// - `#[schema(no_default)]` - Exclude default value from schema
/// - `#[schema(minimum = value)]` - Set minimum constraint
/// - `#[schema(maximum = value)]` - Set maximum constraint
/// - `#[serde(default)]` - Use type's Default implementation
/// - `#[serde(default = "path")]` - Use custom function for default
///
/// # Examples
///
/// Basic struct:
/// ```ignore
/// #[derive(Schema)]
/// struct Point {
///     x: i32,
///     y: i32,
/// }
/// ```
///
/// With attributes:
/// ```ignore
/// #[derive(Schema)]
/// #[serde(rename_all = "kebab-case")]
/// struct Config {
///     #[schema(required, minimum = 0, maximum = 100)]
///     volume: u8,
///     
///     #[serde(default)]
///     muted: bool,
/// }
/// ```
#[proc_macro_derive(Schema, attributes(schema, serde))]
/// Main entry point for the Schema derive macro
///
/// Dispatches to appropriate handler based on whether input is a struct or enum.
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
        assert_eq!(
            Style::Lowercase.convert("Hello_World".to_string()),
            "hello_world"
        );
        assert_eq!(
            Style::Uppercase.convert("Hello_World".to_string()),
            "HELLO_WORLD"
        );
        assert_eq!(
            Style::KebabCase.convert("Hello_World".to_string()),
            "hello-world"
        );
    }
}
