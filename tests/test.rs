use bon::Builder;
use serde::{Serialize, Serializer};
use serde_json::json;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Serialize, Builder)]
#[builder(on(String, into))]
struct TestA {
    #[serde(
        serialize_with = "wrap_include" // 指定自定义序列化逻辑
    )]
    name: Option<String>,
    age: Option<i32>,
}

fn wrap_include<S>(val: &Option<String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(s) = val {
        return json!({ "include": [s] }).serialize(serializer);
    }
    serializer.serialize_none()
}

#[tokio::test]
async fn test() {
    let data = serde_json::to_value(TestA::builder().name("aaaa").build()).unwrap();
    dbg!(&data);
}
