//! Schema Generation Library
//!
//! This library provides functionality to convert Rust types into JSON Schema representations,
//! supporting both primitive types and custom type schema generation.
//!
//! # Features
//!
//! - Supports all primitive integer types and string types
//! - Supports `Option<T>` types
//! - Supports custom structs through derive macros
//! - Generates output compliant with JSON Schema draft-04 specification
//!
//! # Examples
//!
//! ```
//! use summon_schema::{ToSchema, gen_schema};
//!
//! // Primitive types
//! assert_eq!(bool::schema_type(), serde_json::json!("boolean"));
//! assert_eq!(i32::schema_type(), serde_json::json!("integer"));
//! assert_eq!(String::schema_type(), serde_json::json!("string"));
//!
//! // Generate complete schema
//! let schema = gen_schema::<i32>();
//! ```

use serde_json::{Map, Value};
pub use summon_schema_derive::*;

/// Macro for building JSON objects
///
/// This macro provides a concise way to create `serde_json::Map<String, Value>` objects.
/// It uses the `serde_json::json!` macro as the underlying implementation and ensures
/// the return value is an object type.
///
/// # Panics
///
/// If the passed JSON is not an object type, it will trigger `unreachable!()`.
///
/// # Examples
///
/// ```
/// use summon_schema::map;
///
/// let my_map = map! {
///     "key1": "value1",
///     "key2": 42,
/// };
/// ```
#[macro_export]
macro_rules! map {
    ($($tt:tt)*) => {
        match serde_json::json!({ $($tt)* }) {
            serde_json::Value::Object(obj) => obj,
            _ => unreachable!(),
        }
    };
}

/// Trait for converting to Schema
///
/// Types implementing this trait can convert themselves to JSON Schema representation.
/// The trait provides two methods:
/// - `schema_type()`: Returns the JSON Schema type of the type (e.g., "string", "integer", "boolean")
/// - `schema()`: Returns the complete schema map, including additional constraints (e.g., min/max values)
///
/// # Default Implementation
///
/// The `schema()` method provides an empty default implementation. Types that don't need
/// additional constraints only need to implement `schema_type()`.
///
/// # Examples
///
/// ```
/// use summon_schema::ToSchema;
/// use serde_json::Value;
///
/// struct MyType;
///
/// impl ToSchema for MyType {
///     fn schema_type() -> Value {
///         Value::String("object".to_string())
///     }
/// }
/// ```
pub trait ToSchema {
    /// Get the JSON Schema type of the type
    ///
    /// Returns a `Value` representing the type identifier in JSON Schema.
    /// Common return values include:
    /// - `Value::String("boolean")` - Boolean type
    /// - `Value::String("integer")` - Integer type
    /// - `Value::String("string")` - String type
    /// - `Value::Array([...])` - Multiple types (e.g., `Option<T>`)
    fn schema_type() -> Value;
    
    /// Get the complete JSON Schema map
    ///
    /// Returns a complete schema object containing type information.
    /// For types with additional constraints (e.g., integer ranges),
    /// this can be specified here. Returns an empty object by default,
    /// and subtypes can override this method to add additional information.
    fn schema() -> Map<String, Value> {
        map! {}
    }
}

impl ToSchema for bool {
    fn schema_type() -> Value {
        serde_json::json!("boolean")
    }
}

/// Macro for implementing `ToSchema` trait for integer types
///
/// This macro generates `ToSchema` implementations for the specified integer type, including:
/// - `schema_type()` returns "integer"
/// - `schema()` returns a map containing `minimum` and `maximum` constraints
///
/// # Note
///
/// Since different integer types have different value ranges, the implemented
/// `schema()` will contain the minimum and maximum values for that type.
macro_rules! integer_impl {
    ($ty:ident) => {
        impl ToSchema for $ty {
            /// Returns the schema type identifier for integer types
            fn schema_type() -> Value {
                serde_json::json!("integer")
            }

            /// Returns the integer schema with value range constraints
            ///
            /// Contains two properties: `minimum` (type's minimum value) 
            /// and `maximum` (type's maximum value).
            fn schema() -> Map<String, Value> {
                map! {
                    "minimum": $ty::MIN,
                    "maximum": $ty::MAX,
                }
            }
        }
    };
}

integer_impl!(i8);
integer_impl!(i16);
integer_impl!(i32);
integer_impl!(i64);
integer_impl!(i128);
integer_impl!(isize);

integer_impl!(u8);
integer_impl!(u16);
integer_impl!(u32);
integer_impl!(u64);
integer_impl!(u128);
integer_impl!(usize);

impl ToSchema for String {
    /// Returns the schema type identifier for string types
    fn schema_type() -> Value {
        serde_json::json!("string")
    }
}

impl ToSchema for &str {
    /// Returns the schema type identifier for string slice types
    fn schema_type() -> Value {
        serde_json::json!("string")
    }
}

