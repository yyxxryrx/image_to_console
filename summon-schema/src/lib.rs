use serde_json::{Map, Value};
pub use summon_schema_derive::*;

#[macro_export]
macro_rules! map {
    ($($tt:tt)*) => {
        match serde_json::json!({ $($tt)* }) {
            serde_json::Value::Object(obj) => obj,
            _ => unreachable!(),
        }
    };
}

pub trait ToSchema {
    fn schema_type() -> Value;
    fn schema() -> Map<String, Value> {
        map! {}
    }
}

impl ToSchema for bool {
    fn schema_type() -> Value {
        serde_json::json!("boolean")
    }
}

macro_rules! integer_impl {
    ($ty:ident) => {
        impl ToSchema for $ty {
            fn schema_type() -> Value {
                serde_json::json!("integer")
            }

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
    fn schema_type() -> Value {
        serde_json::json!("string")
    }
}

impl ToSchema for &str {
    fn schema_type() -> Value {
        serde_json::json!("string")
    }
}

impl<T> ToSchema for Option<T>
where
    T: ToSchema,
{
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

    fn schema() -> Map<String, Value> {
        T::schema()
    }
}

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
