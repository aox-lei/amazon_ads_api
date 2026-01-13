use serde::{Serialize, Serializer};
use serde_json::json;

pub fn wrap_include_optional<T, S>(val: &Option<Vec<T>>, serializer: S) -> Result<S::Ok, S::Error>
where
    T: ToString,
    S: Serializer,
{
    if let Some(s) = val {
        let string_vec: Vec<String> = s.iter().map(|item| item.to_string()).collect();
        return json!({ "include": string_vec }).serialize(serializer);
    }
    serializer.serialize_none()
}

pub fn wrap_include<T, S>(val: &[T], serializer: S) -> Result<S::Ok, S::Error>
where
    T: ToString,
    S: Serializer,
{
    let string_vec: Vec<String> = val.iter().map(|item| item.to_string()).collect();
    json!({ "include": string_vec }).serialize(serializer)
}
