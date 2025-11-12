// Redis JSON Commands wrapper for HarmonyOS NAPI

use napi_derive_ohos::napi;
use napi_ohos::{Error, Result, Status};
use redis::{Connection, JsonCommands};
use serde_json::Value as JsonValue;

/// Redis JSON Connection for HarmonyOS
///
/// This class provides JSON-specific Redis commands for working with RedisJSON module.
/// All JSON values are passed as JSON strings and returned as JSON strings.
#[napi]
pub struct RedisJsonConnection {
    inner: Connection,
}

impl RedisJsonConnection {
    pub(crate) fn new(conn: Connection) -> Self {
        RedisJsonConnection { inner: conn }
    }
}

#[napi]
impl RedisJsonConnection {
    // ==================== Core JSON Commands ====================

    /// JSON.SET command - Set JSON value at path
    ///
    /// # Arguments
    /// * `key` - The key to set
    /// * `path` - JSONPath expression (use "$" for root)
    /// * `json` - JSON string value to set
    ///
    /// # Example (ArkTS)
    /// ```typescript
    /// conn.jsonSet("user:1", "$", JSON.stringify({name: "John", age: 30}));
    /// conn.jsonSet("user:1", "$.age", "31");
    /// ```
    #[napi]
    pub fn json_set(&mut self, key: String, path: String, json: String) -> Result<bool> {
        // Parse JSON to validate it
        let value: JsonValue = serde_json::from_str(&json)
            .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid JSON: {}", e)))?;

        JsonCommands::json_set(&mut self.inner, key, path, &value)
            .map_err(|e| napi_ohos::Error::from_reason(format!("JSON.SET failed: {}", e)))
    }

    /// JSON.GET command - Get JSON value at path
    ///
    /// # Arguments
    /// * `key` - The key to get
    /// * `path` - JSONPath expression (use "$" for root)
    ///
    /// # Returns
    /// JSON string (wrapped in array for JSONPath results)
    ///
    /// # Example (ArkTS)
    /// ```typescript
    /// const json = conn.jsonGet("user:1", "$");
    /// const age = conn.jsonGet("user:1", "$.age");
    /// ```
    #[napi]
    pub fn json_get(&mut self, key: String, path: String) -> Result<String> {
        JsonCommands::json_get(&mut self.inner, key, path)
            .map_err(|e| napi_ohos::Error::from_reason(format!("JSON.GET failed: {}", e)))
    }

    /// JSON.DEL command - Delete JSON value at path
    ///
    /// # Arguments
    /// * `key` - The key
    /// * `path` - JSONPath expression
    ///
    /// # Returns
    /// Number of paths deleted
    #[napi]
    pub fn json_del(&mut self, key: String, path: String) -> Result<i32> {
        JsonCommands::json_del(&mut self.inner, key, path)
            .map_err(|e| napi_ohos::Error::from_reason(format!("JSON.DEL failed: {}", e)))
    }

    /// JSON.TYPE command - Get the type of JSON value at path
    ///
    /// # Arguments
    /// * `key` - The key
    /// * `path` - JSONPath expression
    ///
    /// # Returns
    /// Type name: "object", "array", "string", "number", "boolean", "null"
    #[napi]
    pub fn json_type(&mut self, key: String, path: String) -> Result<String> {
        JsonCommands::json_type(&mut self.inner, key, path)
            .map_err(|e| napi_ohos::Error::from_reason(format!("JSON.TYPE failed: {}", e)))
    }

    // ==================== JSON Array Commands ====================

    /// JSON.ARRAPPEND command - Append values to JSON array
    ///
    /// # Arguments
    /// * `key` - The key
    /// * `path` - JSONPath to array
    /// * `json` - JSON value to append
    ///
    /// # Returns
    /// New length of the array
    #[napi]
    pub fn json_arr_append(&mut self, key: String, path: String, json: String) -> Result<i32> {
        let value: JsonValue = serde_json::from_str(&json)
            .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid JSON: {}", e)))?;

        JsonCommands::json_arr_append(&mut self.inner, key, path, &value)
            .map_err(|e| napi_ohos::Error::from_reason(format!("JSON.ARRAPPEND failed: {}", e)))
    }

    /// JSON.ARRINDEX command - Find index of JSON value in array
    ///
    /// # Arguments
    /// * `key` - The key
    /// * `path` - JSONPath to array
    /// * `json` - JSON value to search for
    ///
    /// # Returns
    /// Index of first occurrence, or -1 if not found
    #[napi]
    pub fn json_arr_index(&mut self, key: String, path: String, json: String) -> Result<i32> {
        let value: JsonValue = serde_json::from_str(&json)
            .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid JSON: {}", e)))?;

        JsonCommands::json_arr_index(&mut self.inner, key, path, &value)
            .map_err(|e| napi_ohos::Error::from_reason(format!("JSON.ARRINDEX failed: {}", e)))
    }

    /// JSON.ARRINSERT command - Insert values into JSON array
    ///
    /// # Arguments
    /// * `key` - The key
    /// * `path` - JSONPath to array
    /// * `index` - Index to insert at
    /// * `json` - JSON value to insert
    ///
    /// # Returns
    /// New length of the array
    #[napi]
    pub fn json_arr_insert(
        &mut self,
        key: String,
        path: String,
        index: i32,
        json: String,
    ) -> Result<i32> {
        let value: JsonValue = serde_json::from_str(&json)
            .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid JSON: {}", e)))?;

        JsonCommands::json_arr_insert(&mut self.inner, key, path, index as i64, &value)
            .map_err(|e| napi_ohos::Error::from_reason(format!("JSON.ARRINSERT failed: {}", e)))
    }

