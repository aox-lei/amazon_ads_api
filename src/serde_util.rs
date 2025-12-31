use serde::{Serialize, Serializer};
use serde_json::json;

pub fn wrap_include<S>(val: &Option<Vec<String>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(s) = val {
        return json!({ "include": s }).serialize(serializer);
    }
    serializer.serialize_none()
}
