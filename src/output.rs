use crate::serde::Serialize;
use crate::serde_json;

pub fn to_json_pretty<T>(serialze: &T) -> String
    where T: Serialize {
        
    serde_json::to_string_pretty(serialze)
        .expect("Error serializing to json")
}