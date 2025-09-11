use serde_json::Value;

// Trait to support generic conversion from JSON arrays
pub trait FromJsonValue: Sized {
    fn from_json_value(v: &Value) -> Option<Self>;
}

impl FromJsonValue for i32 {
    fn from_json_value(v: &Value) -> Option<Self> {
        v.as_i64().map(|n| n as i32)
    }
}

impl FromJsonValue for f64 {
    fn from_json_value(v: &Value) -> Option<Self> {
        v.as_f64()
    }
}

impl FromJsonValue for String {
    fn from_json_value(v: &Value) -> Option<Self> {
        v.as_str().map(|s| s.to_string())
    }
}

// Converting Json To Generic Vector
pub fn convert_json_to_vec<T: FromJsonValue>(val: &Value) -> Option<Vec<T>> {
    val.as_array()?.iter().map(T::from_json_value).collect()
}

// Converting Json To i32 Vector
pub fn convert_json_to_vec_i32(json_val: &Value) -> Option<Vec<i32>> {
    if let Some(json_array) = json_val.as_array() {
        let mut result_vec = Vec::new();
        for item in json_array {
            if let Some(num_i64) = item.as_i64() {
                // JSON numbers are often parsed as i64 or f64
                // Convert i64 to i32, checking for potential overflow
                if num_i64 >= i32::MIN as i64 && num_i64 <= i32::MAX as i64 {
                    result_vec.push(num_i64 as i32);
                } else {
                    eprintln!("Number {} is out of i32 range.", num_i64);
                    return None; // Or handle error as appropriate
                }
            } else {
                eprintln!("Non-numeric value found in JSON array: {:?}", item);
                return None; // Or handle error
            }
        }
        Some(result_vec)
    } else {
        eprintln!("JSON value is not an array: {:?}", json_val);
        None // Not an array
    }
}