    /// JSON.ARRLEN command - Get length of JSON array
    ///
    /// # Arguments
    /// * `key` - The key
    /// * `path` - JSONPath to array
    ///
    /// # Returns
    /// Length of the array
    #[napi]
    pub fn json_arr_len(&mut self, key: String, path: String) -> Result<i32> {
        JsonCommands::json_arr_len(&mut self.inner, key, path)
            .map_err(|e| napi_ohos::Error::from_reason(format!("JSON.ARRLEN failed: {}", e)))
    }

    /// JSON.ARRPOP command - Remove and return element from array
    ///
    /// # Arguments
    /// * `key` - The key
    /// * `path` - JSONPath to array
    /// * `index` - Index to pop (default: -1 for last element)
    ///
    /// # Returns
    /// The popped JSON value as string
    #[napi]
    pub fn json_arr_pop(
        &mut self,
        key: String,
        path: String,
        index: Option<i32>,
    ) -> Result<String> {
        let idx = index.unwrap_or(-1) as i64;
        JsonCommands::json_arr_pop(&mut self.inner, key, path, idx)
            .map_err(|e| napi_ohos::Error::from_reason(format!("JSON.ARRPOP failed: {}", e)))
    }

    /// JSON.ARRTRIM command - Trim array to specified range
    ///
    /// # Arguments
    /// * `key` - The key
    /// * `path` - JSONPath to array
    /// * `start` - Start index (inclusive)
    /// * `stop` - Stop index (inclusive)
    ///
    /// # Returns
    /// New length of the array
    #[napi]
    pub fn json_arr_trim(
        &mut self,
        key: String,
        path: String,
        start: i32,
        stop: i32,
    ) -> Result<i32> {
        JsonCommands::json_arr_trim(&mut self.inner, key, path, start as i64, stop as i64)
            .map_err(|e| napi_ohos::Error::from_reason(format!("JSON.ARRTRIM failed: {}", e)))
    }

    // ==================== JSON Object Commands ====================

    /// JSON.OBJKEYS command - Get keys of JSON object
    ///
    /// # Arguments
    /// * `key` - The key
    /// * `path` - JSONPath to object
    ///
    /// # Returns
    /// Array of object keys as JSON string
    #[napi]
    pub fn json_obj_keys(&mut self, key: String, path: String) -> Result<Vec<String>> {
        JsonCommands::json_obj_keys(&mut self.inner, key, path)
            .map_err(|e| napi_ohos::Error::from_reason(format!("JSON.OBJKEYS failed: {}", e)))
    }

    /// JSON.OBJLEN command - Get number of keys in JSON object
    ///
    /// # Arguments
    /// * `key` - The key
    /// * `path` - JSONPath to object
    ///
    /// # Returns
    /// Number of keys in the object
    #[napi]
    pub fn json_obj_len(&mut self, key: String, path: String) -> Result<i32> {
        JsonCommands::json_obj_len(&mut self.inner, key, path)
            .map_err(|e| napi_ohos::Error::from_reason(format!("JSON.OBJLEN failed: {}", e)))
    }

    // ==================== JSON String Commands ====================

    /// JSON.STRAPPEND command - Append string to JSON string value
    ///
    /// # Arguments
    /// * `key` - The key
    /// * `path` - JSONPath to string
    /// * `value` - String to append (will be JSON-encoded)
    ///
    /// # Returns
    /// New length of the string
    #[napi]
    pub fn json_str_append(&mut self, key: String, path: String, value: String) -> Result<i32> {
        // JSON-encode the string value
        let json_str = serde_json::to_string(&value).map_err(|e| {
            Error::new(
                Status::InvalidArg,
                format!("Failed to encode string: {}", e),
            )
        })?;

        JsonCommands::json_str_append(&mut self.inner, key, path, &json_str)
            .map_err(|e| napi_ohos::Error::from_reason(format!("JSON.STRAPPEND failed: {}", e)))
    }

    /// JSON.STRLEN command - Get length of JSON string value
    ///
    /// # Arguments
    /// * `key` - The key
    /// * `path` - JSONPath to string
    ///
    /// # Returns
    /// Length of the string
    #[napi]
    pub fn json_str_len(&mut self, key: String, path: String) -> Result<i32> {
        JsonCommands::json_str_len(&mut self.inner, key, path)
            .map_err(|e| napi_ohos::Error::from_reason(format!("JSON.STRLEN failed: {}", e)))
    }

    // ==================== JSON Number Commands ====================

    /// JSON.NUMINCRBY command - Increment number value
    ///
    /// # Arguments
    /// * `key` - The key
    /// * `path` - JSONPath to number
    /// * `value` - Amount to increment by
    ///
    /// # Returns
    /// New value as JSON string
    #[napi]
    pub fn json_num_incr_by(&mut self, key: String, path: String, value: i64) -> Result<String> {
        JsonCommands::json_num_incr_by(&mut self.inner, key, path, value)
            .map_err(|e| napi_ohos::Error::from_reason(format!("JSON.NUMINCRBY failed: {}", e)))
    }
}