impl<T> ToSchema for Option<T>
where
    T: ToSchema,
{
    /// Returns the schema type for optional types
    ///
    /// The type of `Option<T>` is an array containing `"null"` and the type of `T`.
    /// If `T` itself already contains `null` (i.e., `T` is also an `Option`),
    /// it directly returns the type array of `T`.
    fn schema_type() -> Value {
        let null = Value::String("null".to_string());
        let mut ty = vec![null.clone()];
        let sub_ty = T::schema_type();
        match sub_ty {
            Value::Array(arr) if arr.contains(&null) => return Value::Array(arr),
            Value::Array(arr) => ty.extend_from_slice(arr.as_slice()),
            _ => ty.push(sub_ty),
        }
        Value::Array(ty)
    }

    /// Returns the schema map for optional types
    ///
    /// Delegates to the `schema()` implementation of the inner type `T`.
    fn schema() -> Map<String, Value> {
        T::schema()
    }
}

/// Generate a complete JSON Schema
///
/// This function generates a complete JSON Schema object for types implementing
/// the `ToSchema` trait. The generated schema complies with JSON Schema draft-04
/// specification and includes the `$schema` field to identify the specification version.
///
/// # Type Parameters
///
/// * `T` - The type to generate schema for, must implement `ToSchema` trait
///
/// # Returns
///
/// Returns a `serde_json::Value` containing the complete JSON Schema information.
///
/// # Examples
///
/// ```
/// use summon_schema::gen_schema;
///
/// // Generate schema for i32
/// let schema = gen_schema::<i32>();
/// assert!(schema.get("$schema").is_some());
/// assert_eq!(schema.get("type").unwrap(), &serde_json::json!("integer"));
///
/// // Generate schema for String
/// let string_schema = gen_schema::<String>();
/// ```
pub fn gen_schema<T>() -> Value
where
    T: ToSchema,
{
    let mut schema = T::schema();
    schema.extend(map! {
        "$schema": "http://json-schema.org/draft-04/schema#",
    });
    Value::Object(schema)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bool_schema_type() {
        assert_eq!(bool::schema_type(), Value::String("boolean".to_string()));
    }

    #[test]
    fn test_integer_schema_types() {
        assert_eq!(i32::schema_type(), Value::String("integer".to_string()));
        assert_eq!(u64::schema_type(), Value::String("integer".to_string()));
        assert_eq!(isize::schema_type(), Value::String("integer".to_string()));
    }

    #[test]
    fn test_integer_schema_constraints() {
        let i8_schema = i8::schema();
        assert_eq!(i8_schema.get("minimum"), Some(&Value::Number(i8::MIN.into())));
        assert_eq!(i8_schema.get("maximum"), Some(&Value::Number(i8::MAX.into())));

        let u32_schema = u32::schema();
        assert_eq!(u32_schema.get("minimum"), Some(&Value::Number(u32::MIN.into())));
        assert_eq!(u32_schema.get("maximum"), Some(&Value::Number(u32::MAX.into())));
    }

    #[test]
    fn test_string_schema_types() {
        assert_eq!(String::schema_type(), Value::String("string".to_string()));
        assert_eq!(<&str>::schema_type(), Value::String("string".to_string()));
    }

    #[test]
    fn test_option_schema_type() {
        let option_i32_type = Option::<i32>::schema_type();
        match option_i32_type {
            Value::Array(arr) => {
                assert!(arr.contains(&Value::String("null".to_string())));
                assert!(arr.contains(&Value::String("integer".to_string())));
            }
            _ => panic!("Expected array type for Option<i32>"),
        }
    }

    #[test]
    fn test_nested_option_schema_type() {
        let option_option_i32_type = Option::<Option<i32>>::schema_type();
        match option_option_i32_type {
            Value::Array(arr) => {
                assert!(arr.contains(&Value::String("null".to_string())));
                assert!(arr.contains(&Value::String("integer".to_string())));
            }
            _ => panic!("Expected array type for Option<Option<i32>>"),
        }
    }

    #[test]
    fn test_gen_schema_basic() {
        let schema = gen_schema::<bool>();
        assert!(schema.is_object());
        let obj = schema.as_object().unwrap();
        assert_eq!(obj.get("$schema"), Some(&Value::String("http://json-schema.org/draft-04/schema#".to_string())));
        assert_eq!(obj.get("type"), None);
    }

    #[test]
    fn test_gen_schema_with_constraints() {
        let schema = gen_schema::<i16>();
        let obj = schema.as_object().unwrap();
        assert!(obj.contains_key("minimum"));
        assert!(obj.contains_key("maximum"));
        assert_eq!(obj.get("type"), None);
    }

    #[test]
    fn test_map_macro() {
        let my_map = map! {
            "key1": "value1",
            "key2": 42,
        };
        assert_eq!(my_map.get("key1"), Some(&Value::String("value1".to_string())));
        assert_eq!(my_map.get("key2"), Some(&Value::Number(42.into())));
    }
}
